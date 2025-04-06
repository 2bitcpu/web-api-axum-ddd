use crate::models::entities::{auth::AuthEntity, member::MemberEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthMemberDto {
    pub account: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub login_at: Option<DateTime<Utc>>,
    pub prev_login_at: Option<DateTime<Utc>>,
}

impl AuthMemberDto {
    pub fn from_entity(member: MemberEntity, auth: AuthEntity) -> Self {
        Self {
            account: member.account,
            name: member.name,
            email: member.email,
            login_at: auth.login_at,
            prev_login_at: auth.prev_login_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionAuthMemberDto {
    pub auth_member: Option<AuthMemberDto>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::entities::auth::AuthEntity;
    use crate::models::entities::member::MemberEntity;
    use chrono::Utc;

    #[test]
    fn test_auth_member_dto() {
        let member = MemberEntity {
            account: "test".to_string(),
            password: "test".to_string(),
            name: Some("test".to_string()),
            email: Some("test".to_string()),
            created_at: None,
            updated_at: None,
        };
        let auth = AuthEntity {
            account: "test".to_string(),
            issued_tm: Some(1),
            expired_tm: Some(2),
            jwt_id: Some("jwt_id".to_string()),
            missmatch: 0,
            challenge_at: None,
            login_at: Some(Utc::now()),
            prev_login_at: None,
        };
        let dto = AuthMemberDto::from_entity(member.clone(), auth.clone());
        assert_eq!(dto.account, member.account);
        assert_eq!(dto.name, member.name);
        assert_eq!(dto.email, member.email);
        assert_eq!(dto.login_at, auth.login_at);
        assert_eq!(dto.prev_login_at, auth.prev_login_at);
    }
}
