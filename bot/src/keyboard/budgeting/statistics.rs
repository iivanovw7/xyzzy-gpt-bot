use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::types::common::DateFilter;

pub fn create_statistics_date_filter_keyboard() -> InlineKeyboardMarkup {
    let filters: Vec<DateFilter> = vec![
        DateFilter::Today,
        DateFilter::CurrentMonth,
        DateFilter::LastMonth,
        DateFilter::Last3Months,
        DateFilter::CurrentYear,
    ];

    let rows: Vec<Vec<InlineKeyboardButton>> = filters
        .into_iter()
        .map(|date| {
            vec![InlineKeyboardButton::callback(
                date.label(),
                format!("statistics:filter:{:?}", date),
            )]
        })
        .collect();

    InlineKeyboardMarkup::new(rows)
}
