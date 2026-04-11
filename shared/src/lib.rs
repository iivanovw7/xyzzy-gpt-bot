use chrono::{naive::serde::ts_seconds, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub user_id: String,
    pub username: Option<String>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: String,
    pub username: Option<String>,
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
pub struct StatisticsTransaction {
    pub id: i64,
    pub amount: f64,
    pub category: String,
    pub is_income: bool,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub date: NaiveDateTime,
    pub description: String,
    pub accumulated_amount: f64,
    pub is_first_transaction_in_month: bool,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct OverviewResponse {
    pub currency: String,
    pub categories_summary: CategoriesSummary,
    pub month: u32,
    pub month_balance: f64,
    pub month_income: f64,
    pub month_spending: f64,
    pub month_transactions: Vec<OverviewTransaction>,
    pub month_transactions_count: u32,
    pub month_summary: MonthlySummary,
    pub year: u32,
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
pub struct CreateTransactionRequest {
    pub amount: f64,
    pub category: i64,
    pub description: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionResponse {
    pub success: bool,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct CategorySummary {
    pub category: String,
    pub monthly_summaries: Vec<MonthlySummary>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct CategoriesSummary {
    pub year: u32,
    pub categories: Vec<CategorySummary>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct TransactionsResponse {
    pub currency: String,
    pub year: u32,
    pub transactions_categories: Vec<String>,
    pub transactions: Vec<StatisticsTransaction>,
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

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct MonthlyIncomeSummary {
    pub name: String,
    pub amounts: Vec<f64>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct YearlySummary {
    pub year: u32,
    pub monthly_summaries: Vec<MonthlySummary>,
    pub monthly_spending_summaries: Vec<MonthlySpendingSummary>,
    pub monthly_income_summaries: Vec<MonthlyIncomeSummary>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    pub init_data: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct SysInfoResponse {
    pub uptime: u64,
    pub total_mem: u64,
    pub used_mem: u64,
    pub cpu_usage: f32,
    pub os_version: Option<String>,
    pub db_latency_ms: f64,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct SurfReportResponse {
    pub location: String,
    pub daily: Vec<SurfDailyReport>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct SurfDailyReport {
    pub swell_height: f32,
    pub swell_period: f32,
    pub swell_direction: f32,
    pub swell_energy: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub timestamp: String,
    pub sunrise: String,
    pub sunset: String,
    pub hourly_tides: Vec<f32>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct RecurrentPayment {
    pub id: i64,
    pub description: String,
    pub last_amount: f64,
    pub category_name: String,
    pub category_id: i64,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub last_date: NaiveDateTime,
    pub is_paid_this_month: bool,
    pub occurrence_count: i32,
    pub is_income: bool,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct RecurrentDashboards {
    pub incomes: RecurrentDashboard,
    pub expenses: RecurrentDashboard,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct RecurrentDashboard {
    pub sections: Vec<RecurrentSection>,
    pub monthly_stats: RecurrentStats,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct RecurrentSection {
    pub title: String,
    pub items: Vec<RecurrentPayment>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct RecurrentStats {
    pub total_monthly_commitment: f32,
    pub total_paid_so_far: f32,
    pub total_remaining: f32,
    pub percent_paid: f32,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LinkResponse {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub category_name: Option<String>,
    pub tags: Vec<String>,
    #[serde(with = "ts_seconds")]
    #[ts(type = "number")]
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LinksQuery {
    pub category: Option<String>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LinksListResponse {
    pub links: Vec<LinkResponse>,
    pub total_count: i64,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LinkCategoryResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LinkCategoriesResponse {
    pub categories: Vec<LinkCategoryResponse>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct LinkTagsResponse {
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
#[serde(rename_all = "camelCase")]
pub struct DeleteLinkResponse {
    pub success: bool,
}
