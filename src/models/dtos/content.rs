use crate::models::entities::content::ContentEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContentDto {
    pub content_id: i64,
    pub account: String,
    pub post_at: DateTime<Utc>,
    pub title: String,
    pub body: String,
}

impl ContentDto {
    pub fn from_entity(content: ContentEntity) -> Self {
        Self {
            content_id: content.content_id,
            account: content.account,
            post_at: content.post_at,
            title: content.title,
            body: content.body,
        }
    }

    pub fn to_entity(&self) -> ContentEntity {
        ContentEntity {
            content_id: self.content_id,
            account: self.account.clone(),
            post_at: self.post_at,
            title: self.title.clone(),
            body: self.body.clone(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_dto() {
        let content = ContentEntity {
            content_id: 1,
            account: "test".to_string(),
            post_at: Utc::now(),
            title: "test".to_string(),
            body: "test".to_string(),
            created_at: None,
            updated_at: None,
        };
        let dto = ContentDto::from_entity(content.clone());
        let entity = dto.to_entity();
        assert_eq!(entity, content);
    }
}
