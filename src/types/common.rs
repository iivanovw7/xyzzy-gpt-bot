use async_openai::{config::OpenAIConfig, error::OpenAIError, types::ChatCompletionRequestMessage};
use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use strum::Display;
use teloxide::{
    dispatching::dialogue::{InMemStorage, InMemStorageError},
    prelude::*,
    utils::command::BotCommands,
    RequestError,
};

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
    #[error("Json error")]
    Json(serde_json::Error),
    #[error("In-memory storage error: {0}")]
    DialogueStorage(String),
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}

impl From<InMemStorageError> for AppError {
    fn from(err: InMemStorageError) -> Self {
        AppError::DialogueStorage(err.to_string())
    }
}

#[derive(Clone)]
pub struct ConfigParameters {
    pub bot_maintainer: UserId,
    pub maintainer_username: Option<String>,
}

#[derive(BotCommands, Clone, Serialize, Deserialize, Display)]
#[command(
    rename_rule = "lowercase",
    description = "These public commands are supported:"
)]
pub enum PublicCommands {
    #[command(description = "Start")]
    Start,
    #[command(description = "Display this text.")]
    Help,
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
    #[command(description = "Enter chat mode with chat gpt.")]
    Enter,
    #[command(description = "Exit chat mode with chat gpt.")]
    Exit,
    #[command(description = "View chat histories.")]
    View,
    #[command(description = "Clear history chats.")]
    Clear,
    #[command(description = "Budget statistics.")]
    Stats,
    #[command(description = "Adds a new transaction.")]
    Add,
    #[command(description = "Show list of categories.")]
    Categories,
    #[command(description = "Remove income/spending category by id.")]
    RemoveCategory(String),
    #[command(description = "Add spenging category.")]
    AddSpendingCategory(String),
    #[command(description = "Add income category.")]
    AddIncomeCategory(String),
}

pub type OpenAIClient = async_openai::Client<OpenAIConfig>;
pub type ChatMessages = Vec<ChatCompletionRequestMessage>;
pub type ChatHistories = HashMap<ChatId, ChatMessages>;
pub type ChatHistoryState = Arc<Mutex<ChatHistories>>;

pub type HandleResult = Result<(), AppError>;

pub type BotDialogue = Dialogue<DialogueState, InMemStorage<DialogueState>>;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum DialogueState {
    #[default]
    Start,
    InChatMode,
    WaitingForChatRequest,
    WaitingForNewPrompt,
    CategoriesIdle,
    CategoriesAddingKind,
    CategoriesAddingName {
        kind: String,
    },
    CategoriesRemoving,
}
