use teloxide::prelude::*;

use crate::types::main::{ConfigParameters, HandleResult};

pub async fn log(cfg: ConfigParameters, bot: Bot, msg: Message) -> HandleResult {
    let is_maintainer = msg.from.unwrap().id == cfg.bot_maintainer;

    if is_maintainer {
        bot.send_message(msg.chat.id, "Maintainer is you!".to_string())
            .await?;
    } else if let Some(username) = cfg.maintainer_username {
        bot.send_message(msg.chat.id, format!("Maintainer is @{username}"))
            .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            format!("Maintainer ID is {}", cfg.bot_maintainer),
        )
        .await?;
    }

    Ok(())
}
