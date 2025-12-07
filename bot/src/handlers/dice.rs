use teloxide::prelude::*;

use crate::types::common::HandleResult;

pub async fn roll(bot: Bot, msg: Message) -> HandleResult {
    bot.send_dice(msg.chat.id).await?;

    Ok(())
}
