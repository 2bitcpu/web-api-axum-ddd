use crate::commons::types::{BoxError, DbExecutor};
use crate::models::entities::auth::AuthEntity;
use async_trait::async_trait;

#[rustfmt::skip]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create(&self, executor: &mut DbExecutor, entity: AuthEntity) -> Result<AuthEntity, BoxError>;
    async fn find(&self, executor: &mut DbExecutor, account: &str) -> Result<Option<AuthEntity>, BoxError>;
    async fn update(&self, executor: &mut DbExecutor, entity: AuthEntity) -> Result<Option<AuthEntity>, BoxError>;
    async fn delete(&self, executor: &mut DbExecutor, account: &str) -> Result<u64, BoxError>;
}
