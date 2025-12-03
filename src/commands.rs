use std::sync::Arc;

use crate::{
    handlers, keyboard,
    types::{
        common::{
            BotDialogue, ChatHistoryState, Commands, DateFilter, HandleResult, TransactionKind,
        },
        databases::Database,
    },
};
use async_openai::{config::OpenAIConfig, Client};
use teloxide::prelude::*;

pub async fn commands(
    client: Client<OpenAIConfig>,
    state: ChatHistoryState,
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
    db: Arc<Database>,
    cmd: Commands,
) -> HandleResult {
    match cmd {
        Commands::Start => {
            keyboard::core::start(bot, msg).await?;
        }
        Commands::Help => {
            handlers::help::commands(bot, msg).await?;
        }
        Commands::Roll => {
            handlers::dice::roll(bot, msg).await?;
        }
        Commands::Prompt(prompt) => {
            handlers::gpt::prompt::set(prompt, bot, state, msg).await?;
        }
        Commands::Chat(content) => {
            handlers::gpt::chat::message(content, bot, client, state, msg).await?;
        }
        Commands::Enter => {
            handlers::gpt::chat::enter(bot, dialogue, msg).await?;
        }
        Commands::Exit => {
            handlers::gpt::chat::exit(bot, dialogue, msg).await?;
        }
        Commands::View => {
            handlers::gpt::history::view(bot, state, msg).await?;
        }
        Commands::Clear => {
            handlers::gpt::history::clear(bot, state, msg).await?;
        }
        Commands::Stats => {
            handlers::budgeting::statistics::overview(
                bot,
                msg.chat.id.to_string(),
                &db.transactions(),
                DateFilter::CurrentYear,
            )
            .await?;
        }
        Commands::Categories => {
            handlers::budgeting::categories::list(bot, msg, &db.categories()).await?;
        }
        Commands::AddIncomeCategory(category) => {
            handlers::budgeting::categories::add(
                category,
                TransactionKind::Income,
                bot,
                msg,
                &db.categories(),
            )
            .await?;
        }
        Commands::AddSpendingCategory(category) => {
            handlers::budgeting::categories::add(
                category,
                TransactionKind::Spending,
                bot,
                msg,
                &db.categories(),
            )
            .await?;
        }
        Commands::RemoveCategory(id) => {
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
