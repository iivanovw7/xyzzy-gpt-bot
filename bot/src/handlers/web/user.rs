use crate::handlers::auth;
use crate::types::databases::Database;
use crate::{config::Config, env::Env};
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use shared::UserResponse;
use std::sync::Arc;

pub async fn get(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: web::Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;

    let result = user_id.to_string();
    let parsed_user_id: i64 = result.parse::<i64>().unwrap_or_default();

    let user = db.users().get_user_by_telegram_id(parsed_user_id).await;

    let response = UserResponse {
        user_id: result.clone(),
        username: user.and_then(|u| u.username),
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
