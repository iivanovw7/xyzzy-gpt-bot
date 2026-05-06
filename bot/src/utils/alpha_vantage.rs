use crate::env::ENV;
use anyhow::Result;
use reqwest::Client as HttpClient;
use serde::Deserialize;
use tracing::{error, info};

const ALPHA_VANTAGE_BASE_URL: &str = "https://www.alphavantage.co/query";

#[derive(Deserialize, Debug)]
pub struct IncomeStatement {
    #[serde(rename = "quarterlyReports")]
    pub quarterly_reports: Vec<QuarterlyReport>,
}

#[derive(Deserialize, Debug)]
pub struct QuarterlyReport {
    #[serde(rename = "fiscalDateEnding")]
    pub fiscal_date_ending: String,
    #[serde(rename = "reportedCurrency")]
    pub reported_currency: String,
    #[serde(rename = "grossProfit")]
    pub gross_profit: Option<String>,
    #[serde(rename = "totalRevenue")]
    pub total_revenue: Option<String>,
    #[serde(rename = "netIncome")]
    pub net_income: Option<String>,
    #[serde(rename = "eps")]
    pub eps: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BalanceSheet {
    #[serde(rename = "quarterlyReports")]
    pub quarterly_reports: Vec<BalanceSheetReport>,
}

#[derive(Deserialize, Debug)]
pub struct BalanceSheetReport {
    #[serde(rename = "fiscalDateEnding")]
    pub fiscal_date_ending: String,
    #[serde(rename = "reportedCurrency")]
    pub reported_currency: String,
    #[serde(rename = "totalAssets")]
    pub total_assets: Option<String>,
    #[serde(rename = "totalLiabilities")]
    pub total_liabilities: Option<String>,
    #[serde(rename = "totalShareholderEquity")]
    pub total_shareholder_equity: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CashFlow {
    #[serde(rename = "quarterlyReports")]
    pub quarterly_reports: Vec<CashFlowReport>,
}

#[derive(Deserialize, Debug)]
pub struct CashFlowReport {
    #[serde(rename = "fiscalDateEnding")]
    pub fiscal_date_ending: String,
    #[serde(rename = "reportedCurrency")]
    pub reported_currency: String,
    #[serde(rename = "operatingCashflow")]
    pub operating_cashflow: Option<String>,
    #[serde(rename = "capitalExpenditures")]
    pub capital_expenditures: Option<String>,
}

pub async fn fetch_income_statement(
    http_client: &HttpClient,
    symbol: &str,
) -> Result<IncomeStatement> {
    let url = format!(
        "{}?function=INCOME_STATEMENT&symbol={}&apikey={}",
        ALPHA_VANTAGE_BASE_URL, symbol, ENV.alphavantage_api_key
    );
    info!("Alpha Vantage Income Statement URL: {}", url);
    let res = http_client.get(url).send().await?;
    let data: IncomeStatement = res.json().await?;
    Ok(data)
}

pub async fn fetch_balance_sheet(http_client: &HttpClient, symbol: &str) -> Result<BalanceSheet> {
    let url = format!(
        "{}?function=BALANCE_SHEET&symbol={}&apikey={}",
        ALPHA_VANTAGE_BASE_URL, symbol, ENV.alphavantage_api_key
    );
    info!("Alpha Vantage Balance Sheet URL: {}", url);
    let res = http_client.get(url).send().await?;
    let data: BalanceSheet = res.json().await?;
    Ok(data)
}

pub async fn fetch_cash_flow(http_client: &HttpClient, symbol: &str) -> Result<CashFlow> {
    let url = format!(
        "{}?function=CASH_FLOW&symbol={}&apikey={}",
        ALPHA_VANTAGE_BASE_URL, symbol, ENV.alphavantage_api_key
    );
    info!("Alpha Vantage Cash Flow URL: {}", url);
    let res = http_client.get(url).send().await?;
    let data: CashFlow = res.json().await?;
    Ok(data)
}
