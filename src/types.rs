use async_openai::{config::OpenAIConfig, error::OpenAIError, types::ChatCompletionRequestMessage};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use teloxide::{prelude::*, utils::command::BotCommands, RequestError};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("OpenAI api error")]
    OpenAI(#[from] OpenAIError),
    #[error("Teloxide error")]
    Teloxide(#[from] RequestError),
    #[error("Access denied")]
    AccessDenied(String),
    #[error("Internal error")]
    InternalError(String),
}

#[derive(Clone)]
pub struct ConfigParameters {
    pub bot_maintainer: UserId,
    pub maintainer_username: Option<String>,
}

#[derive(BotCommands, Clone, Serialize, Deserialize)]
#[command(
    rename_rule = "lowercase",
    description = "These public commands are supported:"
)]
pub enum PublicCommands {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Get chat id.")]
    MyId,
    #[command(description = "Roll the dice.")]
    Roll,
    #[command(description = "Maintainer info.")]
    Maintainer,
}

#[derive(BotCommands, Clone, Serialize, Deserialize)]
#[command(
    rename_rule = "lowercase",
    description = "These maintainer commands are supported:"
)]
pub enum MaintainerCommands {
    #[command(description = "Set prompt text.")]
    Prompt(String),
    #[command(description = "Chat with gpt.")]
    Chat(String),
    #[command(description = "View chat histories.")]
    View,
    #[command(description = "Clear history chats.")]
    Clear,
}

pub type Client = async_openai::Client<OpenAIConfig>;
pub type ChatMessages = Vec<ChatCompletionRequestMessage>;
pub type ChatHistories = HashMap<ChatId, ChatMessages>;
pub type State = Arc<Mutex<ChatHistories>>;
pub type HandleResult = Result<(), AppError>;
