use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

use crate::types::menu::BudgetingMenuItems;

pub fn create_budgeting_menu_keyboard() -> ReplyMarkup {
    let keyboard_rows: Vec<Vec<KeyboardButton>> = vec![
        vec![
            KeyboardButton::new(BudgetingMenuItems::Statistics),
            KeyboardButton::new(BudgetingMenuItems::AddExpense),
        ],
        vec![KeyboardButton::new(BudgetingMenuItems::Settings)],
        vec![KeyboardButton::new(BudgetingMenuItems::Back)],
    ];

    let custom_keyboard = KeyboardMarkup {
        keyboard: keyboard_rows,
        resize_keyboard: true,
        one_time_keyboard: false,
        is_persistent: true,
        input_field_placeholder: "Budgeting".to_string(),
        selective: false,
    };

    custom_keyboard.into()
}
