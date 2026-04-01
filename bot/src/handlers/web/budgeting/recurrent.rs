use crate::config::Config;
use crate::env::Env;
use crate::handlers::auth;
use crate::types::databases::Database;
use crate::utils::statistics::amount_to_float;
use actix_web::web::Data;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use chrono::{Datelike, Local};
use shared::{RecurrentDashboard, RecurrentPayment, RecurrentSection, RecurrentStats};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get(
    req: HttpRequest,
    jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let (user_id, _) = auth::jwt::authorize_request(req, jwt_secret, config.web.auth)?;
    let parsed_user_id: i64 = user_id.trim().parse::<i64>().unwrap_or_default();

    let today = Local::now().date_naive();
    let start_date = today - chrono::Duration::days(90);
    let start_of_month = today.with_day(1).unwrap();

    let transactions_db = db.transactions();
    let recent_txs = transactions_db
        .list_with_range(parsed_user_id, Some(start_date), None)
        .await;

    let recurrent_income_categories = &config.budgeting.recurrent_income_categories;
    let recurrent_payment_categories = &config.budgeting.recurrent_payment_categories;

    let mut grouped_txs: HashMap<(i64, String), Vec<&crate::types::models::TransactionRow>> =
        HashMap::new();

    for tx in &recent_txs {
        if !recurrent_income_categories.contains(&tx.category_id)
            && !recurrent_payment_categories.contains(&tx.category_id)
        {
            continue;
        }

        let desc_key = tx.description.trim().to_lowercase();

        grouped_txs
            .entry((tx.category_id, desc_key))
            .or_default()
            .push(tx);
    }

    let mut incomes_payments = Vec::new();
    let mut expenses_payments = Vec::new();

    let mut inc_commitment = 0.0;
    let mut inc_paid = 0.0;
    let mut inc_remaining = 0.0;

    let mut exp_commitment = 0.0;
    let mut exp_paid = 0.0;
    let mut exp_remaining = 0.0;

    for ((category_id, _desc_key), mut txs) in grouped_txs {
        if txs.len() < 2 {
            continue;
        }

        txs.sort_by(|a, b| b.date.cmp(&a.date));

        let last_tx = txs[0];
        let last_amount_float = amount_to_float(last_tx.amount).abs();

        let mut is_paid_this_month = false;
        for tx in &txs {
            if tx.date.date() >= start_of_month {
                is_paid_this_month = true;
                break;
            }
        }

        let is_income = last_tx.amount > 0;

        let payment = RecurrentPayment {
            id: last_tx.id,
            description: last_tx.description.clone(),
            last_amount: last_amount_float,
            category_name: last_tx.category_name.clone(),
            category_id,
            last_date: last_tx.date,
            is_paid_this_month,
            occurrence_count: txs.len() as i32,
            is_income,
        };

        if is_income {
            incomes_payments.push(payment);
            inc_commitment += last_amount_float;

            if is_paid_this_month {
                inc_paid += last_amount_float;
            } else {
                inc_remaining += last_amount_float;
            }
        } else {
            expenses_payments.push(payment);
            exp_commitment += last_amount_float;

            if is_paid_this_month {
                exp_paid += last_amount_float;
            } else {
                exp_remaining += last_amount_float;
            }
        }
    }

    incomes_payments.sort_by(|a, b| b.last_date.cmp(&a.last_date));
    expenses_payments.sort_by(|a, b| b.last_date.cmp(&a.last_date));

    let create_dashboard = |payments: Vec<RecurrentPayment>,
                            commitment: f32,
                            paid: f32,
                            remaining: f32|
     -> RecurrentDashboard {
        let mut sections_map: HashMap<String, Vec<RecurrentPayment>> = HashMap::new();

        for p in payments {
            sections_map
                .entry(p.category_name.clone())
                .or_default()
                .push(p);
        }

        let mut sections: Vec<RecurrentSection> = sections_map
            .into_iter()
            .map(|(title, items)| RecurrentSection { title, items })
            .collect();

        sections.sort_by(|a, b| a.title.cmp(&b.title));

        let percent_paid = if commitment > 0.0 {
            ((paid / commitment) * 100.0).round()
        } else {
            0.0
        };

        RecurrentDashboard {
            sections,
            monthly_stats: RecurrentStats {
                total_monthly_commitment: commitment,
                total_paid_so_far: paid,
                total_remaining: remaining,
                percent_paid,
            },
        }
    };

    let response = shared::RecurrentDashboards {
        incomes: create_dashboard(
            incomes_payments,
            inc_commitment as f32,
            inc_paid as f32,
            inc_remaining as f32,
        ),
        expenses: create_dashboard(
            expenses_payments,
            exp_commitment as f32,
            exp_paid as f32,
            exp_remaining as f32,
        ),
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
