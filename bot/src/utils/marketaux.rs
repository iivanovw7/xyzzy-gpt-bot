use crate::env::ENV;
use anyhow::Result;
use reqwest::Client as HttpClient;
use serde::Deserialize;
use tracing::info;

const MARKETAUX_BASE_URL: &str = "https://api.marketaux.com/v1";

#[derive(Deserialize, Debug)]
pub struct MarketAuxResponse {
    pub data: Vec<NewsItem>,
}

#[derive(Deserialize, Debug)]
pub struct NewsItem {
    pub title: String,
    pub snippet: String,
    pub url: String,
    pub published_at: String,
    pub entities: Vec<Entity>,
}

#[derive(Deserialize, Debug)]
pub struct Entity {
    pub symbol: String,
    pub sentiment_score: Option<f64>,
}

pub async fn fetch_stock_news(http_client: &HttpClient, symbol: &str) -> Result<String> {
    let url = format!(
        "{}/news/all?symbols={}&filter_entities=true&language=en&api_token={}",
        MARKETAUX_BASE_URL, symbol, ENV.marketaux_api_key
    );
    info!("MarketAux News URL: {}", url);
    let res = http_client.get(url).send().await?;
    let data: MarketAuxResponse = res.json().await?;

    if data.data.is_empty() {
        return Ok("No recent news found.".to_string());
    }

    let news_summaries: Vec<String> = data.data
        .iter()
        .take(5)
        .map(|item| {
            let sentiment = item.entities.iter()
                .find(|e| e.symbol.to_uppercase() == symbol.to_uppercase())
                .and_then(|e| e.sentiment_score)
                .map(|s| format!(" (Sentiment: {:.2})", s))
                .unwrap_or_default();
            format!("{}: {}{}", item.title, item.snippet, sentiment)
        })
        .collect();

    Ok(news_summaries.join("; "))
}
