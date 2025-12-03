use async_openai::{config::OpenAIConfig, error::OpenAIError, types::ChatCompletionRequestMessage};
use serde::{Deserialize, Serialize};
use std::{
    cmp::PartialEq,
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};
use strum::{Display, EnumIter, EnumProperty, EnumString, IntoStaticStr};
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
    #[error("Internal database error")]
    Database(#[from] sqlx::Error),
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
    #[command(description = "Show list of categories.")]
    Categories,
    #[command(description = "Remove income/spending category by id.")]
    RemoveCategory(String),
    #[command(description = "Add spenging category ([a,b,c] - to add many).")]
    AddSpendingCategory(String),
    #[command(description = "Add income category ([a,b,c] - to add many).")]
    AddIncomeCategory(String),
}

#[derive(
    Debug,
    Clone,
    Copy,
    EnumString,
    EnumProperty,
    PartialEq,
    EnumIter,
    IntoStaticStr,
    Hash,
    sqlx::Type,
)]
#[sqlx(type_name = "TEXT")]
pub enum TransactionKind {
    #[strum(serialize = "income", props(label = "💰 Income"))]
    Income,
    #[strum(serialize = "spending", props(label = "🛒 Spending"))]
    Spending,
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = self.get_str("label").unwrap_or_else(|| self.into());

        write!(f, "{}", label)
    }
}

impl From<TransactionKind> for String {
    fn from(item: TransactionKind) -> Self {
        item.to_string()
    }
}

impl TransactionKind {
    pub fn apply_sign(&self, amount: i64) -> i64 {
        match self {
            TransactionKind::Income => amount,
            TransactionKind::Spending => -amount,
        }
    }
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
    InCategoriesMode,
    WaitingForNewCategoryName {
        kind: TransactionKind,
    },
    InTransactionsMode,
    WaitingForTransactionAmount {
        kind: TransactionKind,
        category_id: String,
    },
}

#[derive(Debug, Clone, Copy, EnumString, EnumIter)]
pub enum DateFilter {
    Today,
    CurrentMonth,
    LastMonth,
    Last3Months,
    CurrentYear,
}

impl DateFilter {
    pub fn label(&self) -> &'static str {
        match self {
            DateFilter::Today => "📅 Today",
            DateFilter::CurrentMonth => "📅 Current Month",
            DateFilter::LastMonth => "📅 Last Month",
            DateFilter::Last3Months => "📅 Last 3 Months",
            DateFilter::CurrentYear => "📅 This Year",
        }
    }
}
