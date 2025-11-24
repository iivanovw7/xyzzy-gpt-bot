use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

use crate::types::menu::BudgetingCategoriesMenuItems;

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
