use crate::env::ENV;
use crate::handlers::auth;
use crate::types::auth::{AuthState, RefreshClaims};
use crate::types::common::AppError;
use crate::{config::Config, env::Env};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use hmac::{Hmac, Mac};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use sha2::Sha256;
use shared::{LoginPayload, LoginResponse};
use std::sync::Arc;
use teloxide::types::UserId;
use tracing::info;
use url::form_urlencoded;

pub fn validate_telegram_init_data(
    init_data: &str,
    bot_token: &str,
) -> Result<(u64, Option<String>), Box<AppError>> {
    let mut params: Vec<(String, String)> = form_urlencoded::parse(init_data.as_bytes())
        .into_owned()
        .collect();

    if cfg!(debug_assertions) && init_data == "dev_mode_active" {
        tracing::info!("dev mode, validation bypassed");

        return Ok((ENV.user_id, Some("dev_user".to_string())));
    }

    let hash = params
        .iter()
        .find(|(k, _)| k == "hash")
        .map(|(_, v)| v.clone())
        .ok_or_else(|| AppError::InternalError("No hash found".into()))?;

    params.sort_by(|a, b| a.0.cmp(&b.0));

    let data_check_string = params
        .iter()
        .filter(|(k, _)| k != "hash")
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    let mut mac = Hmac::<Sha256>::new_from_slice(b"WebAppData")
        .map_err(|_| AppError::InternalError("HMAC error".into()))?;

    mac.update(bot_token.as_bytes());

    let secret_key = mac.finalize().into_bytes();

    let mut mac = Hmac::<Sha256>::new_from_slice(&secret_key)
        .map_err(|_| AppError::InternalError("HMAC error".into()))?;

    mac.update(data_check_string.as_bytes());

    let calculated_hash = hex::encode(mac.finalize().into_bytes());

    if calculated_hash != hash {
        return Err(Box::new(AppError::InternalError("Invalid hash".into())));
    }

    let user_json = params
        .iter()
        .find(|(k, _)| k == "user")
        .map(|(_, v)| v.clone())
        .ok_or_else(|| AppError::InternalError("No user data".into()))?;

    let user_json_value: serde_json::Value = serde_json::from_str(&user_json).unwrap();

    let user_id = user_json_value["id"]
        .as_u64()
        .ok_or_else(|| AppError::InternalError("Invalid User ID".into()))?;

    let username = user_json_value["username"]
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            let first_name = user_json_value["first_name"].as_str().unwrap_or("");
            let last_name = user_json_value["last_name"].as_str().unwrap_or("");

            let name = format!("{} {}", first_name, last_name).trim().to_string();

            if name.is_empty() {
                None
            } else {
                Some(name)
            }
        });

    Ok((user_id, username))
}

fn create_refresh_cookie(refresh_token: String) -> Cookie<'static> {
    Cookie::build("refresh_token", refresh_token)
        .http_only(true)
        .secure(true)
        .path("/api/auth/refresh")
        .max_age(Duration::weeks(1))
        .finish()
}

pub async fn login(
    payload: web::Json<LoginPayload>,
    jwt_secret: web::Data<String>,
    _config: web::Data<Arc<Config>>,
    auth_state: AuthState,
    db: web::Data<Arc<crate::types::databases::Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id_u64, web_username) =
        validate_telegram_init_data(&payload.init_data, &ENV.token)
            .map_err(|_| actix_web::error::ErrorUnauthorized("Telegram Auth Failed"))?;

    let user_id = UserId(user_id_u64);
    let user_id_str = user_id_u64.to_string();
    let secret = jwt_secret.as_bytes();

    let tokens = auth::jwt::create_tokens(user_id, secret, auth_state)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token issue failed"))?;

    db.users()
        .upsert_user(user_id_u64 as i64, web_username)
        .await;
    let user = db.users().get_user_by_telegram_id(user_id_u64 as i64).await;
    info!("user from db: {:?}", user);

    let response = LoginResponse {
        access_token: tokens.access_token,
        user_id: user_id_str,
        username: user.and_then(|u| u.username),
    };

    let refresh_cookie = create_refresh_cookie(tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(refresh_cookie).json(response))
}

pub async fn refresh(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    refresh_state: AuthState,
    db: web::Data<Arc<crate::types::databases::Database>>,
) -> Result<HttpResponse, ActixError> {
    if !config.web.auth {
        return Err(actix_web::error::ErrorForbidden(
            "Authentication is disabled",
        ));
    }

    let refresh_token = req
        .cookie("refresh_token")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Refresh token missing from cookie."))?
        .value()
        .to_string();

    let secret = jwt_secret.as_bytes();

    let user_id_str = match jsonwebtoken::decode::<RefreshClaims>(
        &refresh_token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data.claims.sub,
        Err(e) => {
            tracing::error!("Basic refresh token decode failed: {:?}", e);

            return Err(actix_web::error::ErrorUnauthorized(
                "Invalid refresh token format",
            ));
        }
    };

    let user_id = UserId(user_id_str.parse::<u64>().unwrap_or_default());

    let new_tokens = match auth::jwt::refresh_access_token(
        user_id,
        refresh_token,
        secret,
        refresh_state,
    )
    .await
    {
        Ok(tokens) => tokens,
        Err(e) => {
            tracing::error!(
                "Token refresh failed (Invalid or Expired Refresh Token): {:?}",
                e
            );

            return Err(actix_web::error::ErrorUnauthorized(
                "Invalid or expired refresh token. Please re-login.",
            ));
        }
    };

    let refresh_cookie = create_refresh_cookie(new_tokens.refresh_token);

    let parsed_user_id = user_id_str.parse::<i64>().unwrap_or_default();
    let user = db.users().get_user_by_telegram_id(parsed_user_id).await;

    let response = LoginResponse {
        access_token: new_tokens.access_token,
        user_id: user_id_str,
        username: user.and_then(|u| u.username),
    };

    Ok(HttpResponse::Ok().cookie(refresh_cookie).json(response))
}
