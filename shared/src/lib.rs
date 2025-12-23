use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct OverviewTransaction {
    pub id: i64,
    pub amount: f64,
    pub category: String,
    pub is_income: bool,
    #[ts(type = "string")]
    pub date: NaiveDate,
    pub description: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct OverviewResponse {
    pub currency: String,
    pub month: u32,
    pub month_balance: f64,
    pub month_income: f64,
    pub month_spending: f64,
    pub month_transactions: Vec<OverviewTransaction>,
    pub month_transactions_count: u32,
}
