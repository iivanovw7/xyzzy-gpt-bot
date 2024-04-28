use crate::{
    handlers,
    types::{ConfigParameters, HandleResult, MaintainerCommands, PublicCommands, State},
};
use async_openai::{config::OpenAIConfig, Client};
use teloxide::{prelude::*, types::Me};

pub async fn public_commands(
    cfg: ConfigParameters,
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: PublicCommands,
) -> HandleResult {
    let my_id = format!("{}", msg.from().unwrap().id);

    match cmd {
        PublicCommands::Help => {
            handlers::help::commands(cfg, bot, me, msg).await?;
        }
        PublicCommands::MyId => {
            bot.send_message(msg.chat.id, my_id).await?;
        }
        PublicCommands::Roll => {
            handlers::dice::roll(bot, msg).await?;
        }
        PublicCommands::Maintainer => {
            handlers::maintainer::log(cfg, bot, msg).await?;
        }
    }

    Ok(())
}

pub async fn maintainer_commands(
    client: Client<OpenAIConfig>,
    state: State,
    bot: Bot,
    msg: Message,
    cmd: MaintainerCommands,
) -> HandleResult {
    match cmd {
        MaintainerCommands::Prompt(prompt) => {
            handlers::prompt::set(prompt, bot, state, msg).await?;
        }
        MaintainerCommands::Chat(content) => {
            handlers::chat::message(content, bot, client, state, msg).await?;
        }
        MaintainerCommands::View => {
            handlers::history::view(bot, state, msg).await?;
        }
        MaintainerCommands::Clear => {
            handlers::history::clear(bot, state, msg).await?;
        }
    }

    Ok(())
}
