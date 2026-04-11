use crate::handlers::auth;
use crate::types::databases::Database;
use crate::{config::Config, env::Env};
use actix_web::web::Data;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use shared::{
    LinkResponse, LinksListResponse, LinksQuery, LinkCategoryResponse, LinkCategoriesResponse,
    LinkTagsResponse, DeleteLinkResponse,
};
use std::sync::Arc;

pub async fn get_links(
    req: HttpRequest,
    query: web::Query<LinksQuery>,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;
    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let (links, count) = db.links().get_filtered_links(
        parsed_user_id,
        query.category.as_deref(),
        query.tag.as_deref(),
        query.search.as_deref(),
        limit,
        offset
    ).await.unwrap_or_default();

    let links_response = links.into_iter().map(|l| LinkResponse {
        id: l.id,
        url: l.url,
        title: l.title,
        description: l.description,
        thumbnail_url: l.thumbnail_url,
        category_name: l.category_name,
        tags: l.tags.map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()).unwrap_or_default(),
        created_at: l.created_at,
    }).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": LinksListResponse {
        links: links_response,
        total_count: count,
    } })))
}

pub async fn get_categories(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;
    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();

    let categories = db.links().list_categories(parsed_user_id).await.unwrap_or_default();

    let categories_response = categories.into_iter().map(|c| LinkCategoryResponse {
        id: c.id,
        name: c.name,
    }).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": LinkCategoriesResponse {
        categories: categories_response,
    } })))
}

pub async fn get_tags(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;
    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();

    let tags = db.links().list_tags(parsed_user_id).await.unwrap_or_default();

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": LinkTagsResponse {
        tags,
    } })))
}

pub async fn delete_link(
    req: HttpRequest,
    path: web::Path<i64>,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;
    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();
    let link_id = path.into_inner();

    // Verify ownership
    let link = match db.links().get_link(link_id).await {
        Ok(Some(l)) if l.user_id == parsed_user_id => l,
        _ => return Ok(HttpResponse::NotFound().json(serde_json::json!({ "message": "Link not found or unauthorized" }))),
    };

    let success = match db.links().delete_link(link.id).await {
        Ok(result) => result,
        Err(e) => {
            println!("Error deleting link {}: {:?}", link.id, e);
            false
        }
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": DeleteLinkResponse {
        success,
    } })))
}