use std::sync::Arc;

use crate::{
    handlers, keyboard,
    types::{
        auth::AuthState,
        common::{BotDialogue, ChatHistoryState, Commands, HandleResult},
        databases::Database,
    },
};
use async_openai::{config::OpenAIConfig, Client};
use teloxide::prelude::*;

#[allow(clippy::too_many_arguments)]
pub async fn commands(
    client: Client<OpenAIConfig>,
    state: ChatHistoryState,
    _auth_state: AuthState,
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
        Commands::Delete => {
            handlers::budgeting::transactions::delete_last(bot, msg, &db.transactions()).await?;
        }
        Commands::Reset => {
            handlers::reset::reset(bot, msg).await?;
        }
        Commands::Links => {
            handlers::links::links::list_recent(bot, msg, &db).await?;
        }
        Commands::RemapCategory { from_id, to_id } => {
            handlers::budgeting::bot::remap_category_command(bot, msg, db, from_id, to_id).await?;
        }
    }

    Ok(())
}
