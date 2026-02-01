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

    let year_transactions = transactions_db
        .list_filtered(parsed_user_id, DateFilter::CurrentYear)
        .await;

    let mut month_spending = 0.0;
    let mut month_income = 0.0;
    let mut month_transactions: Vec<OverviewTransaction> = vec![];

    let mut monthly_summaries_map: std::collections::BTreeMap<u32, (f64, f64)> =
        (1..=12).map(|m| (m, (0.0, 0.0))).collect();

    let mut category_monthly_map: std::collections::HashMap<
        String,
        std::collections::BTreeMap<u32, (f64, f64)>,
    > = std::collections::HashMap::new();

    let mut monthly_spending_summaries_map: std::collections::BTreeMap<String, Vec<f64>> =
        std::collections::BTreeMap::new();

    for tx in &year_transactions {
        let tx_amount_float = amount_to_float(tx.amount);
        let tx_amount_abs = tx_amount_float.abs();
        let tx_month = tx.date.month();

        let global_entry = monthly_summaries_map.entry(tx_month).or_insert((0.0, 0.0));

        let cat_map = category_monthly_map
            .entry(tx.category_name.clone())
            .or_insert_with(|| (1..=12).map(|m| (m, (0.0, 0.0))).collect());

        let cat_entry = cat_map.entry(tx_month).or_insert((0.0, 0.0));

        if tx.amount > 0 {
            global_entry.0 += tx_amount_float;
            cat_entry.0 += tx_amount_float;
        } else {
            global_entry.1 += tx_amount_abs;
            cat_entry.1 += tx_amount_abs;

            let amounts = monthly_spending_summaries_map
                .entry(tx.category_name.clone())
                .or_insert(vec![0.0; 12]);

            amounts[(tx_month - 1) as usize] += tx_amount_abs;
        }

        if tx_month == current_month {
            if tx.amount > 0 {
                month_income += tx_amount_float;
            } else {
                month_spending += tx_amount_abs;
            }

            month_transactions.push(OverviewTransaction {
                id: tx.id,
                amount: tx_amount_abs,
                category: tx.category_name.clone(),
                is_income: tx.amount > 0,
                date: tx.date,
                description: tx.description.clone(),
            });
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

    let categories_summary = shared::CategoriesSummary {
        year: current_year,
        categories: category_monthly_map
            .into_iter()
            .map(|(category, months)| shared::CategorySummary {
                category,
                monthly_summaries: months
                    .into_iter()
                    .map(|(month, (income, spending))| shared::MonthlySummary {
                        month,
                        income,
                        spending,
                    })
                    .collect(),
            })
            .collect(),
    };

    let year_summary = shared::YearlySummary {
        year: current_year,
        monthly_summaries,
        monthly_spending_summaries: monthly_spending_summaries_map
            .clone()
            .into_iter()
            .map(|(name, amounts)| shared::MonthlySpendingSummary { name, amounts })
            .collect(),
    };

    month_transactions.sort_by(|a, b| b.date.cmp(&a.date));

    let month_transactions_count = month_transactions.len() as u32;

    let response = OverviewResponse {
        currency: "EUR".to_string(),
        categories_summary,
        month: current_month,
        month_income,
        month_spending,
        month_balance: round_balance(month_income, month_spending),
        month_transactions,
        month_transactions_count,
        month_summary: shared::MonthlySummary {
            month: current_month,
            income: month_income,
            spending: month_spending,
        },
        year: current_year,
        year_summary,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
