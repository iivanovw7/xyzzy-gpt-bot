use crate::handlers::auth;
use crate::types::common::DateFilter;
use crate::types::databases::Database;
use crate::utils::statistics::amount_to_float;
use crate::{config::Config, env::Env};
use actix_web::web::Data;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use chrono::Datelike;
use chrono::Local;
use shared::{BudgetingTransaction, TransactionQuery, TransactionsResponse};
use std::sync::Arc;

pub async fn get(
    req: HttpRequest,
    query: web::Query<TransactionQuery>,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;
    let today = Local::now().date_naive();
    let current_year = today.year() as u32;
    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();
    let transactions_db = &db.transactions();

    let mut all_year_txs = transactions_db
        .list_filtered(parsed_user_id, DateFilter::CurrentYear)
        .await;

    all_year_txs.sort_by(|a, b| a.date.cmp(&b.date));

    let mut accumulated_balance = 0.0;
    let mut transactions: Vec<BudgetingTransaction> = vec![];
    let mut categories_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    let category_filter = query.category.as_deref();
    let description_filter = query.description.as_ref().map(|d| d.to_lowercase());

    for tx in all_year_txs {
        categories_set.insert(tx.category_name.clone());

        let matches_category = match category_filter {
            Some(c) => c == tx.category_name,
            None => true,
        };

        let matches_description = match description_filter.as_deref() {
            Some(search) => tx.description.to_lowercase().contains(search),
            None => true,
        };

        if matches_category && matches_description {
            let tx_amount_float = amount_to_float(tx.amount);

            accumulated_balance += tx_amount_float;

            transactions.push(BudgetingTransaction {
                id: tx.id,
                amount: tx_amount_float.abs().round(),
                category: tx.category_name.clone(),
                is_income: tx.amount > 0,
                date: tx.date,
                description: tx.description.clone(),
                accumulatded_amount: accumulated_balance,
            });
        }
    }

    transactions.reverse();

    let response = TransactionsResponse {
        currency: "EUR".to_string(),
        year: current_year,
        transactions_count: transactions.len() as u32,
        transactions_categories: categories_set.into_iter().collect(),
        transactions,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
