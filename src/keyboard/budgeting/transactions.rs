use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::types::common::DateFilter;

pub fn create_transactions_date_filter_keyboard() -> InlineKeyboardMarkup {
    let filters: Vec<DateFilter> = vec![
        DateFilter::CurrentMonth,
        DateFilter::LastMonth,
        DateFilter::Last3Months,
    ];

    let rows: Vec<Vec<InlineKeyboardButton>> = filters
        .into_iter()
        .map(|date| {
            vec![InlineKeyboardButton::callback(
                date.label(),
                format!("transactions:filter:{:?}", date),
            )]
        })
        .collect();

    InlineKeyboardMarkup::new(rows)
}
