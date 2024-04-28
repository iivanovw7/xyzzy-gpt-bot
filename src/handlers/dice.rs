use crate::types::HandleResult;
use teloxide::prelude::*;

pub async fn roll(bot: Bot, msg: Message) -> HandleResult {
    bot.send_dice(msg.chat.id).await?;

    Ok(())
}
