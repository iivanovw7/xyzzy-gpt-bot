use crate::handlers::auth;
use crate::types::auth::{AuthState, RefreshClaims};
use crate::{config::Config, env::Env};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use shared::LoginResponse;
use std::sync::Arc;
use teloxide::types::UserId;

fn create_refresh_cookie(refresh_token: String) -> Cookie<'static> {
    Cookie::build("refresh_token", refresh_token)
        .http_only(true)
        .secure(true)
        .path("/api/auth/refresh")
        .max_age(Duration::weeks(1))
        .finish()
}

pub async fn login(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    config: web::Data<Arc<Config>>,
    auth_state: AuthState,
) -> Result<HttpResponse, ActixError> {
    let (user_id_str, _access_token) =
        auth::jwt::authorize_request(req, jwt_secret.clone(), config.web.auth)?;

    let user_id = UserId(user_id_str.parse::<u64>().unwrap_or_default());
    let secret = jwt_secret.as_bytes();

    let tokens = auth::jwt::create_tokens(user_id, secret, auth_state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create new tokens during exchange: {:?}", e);
            actix_web::error::ErrorInternalServerError("Could not issue new token pair")
        })?;

    let response = LoginResponse {
        access_token: tokens.access_token,
        user_id: user_id_str,
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

    let response = LoginResponse {
        access_token: new_tokens.access_token,
        user_id: user_id_str,
    };

    Ok(HttpResponse::Ok().cookie(refresh_cookie).json(response))
}
