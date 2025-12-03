use std::sync::Arc;

use crate::{
    handlers, keyboard,
    types::{
        common::{
            BotDialogue, ChatHistoryState, ConfigParameters, DateFilter, HandleResult,
            MaintainerCommands, PublicCommands, TransactionKind,
        },
        databases::Database,
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
    _db: Arc<Database>,
    cmd: PublicCommands,
) -> HandleResult {
    match cmd {
        PublicCommands::Start => {
            keyboard::core::start(bot, msg).await?;
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
    db: Arc<Database>,
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
            handlers::budgeting::statistics::overview(
                bot,
                msg.chat.id.to_string(),
                &db.transactions(),
                DateFilter::CurrentYear,
            )
            .await?;
        }
        MaintainerCommands::Categories => {
            handlers::budgeting::categories::list(bot, msg, &db.categories()).await?;
        }
        MaintainerCommands::AddIncomeCategory(category) => {
            handlers::budgeting::categories::add(
                category,
                TransactionKind::Income,
                bot,
                msg,
                &db.categories(),
            )
            .await?;
        }
        MaintainerCommands::AddSpendingCategory(category) => {
            handlers::budgeting::categories::add(
                category,
                TransactionKind::Spending,
                bot,
                msg,
                &db.categories(),
            )
            .await?;
        }
        MaintainerCommands::RemoveCategory(id) => {
            handlers::budgeting::categories::remove(
                id,
                &db.categories(),
                &db.transactions(),
                bot,
                msg.chat.id.to_string(),
            )
            .await?
        }
    }

    Ok(())
}
