use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::web;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

pub type AuthState = web::Data<Arc<Mutex<HashMap<UserId, String>>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub auth: String,
}

impl Claims {
    pub fn new_access(user_id: UserId) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::minutes(15)).timestamp();

        Claims {
            sub: user_id.to_string(),
            exp,
            iat,
            auth: "access".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub auth: String,
}

impl RefreshClaims {
    pub fn new_refresh(user_id: UserId) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::days(7)).timestamp();

        RefreshClaims {
            sub: user_id.to_string(),
            exp,
            iat,
            auth: "refresh".to_string(),
        }
    }
}

pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}
