use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    Bot,
};

use crate::{
    handlers,
    types::{
        common::{DateFilter, TransactionKind},
        databases::TransactionsDb,
    },
    utils::transactions::format_transaction_amount,
};

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

pub async fn create_transactions_suggestions_keyboard(
    bot: Bot,
    user_id: String,
    transactions_db: &TransactionsDb,
    search: &str,
) -> InlineKeyboardMarkup {
    let transactions =
        handlers::budgeting::transactions::search(bot, user_id, transactions_db, search)
            .await
            .unwrap();

    let rows: Vec<Vec<InlineKeyboardButton>> = transactions
        .into_iter()
        .map(|transaction| {
            let kind = if transaction.amount.is_positive() {
                TransactionKind::Income
            } else {
                TransactionKind::Spending
            };

            vec![InlineKeyboardButton::callback(
                format!(
                    "{} {} [{}] ",
                    format_transaction_amount(transaction.amount, "+"),
                    transaction.description,
                    transaction.category_name,
                ),
                format!(
                    "transactions:recent:{}:{}:{}:{}",
                    transaction.category_id,
                    transaction.category_name,
                    <TransactionKind as Into<&'static str>>::into(kind),
                    transaction.description,
                ),
            )]
        })
        .collect();

    InlineKeyboardMarkup::new(rows)
}
