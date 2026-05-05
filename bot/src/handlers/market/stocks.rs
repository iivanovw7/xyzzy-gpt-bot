use crate::{
    config::CONFIG,
    env::ENV,
    types::market::GptMarketAnalysis,
    utils::{market_indicators::MarketHistory, market_news::fetch_market_news},
};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client as OpenAiClient,
};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::time::Duration;
use teloxide::{prelude::*, types::UserId};
use tracing::{error, info};

#[derive(Deserialize, Debug)]
struct YahooResponse {
    chart: YahooChart,
}

#[derive(Deserialize, Debug)]
struct YahooChart {
    result: Option<Vec<YahooResult>>,
}

#[derive(Deserialize, Debug)]
struct YahooResult {
    meta: YahooMeta,
    indicators: YahooIndicators,
}

#[derive(Deserialize, Debug)]
struct YahooMeta {
    #[serde(rename = "regularMarketPrice")]
    price: f64,
    #[serde(rename = "regularMarketVolume")]
    volume: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct YahooIndicators {
    quote: Vec<YahooQuote>,
}

#[derive(Deserialize, Debug)]
struct YahooQuote {
    close: Vec<Option<f64>>,
    volume: Vec<Option<f64>>,
}

async fn fetch_yahoo_data(
    http_client: &HttpClient,
    symbol: &str,
    interval: &str,
    range: &str,
) -> anyhow::Result<(f64, Option<f64>, Vec<f64>, Vec<f64>)> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval={}&range={}",
        symbol, interval, range
    );
    let res = http_client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    let data: YahooResponse = res.json().await?;
    let result = data
        .chart
        .result
        .ok_or_else(|| anyhow::anyhow!("No results from Yahoo"))?;
    let result = result
        .first()
        .ok_or_else(|| anyhow::anyhow!("Empty results from Yahoo"))?;

    let current_price = result.meta.price;
    let current_volume = result.meta.volume;

    let quote = result
        .indicators
        .quote
        .first()
        .ok_or_else(|| anyhow::anyhow!("No quotes from Yahoo"))?;

    let prices: Vec<f64> = quote.close.iter().filter_map(|&p| p).collect();
    let volumes: Vec<f64> = quote.volume.iter().filter_map(|&v| v).collect();

    Ok((current_price, current_volume, prices, volumes))
}

async fn analyze_asset(
    bot: &Bot,
    openai_client: &OpenAiClient<OpenAIConfig>,
    http_client: &HttpClient,
    symbol: &str,
    target_user_id: UserId,
) {
    info!("Analyzing market data for {}...", symbol);

    let (current_price, current_volume, prices_1h, vols_1h) =
        match fetch_yahoo_data(http_client, symbol, "1h", "1mo").await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to fetch 1h data for {}: {}", symbol, e);
                return;
            }
        };

    let (_, _, prices_1d, vols_1d) =
        match fetch_yahoo_data(http_client, symbol, "1d", "1y").await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to fetch 1d data for {}: {}", symbol, e);
                return;
            }
        };

    if prices_1h.len() < 30 || prices_1d.len() < 30 {
        error!(
            "Not enough historical data for {}. 1h: {}, 1d: {}",
            symbol,
            prices_1h.len(),
            prices_1d.len()
        );
        return;
    }

    let mut hist_1h = MarketHistory::new_with_volume(prices_1h, vols_1h);
    hist_1h.update_price(current_price);
    if let Some(vol) = current_volume {
        hist_1h.update_volume(vol);
    }

    let mut hist_1d = MarketHistory::new_with_volume(prices_1d, vols_1d);
    hist_1d.update_price(current_price);
    if let Some(vol) = current_volume {
        hist_1d.update_volume(vol);
    }

    let rsi_1h = hist_1h.calculate_rsi();
    let (macd_line_1h, macd_signal_1h, macd_hist_1h) = hist_1h.calculate_macd();
    let sma_50_1h = hist_1h.calculate_sma(50);

    let rsi_1d = hist_1d.calculate_rsi();
    let (macd_line_1d, macd_signal_1d, macd_hist_1d) = hist_1d.calculate_macd();
    let sma_50_1d = hist_1d.calculate_sma(50);
    let sma_200_1d = hist_1d.calculate_sma(200);

    let news = fetch_market_news(http_client, symbol).await;

    info!(
        "Successfully fetched {} data | Price: ${:.2} | 1h RSI: {:.2} | 1d RSI: {:.2}",
        symbol, current_price, rsi_1h, rsi_1d
    );

    let system_prompt = include_str!("../../prompts/market_analysis.md");

    let user_prompt = format!(
        "Asset: {}\n\
        Current Price: ${:.2}\n\
        Current Volume: {:.0}\n\n\
        [HOURLY TIME FRAME (Short-Term)]\n\
        RSI (14): {:.2}\n\
        MACD Line: {:.4} | Signal: {:.4} | Histogram: {:.4}\n\
        SMA 50: ${:.2}\n\n\
        [DAILY TIME FRAME (Long-Term)]\n\
        RSI (14): {:.2}\n\
        MACD Line: {:.4} | Signal: {:.4} | Histogram: {:.4}\n\
        SMA 50: ${:.2} | SMA 200: ${:.2}\n\n\
        [FUNDAMENTAL CONTEXT]\n\
        Recent News: {}",
        symbol, current_price, current_volume.unwrap_or(0.0), 
        rsi_1h, macd_line_1h, macd_signal_1h, macd_hist_1h, sma_50_1h,
        rsi_1d, macd_line_1d, macd_signal_1d, macd_hist_1d, sma_50_1d, sma_200_1d,
        news
    );

    let request = match CreateChatCompletionRequestArgs::default()
        .model(&CONFIG.open_ai.model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_prompt)
                .build()
                .unwrap()
                .into(),
        ])
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to build request: {}", e);
            return;
        }
    };

    match openai_client.chat().create(request).await {
        Ok(ai_response) => {
            let content = ai_response
                .choices
                .first()
                .and_then(|c| c.message.content.clone())
                .unwrap_or_default();

            let content = content
                .trim_start_matches("```json\n")
                .trim_start_matches("```\n")
                .trim_end_matches("\n```")
                .trim_end_matches("```")
                .trim();

            match serde_json::from_str::<GptMarketAnalysis>(content) {
                Ok(analysis) => {
                    let emoji = match analysis.opportunity.as_str() {
                        "BUY" => "🟢",
                        "SELL" => "🔴",
                        _ => "⚪",
                    };

                    let msg_text = format!(
                        "{} *Stock Signal: {}* ({}%)\n\
                        *Asset:* {} | *Price:* ${:.2}\n\
                        *Regime:* {}\n\n\
                        *Hourly (Short-term):*\n\
                        \\- RSI: {:.2}\n\
                        \\- MACD Hist: {:.2}\n\n\
                        *Daily (Long-term):*\n\
                        \\- RSI: {:.2}\n\
                        \\- SMA 50: ${:.2} | SMA 200: ${:.2}\n\n\
                        *Analysis:*\n{}",
                        emoji,
                        crate::utils::markdown::escape_markdown_v2(&analysis.opportunity),
                        analysis.confidence_score,
                        crate::utils::markdown::escape_markdown_v2(symbol),
                        current_price,
                        crate::utils::markdown::escape_markdown_v2(&analysis.market_regime),
                        rsi_1h,
                        macd_hist_1h,
                        rsi_1d,
                        sma_50_1d,
                        sma_200_1d,
                        crate::utils::markdown::escape_markdown_v2(&analysis.analysis_reasoning)
                    );

                    if let Err(e) = bot
                        .send_message(target_user_id, msg_text)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .await
                    {
                        error!("Failed to send market signal to user: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to parse GPT response: {}\nResponse: {}", e, content);
                }
            }
        }
        Err(e) => {
            error!("OpenAI request failed: {}", e);
        }
    }
}

pub async fn start_stock_loop(bot: Bot, openai_client: OpenAiClient<OpenAIConfig>) {
    info!("Starting stock data fetcher service...");

    let client = HttpClient::new();
    let target_user_id = UserId(ENV.user_id);

    let assets = vec!["NVDA"];

    loop {        for symbol in &assets {
            analyze_asset(&bot, &openai_client, &client, symbol, target_user_id).await;
            tokio::time::sleep(Duration::from_secs(10)).await;
        }

        info!("Stock analysis cycle complete. Sleeping for 2 hours...");
        tokio::time::sleep(Duration::from_secs(7200)).await;
    }
}
