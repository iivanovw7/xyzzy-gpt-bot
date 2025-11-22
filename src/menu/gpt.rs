use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

use crate::types::menu::OpenAIMenuItems;

pub fn create_gpt_menu_keyboard() -> ReplyMarkup {
    let keyboard_rows: Vec<Vec<KeyboardButton>> = vec![
        vec![
            KeyboardButton::new(OpenAIMenuItems::StartChat),
            KeyboardButton::new(OpenAIMenuItems::EnterChatMode),
            KeyboardButton::new(OpenAIMenuItems::ExitChatMode),
        ],
        vec![
            KeyboardButton::new(OpenAIMenuItems::SetPrompt),
            KeyboardButton::new(OpenAIMenuItems::ViewHistory),
        ],
        vec![KeyboardButton::new(OpenAIMenuItems::ClearHistory)],
        vec![KeyboardButton::new(OpenAIMenuItems::Back)],
    ];

    let custom_keyboard = KeyboardMarkup {
        keyboard: keyboard_rows,
        resize_keyboard: true,
        one_time_keyboard: false,
        is_persistent: true,
        input_field_placeholder: "Configure your AI Chat...".to_string(),
        selective: false,
    };

    custom_keyboard.into()
}
