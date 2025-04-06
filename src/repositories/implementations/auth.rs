use crate::commons::types::{BoxError, DbExecutor};
use crate::models::entities::auth::AuthEntity;
use crate::repositories::interfaces::auth::AuthRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct AuthRepositoryImpl;

impl AuthRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[rustfmt::skip]
#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn create(&self, executor: &mut DbExecutor, entity: AuthEntity) -> Result<AuthEntity, BoxError> {
        Ok(
            sqlx::query_as::<_, AuthEntity>(
                "INSERT INTO auth (account, issued_tm, expired_tm, jwt_id, missmatch, login_at, prev_login_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            )
            .bind(&entity.account)
            .bind(&entity.issued_tm)
            .bind(&entity.expired_tm)
            .bind(&entity.jwt_id)
            .bind(&entity.missmatch)
            .bind(&entity.login_at)
            .bind(&entity.prev_login_at)
            .fetch_one(&mut *executor)
            .await?,
        )
    }

    async fn find(&self, executor: &mut DbExecutor, account: &str) -> Result<Option<AuthEntity>, BoxError> {
        Ok(
            sqlx::query_as::<_, AuthEntity>("SELECT * FROM auth WHERE account = $1")
                .bind(account)
                .fetch_optional(&mut *executor)
                .await?,
        )
    }

    async fn update(&self, executor: &mut DbExecutor, entity: AuthEntity) -> Result<Option<AuthEntity>, BoxError> {
        Ok(
            sqlx::query_as::<_, AuthEntity>(
                "UPDATE auth SET issued_tm = $2, expired_tm = $3, jwt_id = $4, missmatch = $5, login_at = $6, prev_login_at = $7 WHERE account = $1 RETURNING *",
            )
            .bind(&entity.account)
            .bind(&entity.issued_tm)
            .bind(&entity.expired_tm)
            .bind(&entity.jwt_id)
            .bind(&entity.missmatch)
            .bind(&entity.login_at)
            .bind(&entity.prev_login_at)
            .fetch_optional(&mut *executor)
            .await?,
        )
    }

    async fn delete(&self, executor: &mut DbExecutor, account: &str) -> Result<u64, BoxError> {
        Ok(
            sqlx::query("DELETE FROM auth WHERE account = $1")
                .bind(account)
                .execute(&mut *executor)
                .await?
                .rows_affected(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commons;
    use chrono::Utc;

    #[tokio::test]
    async fn test_auth_repository_create() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = AuthRepositoryImpl::new();

        let account = "account".to_string();
        let entity = AuthEntity {
            account: account.clone(),
            issued_tm: Some(1),
            expired_tm: Some(2),
            jwt_id: Some("jwt_id".to_string()),
            missmatch: 0,
            challenge_at: None,
            login_at: Some(Utc::now()),
            prev_login_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        println!("result: {:#?}", result);
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.account.clone(), account.clone());

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_auth_repository_find() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = AuthRepositoryImpl::new();

        let account = "account".to_string();
        let entity = AuthEntity {
            account: account.clone(),
            issued_tm: Some(1),
            expired_tm: Some(2),
            jwt_id: Some("jwt_id".to_string()),
            missmatch: 0,
            challenge_at: None,
            login_at: Some(Utc::now()),
            prev_login_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = repository.find(&mut *executor, &account).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.account.clone(), account.clone());

        let result = repository.find(&mut *executor, "unknown").await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_auth_repository_update() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = AuthRepositoryImpl::new();

        let account = "account".to_string();
        let entity = AuthEntity {
            account: account.clone(),
            issued_tm: Some(1),
            expired_tm: Some(2),
            jwt_id: Some("jwt_id".to_string()),
            missmatch: 0,
            challenge_at: None,
            login_at: Some(Utc::now()),
            prev_login_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let mut entity = result.unwrap();
        entity.missmatch = 9;

        let result = repository.update(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let result = result.unwrap();
        assert_eq!(result.missmatch, 9);

        entity.account = "unknown".to_string();

        let result = repository.update(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_auth_repository_delete() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = AuthRepositoryImpl::new();

        let account = "account".to_string();
        let entity = AuthEntity {
            account: account.clone(),
            issued_tm: Some(1),
            expired_tm: Some(2),
            jwt_id: Some("jwt_id".to_string()),
            missmatch: 0,
            challenge_at: None,
            login_at: Some(Utc::now()),
            prev_login_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = repository.delete(&mut *executor, &account).await;
        assert!(result.is_ok());
        let count = result.unwrap();

        assert_eq!(count, 1);

        let result = repository.delete(&mut *executor, &account).await;
        assert!(result.is_ok());
        let count = result.unwrap();

        assert_eq!(count, 0);
    }
}
