use crate::types::common::DateFilter;
use chrono::{Datelike, Duration, Local, NaiveDate};
use num_format::{Locale, ToFormattedString};

pub fn format_transaction_date(date: NaiveDate) -> String {
    date.format("%B %e, %Y").to_string()
}

pub fn format_transaction_amount(amount: i64, plus_sign: &str) -> String {
    let sign = if amount < 0 { "-" } else { plus_sign };
    let abs = amount.abs();

    let units = abs / 100;
    let cents = abs % 100;

    let units_formatted = units.to_formatted_string(&Locale::en);

    format!("{sign}{units_formatted}.{cents:02} €")
}

pub fn day_key_from_timestamp(ts: NaiveDate) -> String {
    format!("{}-{:02}-{:02}", ts.year(), ts.month(), ts.day())
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_month_year = if month == 12 { year + 1 } else { year };
    let first_next_month = NaiveDate::from_ymd_opt(next_month_year, next_month, 1).unwrap();

    (first_next_month - Duration::days(1)).day()
}

impl DateFilter {
    pub fn range(&self) -> (Option<NaiveDate>, Option<NaiveDate>) {
        let today = Local::now().date_naive();

        match self {
            DateFilter::Today => (Some(today), Some(today)),
            DateFilter::CurrentMonth => {
                let start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
                (Some(start), Some(today))
            }
            DateFilter::LastMonth => {
                let (year, month) = if today.month() == 1 {
                    (today.year() - 1, 12)
                } else {
                    (today.year(), today.month() - 1)
                };

                let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                let end =
                    NaiveDate::from_ymd_opt(year, month, last_day_of_month(year, month)).unwrap();

                (Some(start), Some(end))
            }
            DateFilter::Last3Months => {
                let mut year = today.year();
                let mut month = today.month() as i32;

                month -= 3;

                if month <= 0 {
                    year -= 1;
                    month += 12;
                }

                let start = NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap();

                (Some(start), Some(today))
            }
            DateFilter::CurrentYear => {
                let start = NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();

                (Some(start), Some(today))
            }
        }
    }
}
