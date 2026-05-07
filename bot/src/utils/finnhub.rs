use crate::env::ENV;
use anyhow::Result;
use reqwest::Client as HttpClient;
use serde::Deserialize;
use tracing::info;

const FINNHUB_BASE_URL: &str = "https://finnhub.io/api/v1";

#[derive(Deserialize, Debug)]
pub struct BasicFinancials {
    pub series: Option<Series>,
    pub metric: Option<Metric>,
}

#[derive(Deserialize, Debug)]
pub struct Series {
    pub quarterly: Option<QuarterlySeries>,
}

#[derive(Deserialize, Debug)]
pub struct QuarterlySeries {
    pub eps: Option<Vec<FinancialValue>>,
    #[serde(rename = "netIncome")]
    pub net_income: Option<Vec<FinancialValue>>,
    #[serde(rename = "totalRevenue")]
    pub total_revenue: Option<Vec<FinancialValue>>,
    #[serde(rename = "totalAssets")]
    pub total_assets: Option<Vec<FinancialValue>>,
    #[serde(rename = "totalLiabilities")]
    pub total_liabilities: Option<Vec<FinancialValue>>,
    #[serde(rename = "operatingCashFlow")]
    pub operating_cash_flow: Option<Vec<FinancialValue>>,
}

#[derive(Deserialize, Debug)]
pub struct FinancialValue {
    pub period: String,
    pub v: f64,
}

#[derive(Deserialize, Debug)]
pub struct Metric {
    #[serde(rename = "peAnnual")]
    pub pe_annual: Option<f64>,
    #[serde(rename = "epsAnnual")]
    pub eps_annual: Option<f64>,
}

pub async fn fetch_basic_financials(
    http_client: &HttpClient,
    symbol: &str,
) -> Result<BasicFinancials> {
    let url = format!(
        "{}/stock/metric?symbol={}&metric=all&token={}",
        FINNHUB_BASE_URL, symbol, ENV.finnhub_api_key
    );
    info!("Finnhub Basic Financials URL: {}", url);
    let res = http_client.get(url).send().await?;
    let data: BasicFinancials = res.json().await?;
    Ok(data)
}
