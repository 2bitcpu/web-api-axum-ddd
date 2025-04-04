use crate::commons::types::{BoxError, DbExecutor};
use crate::models::entities::member::MemberEntity;
use async_trait::async_trait;

#[rustfmt::skip]
#[async_trait]
pub trait MemberRepository {
    async fn create(&self, executor: &mut DbExecutor, entity: MemberEntity) -> Result<MemberEntity, BoxError>;
    async fn find(&self, executor: &mut DbExecutor, account: &str) -> Result<Option<MemberEntity>, BoxError>;
    async fn update(&self, executor: &mut DbExecutor, entity: MemberEntity) -> Result<Option<MemberEntity>, BoxError>;
    async fn delete(&self, executor: &mut DbExecutor, account: &str) -> Result<u64, BoxError>;
}
