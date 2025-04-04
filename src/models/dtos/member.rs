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
