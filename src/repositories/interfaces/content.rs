use crate::commons::types::{BoxError, DbExecutor};
use crate::models::entities::content::ContentEntity;
use async_trait::async_trait;

#[rustfmt::skip]
#[async_trait]
pub trait ContentRepository {
    async fn create(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<ContentEntity, BoxError>;
    async fn find(&self, executor: &mut DbExecutor, content_id: i64) -> Result<Option<ContentEntity>, BoxError>;
    async fn update(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<Option<ContentEntity>, BoxError>;
    async fn delete(&self, executor: &mut DbExecutor, content_id: i64) -> Result<u64, BoxError>;
}
