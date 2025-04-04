use crate::commons::types::{BoxError, DbPool};
use crate::models::dtos::content::ContentDto;
use crate::repositories::RepositoriesExt;
use crate::repositories::interfaces::content::ContentRepository;
use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct ContentUseCases<R: RepositoriesExt> {
    pool: DbPool,
    repositories: Arc<R>,
}

impl<R: RepositoriesExt> ContentUseCases<R> {
    pub async fn post(&self, dto: ContentDto) -> Result<ContentDto, BoxError> {
        let mut executor = self.pool.begin().await?;

        let content = self
            .repositories
            .content_repository()
            .create(&mut *executor, dto.to_entity())
            .await?;

        executor.commit().await?;

        Ok(ContentDto::from_entity(content))
    }

    pub async fn get(&self, content_id: i64) -> Result<Option<ContentDto>, BoxError> {
        let mut executor = self.pool.acquire().await?;

        let content = self
            .repositories
            .content_repository()
            .find(&mut *executor, content_id)
            .await?;

        Ok(match content {
            Some(content) => Some(ContentDto::from_entity(content)),
            None => None,
        })
    }

    pub async fn edit(&self, dto: ContentDto) -> Result<ContentDto, BoxError> {
        let mut executor = self.pool.begin().await?;

        let content = self
            .repositories
            .content_repository()
            .update(&mut *executor, dto.to_entity())
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        executor.commit().await?;

        Ok(ContentDto::from_entity(content))
    }

    pub async fn remove(&self, content_id: i64) -> Result<u64, BoxError> {
        let mut executor = self.pool.begin().await?;

        let count = self
            .repositories
            .content_repository()
            .delete(&mut *executor, content_id)
            .await?;

        if count == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        executor.commit().await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use crate::commons::setup;
    use crate::models::dtos::content::ContentDto;
    use crate::repositories::Repositories;
    use crate::use_cases::content::ContentUseCases;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_content_use_cases() {
        let pool = setup::initialize_db("sqlite::memory:").await.unwrap();

        let repositories = Repositories::new();
        let use_cases = ContentUseCases::new(pool.clone(), Arc::new(repositories));

        let dto = ContentDto {
            content_id: 0,
            account: "account".to_string(),
            post_at: chrono::Utc::now(),
            title: "title".to_string(),
            body: "body".to_string(),
        };

        let result = use_cases.post(dto.clone()).await;
        assert!(result.is_ok());

        let dto = result.unwrap();

        let result = use_cases.get(dto.content_id.clone()).await;
        assert!(result.is_ok());

        let mut dto = result.unwrap().unwrap();
        dto.title = "title2".to_string();
        dto.body = "body2".to_string();

        let result = use_cases.edit(dto.clone()).await;
        assert!(result.is_ok());

        let result = use_cases.remove(dto.content_id.clone()).await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 1);
    }
}
