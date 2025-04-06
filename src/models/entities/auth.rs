use crate::commons::config::{LOCK_HOUR, MAX_MISSMATCH_COUNT};
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
    pub challenge_at: Option<DateTime<Utc>>,
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
            challenge_at: Some(Utc::now()),
            login_at: None,
            prev_login_at: None,
        }
    }

    pub fn missmatched(&mut self) {
        self.signout();
        self.missmatch += 1;
        self.challenge_at = Some(Utc::now());
    }

    pub fn new_signin(claims: Claims) -> Self {
        Self {
            account: claims.sub,
            issued_tm: Some(claims.iat),
            expired_tm: Some(claims.exp),
            jwt_id: Some(claims.jti),
            missmatch: 0,
            challenge_at: None,
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
        self.challenge_at = None;
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

    pub fn is_locked(&self) -> bool {
        if self.missmatch < *MAX_MISSMATCH_COUNT {
            return false;
        }
        if self.challenge_at.is_none() {
            return true;
        }
        let challenge_at = self.challenge_at.unwrap();
        let unlock_at = challenge_at + chrono::Duration::hours(*LOCK_HOUR * self.missmatch as i64);

        println!("{:?} < {:?}", Utc::now(), unlock_at);

        Utc::now() < unlock_at
    }

    pub fn is_signin(&self, claims: Claims) -> bool {
        self.jwt_id == Some(claims.jti)
            && self.issued_tm == Some(claims.iat)
            && self.expired_tm == Some(claims.exp)
            && self.missmatch == 0
            && self.challenge_at.is_none()
            && !self.is_timeout()
    }

    pub fn signout(&mut self) {
        self.jwt_id = None;
        self.issued_tm = None;
        self.expired_tm = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::commons;

    use super::*;

    #[test]
    fn test_auth_missmatched() {
        let mut auth = AuthEntity::new_missmatched("tester".to_string());
        assert_eq!(auth.missmatch, 1);
        assert!(auth.challenge_at.is_some());

        auth.missmatched();
        assert_eq!(auth.missmatch, 2);
        assert!(auth.challenge_at.is_some());
    }

    #[test]
    fn test_auth_signin() {
        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        let mut auth = AuthEntity::new_signin(claim);
        assert!(auth.jwt_id.is_some());
        assert!(auth.issued_tm.is_some());
        assert!(auth.expired_tm.is_some());
        assert_eq!(auth.missmatch, 0);
        assert!(auth.challenge_at.is_none());
        assert!(auth.login_at.is_some());
        assert!(auth.prev_login_at.is_none());

        auth.missmatched();

        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        auth.signin(claim);
        assert!(auth.jwt_id.is_some());
        assert!(auth.issued_tm.is_some());
        assert!(auth.expired_tm.is_some());
        assert_eq!(auth.missmatch, 0);
        assert!(auth.challenge_at.is_none());
        assert!(auth.login_at.is_some());
        assert!(auth.prev_login_at.is_some());
    }

    #[test]
    fn test_auth_is_timeout() {
        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        let mut auth = AuthEntity::new_signin(claim);
        assert!(!auth.is_timeout());

        auth.expired_tm = Some(Utc::now().timestamp() - 1);
        assert!(auth.is_timeout());
    }

    #[test]
    fn test_auth_signout() {
        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        let mut auth = AuthEntity::new_signin(claim);
        assert!(auth.jwt_id.is_some());
        assert!(auth.issued_tm.is_some());
        assert!(auth.expired_tm.is_some());
        assert_eq!(auth.missmatch, 0);
        assert!(auth.challenge_at.is_none());
        assert!(auth.login_at.is_some());
        assert!(auth.prev_login_at.is_none());

        auth.signout();
        assert!(auth.jwt_id.is_none());
        assert!(auth.issued_tm.is_none());
        assert!(auth.expired_tm.is_none());
    }

    #[test]
    fn test_auth_is_locked() {
        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        let mut auth = AuthEntity::new_signin(claim);
        assert!(!auth.is_locked());

        auth.missmatched();
        assert!(!auth.is_locked());
        auth.missmatched();
        assert!(!auth.is_locked());
        auth.missmatched();
        println!("{:?}", auth);
        assert!(auth.is_locked());
        auth.missmatched();
        assert!(auth.is_locked());
    }

    #[test]
    fn test_autn_is_not_locked() {
        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        let mut auth = AuthEntity::new_signin(claim);
        assert!(!auth.is_locked());

        auth.missmatched();
        auth.missmatched();
        auth.missmatched();
        assert!(auth.is_locked());

        let challenge_at = auth.challenge_at.unwrap();
        let unlock_at = challenge_at - chrono::Duration::hours(*LOCK_HOUR * auth.missmatch as i64);
        auth.challenge_at = Some(unlock_at);
        assert!(!auth.is_locked());
    }

    #[test]
    fn test_auth_is_signin() {
        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        let mut auth = AuthEntity::new_signin(claim.clone());
        assert!(auth.is_signin(claim));

        let claim = simple_jwt::Claims::new("tester", *commons::config::JWT_EXPIRATION_SECONDS);
        auth.signin(claim.clone());
        assert!(auth.is_signin(claim.clone()));

        auth.signout();
        assert!(!auth.is_signin(claim));
    }
}
