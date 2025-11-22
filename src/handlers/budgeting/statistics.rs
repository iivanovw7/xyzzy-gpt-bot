use teloxide::prelude::*;

use crate::types::main::HandleResult;

pub async fn overview(bot: Bot, msg: Message) -> HandleResult {
    bot.send_message(msg.chat.id, "Bugeting statistics overview.")
        .await?;

    Ok(())
}
