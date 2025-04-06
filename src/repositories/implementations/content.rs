use crate::commons::types::{BoxError, DbExecutor};
use crate::models::entities::content::ContentEntity;
use crate::repositories::interfaces::content::ContentRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct ContentRepositoryImpl;

impl ContentRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[rustfmt::skip]
#[async_trait]
impl ContentRepository for ContentRepositoryImpl {
    async fn create(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<ContentEntity, BoxError> {
        Ok(
            sqlx::query_as::<_, ContentEntity>(
                "INSERT INTO content (account, post_at, title, body) VALUES ($1, $2, $3, $4) RETURNING *",
            )
            .bind(&entity.account)
            .bind(&entity.post_at)
            .bind(&entity.title)
            .bind(&entity.body)
            .fetch_one(&mut *executor)
            .await?,
        )
    }

    async fn find(&self, executor: &mut DbExecutor, content_id: i64) -> Result<Option<ContentEntity>, BoxError> {
        Ok(
            sqlx::query_as::<_, ContentEntity>("SELECT * FROM content WHERE content_id = $1")
                .bind(content_id)
                .fetch_optional(&mut *executor)
                .await?,
        )
    }

    async fn update(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<Option<ContentEntity>, BoxError> {
        Ok(
            sqlx::query_as::<_, ContentEntity>(
                "UPDATE content SET account = $2, post_at = $3, title = $4, body = $5 WHERE content_id = $1 RETURNING *",
            )
            .bind(&entity.content_id)
            .bind(&entity.account)
            .bind(&entity.post_at)
            .bind(&entity.title)
            .bind(&entity.body)
            .fetch_optional(&mut *executor)
            .await?,
        )
    }

    async fn delete(&self, executor: &mut DbExecutor, content_id: i64) -> Result<u64, BoxError> {
        Ok(
            sqlx::query("DELETE FROM content WHERE content_id = $1")
                .bind(content_id)
                .execute(&mut *executor)
                .await?
                .rows_affected(),
        )
    }

    async fn list(&self, executor: &mut DbExecutor, title: Option<&str>, page: i32, size: i32) -> Result<Vec<ContentEntity>, BoxError> {
        Ok(
            match title {
                Some(title) => {
                    sqlx::query_as::<_, ContentEntity>("SELECT * FROM content WHERE title LIKE $1 || '%' ORDER BY post_at DESC LIMIT $2 OFFSET $3")
                        .bind(title)
                        .bind(size)
                        .bind((page - 1) * size)
                        .fetch_all(&mut *executor)
                        .await?
                }
                None => {
                    sqlx::query_as::<_, ContentEntity>("SELECT * FROM content ORDER BY post_at DESC LIMIT $1 OFFSET $2")
                        .bind(size)
                        .bind((page - 1) * size)
                        .fetch_all(&mut *executor)
                        .await?
                }
                
            }
        )
    }

    async fn count(&self, executor: &mut DbExecutor, title: Option<&str>) -> Result<i64, BoxError> {
        Ok(
            match title {
                Some(title) => {
                    sqlx::query_scalar("SELECT COUNT(*) FROM content WHERE title LIKE $1 || '%'")
                        .bind(title)
                        .fetch_one(&mut *executor)
                        .await?
                }
                None => {
                    sqlx::query_scalar("SELECT COUNT(*) FROM content")
                        .fetch_one(&mut *executor)
                        .await?
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commons;
    use chrono::Utc;

    #[tokio::test]
    async fn test_content_repository_create() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();
        let repository = ContentRepositoryImpl::new();

        let entity = ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = repository.create(&mut executor, entity).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.account, "test");
        assert_eq!(result.title, "test");
        assert_eq!(result.body, "test");
    }

    #[tokio::test]
    async fn test_content_repository_find() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();
        let repository = ContentRepositoryImpl::new();

        let entity = ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = repository.create(&mut executor, entity).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        let result = repository.find(&mut executor, result.content_id).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result.account, "test".to_string());
        assert_eq!(result.title, "test".to_string());
        assert_eq!(result.body, "test".to_string());
    }

    #[tokio::test]
    async fn test_content_repository_update() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();
        let repository = ContentRepositoryImpl::new();

        let entity = ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = repository.create(&mut executor, entity).await;

        assert!(result.is_ok());
        let mut entity = result.unwrap();

        entity.account = "test2".to_string();
        entity.title = "test2".to_string();
        entity.body = "test2".to_string();
        entity.post_at = Utc::now();

        let result = repository.update(&mut executor, entity.clone()).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result.account, "test2".to_string());
        assert_eq!(result.title, "test2".to_string());
        assert_eq!(result.body, "test2".to_string());

        entity.content_id = 999;

        let result = repository.update(&mut executor, entity).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_content_repository_delete() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();
        let repository = ContentRepositoryImpl::new();

        let entity = ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = repository.create(&mut executor, entity).await;

        assert!(result.is_ok());
        let entity = result.unwrap();

        let result = repository.delete(&mut executor, entity.content_id).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, 1);

        let result = repository.delete(&mut executor, 999).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_content_repository_list() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();
        let repository = ContentRepositoryImpl::new();

        let entity = ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        for _ in 0..10 {
            let result = repository.create(&mut *executor, entity.clone()).await;
            assert!(result.is_ok());
        }

        let entity = ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "title".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        for _ in 0..10 {
            let result = repository.create(&mut *executor, entity.clone()).await;
            assert!(result.is_ok());
        }
        executor.commit().await.unwrap();

        let mut executor = pool.acquire().await.unwrap();

        let result = repository.count(&mut *executor, None).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 20);

        let result = repository.count(&mut *executor, Some("title")).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, 10);

        let result = repository.list(&mut *executor, Some("tit"), 1, 10).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 10);

        let result = repository.list(&mut *executor, None, 2, 10).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 10);

        let result = repository.list(&mut *executor, None, 3, 10).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 0);
    }
}
