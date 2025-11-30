use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

use crate::types::{
    common::{AppError, BotDialogue, DialogueState},
    keyboard::OpenAIMenuItems,
};

pub async fn create_gpt_menu_keyboard(dialogue: BotDialogue) -> Result<ReplyMarkup, AppError> {
    let state = dialogue.get_or_default().await?;

    let mut chat_controls = vec![KeyboardButton::new(OpenAIMenuItems::StartChat)];

    match state {
        DialogueState::InChatMode => {
            chat_controls.push(KeyboardButton::new(OpenAIMenuItems::ExitChatMode));
        }
        _ => {
            chat_controls.push(KeyboardButton::new(OpenAIMenuItems::EnterChatMode));
        }
    }

    let keyboard_rows: Vec<Vec<KeyboardButton>> = vec![
        chat_controls,
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

    Ok(custom_keyboard.into())
}
