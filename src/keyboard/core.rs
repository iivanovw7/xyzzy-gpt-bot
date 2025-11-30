use async_openai::{config::OpenAIConfig, Client};
use std::{str::FromStr, sync::Arc};
use teloxide::{
    prelude::*,
    types::{KeyboardButton, KeyboardMarkup, Me, ReplyMarkup},
};
use tracing::info;

use crate::{
    handlers,
    keyboard::{
        budgeting::{
            categories::{
                create_budgeting_categories_menu_keyboard, create_categories_keyboard,
                create_kinds_keyboard,
            },
            core::create_budgeting_menu_keyboard,
        },
        gpt::create_gpt_menu_keyboard,
        util::create_date_filter_keyboard,
    },
    types::{
        common::{
            BotDialogue, ChatHistoryState, ConfigParameters, DateFilter, DialogueState,
            HandleResult, TransactionKind,
        },
        databases::Database,
        keyboard::{
            BudgetingCategoriesMenuItems, BudgetingMenuItems, MainMenuItems, OpenAIMenuItems,
        },
    },
    utils::{markdown::escape_markdown_v2, strings::parse_amount},
};

pub fn create_main_menu_keyboard() -> ReplyMarkup {
    let keyboard_rows: Vec<Vec<KeyboardButton>> = vec![
        vec![
            KeyboardButton::new(MainMenuItems::AiTools),
            KeyboardButton::new(MainMenuItems::Budgeting),
        ],
        vec![
            KeyboardButton::new(MainMenuItems::Roll),
            KeyboardButton::new(MainMenuItems::Help),
            KeyboardButton::new(MainMenuItems::Maintainer),
        ],
    ];

    let custom_keyboard = KeyboardMarkup {
        keyboard: keyboard_rows,
        resize_keyboard: true,
        one_time_keyboard: false,
        is_persistent: true,
        input_field_placeholder: "Select a menu or type a command...".to_string(),
        selective: false,
    };

    custom_keyboard.into()
}

#[allow(clippy::too_many_arguments)]
pub async fn handle_keyboard(
    cfg: ConfigParameters,
    client: Client<OpenAIConfig>,
    state: ChatHistoryState,
    bot: Bot,
    me: Me,
    dialogue: BotDialogue,
    msg: Message,
    db: Arc<Database>,
    text: String,
) -> HandleResult {
    let chat_id = msg.chat.id;

    if let Ok(item) = <MainMenuItems as FromStr>::from_str(&text) {
        match item {
            MainMenuItems::Roll => {
                handlers::dice::roll(bot.clone(), msg.clone()).await?;
            }
            MainMenuItems::Help => {
                handlers::help::commands(cfg, bot.clone(), me, msg.clone()).await?;
            }
            MainMenuItems::Maintainer => {
                handlers::maintainer::log(cfg, bot.clone(), msg.clone()).await?;
            }
            MainMenuItems::AiTools => {
                let gpt_menu_keyboard = create_gpt_menu_keyboard(dialogue.clone()).await?;

                bot.send_message(chat_id, "AI Chat tools")
                    .reply_markup(gpt_menu_keyboard)
                    .await?;
            }
            MainMenuItems::Budgeting => {
                bot.send_message(chat_id, "Budgeting")
                    .reply_markup(create_budgeting_menu_keyboard())
                    .await?;
            }
        };

        return Ok(());
    }

    if let Ok(item) = <OpenAIMenuItems as FromStr>::from_str(&text) {
        match item {
            OpenAIMenuItems::StartChat => {
                bot.send_message(chat_id, "Type your request for the AI below:")
                    .await?;

                dialogue
                    .update(DialogueState::WaitingForChatRequest)
                    .await?;
            }
            OpenAIMenuItems::EnterChatMode => {
                handlers::gpt::chat::enter(bot.clone(), dialogue.clone(), msg.clone()).await?;
            }
            OpenAIMenuItems::ExitChatMode => {
                handlers::gpt::chat::exit(bot.clone(), dialogue.clone(), msg.clone()).await?;
            }
            OpenAIMenuItems::SetPrompt => {
                bot.send_message(chat_id, "System prompt you want to set for the AI")
                    .await?;

                dialogue.update(DialogueState::WaitingForNewPrompt).await?;
            }
            OpenAIMenuItems::ViewHistory => {
                handlers::gpt::history::view(bot.clone(), state.clone(), msg.clone()).await?;
            }
            OpenAIMenuItems::ClearHistory => {
                handlers::gpt::history::clear(bot.clone(), state.clone(), msg.clone()).await?;
            }
            OpenAIMenuItems::Back => {
                bot.send_message(chat_id, "Returning to Main Menu.")
                    .reply_markup(create_main_menu_keyboard())
                    .await?;
            }
        }

        return Ok(());
    }

    if let Ok(item) = <BudgetingMenuItems as FromStr>::from_str(&text) {
        match item {
            BudgetingMenuItems::Statistics => {
                let keyboard = create_date_filter_keyboard("statistics");
                let message = escape_markdown_v2("Select statistics date filter:");

                bot.send_message(msg.chat.id, message)
                    .reply_markup(keyboard)
                    .await?;
            }
            BudgetingMenuItems::Transactions => {
                let keyboard = create_date_filter_keyboard("transactions");
                let message = escape_markdown_v2("Select transactions date filter:");

                bot.send_message(msg.chat.id, message)
                    .reply_markup(keyboard)
                    .await?;
            }
            BudgetingMenuItems::AddIncome => {
                handlers::budgeting::transactions::add_kind(
                    TransactionKind::Income,
                    dialogue,
                    &db.categories(),
                    bot,
                    msg,
                )
                .await?;
            }
            BudgetingMenuItems::AddSpending => {
                handlers::budgeting::transactions::add_kind(
                    TransactionKind::Spending,
                    dialogue,
                    &db.categories(),
                    bot,
                    msg,
                )
                .await?;
            }
            BudgetingMenuItems::Settings => {
                handlers::budgeting::settings::open(bot, msg).await?;
            }
            BudgetingMenuItems::Categories => {
                bot.send_message(chat_id, "Budgeting categories")
                    .reply_markup(create_budgeting_categories_menu_keyboard())
                    .await?;

                handlers::budgeting::categories::list(bot, msg, &db.categories()).await?
            }
            BudgetingMenuItems::Back => {
                bot.send_message(chat_id, "Returning to Main Menu.")
                    .reply_markup(create_main_menu_keyboard())
                    .await?;
            }
        }

        return Ok(());
    }

    if let Ok(item) = <BudgetingCategoriesMenuItems as FromStr>::from_str(&text) {
        match item {
            BudgetingCategoriesMenuItems::List => {
                handlers::budgeting::categories::list(bot, msg, &db.categories()).await?
            }
            BudgetingCategoriesMenuItems::Add => {
                let prefix = "category:kind";
                let keyboard = create_kinds_keyboard(prefix);
                let message = escape_markdown_v2("✏️ Category kind ?");

                bot.send_message(msg.chat.id, message)
                    .reply_markup(keyboard)
                    .await?;
            }
            BudgetingCategoriesMenuItems::Remove => {
                let prefix = "category:remove";
                let keyboard = create_categories_keyboard(prefix, &db.categories()).await;
                let message = escape_markdown_v2("🗑 Select category to remove");

                bot.send_message(msg.chat.id, message)
                    .reply_markup(keyboard)
                    .await?;
            }
            BudgetingCategoriesMenuItems::Back => {
                dialogue.update(DialogueState::Start).await?;

                bot.send_message(chat_id, "Returning to Budgeting.")
                    .reply_markup(create_budgeting_menu_keyboard())
                    .await?;
            }
        }

        return Ok(());
    }

    let dialogue_state = dialogue.get_or_default().await?;

    match dialogue_state {
        DialogueState::WaitingForChatRequest => {
            handlers::gpt::chat::message(
                text.clone(),
                bot.clone(),
                client.clone(),
                state.clone(),
                msg.clone(),
            )
            .await?;

            dialogue.update(DialogueState::Start).await?;
        }
        DialogueState::WaitingForNewPrompt => {
            handlers::gpt::prompt::set(text.clone(), bot.clone(), state.clone(), msg.clone())
                .await?;

            dialogue.update(DialogueState::Start).await?;
        }
        DialogueState::InChatMode => {
            handlers::gpt::chat::message_in_chat_mode(
                client.clone(),
                state.clone(),
                bot.clone(),
                msg.clone(),
                text.clone(),
            )
            .await?;
        }
        DialogueState::InCategoriesMode => {
            handlers::budgeting::categories::list(bot, msg, &db.categories()).await?;
        }
        DialogueState::WaitingForNewCategoryName { kind } => {
            handlers::budgeting::categories::add(text, kind, bot, msg, &db.categories()).await?;

            dialogue.update(DialogueState::InCategoriesMode).await?;
        }
        DialogueState::InTransactionsMode => {
            bot.send_message(msg.chat.id, "Add new transaction").await?;
        }
        DialogueState::WaitingForTransactionAmount { kind, category_id } => {
            let mut parts = text.splitn(2, ' ');

            let amount_str = parts.next().unwrap_or("0").trim();
            let description = parts.next().unwrap_or("no description").trim();

            let amount = match parse_amount(amount_str) {
                Some(a) => a,
                None => {
                    let msg_text =
                        "Invalid amount. Example: `188 Coffee` or `5 Coffee` -> becomes 500";
                    bot.send_message(msg.chat.id, msg_text).await?;

                    return Ok(());
                }
            };

            crate::handlers::budgeting::transactions::add_transaction(
                amount,
                description.to_string(),
                category_id.clone(),
                &db.transactions(),
                bot.clone(),
                msg.chat.id.to_string(),
                kind,
            )
            .await?;
        }
        DialogueState::Start => {
            return Ok(());
        }
    }

    Ok(())
}

pub async fn start(bot: Bot, msg: Message) -> HandleResult {
    let keyboard = create_main_menu_keyboard();

    bot.send_message(msg.chat.id, "Welcome! Here is your main menu:")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn callback(
    _cfg: ConfigParameters,
    _client: Client<OpenAIConfig>,
    _state: ChatHistoryState,
    bot: Bot,
    _me: Me,
    dialogue: BotDialogue,
    db: Arc<Database>,
    q: CallbackQuery,
) -> HandleResult {
    if let Some(data) = &q.data {
        let parts: Vec<&str> = data.split(':').collect();

        match parts.as_slice() {
            ["category", "kind", "income"] => {
                handlers::budgeting::categories::add_kind(
                    TransactionKind::Income,
                    dialogue,
                    bot.clone(),
                    q.from.id.to_string(),
                )
                .await?;
            }
            ["category", "kind", "spending"] => {
                handlers::budgeting::categories::add_kind(
                    TransactionKind::Spending,
                    dialogue,
                    bot.clone(),
                    q.from.id.to_string(),
                )
                .await?;
            }
            ["category", "remove", id_str] => {
                handlers::budgeting::categories::remove(
                    id_str.to_string(),
                    &db.categories(),
                    &db.transactions(),
                    bot.clone(),
                    q.from.id.to_string(),
                )
                .await?;

                dialogue.update(DialogueState::InCategoriesMode).await?;
            }
            ["transaction", kind_string, "add", "category", id] => {
                let category_id = id.to_string();

                let kind = TransactionKind::from_str(kind_string).unwrap_or_else(|_| {
                    panic!("Invalid transaction kind received: {}", kind_string)
                });

                bot.send_message(
                    q.from.id.to_string(),
                    "Add transaction (amount description)",
                )
                .await?;

                dialogue
                    .update(DialogueState::WaitingForTransactionAmount { kind, category_id })
                    .await?;
            }
            ["transactions", "filter", filter] => {
                let parsed_filter =
                    DateFilter::from_str(filter).expect("Invalid filter string received");

                handlers::budgeting::transactions::list(
                    bot.clone(),
                    q.from.id.to_string(),
                    &db.transactions(),
                    parsed_filter,
                )
                .await?;

                dialogue.update(DialogueState::Start).await?;
            }
            ["statistics", "filter", filter] => {
                let parsed_filter = DateFilter::from_str(filter)
                    .unwrap_or_else(|_| panic!("Invalid filter string received: {}", filter));

                handlers::budgeting::statistics::overview(
                    bot.clone(),
                    q.from.id.to_string(),
                    &db.transactions(),
                    parsed_filter,
                )
                .await?;

                dialogue.update(DialogueState::Start).await?;
            }
            _ => {
                info!("Received unknown callback data: {}", data);
            }
        }
    } else {
        info!("callback query has no data");
    }

    bot.answer_callback_query(q.id).await?;

    Ok(())
}
