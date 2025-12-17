use std::fmt;

use strum::EnumProperty;
use strum::EnumString;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumString, EnumProperty, EnumIter)]
pub enum MainMenuItems {
    #[strum(serialize = "🎲 Roll", props(Label = "🎲 Roll"))]
    Roll,
    #[strum(serialize = "❓ Help", props(Label = "❓ Help"))]
    Help,
    #[strum(serialize = "🤖 AI Chat Tools", props(Label = "🤖 AI Chat Tools"))]
    AiTools,
    #[strum(serialize = "💰 Budgeting", props(Label = "💰 Budgeting"))]
    Budgeting,
    #[strum(serialize = "🔄 Start", props(Label = "🔄 Start"))]
    Start,
}

#[derive(Debug, Clone, Copy, EnumString, EnumProperty, EnumIter)]
pub enum OpenAIMenuItems {
    #[strum(serialize = "💬 Ask AI", props(Label = "💬 Ask AI"))]
    StartChat,
    #[strum(serialize = "⚫ Chat Mode", props(Label = "⚫ Chat Mode"))]
    EnterChatMode,
    #[strum(serialize = "🟢 Chat Mode", props(Label = "🟢 Chat Mode"))]
    ExitChatMode,
    #[strum(serialize = "⚙️ Set AI Prompt", props(Label = "⚙️ Set AI Prompt"))]
    SetPrompt,
    #[strum(serialize = "📜 View History", props(Label = "📜 View History"))]
    ViewHistory,
    #[strum(serialize = "🧹 Clear History", props(Label = "🧹 Clear History"))]
    ClearHistory,
    #[strum(serialize = "⬅️ Back", props(Label = "⬅️ Back"))]
    Back,
}

#[derive(Debug, Clone, Copy, EnumString, EnumProperty, EnumIter)]
pub enum BudgetingMenuItems {
    #[strum(serialize = "📊 Statistics", props(Label = "📊 Statistics"))]
    Statistics,
    #[strum(serialize = "🧾 Transactions", props(Label = "🧾 Transactions"))]
    Transactions,
    #[strum(serialize = "➕ Add Income", props(Label = "➕ Add Income"))]
    AddIncome,
    #[strum(serialize = "➖ Add Spending", props(Label = "➖ Add Spending"))]
    AddSpending,
    #[strum(serialize = "⚙️ Settings", props(Label = "⚙️ Settings"))]
    Settings,
    #[strum(serialize = "📋 Categories", props(Label = "📋 Categories"))]
    Categories,
    #[strum(serialize = "⬅️ Back", props(Label = "⬅️ Back"))]
    Back,
}

#[derive(Debug, Clone, Copy, EnumString, EnumProperty, EnumIter)]
pub enum BudgetingCategoriesMenuItems {
    #[strum(serialize = "📋 Show Categories", props(Label = "📋 Show Categories"))]
    List,
    #[strum(serialize = "➕ Add Category", props(Label = "➕ Add Category"))]
    Add,
    #[strum(serialize = "➖ Remove Category", props(Label = "➖ Remove Category"))]
    Remove,
    #[strum(
        serialize = "⬅️ Back to Budgeting",
        props(Label = "⬅️ Back to Budgeting")
    )]
    Back,
}

impl From<MainMenuItems> for String {
    fn from(item: MainMenuItems) -> Self {
        item.to_string()
    }
}

impl From<OpenAIMenuItems> for String {
    fn from(item: OpenAIMenuItems) -> Self {
        item.to_string()
    }
}

impl From<BudgetingMenuItems> for String {
    fn from(item: BudgetingMenuItems) -> Self {
        item.to_string()
    }
}

impl From<BudgetingCategoriesMenuItems> for String {
    fn from(item: BudgetingCategoriesMenuItems) -> Self {
        item.to_string()
    }
}

impl fmt::Display for MainMenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_str("Label").unwrap())
    }
}

impl fmt::Display for OpenAIMenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_str("Label").unwrap())
    }
}

impl fmt::Display for BudgetingMenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_str("Label").unwrap())
    }
}

impl fmt::Display for BudgetingCategoriesMenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_str("Label").unwrap())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BudgetingCallback {
    pub path: String,
}
