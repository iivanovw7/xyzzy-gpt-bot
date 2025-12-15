use teloxide::prelude::*;
use teloxide::requests::Requester;
use teloxide::types::MenuButton;
use teloxide::types::{BotCommand, Message};

use crate::types::common::HandleResult;

pub async fn reset(bot: Bot, msg: Message) -> HandleResult {
    let commands = vec![BotCommand::new(
        "start",
        "Start the bot and show the main menu",
    )];

    bot.set_my_commands(commands)
        .await
        .expect("Failed to set default bot commands.");

    let commands_menu_button = MenuButton::Commands;

    bot.set_chat_menu_button()
        .chat_id(msg.chat.id)
        .menu_button(commands_menu_button.clone())
        .await
        .expect("Failed to set default menu button.");

    bot.send_message(
        msg.chat.id,
        "✅ Configuration reset. \n\n Please run /start now to re-activate menu.",
    )
    .await?;

    Ok(())
}
