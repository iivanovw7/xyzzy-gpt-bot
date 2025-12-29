use chrono::{naive::serde::ts_seconds, NaiveDateTime};
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
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: NaiveDateTime,
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
    pub month_summary: MonthlySummary,
    pub year_summary: YearlySummary,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct TransactionQuery {
    pub category: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct BudgetingTransaction {
    pub id: i64,
    pub amount: f64,
    pub category: String,
    pub is_income: bool,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: NaiveDateTime,
    pub description: String,
    pub accumulatded_amount: f64,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct TransactionsResponse {
    pub currency: String,
    pub year: u32,
    pub transactions_categories: Vec<String>,
    pub transactions: Vec<BudgetingTransaction>,
    pub transactions_count: u32,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct MonthlySummary {
    pub month: u32,
    pub income: f64,
    pub spending: f64,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct MonthlySpendingSummary {
    pub name: String,
    pub amounts: Vec<f64>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct YearlySummary {
    pub year: u32,
    pub monthly_summaries: Vec<MonthlySummary>,
    pub monthly_spending_summaries: Vec<MonthlySpendingSummary>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    pub init_data: String,
}
