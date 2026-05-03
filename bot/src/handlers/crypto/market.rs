use crate::{
    config::CONFIG,
    env::ENV,
    utils::market_indicators::{CoinGeckoHistory, MarketHistory},
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
    #[serde(rename = "the-open-network")]
    pub ton: Option<TonData>,
}

#[derive(Deserialize, Debug)]
pub struct TonData {
    pub usd: f64,
    #[serde(rename = "usd_24h_vol")]
    pub usd_24h_vol: f64,
    #[serde(rename = "usd_24h_change")]
    pub usd_24h_change: f64,
}

#[derive(Deserialize, Debug)]
pub struct GptCryptoMetrics {
    pub current_price: f64,
    pub rsi: f64,
    pub bollinger_band_status: String,
    pub macd_line: f64,
    pub macd_signal: f64,
    pub macd_histogram: f64,
    pub stoch_k: f64,
    pub stoch_d: f64,
}

#[derive(Deserialize, Debug)]
pub struct GptCryptoAnalysis {
    pub opportunity: String,
    pub confidence_score: u8,
    pub asset: String,
    pub metrics: GptCryptoMetrics,
    pub analysis_reasoning: String,
    pub recommended_action_prompt: String,
}

async fn fetch_and_analyze_market_data(
    bot: &Bot,
    openai_client: &OpenAiClient<OpenAIConfig>,
    http_client: &HttpClient,
    current_price_url: &str,
    history_url: &str,
    target_user_id: UserId,
) {
    info!("Fetching live trend data from CoinGecko...");

    let mut initial_prices = Vec::new();

    if let Ok(history_res) = http_client
        .get(history_url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
    {
        if let Ok(history_data) = history_res.json::<CoinGeckoHistory>().await {
            initial_prices = history_data.prices.iter().map(|item| item[1]).collect();
        } else {
            error!("Failed to parse history data");
        }
    } else {
        error!("Failed to fetch history data");
    }

    if initial_prices.len() < 35 {
        error!(
            "Not enough historical data from CoinGecko. Skipping market evaluation for this cycle."
        );
        return;
    }

    let mut history = MarketHistory::new(initial_prices);

    if let Ok(response) = http_client
        .get(current_price_url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
    {
        if let Ok(data) = response.json::<CoinGeckoResponse>().await {
            if let Some(ton) = data.ton {
                history.update_price(ton.usd);

                let rsi = history.calculate_rsi();
                let (lower, mid, upper, bb_status) = history.calculate_bollinger_bands();
                let (macd_line, macd_signal, macd_histogram) = history.calculate_macd();
                let (stoch_k, stoch_d) = history.calculate_stochastic_oscillator();

                info!(
                    "Successfully fetched market data | RSI: {:.2} | Price: ${:.4} | MACD: {:.2} | Stoch %K: {:.2}",
                    rsi, ton.usd, macd_line, stoch_k
                );

                let system_prompt = include_str!("../../prompts/crypto_analysis.md");

                let user_prompt = format!(
                    "Asset: TON/USDT\nCurrent Price: ${:.4}\n24h Volume: ${:.2}\n24h Price Change: {:.2}%\nRSI (14-period): {:.2}\nBollinger Band Status: {}\nLower Band: ${:.4}\nMiddle Band: ${:.4}\nUpper Band: ${:.4}\nMACD Line: {:.4}\nMACD Signal: {:.4}\nMACD Histogram: {:.4}\nStochastic %K: {:.2}\nStochastic %D: {:.2}",
                    ton.usd, ton.usd_24h_vol, ton.usd_24h_change, rsi, bb_status, lower, mid, upper, macd_line, macd_signal, macd_histogram, stoch_k, stoch_d
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

                        match serde_json::from_str::<GptCryptoAnalysis>(content) {
                            Ok(analysis) => {
                                let emoji = match analysis.opportunity.as_str() {
                                    "BUY" => "🟢",
                                    "SELL" => "🔴",
                                    _ => "⚪",
                                };

                                let msg_text = format!(
                                    "{} *Trading Signal: {}*\n\n\
                                    *Asset:* {}\n\
                                    *Price:* ${}\n\
                                    *Confidence:* {}%\n\n\
                                    *Metrics:*\n\
                                    \\- RSI: {}\n\
                                    \\- BB Status: {}\n\
                                    \\- MACD: {}\n\
                                    \\- MACD Signal: {}\n\
                                    \\- MACD Histogram: {}\n\
                                    \\- Stochastic %K: {}\n\
                                    \\- Stochastic %D: {}\n\n\
                                    *Analysis:*\n{}\n\n\
                                    *Action:*\n{}",
                                    emoji,
                                    crate::utils::markdown::escape_markdown_v2(
                                        &analysis.opportunity
                                    ),
                                    crate::utils::markdown::escape_markdown_v2(&analysis.asset),
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.4}",
                                        analysis.metrics.current_price
                                    )),
                                    analysis.confidence_score,
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.2}",
                                        analysis.metrics.rsi
                                    )),
                                    crate::utils::markdown::escape_markdown_v2(
                                        &analysis.metrics.bollinger_band_status
                                    ),
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.4}",
                                        analysis.metrics.macd_line
                                    )),
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.4}",
                                        analysis.metrics.macd_signal
                                    )),
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.4}",
                                        analysis.metrics.macd_histogram
                                    )),
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.2}",
                                        analysis.metrics.stoch_k
                                    )),
                                    crate::utils::markdown::escape_markdown_v2(&format!(
                                        "{:.2}",
                                        analysis.metrics.stoch_d
                                    )),
                                    crate::utils::markdown::escape_markdown_v2(
                                        &analysis.analysis_reasoning
                                    ),
                                    crate::utils::markdown::escape_markdown_v2(
                                        &analysis.recommended_action_prompt
                                    )
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
        error!("Failed to fetch current price");
    }
}

pub async fn start_market_loop(bot: Bot, openai_client: OpenAiClient<OpenAIConfig>) {
    info!("Starting market data fetcher service with live indicators...");

    let client = HttpClient::new();
    let current_price_url = "https://api.coingecko.com/api/v3/simple/price?ids=the-open-network&vs_currencies=usd&include_24hr_vol=true&include_24hr_change=true";
    let history_url = "https://api.coingecko.com/api/v3/coins/the-open-network/market_chart?vs_currency=usd&days=90&interval=hourly";
    let target_user_id = UserId(ENV.user_id);

    fetch_and_analyze_market_data(
        &bot,
        &openai_client,
        &client,
        current_price_url,
        history_url,
        target_user_id,
    )
    .await;

    loop {
        tokio::time::sleep(Duration::from_secs(7200)).await;
        fetch_and_analyze_market_data(
            &bot,
            &openai_client,
            &client,
            current_price_url,
            history_url,
            target_user_id,
        )
        .await;
    }
}
