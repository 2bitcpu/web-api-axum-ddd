use crate::models::entities::member::MemberEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignupDto {
    pub account: String,
    pub password: String,
    pub confirm_password: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl SignupDto {
    pub fn to_entity(&self) -> MemberEntity {
        MemberEntity {
            account: self.account.clone(),
            password: self.password.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SigninDto {
    pub account: String,
    pub password: String,
}
