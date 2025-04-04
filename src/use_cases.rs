pub mod auth;
pub mod content;

use crate::commons::types::DbPool;
use crate::repositories::{Repositories, RepositoriesExt};
use crate::use_cases::{auth::AuthUseCases, content::ContentUseCases};
use std::sync::Arc;

#[derive(Clone)]
pub struct Modules {
    pub auth: AuthUseCases<Repositories>,
    pub content: ContentUseCases<Repositories>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesExt;

    fn auth(&self) -> &AuthUseCases<Self::RepositoriesModule>;
    fn content(&self) -> &ContentUseCases<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = Repositories;

    fn auth(&self) -> &AuthUseCases<Self::RepositoriesModule> {
        &self.auth
    }

    fn content(&self) -> &ContentUseCases<Self::RepositoriesModule> {
        &self.content
    }
}

impl Modules {
    pub fn new(pool: DbPool) -> Self {
        let repositories = Arc::new(Repositories::new());

        let auth = AuthUseCases::new(pool.clone(), repositories.clone());
        let content = ContentUseCases::new(pool, repositories);

        Self { auth, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commons::setup;
    use crate::models::dtos::auth::{SigninDto, SignupDto};
    use crate::models::dtos::content::ContentDto;

    #[tokio::test]
    async fn test_modules() {
        let pool = setup::initialize_db("sqlite::memory:").await.unwrap();
        let modules = Modules::new(pool);

        let accunt = "account".to_string();
        let password = "password".to_string();

        let dto = SignupDto {
            account: accunt.clone(),
            password: password.clone(),
            confirm_password: password.clone(),
            name: None,
            email: None,
        };

        let result = modules.auth().signup(dto).await;
        assert!(result.is_ok());

        let dto = SigninDto {
            account: accunt.clone(),
            password: password.clone(),
        };

        let result = modules.auth().signin(dto).await;
        assert!(result.is_ok());

        let token = result.unwrap();

        let result = modules.auth().authenticate(&token).await;
        assert!(result.is_ok());

        let dto = result.unwrap();
        assert_eq!(accunt.clone(), dto.account.clone());

        let dto = ContentDto {
            content_id: 0,
            account: accunt.clone(),
            post_at: chrono::Utc::now(),
            title: "title".to_string(),
            body: "body".to_string(),
        };

        let result = modules.content().post(dto.clone()).await;
        assert!(result.is_ok());

        let dto = result.unwrap();
        assert_eq!(dto.account.clone(), accunt.clone());
        assert_eq!(dto.title.clone(), "title".to_string());
        assert_eq!(dto.body.clone(), "body".to_string());

        let result = modules.content().get(dto.content_id.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let mut dto = result.unwrap();
        dto.title = "title2".to_string();
        dto.body = "body2".to_string();

        let result = modules.content().edit(dto.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.title.clone(), "title2".to_string());
        assert_eq!(result.body.clone(), "body2".to_string());

        let result = modules.content().remove(dto.content_id.clone()).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, 1);
    }
}
