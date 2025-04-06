pub mod implementations;
pub mod interfaces;

use crate::repositories::implementations::{
    auth::AuthRepositoryImpl, content::ContentRepositoryImpl, member::MemberRepositoryImpl,
};
use crate::repositories::interfaces::{
    auth::AuthRepository, content::ContentRepository, member::MemberRepository,
};

#[derive(Clone)]
pub struct Repositories {
    pub auth_repository: AuthRepositoryImpl,
    pub content_repository: ContentRepositoryImpl,
    pub member_repository: MemberRepositoryImpl,
}

pub trait RepositoriesExt {
    type AuthRepository: AuthRepository;
    type ContentRepository: ContentRepository;
    type MemberRepository: MemberRepository;

    fn auth_repository(&self) -> &Self::AuthRepository;
    fn content_repository(&self) -> &Self::ContentRepository;
    fn member_repository(&self) -> &Self::MemberRepository;
}

impl RepositoriesExt for Repositories {
    type AuthRepository = AuthRepositoryImpl;
    type ContentRepository = ContentRepositoryImpl;
    type MemberRepository = MemberRepositoryImpl;

    fn auth_repository(&self) -> &Self::AuthRepository {
        &self.auth_repository
    }
    fn content_repository(&self) -> &Self::ContentRepository {
        &self.content_repository
    }
    fn member_repository(&self) -> &Self::MemberRepository {
        &self.member_repository
    }
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            auth_repository: AuthRepositoryImpl::new(),
            content_repository: ContentRepositoryImpl::new(),
            member_repository: MemberRepositoryImpl::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commons;
    use crate::models;

    use super::*;

    #[tokio::test]
    async fn test_repositories_member() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();

        let repository = Repositories::new();

        let entity = models::entities::member::MemberEntity {
            account: "test".to_string(),
            password: "test".to_string(),
            name: None,
            email: None,
            created_at: None,
            updated_at: None,
        };

        let result = repository
            .member_repository()
            .create(&mut *executor, entity.clone())
            .await;
        assert!(result.is_ok());

        let mut entity = result.unwrap();
        entity.name = Some("name".to_string());
        entity.email = Some("email".to_string());

        let result = repository
            .member_repository()
            .update(&mut *executor, entity.clone())
            .await;
        assert!(result.is_ok());

        let result = repository
            .member_repository()
            .find(&mut *executor, &entity.account.clone())
            .await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let result = repository
            .member_repository()
            .delete(&mut *executor, &entity.account.clone())
            .await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_repositories_auth() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();

        let repository = Repositories::new();

        let entity = models::entities::auth::AuthEntity {
            account: "test".to_string(),
            issued_tm: Some(1),
            expired_tm: Some(2),
            jwt_id: Some("jwt_id".to_string()),
            missmatch: 1,
            challenge_at: Some(chrono::Utc::now()),
            login_at: Some(chrono::Utc::now()),
            prev_login_at: None,
        };

        let result = repository
            .auth_repository()
            .create(&mut *executor, entity.clone())
            .await;
        assert!(result.is_ok());

        let mut entity = result.unwrap();
        entity.missmatch = 2;

        let result = repository
            .auth_repository()
            .update(&mut *executor, entity.clone())
            .await;
        assert!(result.is_ok());

        let result = repository
            .auth_repository()
            .find(&mut *executor, &entity.account)
            .await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let result = repository
            .auth_repository()
            .delete(&mut *executor, &entity.account)
            .await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_repositories_content() {
        let pool = commons::setup::initialize_db("sqlite::memory:")
            .await
            .unwrap();
        let mut executor = pool.begin().await.unwrap();

        let repository = Repositories::new();

        let entity = models::entities::content::ContentEntity {
            content_id: 0,
            account: "test".to_string(),
            post_at: chrono::Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = repository
            .content_repository()
            .create(&mut *executor, entity.clone())
            .await;
        assert!(result.is_ok());

        let mut entity = result.unwrap();
        entity.title = "test2".to_string();
        entity.body = "test2".to_string();

        let result = repository
            .content_repository()
            .update(&mut *executor, entity.clone())
            .await;
        assert!(result.is_ok());

        let result = repository
            .content_repository()
            .find(&mut *executor, entity.content_id)
            .await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let result = repository
            .content_repository()
            .delete(&mut *executor, entity.content_id)
            .await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 1);
    }
}
