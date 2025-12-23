use crate::handlers::auth;
use crate::types::common::DateFilter;
use crate::types::databases::Database;
use crate::utils::statistics::amount_to_float;
use crate::{config::Config, env::Env};
use actix_web::web::Data;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use chrono::Datelike;
use chrono::Local;
use shared::{OverviewResponse, OverviewTransaction};
use std::sync::Arc;

pub async fn get(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let today = Local::now().date_naive();
    let month = today.month();

    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;

    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();
    let transactions_db = &db.transactions();

    let current_month_transactions = transactions_db
        .list_filtered(parsed_user_id, DateFilter::CurrentMonth)
        .await;

    let mut month_spending = 0.0;
    let mut month_income = 0.0;
    let mut month_transactions: Vec<OverviewTransaction> = vec![];

    for tx in &current_month_transactions {
        let amount_f = amount_to_float(tx.amount);

        let entry = OverviewTransaction {
            id: tx.id,
            amount: amount_f.abs(),
            category: tx.category_name.clone(),
            is_income: tx.amount > 0,
            date: tx.date,
            description: tx.description.clone(),
        };

        if tx.amount < 0 {
            month_spending += -amount_f;
        } else {
            month_income += amount_f;
        }

        month_transactions.push(entry);
    }

    month_transactions.sort_by(|a, b| b.date.cmp(&a.date));

    let count = current_month_transactions.len() as u32;

    let response = OverviewResponse {
        currency: "EUR".to_string(),
        month,
        month_income,
        month_spending,
        month_balance: month_income - month_spending,
        month_transactions,
        month_transactions_count: count,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
