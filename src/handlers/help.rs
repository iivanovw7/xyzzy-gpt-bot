use crate::types::common::{Commands, HandleResult};
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

pub async fn commands(bot: Bot, msg: Message) -> HandleResult {
    bot.send_message(msg.chat.id, Commands::descriptions().to_string())
        .await?;

    Ok(())
}
