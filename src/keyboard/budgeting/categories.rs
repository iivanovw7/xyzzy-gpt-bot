use strum::EnumProperty;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, ReplyMarkup,
};

use crate::types::{
    common::TransactionKind, databases::CategoriesDb, keyboard::BudgetingCategoriesMenuItems,
};

pub fn create_budgeting_categories_menu_keyboard() -> ReplyMarkup {
    let keyboard_rows: Vec<Vec<KeyboardButton>> = vec![
        vec![
            KeyboardButton::new(BudgetingCategoriesMenuItems::Add),
            KeyboardButton::new(BudgetingCategoriesMenuItems::Remove),
        ],
        vec![KeyboardButton::new(BudgetingCategoriesMenuItems::List)],
        vec![KeyboardButton::new(BudgetingCategoriesMenuItems::Back)],
    ];

    let custom_keyboard = KeyboardMarkup {
        keyboard: keyboard_rows,
        resize_keyboard: true,
        one_time_keyboard: false,
        is_persistent: true,
        input_field_placeholder: "Budgeting categories".to_string(),
        selective: false,
    };

    custom_keyboard.into()
}

pub fn create_kinds_keyboard(prefix: &str) -> InlineKeyboardMarkup {
    let kinds: Vec<TransactionKind> = vec![TransactionKind::Income, TransactionKind::Spending];

    let rows: Vec<Vec<InlineKeyboardButton>> = kinds
        .into_iter()
        .map(|kind| {
            vec![InlineKeyboardButton::callback(
                kind.get_str("label").unwrap(),
                format!(
                    "{}:{}",
                    prefix,
                    <TransactionKind as Into<&'static str>>::into(kind)
                ),
            )]
        })
        .collect();

    InlineKeyboardMarkup::new(rows)
}

pub async fn create_categories_keyboard(
    prefix: &str,
    categories_db: &CategoriesDb,
) -> InlineKeyboardMarkup {
    let kinds: Vec<TransactionKind> = vec![TransactionKind::Income, TransactionKind::Spending];

    let mut rows: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    for kind in kinds {
        rows.push(vec![InlineKeyboardButton::callback(
            kind.get_str("label").unwrap(),
            "ignore",
        )]);

        let mut categories = categories_db.list(kind).await;

        categories.sort_by(|a, b| a.id.cmp(&b.id));

        for category in categories {
            let id = category.id;
            let name = category.name.clone();

            rows.push(vec![InlineKeyboardButton::callback(
                name,
                format!("{}:{}", prefix, id),
            )]);
        }
    }

    InlineKeyboardMarkup::new(rows)
}
