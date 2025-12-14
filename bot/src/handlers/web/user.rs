use crate::handlers::auth;
use crate::{config::Config, env::Env};
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    user_id: String,
}

pub async fn get(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.app_auth)?;

    let result = user_id.to_string();

    let response = UserResponse {
        user_id: result.clone(),
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
