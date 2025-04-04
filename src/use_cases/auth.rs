use crate::commons::config::{JWT_EXPIRATION_SECONDS, MAX_MISSMATCH_COUNT};
use crate::commons::types::{BoxError, DbPool};
use crate::models::dtos::auth::{SigninDto, SignupDto};
use crate::models::dtos::member::AuthMemberDto;
use crate::models::entities::auth::AuthEntity;
use crate::repositories::RepositoriesExt;
use crate::repositories::interfaces::{auth::AuthRepository, member::MemberRepository};
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct AuthUseCases<R: RepositoriesExt> {
    pool: DbPool,
    repositories: Arc<R>,
}

impl<R: RepositoriesExt> AuthUseCases<R> {
    pub async fn signup(&self, dto: SignupDto) -> Result<(), BoxError> {
        if dto.password.clone() != dto.confirm_password.clone() {
            return Err("password does not match".into());
        }

        let password = async_argon2::hash(dto.password.clone()).await?;
        let mut entity = dto.to_entity();
        entity.password = password;

        let mut executor = self.pool.begin().await?;

        self.repositories
            .member_repository()
            .create(&mut *executor, entity)
            .await?;

        executor.commit().await?;

        Ok(())
    }

    pub async fn signin(&self, dto: SigninDto) -> Result<String, BoxError> {
        let mut executor = self.pool.acquire().await?;

        let member = self
            .repositories
            .member_repository()
            .find(&mut *executor, &dto.account)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        let auth = self
            .repositories
            .auth_repository()
            .find(&mut *executor, &dto.account)
            .await?;

        if let Some(auth) = auth.clone() {
            if auth.missmatch >= *MAX_MISSMATCH_COUNT {
                return Err("account is locked".into());
            }
        }

        let mut executor = self.pool.begin().await?;

        if !async_argon2::verify(dto.password, member.password).await? {
            match auth {
                Some(mut auth) => {
                    auth.missmatched();
                    self.repositories
                        .auth_repository()
                        .update(&mut *executor, auth)
                        .await?;
                }
                None => {
                    self.repositories
                        .auth_repository()
                        .create(&mut *executor, AuthEntity::new_missmatched(dto.account))
                        .await?;
                }
            }
            executor.commit().await?;

            return Err("password does not match".into());
        }

        let claims = simple_jwt::Claims::new(&dto.account, *JWT_EXPIRATION_SECONDS);
        let token = simple_jwt::encode(&claims.clone())?;

        match auth {
            Some(mut auth) => {
                auth.signin(claims.clone());
                self.repositories
                    .auth_repository()
                    .update(&mut *executor, auth)
                    .await?;
            }
            None => {
                self.repositories
                    .auth_repository()
                    .create(&mut *executor, AuthEntity::new_signin(claims))
                    .await?;
            }
        }
        executor.commit().await?;

        Ok(token)
    }

    pub async fn authenticate(&self, token: &str) -> Result<AuthMemberDto, BoxError> {
        let claims = simple_jwt::decode(token)?;

        let mut executor = self.pool.acquire().await?;

        let auth = self
            .repositories
            .auth_repository()
            .find(&mut *executor, &claims.sub)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        if auth.jwt_id != Some(claims.jti)
            || auth.issued_tm != Some(claims.iat)
            || auth.expired_tm != Some(claims.exp)
        {
            return Err("invalid token".into());
        }

        let member = self
            .repositories
            .member_repository()
            .find(&mut *executor, &claims.sub)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        Ok(AuthMemberDto::from_entity(member, auth))
    }

    pub async fn signout(&self, account: &str) -> Result<(), BoxError> {
        let mut executor = self.pool.begin().await?;

        let mut entity = match self
            .repositories
            .auth_repository()
            .find(&mut *executor, &account)
            .await?
        {
            Some(entity) => entity,
            None => return Ok(()),
        };

        entity.signout();

        let entity = self
            .repositories
            .auth_repository()
            .update(&mut *executor, entity)
            .await?;

        if entity.is_some() {
            executor.commit().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::commons::setup;
    use crate::models::dtos::auth::{SigninDto, SignupDto};
    use crate::repositories::Repositories;
    use crate::use_cases::auth::AuthUseCases;

    use std::sync::Arc;

    #[tokio::test]
    async fn test_auth_use_cases() {
        let pool = setup::initialize_db("sqlite::memory:").await.unwrap();

        let repositories = Repositories::new();
        let use_cases = AuthUseCases::new(pool.clone(), Arc::new(repositories));

        let account = "account".to_string();
        let password = "password".to_string();

        let signup_dto = SignupDto {
            account: account.clone(),
            password: password.clone(),
            confirm_password: password.clone(),
            name: None,
            email: None,
        };

        let result = use_cases.signup(signup_dto.clone()).await;
        assert!(result.is_ok());

        let signin_dto = SigninDto {
            account: account.clone(),
            password: password.clone(),
        };

        let result = use_cases.signin(signin_dto.clone()).await;
        assert!(result.is_ok());

        let token = result.unwrap();

        let result = use_cases.authenticate(&token).await;
        assert!(result.is_ok());

        let auth_member_dto = result.unwrap();

        assert_eq!(auth_member_dto.account, account.clone());
    }

    #[tokio::test]
    async fn test_auth_use_cases_pwd_lock() {
        let pool = setup::initialize_db("sqlite::memory:").await.unwrap();

        let repositories = Repositories::new();
        let use_cases = AuthUseCases::new(pool.clone(), Arc::new(repositories));

        let account = "account".to_string();
        let password = "password".to_string();

        let signup_dto = SignupDto {
            account: account.clone(),
            password: password.clone(),
            confirm_password: password.clone(),
            name: None,
            email: None,
        };

        let result = use_cases.signup(signup_dto.clone()).await;
        assert!(result.is_ok());

        let miss_password = "missmatch".to_string();

        let signin_dto = SigninDto {
            account: account.clone(),
            password: miss_password.clone(),
        };

        let result = use_cases.signin(signin_dto.clone()).await;
        assert!(result.is_err());

        let result = use_cases.signin(signin_dto.clone()).await;
        assert!(result.is_err());

        let result = use_cases.signin(signin_dto.clone()).await;
        assert!(result.is_err());

        let signin_dto = SigninDto {
            account: account.clone(),
            password: password.clone(),
        };

        let result = use_cases.signin(signin_dto.clone()).await;
        assert!(result.is_err());
    }
}
