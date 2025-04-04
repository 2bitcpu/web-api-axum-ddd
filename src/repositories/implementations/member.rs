use crate::commons::types::{BoxError, DbExecutor};
use crate::models::entities::member::MemberEntity;
use crate::repositories::interfaces::member::MemberRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct MemberRepositoryImpl;

impl MemberRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[rustfmt::skip]
#[async_trait]
impl MemberRepository for MemberRepositoryImpl {
    async fn create(&self, executor: &mut DbExecutor, entity: MemberEntity) -> Result<MemberEntity, BoxError> {
        Ok(
            sqlx::query_as::<_, MemberEntity>(
                "INSERT INTO member (account, password, name, email) VALUES ($1, $2, $3, $4) RETURNING *",
            )
            .bind(&entity.account)
            .bind(&entity.password)
            .bind(&entity.name)
            .bind(&entity.email)
            .fetch_one(&mut *executor)
            .await?,
        )
    }

    async fn find(&self, executor: &mut DbExecutor, account: &str) -> Result<Option<MemberEntity>, BoxError> {
        Ok(
            sqlx::query_as::<_, MemberEntity>("SELECT * FROM member WHERE account = $1")
                .bind(account)
                .fetch_optional(&mut *executor)
                .await?,
        )
    }

    async fn update(&self, executor: &mut DbExecutor, entity: MemberEntity) -> Result<Option<MemberEntity>, BoxError> {
        Ok(
            sqlx::query_as::<_, MemberEntity>(
                "UPDATE member SET password = $2, name = $3, email = $4, updated_at = CURRENT_TIMESTAMP WHERE account = $1 RETURNING *",
            )
            .bind(&entity.account)
            .bind(&entity.password)
            .bind(&entity.name)
            .bind(&entity.email)
            .fetch_optional(&mut *executor)
            .await?,
        )
    }

    async fn delete(&self, executor: &mut DbExecutor, account: &str) -> Result<u64, BoxError> {
        Ok(
            sqlx::query("DELETE FROM member WHERE account = $1")
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

    #[tokio::test]
    async fn test_member_repository_create() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = MemberRepositoryImpl::new();

        let account = "account".to_string();
        let password = "password".to_string();

        let entity = MemberEntity {
            account: account.clone(),
            password: password.clone(),
            name: None,
            email: None,
            created_at: None,
            updated_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.account.clone(), account.clone());
        assert_eq!(result.password.clone(), password.clone());
    }

    #[tokio::test]
    async fn test_member_repository_find() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = MemberRepositoryImpl::new();

        let account = "account".to_string();
        let password = "password".to_string();

        let entity = MemberEntity {
            account: account.clone(),
            password: password.clone(),
            name: None,
            email: None,
            created_at: None,
            updated_at: None,
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
        assert_eq!(result.password.clone(), password.clone());

        let result = repository.find(&mut *executor, "account2").await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_member_repository_updae() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = MemberRepositoryImpl::new();

        let account = "account".to_string();
        let password = "password".to_string();

        let entity = MemberEntity {
            account: account.clone(),
            password: password.clone(),
            name: None,
            email: None,
            created_at: None,
            updated_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let mut entity = result.unwrap();
        entity.name = Some("name".to_string());
        entity.email = Some("email".to_string());

        let result = repository.update(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result.account.clone(), account.clone());
        assert_eq!(result.password.clone(), password.clone());
        assert_eq!(result.name.clone(), Some("name".to_string()));
        assert_eq!(result.email.clone(), Some("email".to_string()));

        entity.account = "account2".to_string();

        let result = repository.update(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_member_repository_delete() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();

        let repository = MemberRepositoryImpl::new();

        let account = "account".to_string();
        let password = "password".to_string();

        let entity = MemberEntity {
            account: account.clone(),
            password: password.clone(),
            name: None,
            email: None,
            created_at: None,
            updated_at: None,
        };

        let mut executor = pool.begin().await.unwrap();

        let result = repository.create(&mut *executor, entity.clone()).await;
        assert!(result.is_ok());

        let result = repository.delete(&mut *executor, &account).await;
        assert!(result.is_ok());
        let count = result.unwrap();

        assert_eq!(count, 1);

        let result = repository.delete(&mut *executor, "account2").await;
        assert!(result.is_ok());
        let count = result.unwrap();

        assert_eq!(count, 0);
    }
}
