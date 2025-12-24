use crate::handlers::auth;
use crate::types::common::DateFilter;
use crate::types::databases::Database;
use crate::utils::statistics::amount_to_float;
use crate::utils::transactions::round_balance;
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
    let current_month = today.month();
    let current_year = today.year() as u32;

    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;

    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();
    let transactions_db = &db.transactions();

    let current_month_transactions = transactions_db
        .list_filtered(parsed_user_id, DateFilter::CurrentMonth)
        .await;

    let year_transactions = transactions_db
        .list_filtered(parsed_user_id, DateFilter::CurrentYear)
        .await;

    let mut month_spending = 0.0;
    let mut month_income = 0.0;
    let mut month_transactions: Vec<OverviewTransaction> = vec![];

    let mut monthly_summaries_map: std::collections::BTreeMap<u32, (f64, f64)> =
        std::collections::BTreeMap::new();

    let mut monthly_spending_summaries_map: std::collections::BTreeMap<String, Vec<f64>> =
        std::collections::BTreeMap::new();

    for m in 1..=12 {
        monthly_summaries_map.insert(m, (0.0, 0.0));
    }

    for tx in &year_transactions {
        let tx_amount_float = amount_to_float(tx.amount);
        let tx_amount_float_abs = tx_amount_float.abs();
        let tx_month = tx.date.month();
        let entry = monthly_summaries_map.entry(tx_month).or_insert((0.0, 0.0));

        if tx.amount > 0 {
            entry.0 += tx_amount_float;
        } else {
            entry.1 += tx_amount_float_abs;

            let cat_amounts = monthly_spending_summaries_map
                .entry(tx.category_name.clone())
                .or_insert(vec![0.0; 12]);

            cat_amounts[(tx_month - 1) as usize] += tx_amount_float_abs;
        }

        if tx_month == current_month {
            if tx.amount > 0 {
                month_income += tx_amount_float;
            } else {
                month_spending += tx_amount_float_abs;
            }

            month_transactions.push(OverviewTransaction {
                id: tx.id,
                amount: tx_amount_float.abs(),
                category: tx.category_name.clone(),
                is_income: tx.amount > 0,
                date: tx.date,
                description: tx.description.clone(),
            })
        }
    }

    let monthly_summaries: Vec<shared::MonthlySummary> = monthly_summaries_map
        .into_iter()
        .map(|(month, (income, spending))| shared::MonthlySummary {
            month,
            income,
            spending,
        })
        .collect();

    let monthly_spending_summaries: Vec<shared::MonthlySpendingSummary> =
        monthly_spending_summaries_map
            .into_iter()
            .map(|(name, amounts)| shared::MonthlySpendingSummary { name, amounts })
            .collect();

    let month_summary = shared::MonthlySummary {
        month: current_month,
        income: month_income,
        spending: month_spending,
    };

    let year_summary = shared::YearlySummary {
        year: current_year,
        monthly_summaries,
        monthly_spending_summaries,
    };

    month_transactions.sort_by(|a, b| b.date.cmp(&a.date));

    let response = OverviewResponse {
        currency: "EUR".to_string(),
        month: current_month,
        month_income,
        month_spending,
        month_balance: round_balance(month_income, month_spending),
        month_transactions,
        month_transactions_count: current_month_transactions.len() as u32,
        month_summary,
        year_summary,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
