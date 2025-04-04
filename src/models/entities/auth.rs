use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use simple_jwt::Claims;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct AuthEntity {
    pub account: String,
    pub issued_tm: Option<i64>,
    pub expired_tm: Option<i64>,
    pub jwt_id: Option<String>,
    pub missmatch: i32,
    pub login_at: Option<DateTime<Utc>>,
    pub prev_login_at: Option<DateTime<Utc>>,
}

impl AuthEntity {
    pub fn new_missmatched(account: String) -> Self {
        Self {
            account,
            issued_tm: None,
            expired_tm: None,
            jwt_id: None,
            missmatch: 1,
            login_at: None,
            prev_login_at: None,
        }
    }

    pub fn missmatched(&mut self) {
        self.jwt_id = None;
        self.missmatch += 1;
    }

    pub fn new_signin(claims: Claims) -> Self {
        Self {
            account: claims.sub,
            issued_tm: Some(claims.iat),
            expired_tm: Some(claims.exp),
            jwt_id: Some(claims.jti),
            missmatch: 0,
            login_at: Some(Utc::now()),
            prev_login_at: None,
        }
    }

    pub fn signin(&mut self, claims: Claims) {
        self.jwt_id = Some(claims.jti.clone());
        self.issued_tm = Some(claims.iat.clone());
        self.expired_tm = Some(claims.exp.clone());
        self.jwt_id = Some(claims.jti.clone());
        self.missmatch = 0;
        self.prev_login_at = self.login_at;
        self.login_at = Some(Utc::now());
    }

    pub fn is_timeout(&self) -> bool {
        if let Some(expired_tm) = self.expired_tm {
            Utc::now().timestamp() > expired_tm
        } else {
            true
        }
    }

    pub fn signout(&mut self) {
        self.jwt_id = None;
        self.issued_tm = None;
        self.expired_tm = None;
    }
}
