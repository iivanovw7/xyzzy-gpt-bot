use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::types::common::DateFilter;

pub fn create_date_filter_keyboard(prefix: &str) -> InlineKeyboardMarkup {
    let filters: Vec<DateFilter> = vec![
        DateFilter::Today,
        DateFilter::CurrentWeek,
        DateFilter::CurrentMonth,
        DateFilter::LastMonth,
        DateFilter::Last3Months,
        DateFilter::CurrentYear,
        DateFilter::AllTime,
    ];

    let rows: Vec<Vec<InlineKeyboardButton>> = filters
        .into_iter()
        .map(|date| {
            vec![InlineKeyboardButton::callback(
                date.label(),
                format!("{}:filter:{:?}", prefix, date),
            )]
        })
        .collect();

    InlineKeyboardMarkup::new(rows)
}
