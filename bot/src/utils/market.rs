use chrono::{DateTime, Datelike, LocalResult, NaiveDate, TimeZone, Timelike, Utc, Weekday};

use async_openai::{config::OpenAIConfig, Client as OpenAiClient};

use chrono_tz::{America::New_York, Tz};

use futures::future::BoxFuture;
use reqwest::Client as HttpClient;
use teloxide::{types::UserId, Bot};
use tokio::time::{sleep, Duration};
use tracing::info;

use crate::{config::CONFIG, env::ENV};

pub async fn start_stock_loop<F>(bot: Bot, openai_client: &OpenAiClient<OpenAIConfig>, analyzer: F)
where
    F: for<'a> Fn(
        &'a Bot,
        &'a OpenAiClient<OpenAIConfig>,
        &'a HttpClient,
        &'a str,
        UserId,
    ) -> BoxFuture<'a, ()>,
{
    info!("Starting stock loop...");

    let client = HttpClient::new();
    let user_id = UserId(ENV.user_id);

    loop {
        let now = market_now();

        if !is_market_open(now) {
            let sleep_duration = duration_until_next_market_open(now);

            info!(
                "Market is closed. Sleeping until next open for {:?}",
                sleep_duration
            );

            sleep(sleep_duration).await;

            continue;
        }

        info!("Market open. Running analysis loop");

        for symbol in CONFIG.market.stocks_symbols.clone() {
            analyzer(&bot, openai_client, &client, symbol.as_str(), user_id).await;

            sleep(Duration::from_secs(10)).await;
        }

        info!(
            "Cycle complete. Sleeping {} minutes.",
            CONFIG.market.stocks_analysis_interval_sec / 60
        );

        sleep(Duration::from_secs(
            CONFIG.market.stocks_analysis_interval_sec,
        ))
        .await;
    }
}

fn market_now() -> DateTime<Tz> {
    Utc::now().with_timezone(&New_York)
}

fn is_weekend(weekday: Weekday) -> bool {
    matches!(weekday, Weekday::Sat | Weekday::Sun)
}

fn is_market_open(now: DateTime<Tz>) -> bool {
    if is_weekend(now.weekday()) {
        return false;
    }

    let minutes = u64::from(now.hour() * 60 + now.minute());
    let market_open =
        CONFIG.market.stocks_market_open_hour * 60 + CONFIG.market.stocks_market_open_minute;
    let market_close =
        CONFIG.market.stocks_market_close_hour * 60 + CONFIG.market.stocks_market_close_minute;

    minutes >= market_open && minutes < market_close
}

fn duration_until_next_market_open(now: DateTime<Tz>) -> Duration {
    let next_open = next_market_open(now);

    let duration = next_open - now;

    Duration::from_secs(duration.num_seconds().max(0) as u64)
}

fn next_market_open(now: DateTime<Tz>) -> DateTime<Tz> {
    let today = now.date_naive();

    if !is_weekend(now.weekday()) {
        let today_open = market_open_daytime(today);

        if now < today_open {
            return today_open;
        }
    }

    let mut next_day = today.succ_opt().expect("valid next day");

    loop {
        let weekday = next_day.weekday();

        if !is_weekend(weekday) {
            return market_open_daytime(next_day);
        }

        next_day = next_day.succ_opt().expect("valid vext date")
    }
}

fn market_open_daytime(date: NaiveDate) -> DateTime<Tz> {
    let open_hour: u32 = CONFIG
        .market
        .stocks_market_open_hour
        .try_into()
        .expect("invalid market hour");

    let open_min: u32 = CONFIG
        .market
        .stocks_market_open_minute
        .try_into()
        .expect("invalid market open minute");

    let naive_date = date
        .and_hms_opt(open_hour, open_min, 0)
        .expect("Invalid market open time");

    match New_York.from_local_datetime(&naive_date) {
        LocalResult::Single(dt) => dt,
        _ => panic!("failed to construct NY market open datetime"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ny_datetime(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Tz> {
        let naive = NaiveDate::from_ymd_opt(year, month, day)
            .expect("valid date")
            .and_hms_opt(hour, minute, 0)
            .expect("valid time");

        match New_York.from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt,
            _ => panic!("failed to create datetime"),
        }
    }

    #[test]
    fn weekend_detection() {
        assert!(is_weekend(Weekday::Sat));
        assert!(is_weekend(Weekday::Sun));

        assert!(!is_weekend(Weekday::Mon));
        assert!(!is_weekend(Weekday::Tue));
        assert!(!is_weekend(Weekday::Wed));
        assert!(!is_weekend(Weekday::Thu));
        assert!(!is_weekend(Weekday::Fri));
    }

    #[test]
    fn market_is_closed_on_weekend() {
        let saturday = ny_datetime(2026, 5, 9, 10, 0);

        assert!(!is_market_open(saturday));
    }

    #[test]
    fn market_is_open_during_regular_hours() {
        let monday = ny_datetime(
            2026,
            5,
            11,
            CONFIG.market.stocks_market_open_hour.try_into().unwrap(),
            CONFIG.market.stocks_market_open_minute.try_into().unwrap(),
        );

        assert!(is_market_open(monday));
    }

    #[test]
    fn market_is_closed_before_open() {
        let open_hour: u32 = CONFIG.market.stocks_market_open_hour.try_into().unwrap();

        let monday = ny_datetime(2026, 5, 11, open_hour.saturating_sub(1), 0);

        assert!(!is_market_open(monday));
    }

    #[test]
    fn market_is_closed_after_close() {
        let close_hour: u32 = CONFIG.market.stocks_market_close_hour.try_into().unwrap();

        let monday = ny_datetime(2026, 5, 11, close_hour.saturating_add(1), 0);

        assert!(!is_market_open(monday));
    }

    #[test]
    fn next_market_open_before_open_today() {
        let open_hour: u32 = CONFIG.market.stocks_market_open_hour.try_into().unwrap();

        let now = ny_datetime(2026, 5, 11, open_hour.saturating_sub(1), 0);

        let next = next_market_open(now);

        assert_eq!(next.date_naive(), now.date_naive());
        assert_eq!(next.hour(), open_hour);
    }

    #[test]
    fn next_market_open_after_close_moves_to_next_weekday() {
        let close_hour: u32 = CONFIG.market.stocks_market_close_hour.try_into().unwrap();
        let open_hour: u32 = CONFIG.market.stocks_market_open_hour.try_into().unwrap();

        let friday_after_close = ny_datetime(2026, 5, 8, close_hour.saturating_add(1), 0);

        let next = next_market_open(friday_after_close);

        assert_eq!(next.weekday(), Weekday::Mon);
        assert_eq!(next.hour(), open_hour);
    }

    #[test]
    fn next_market_open_skips_weekend() {
        let saturday = ny_datetime(2026, 5, 9, 12, 0);

        let next = next_market_open(saturday);

        assert_eq!(next.weekday(), Weekday::Mon);
    }

    #[test]
    fn duration_until_next_market_open_is_positive() {
        let saturday = ny_datetime(2026, 5, 9, 12, 0);

        let duration = duration_until_next_market_open(saturday);

        assert!(duration.as_secs() > 0);
    }

    #[test]
    fn market_open_daytime_constructs_expected_time() {
        let date = NaiveDate::from_ymd_opt(2026, 5, 11).unwrap();

        let dt = market_open_daytime(date);

        let expected_hour: u32 = CONFIG.market.stocks_market_open_hour.try_into().unwrap();

        let expected_min: u32 = CONFIG.market.stocks_market_open_minute.try_into().unwrap();

        assert_eq!(dt.hour(), expected_hour);
        assert_eq!(dt.minute(), expected_min);
        assert_eq!(dt.weekday(), Weekday::Mon);
    }
}
