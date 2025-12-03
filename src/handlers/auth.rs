use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::{
    types::{CallbackQuery, Me},
    Bot,
};

use crate::types::common::{AppError, HandleResult};
use crate::types::{common::BotDialogue, databases::Database};

pub async fn unauthorized_access_callback(
    bot: Bot,
    _me: Me,
    _dialogue: BotDialogue,
    _db: Arc<Database>,
    q: CallbackQuery,
) -> HandleResult {
    if let Some(msg) = q.message {
        bot.send_message(msg.chat().id, "⛔ You are not authorized to use this bot.")
            .await
            .ok();
    }
    Ok::<(), AppError>(())
}

pub async fn unauthorized_access_command(
    bot: Bot,
    _me: Me,
    _dialogue: BotDialogue,
    msg: Message,
    _db: Arc<Database>,
) -> HandleResult {
    bot.send_message(msg.chat.id, "⛔ You are not authorized to use this bot.")
        .await
        .ok();

    Ok::<(), AppError>(())
}
