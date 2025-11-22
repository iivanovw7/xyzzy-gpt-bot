use teloxide::prelude::*;

use crate::types::main::HandleResult;

pub async fn open(bot: Bot, msg: Message) -> HandleResult {
    bot.send_message(msg.chat.id, "Open settings").await?;

    Ok(())
}
