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
pub struct CoinGeckoResponse {
    #[serde(rename = "bitcoin")]
    pub btc: Option<CryptoData>,
}

#[derive(Deserialize, Debug)]
pub struct CryptoData {
    pub usd: f64,
    #[serde(rename = "usd_24h_vol")]
    pub usd_24h_vol: f64,
    #[serde(rename = "usd_24h_change")]
    pub usd_24h_change: f64,
}

#[derive(Deserialize, Debug)]
pub struct CoinGeckoHistory {
    pub prices: Vec<Vec<f64>>,
}

async fn fetch_coingecko_history(
    http_client: &HttpClient,
    days: &str,
) -> anyhow::Result<Vec<f64>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days={}&interval=hourly",
        days
    );
    let res = http_client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    let data: CoinGeckoHistory = res.json().await?;
    Ok(data.prices.iter().map(|item| item[1]).collect())
}

async fn fetch_and_analyze_crypto_data(
    bot: &Bot,
    openai_client: &OpenAiClient<OpenAIConfig>,
    http_client: &HttpClient,
    current_price_url: &str,
    target_user_id: UserId,
) {
    info!("Fetching live crypto trend data from CoinGecko...");

    let prices_1h = match fetch_coingecko_history(http_client, "90").await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to fetch 1h crypto history: {}", e);
            return;
        }
    };

    let prices_1d = match fetch_coingecko_history(http_client, "365").await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to fetch 1d crypto history: {}", e);
            return;
        }
    };

    if prices_1h.len() < 30 || prices_1d.len() < 30 {
        error!(
            "Not enough historical data from CoinGecko. 1h: {}, 1d: {}",
            prices_1h.len(), prices_1d.len()
        );
        return;
    }

    let mut hist_1h = MarketHistory::new(prices_1h);
    let mut hist_1d = MarketHistory::new(prices_1d);

    if let Ok(response) = http_client
        .get(current_price_url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
    {
        if let Ok(data) = response.json::<CoinGeckoResponse>().await {
            if let Some(btc) = data.btc {
                hist_1h.update_price(btc.usd);
                hist_1d.update_price(btc.usd);

                let rsi_1h = hist_1h.calculate_rsi();
                let (macd_line_1h, macd_signal_1h, macd_hist_1h) = hist_1h.calculate_macd();
                let sma_50_1h = hist_1h.calculate_sma(50);

                let rsi_1d = hist_1d.calculate_rsi();
                let (macd_line_1d, macd_signal_1d, macd_hist_1d) = hist_1d.calculate_macd();
                let sma_50_1d = hist_1d.calculate_sma(50);
                let sma_200_1d = hist_1d.calculate_sma(200);

                let news = fetch_market_news(http_client, "BTC-USD").await;

                info!(
                    "Successfully fetched crypto data | Price: ${:.2} | 1h RSI: {:.2} | 1d RSI: {:.2}",
                    btc.usd, rsi_1h, rsi_1d
                );

                let system_prompt = include_str!("../../prompts/crypto_analysis.md");

                let user_prompt = format!(
                    "Asset: BTC/USD\n\
                    Current Price: ${:.2}\n\
                    24h Volume: ${:.2}\n\
                    24h Price Change: {:.2}%\n\n\
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
                    btc.usd, btc.usd_24h_vol, btc.usd_24h_change, 
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
                                    "{} *Crypto Signal: {}* ({}%)\n\
                                    *Asset:* BTC/USD | *Price:* ${:.2}\n\
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
                                    btc.usd,
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
                                    error!("Failed to send crypto signal to user: {}", e);
                                }
                            }
                            Err(e) => {
                                error!(
                                    "Failed to parse GPT response: {}\nResponse: {}",
                                    e, content
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("OpenAI request failed: {}", e);
                    }
                }
            }
        } else {
            error!("Failed to parse CoinGeckoResponse");
        }
    } else {
        error!("Failed to fetch current crypto price");
    }
}

pub async fn start_crypto_loop(bot: Bot, openai_client: OpenAiClient<OpenAIConfig>) {
    info!("Starting crypto data fetcher service with live indicators...");

    let client = HttpClient::new();
    let current_price_url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_24hr_vol=true&include_24hr_change=true";
    let target_user_id = UserId(ENV.user_id);

    loop {
        fetch_and_analyze_crypto_data(
            &bot,
            &openai_client,
            &client,
            current_price_url,
            target_user_id,
        )
        .await;

        info!("Crypto analysis cycle complete. Sleeping for 2 hours...");
        tokio::time::sleep(Duration::from_secs(7200)).await;
    }
}
