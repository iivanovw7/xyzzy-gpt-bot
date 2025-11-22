use crate::{
    handlers, menu,
    types::main::{
        BotDialogue, ChatHistoryState, ConfigParameters, HandleResult, MaintainerCommands,
        PublicCommands,
    },
};
use async_openai::{config::OpenAIConfig, Client};
use teloxide::{prelude::*, types::Me};

pub async fn public_commands(
    cfg: ConfigParameters,
    bot: Bot,
    me: Me,
    _dialogue: BotDialogue,
    msg: Message,
    cmd: PublicCommands,
) -> HandleResult {
    match cmd {
        PublicCommands::Start => {
            menu::main::start(bot, msg).await?;
        }
        PublicCommands::Help => {
            handlers::help::commands(cfg, bot, me, msg).await?;
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
    state: ChatHistoryState,
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
    cmd: MaintainerCommands,
) -> HandleResult {
    match cmd {
        MaintainerCommands::Prompt(prompt) => {
            handlers::gpt::prompt::set(prompt, bot, state, msg).await?;
        }
        MaintainerCommands::Chat(content) => {
            handlers::gpt::chat::message(content, bot, client, state, msg).await?;
        }
        MaintainerCommands::Enter => {
            handlers::gpt::chat::enter(bot, dialogue, msg).await?;
        }
        MaintainerCommands::Exit => {
            handlers::gpt::chat::exit(bot, dialogue, msg).await?;
        }
        MaintainerCommands::View => {
            handlers::gpt::history::view(bot, state, msg).await?;
        }
        MaintainerCommands::Clear => {
            handlers::gpt::history::clear(bot, state, msg).await?;
        }
        MaintainerCommands::Stats => {
            handlers::budgeting::statistics::overview(bot, msg).await?;
        }
        MaintainerCommands::Add => {
            handlers::budgeting::records::add(bot, msg).await?;
        }
    }

    Ok(())
}
