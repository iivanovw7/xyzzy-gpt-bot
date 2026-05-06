use crate::env::ENV;
use anyhow::Result;
use reqwest::Client as HttpClient;
use serde::Deserialize;
use tracing::{error, info};

const FRED_API_BASE_URL: &str = "https://api.stlouisfed.org/fred/series/observations";

#[derive(Deserialize, Debug)]
pub struct FredSeries {
    pub observations: Vec<FredObservation>,
}

#[derive(Deserialize, Debug)]
pub struct FredObservation {
    pub date: String,
    pub value: String,
}

pub async fn fetch_fred_series(
    http_client: &HttpClient,
    series_id: &str,
) -> Result<Option<String>> {
    let url = format!(
        "{}?series_id={}&api_key={}&file_type=json&sort_order=desc&limit=1",
        FRED_API_BASE_URL, series_id, ENV.fred_api_key
    );

    info!("FRED API URL: {}", url);

    let res = http_client.get(url).send().await?;
    let body = res.text().await?;
    info!("FRED API Raw Response: {}", body);
    let data: FredSeries = serde_json::from_str(&body)?;

    info!("FRED DATA: {:?}", data);

    if let Some(latest_observation) = data.observations.first() {
        info!(
            "Successfully fetched FRED series {}: {} on {}",
            series_id, latest_observation.value, latest_observation.date
        );
        Ok(Some(latest_observation.value.clone()))
    } else {
        info!("No observations found for FRED series {}", series_id);
        Ok(None)
    }
}

pub async fn get_gdp(http_client: &HttpClient) -> Result<Option<String>> {
    fetch_fred_series(http_client, "GDP").await
}

pub async fn get_cpi(http_client: &HttpClient) -> Result<Option<String>> {
    fetch_fred_series(http_client, "CPIAUCSL").await
}

pub async fn get_fed_funds_rate(http_client: &HttpClient) -> Result<Option<String>> {
    fetch_fred_series(http_client, "FEDFUNDS").await
}
