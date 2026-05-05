use reqwest::Client as HttpClient;

pub async fn fetch_market_news(http_client: &HttpClient, symbol: &str) -> String {
    let url = format!("https://finance.yahoo.com/quote/{}/news/", symbol);
    match http_client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
    {
        Ok(res) => {
            if let Ok(text) = res.text().await {
                let document = scraper::Html::parse_document(&text);
                let selector = scraper::Selector::parse("h3").unwrap();
                let news: Vec<String> = document
                    .select(&selector)
                    .take(5)
                    .map(|e| e.text().collect::<String>().trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if news.is_empty() {
                    "No recent news found.".to_string()
                } else {
                    news.join("; ")
                }
            } else {
                "Failed to read news text.".to_string()
            }
        }
        Err(_) => "Failed to fetch news.".to_string(),
    }
}
