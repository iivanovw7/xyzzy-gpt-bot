use crate::{
    config::CONFIG,
    env::ENV,
    types::market::GptMarketAnalysis,
    utils::{economic_indicators, finnhub, market_indicators::MarketHistory, marketaux},
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
pub struct YahooQuote {
    open: Vec<Option<f64>>,
    high: Vec<Option<f64>>,
    low: Vec<Option<f64>>,
    close: Vec<Option<f64>>,
    volume: Vec<Option<f64>>,
}

async fn fetch_yahoo_data(
    http_client: &HttpClient,
    symbol: &str,
    interval: &str,
    range: &str,
) -> anyhow::Result<(
    f64,
    Option<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
)> {
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

    let open_prices: Vec<f64> = quote.open.iter().filter_map(|&p| p).collect();
    let high_prices: Vec<f64> = quote.high.iter().filter_map(|&p| p).collect();
    let low_prices: Vec<f64> = quote.low.iter().filter_map(|&p| p).collect();
    let close_prices: Vec<f64> = quote.close.iter().filter_map(|&p| p).collect();
    let volumes: Vec<f64> = quote.volume.iter().filter_map(|&v| v).collect();

    Ok((
        current_price,
        current_volume,
        open_prices,
        high_prices,
        low_prices,
        close_prices,
        volumes,
    ))
}

async fn analyze_asset(
    bot: &Bot,
    openai_client: &OpenAiClient<OpenAIConfig>,
    http_client: &HttpClient,
    symbol: &str,
    target_user_id: UserId,
) {
    info!("Analyzing market data for {}...", symbol);

    let (current_price, current_volume, _, high_prices_1d, low_prices_1d, close_prices_1d, vols_1d) =
        match fetch_yahoo_data(http_client, symbol, "1d", "1y").await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to fetch 1d data for {}: {}", symbol, e);
                return;
            }
        };

    let (_, _, _, high_prices_1wk, low_prices_1wk, close_prices_1wk, vols_1wk) =
        match fetch_yahoo_data(http_client, symbol, "1wk", "5y").await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to fetch 1wk data for {}: {}", symbol, e);
                return;
            }
        };

    if close_prices_1d.len() < 30 || close_prices_1wk.len() < 30 {
        error!(
            "Not enough historical data for {}. 1d: {}, 1wk: {}",
            symbol,
            close_prices_1d.len(),
            close_prices_1wk.len()
        );
        return;
    }

    let mut hist_1d =
        MarketHistory::new_with_ohlc(close_prices_1d, high_prices_1d, low_prices_1d, vols_1d);
    hist_1d.update_price(current_price);
    if let Some(vol) = current_volume {
        hist_1d.update_volume(vol);
    }

    let rsi_1d = hist_1d.calculate_rsi();
    let (macd_line_1d, macd_signal_1d, macd_hist_1d) = hist_1d.calculate_macd();
    let sma_50_1d = hist_1d.calculate_sma(50);
    let sma_200_1d = hist_1d.calculate_sma(200);
    let atr_1d = hist_1d.calculate_atr(14);
    let historical_volatility_1d = hist_1d.calculate_historical_volatility(30);

    let mut hist_1wk =
        MarketHistory::new_with_ohlc(close_prices_1wk, high_prices_1wk, low_prices_1wk, vols_1wk);
    hist_1wk.update_price(current_price);
    if let Some(vol) = current_volume {
        hist_1wk.update_volume(vol);
    }

    let rsi_1wk = hist_1wk.calculate_rsi();
    let (macd_line_1wk, macd_signal_1wk, macd_hist_1wk) = hist_1wk.calculate_macd();
    let sma_50_1wk = hist_1wk.calculate_sma(50);
    let sma_200_1wk = hist_1wk.calculate_sma(200);
    let atr_1wk = hist_1wk.calculate_atr(14);
    let historical_volatility_1wk = hist_1wk.calculate_historical_volatility(30);

    let financials = match finnhub::fetch_basic_financials(http_client, symbol).await {
        Ok(data) => Some(data),
        Err(e) => {
            error!("Failed to fetch financials for {}: {}", symbol, e);
            None
        }
    };

    let gdp_data = match economic_indicators::get_gdp(http_client).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch GDP: {}", e);
            None
        }
    };

    let cpi_data = match economic_indicators::get_cpi(http_client).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch CPI: {}", e);
            None
        }
    };

    let fed_funds_rate_data = match economic_indicators::get_fed_funds_rate(http_client).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch Fed Funds Rate: {}", e);
            None
        }
    };

    let news = marketaux::fetch_stock_news(http_client, symbol)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to fetch news for {}: {}", symbol, e);
            "No recent news found.".to_string()
        });

    info!(
        "Successfully fetched {} data | Price: ${:.2} | 1d RSI: {:.2} | 1wk RSI: {:.2}",
        symbol, current_price, rsi_1d, rsi_1wk
    );

    let system_prompt = include_str!("../../prompts/market_analysis.md");

    let user_prompt = format!(
        "Asset: {}\n\
        Current Price: ${:.2}\n\
        Current Volume: {:.0}\n\n\
        [DAILY TIME FRAME (Medium-Term)]\n\
        RSI (14): {:.2}\n\
        MACD Line: {:.4} | Signal: {:.4} | Histogram: {:.4}\n\
        SMA 50: ${:.2} | SMA 200: ${:.2}\n\
        ATR (14): {:.2}\n\
        Historical Volatility (30d): {:.2}%\n\n\
        [WEEKLY TIME FRAME (Long-Term)]\n\
        RSI (14): {:.2}\n\
        MACD Line: {:.4} | Signal: {:.4} | Histogram: {:.4}\n\
        SMA 50: ${:.2} | SMA 200: ${:.2}\n\
        ATR (14): {:.2}\n\
        Historical Volatility (30d): {:.2}%\n\n\
        [FUNDAMENTAL DATA]\n\
        Basic Financials (latest quarter): {}\n\n\
        [MACROECONOMIC INDICATORS (Latest)]\n\
        GDP: {}\n\
        CPI: {}\n\
        Fed Funds Rate: {}\n\n\
        [ANALYSIS INSTRUCTIONS]\n\
        Given the provided multi-timeframe technical data, fundamental data, macroeconomic indicators, and recent news, provide a comprehensive market analysis suitable for a trading horizon of days to weeks. Focus on identifying strong trends, potential reversals, significant long-term catalysts (e.g., earnings, economic shifts), and potential risks. Explain how the different timeframes' indicators (daily, weekly), fundamental health, and economic environment align or diverge to support your long-term outlook. Crucially, assess the asset's volatility and suggest an appropriate position size based on the overall risk-reward profile and your confidence in the opportunity. Consider a position size as a percentage of a typical portfolio allocation.\n\n        Recent News: {}",
        symbol,
        current_price,
        current_volume.unwrap_or(0.0),
        rsi_1d,
        macd_line_1d,
        macd_signal_1d,
        macd_hist_1d,
        sma_50_1d,
        sma_200_1d,
        atr_1d.unwrap_or(0.0),
        historical_volatility_1d.unwrap_or(0.0) * 100.0,
        rsi_1wk,
        macd_line_1wk,
        macd_signal_1wk,
        macd_hist_1wk,
        sma_50_1wk,
        sma_200_1wk,
        atr_1wk.unwrap_or(0.0),
        historical_volatility_1wk.unwrap_or(0.0) * 100.0,
        financials
            .and_then(|f| {
                f.series.and_then(|s| {
                    s.quarterly.map(|q| {
                        format!(
                            "Revenue: {}, Net Income: {}, EPS: {}, Total Assets: {}, Total Liabilities: {}, Operating Cashflow: {}",
                            q.total_revenue.as_ref().and_then(|v| v.first()).map(|v| v.v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                            q.net_income.as_ref().and_then(|v| v.first()).map(|v| v.v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                            q.eps.as_ref().and_then(|v| v.first()).map(|v| v.v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                            q.total_assets.as_ref().and_then(|v| v.first()).map(|v| v.v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                            q.total_liabilities.as_ref().and_then(|v| v.first()).map(|v| v.v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                            q.operating_cash_flow.as_ref().and_then(|v| v.first()).map(|v| v.v.to_string()).unwrap_or_else(|| "N/A".to_string())
                        )
                    })
                })
            })
            .unwrap_or_else(|| "N/A".to_string()),
        gdp_data.unwrap_or_else(|| "N/A".to_string()),
        cpi_data.unwrap_or_else(|| "N/A".to_string()),
        fed_funds_rate_data.unwrap_or_else(|| "N/A".to_string()),
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
                        "{} <b>Stock Signal: {}</b> ({}%)\n\
                        <b>Asset:</b> {} | <b>Price:</b> ${:.2}\n\
                        <b>Regime:</b> {}\n\n\
                        <blockquote expandable><b>Daily (Medium-term):</b>\n\
                        - RSI: {:.2}\n\
                        - MACD Line: {:.4} | Signal: {:.4} | Histogram: {:.4}\n\
                        - SMA 50: ${:.2} | SMA 200: ${:.2}\n\
                        - ATR (14): {:.2}\n\
                        - Historical Volatility (30d): {:.2}%\n\n\
                        <b>Weekly (Long-term):</b>\n\
                        - RSI: {:.2}\n\
                        - MACD Line: {:.4} | Signal: {:.4} | Histogram: {:.4}\n\
                        - SMA 50: ${:.2} | SMA 200: ${:.2}\n\
                        - ATR (14): {:.2}\n\
                        - Historical Volatility (30d): {:.2}%\n\n\
                        <b>Fundamentals:</b> {}\n\
                        <b>Economic Outlook:</b> {}\n\
                        <b>Volatility:</b> {}\n\
                        <b>Suggested Position:</b> {}\n\n\
                        <b>Analysis:</b>\n{}</blockquote>",
                        emoji,
                        crate::utils::markdown::escape_html(&analysis.opportunity),
                        analysis.confidence_score,
                        crate::utils::markdown::escape_html(symbol),
                        current_price,
                        crate::utils::markdown::escape_html(&analysis.market_regime),
                        rsi_1d,
                        macd_line_1d,
                        macd_signal_1d,
                        macd_hist_1d,
                        sma_50_1d,
                        sma_200_1d,
                        atr_1d.unwrap_or(0.0),
                        historical_volatility_1d.unwrap_or(0.0) * 100.0,
                        rsi_1wk,
                        macd_line_1wk,
                        macd_signal_1wk,
                        macd_hist_1wk,
                        sma_50_1wk,
                        sma_200_1wk,
                        atr_1wk.unwrap_or(0.0),
                        historical_volatility_1wk.unwrap_or(0.0) * 100.0,
                        crate::utils::markdown::escape_html(&analysis.key_fundamentals),
                        crate::utils::markdown::escape_html(&analysis.economic_outlook),
                        crate::utils::markdown::escape_html(&analysis.volatility_metrics),
                        crate::utils::markdown::escape_html(&analysis.suggested_position_size),
                        crate::utils::markdown::escape_html(&analysis.analysis_reasoning)
                    );

                    if let Err(e) = bot
                        .send_message(target_user_id, msg_text)
                        .parse_mode(teloxide::types::ParseMode::Html)
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

    loop {
        for symbol in &assets {
            analyze_asset(&bot, &openai_client, &client, symbol, target_user_id).await;
            tokio::time::sleep(Duration::from_secs(10)).await;
        }

        info!("Stock analysis cycle complete. Sleeping for 2 hours...");
        tokio::time::sleep(Duration::from_secs(14400)).await;
    }
}
