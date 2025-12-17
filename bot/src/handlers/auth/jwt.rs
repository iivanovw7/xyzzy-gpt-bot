use actix_web::{http::header, web, Error as ActixError, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use teloxide::types::UserId;
use tracing::{error, info};

use crate::types::auth::{AuthState, AuthTokens, Claims, RefreshClaims};

async fn save_refresh_token(state: AuthState, user_id: UserId, token: String) {
    if let Ok(mut map) = state.lock() {
        map.insert(user_id, token);
        info!("Refresh token saved to state for user: {}", user_id);
    } else {
        error!("Failed to lock AuthState for saving.");
    }
}

fn has_valid_token(state: AuthState, user_id: UserId, token: &str) -> bool {
    if let Ok(map) = state.lock() {
        if let Some(valid_token) = map.get(&user_id) {
            return valid_token == token;
        }
    } else {
        error!("Failed to lock AuthState for validation.");
    }
    false
}

fn delete_refresh_token(state: AuthState, user_id: UserId) {
    if let Ok(mut map) = state.lock() {
        map.remove(&user_id);
        info!("Refresh token deleted from state for user: {}", user_id);
    } else {
        error!("Failed to lock AuthState for deletion.");
    }
}

pub async fn create_tokens(
    user_id: UserId,
    secret_key: &[u8],
    state: AuthState,
) -> Result<AuthTokens, jsonwebtoken::errors::Error> {
    let access_claims = Claims::new_access(user_id);
    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(secret_key),
    )?;

    let refresh_claims = RefreshClaims::new_refresh(user_id);
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secret_key),
    )?;

    save_refresh_token(state, user_id, refresh_token.clone()).await;

    Ok(AuthTokens {
        access_token,
        refresh_token,
    })
}

async fn validate_refresh_token(
    token: &str,
    secret_key: &[u8],
    state: AuthState,
) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data =
        decode::<RefreshClaims>(token, &DecodingKey::from_secret(secret_key), &validation)?;

    if token_data.claims.auth != "refresh" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    let user_id_str = token_data.claims.sub.clone();
    let user_id = UserId(user_id_str.parse::<u64>().unwrap_or_default());

    if !has_valid_token(state, user_id, token) {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::ExpiredSignature,
        ));
    }

    tracing::info!("Refresh token validated via state: {}", user_id_str);

    Ok(user_id_str)
}

pub async fn refresh_access_token(
    user_id: UserId,
    refresh_token: String,
    secret_key: &[u8],
    state: AuthState,
) -> Result<AuthTokens, jsonwebtoken::errors::Error> {
    validate_refresh_token(&refresh_token, secret_key, state.clone()).await?;

    delete_refresh_token(state.clone(), user_id);

    let new_tokens = create_tokens(user_id, secret_key, state).await?;

    Ok(new_tokens)
}

pub fn authorize_request(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    auth_enabled: bool,
) -> Result<(String, String), ActixError> {
    if !auth_enabled {
        return Ok(("0".to_string(), "NO_TOKEN".to_string()));
    }

    let secret = jwt_secret.as_bytes();
    let token: String;

    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(token_str) = auth_header.to_str() {
            if let Some(stripped) = token_str.strip_prefix("Bearer ") {
                token = stripped.to_string();
            } else {
                return Err(actix_web::error::ErrorUnauthorized(
                    "Missing Bearer scheme in Authorization header",
                ));
            }
        } else {
            return Err(actix_web::error::ErrorUnauthorized(
                "Invalid Authorization header format",
            ));
        }
    } else if let Some(query_str) = req.uri().query() {
        if let Some(token_val) =
            web::Query::<std::collections::HashMap<String, String>>::from_query(query_str)
                .ok()
                .and_then(|q| q.get("token").cloned())
        {
            token = token_val;
        } else {
            return Err(actix_web::error::ErrorUnauthorized(
                "Missing Authorization header or URL token",
            ));
        }
    } else {
        return Err(actix_web::error::ErrorUnauthorized(
            "Missing Authorization header or URL token",
        ));
    }

    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(&token, &DecodingKey::from_secret(secret), &validation) {
        Ok(token_data) => {
            if token_data.claims.auth != "access" {
                return Err(actix_web::error::ErrorUnauthorized("Invalid token type"));
            }

            Ok((token_data.claims.sub, token))
        }
        Err(e) => {
            tracing::error!("Token Validation Failed: {:?}", e);

            Err(actix_web::error::ErrorUnauthorized(
                "Invalid or expired Access Token",
            ))
        }
    }
}
