use teloxide::prelude::*;
use teloxide::{types::CallbackQuery, Bot};

use crate::types::common::{AppError, HandleResult};

pub async fn unauthorized_access_callback(bot: Bot, q: CallbackQuery) -> HandleResult {
    if let Some(msg) = q.message {
        bot.send_message(msg.chat().id, "⛔ You are not authorized to use this bot.")
            .await
            .ok();
    }
    Ok::<(), AppError>(())
}

pub async fn unauthorized_access_command(bot: Bot, msg: Message) -> HandleResult {
    bot.send_message(msg.chat.id, "⛔ You are not authorized to use this bot.")
        .await
        .ok();

    Ok::<(), AppError>(())
}
