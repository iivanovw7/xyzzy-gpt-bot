use crate::types::{ConfigParameters, HandleResult, MaintainerCommands, PublicCommands};
use teloxide::{prelude::*, types::Me, utils::command::BotCommands};

pub async fn commands(cfg: ConfigParameters, bot: Bot, me: Me, msg: Message) -> HandleResult {
    let is_maintainer = msg.from().unwrap().id == cfg.bot_maintainer;
    let is_group_or_supergroup = msg.chat.is_group() || msg.chat.is_supergroup();

    if is_maintainer {
        bot.send_message(
            msg.chat.id,
            format!(
                "{}\n\n{}",
                PublicCommands::descriptions(),
                MaintainerCommands::descriptions()
            ),
        )
        .await?;
    } else if is_group_or_supergroup {
        bot.send_message(
            msg.chat.id,
            PublicCommands::descriptions()
                .username_from_me(&me)
                .to_string(),
        )
        .await?;
    } else {
        bot.send_message(msg.chat.id, PublicCommands::descriptions().to_string())
            .await?;
    }

    Ok(())
}
