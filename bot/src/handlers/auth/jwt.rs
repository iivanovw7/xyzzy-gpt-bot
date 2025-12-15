use actix_web::{web, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

pub fn extract_token_from_request(req: &HttpRequest) -> Option<String> {
    let token_from_query = req
        .query_string()
        .split('&')
        .find(|s| s.starts_with("token="))
        .and_then(|s| s.split('=').nth(1))
        .map(|s| s.to_string());

    if token_from_query.is_some() {
        return token_from_query;
    }

    req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.trim().to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::minutes(30)).timestamp();

        Claims {
            sub: user_id.to_string(),
            exp,
            iat,
        }
    }
}

pub fn create_jwt(
    user_id: UserId,
    secret_key: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
}

pub fn validate_jwt(token: &str, secret_key: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret_key), &validation)?;

    tracing::info!("jwt validated: {}", token_data.claims.sub);

    Ok(token_data.claims.sub)
}

pub fn authorize_request(
    req: HttpRequest,
    secret_key: web::Data<String>,
    auth_enabled: bool,
) -> Result<(String, String), actix_web::Error> {
    if !auth_enabled {
        let dummy_user_id = "999".to_string();

        tracing::warn!(
            "JWT authentication bypassed (Auth Mode OFF). Using dummy User ID: {}",
            dummy_user_id
        );

        return Ok((dummy_user_id, "999".to_string()));
    }

    let token = match extract_token_from_request(&req) {
        Some(t) => t,
        None => {
            return Err(actix_web::error::ErrorUnauthorized(
                "Missing authorization token",
            ))
        }
    };

    match validate_jwt(&token, secret_key.as_bytes()) {
        Ok(user_id) => Ok((user_id, token)),
        Err(e) => {
            tracing::error!("Token validation failed: {:?}", e);
            Err(actix_web::error::ErrorUnauthorized(format!(
                "Invalid or expired token: {}",
                e
            )))
        }
    }
}
