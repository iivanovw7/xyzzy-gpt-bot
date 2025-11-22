use teloxide::prelude::*;

use crate::types::main::HandleResult;

pub async fn add(bot: Bot, msg: Message) -> HandleResult {
    bot.send_message(msg.chat.id, "Add budgeting record.")
        .await?;

    Ok(())
}
