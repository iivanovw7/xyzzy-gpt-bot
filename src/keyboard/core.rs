use async_openai::{config::OpenAIConfig, Client};
use std::{str::FromStr, sync::Arc};
use teloxide::{
    prelude::*,
    types::{KeyboardButton, KeyboardMarkup, Me, ReplyMarkup},
};

use crate::{
    handlers, keyboard,
    types::{
        common::{BotDialogue, ChatHistoryState, ConfigParameters, DialogueState, HandleResult},
        databases::Database,
        menu::{BudgetingCategoriesMenuItems, BudgetingMenuItems, MainMenuItems, OpenAIMenuItems},
    },
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
                bot.send_message(chat_id, "AI Chat tools")
                    .reply_markup(keyboard::gpt::create_gpt_menu_keyboard())
                    .await?;
            }
            MainMenuItems::Budgeting => {
                bot.send_message(chat_id, "Budgeting")
                    .reply_markup(keyboard::budgeting::main::create_budgeting_menu_keyboard())
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
                handlers::budgeting::statistics::overview(bot, msg).await?
            }
            BudgetingMenuItems::AddExpense => {
                handlers::budgeting::transactions::add(bot, msg).await?
            }
            BudgetingMenuItems::Settings => handlers::budgeting::settings::open(bot, msg).await?,
            BudgetingMenuItems::Categories => {
                bot.send_message(chat_id, "Budgeting categories")
                    .reply_markup(
                        keyboard::budgeting::categories::create_budgeting_categories_menu_keyboard(
                        ),
                    )
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
                dialogue.update(DialogueState::CategoriesAddingKind).await?;

                bot.send_message(msg.chat.id, "➡️ What kind? (`income` or `spending`)")
                    .await?;
            }
            BudgetingCategoriesMenuItems::Remove => {
                dialogue.update(DialogueState::CategoriesRemoving).await?;

                bot.send_message(msg.chat.id, "🗑 Send category id to remove:")
                    .await?;
            }
            BudgetingCategoriesMenuItems::Back => {
                dialogue.update(DialogueState::Start).await?;

                bot.send_message(chat_id, "Budgeting")
                    .reply_markup(keyboard::budgeting::main::create_budgeting_menu_keyboard())
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
        DialogueState::CategoriesIdle => {
            bot.send_message(
                msg.chat.id,
                "❓ Use Add or Remove button to edit categories",
            )
            .await?;
        }
        DialogueState::CategoriesAddingKind => {
            handlers::budgeting::categories::add_kind(text, dialogue, bot, msg).await?;
        }
        DialogueState::CategoriesRemoving => {
            handlers::budgeting::categories::remove(text, &db.categories(), bot, msg).await?;

            dialogue.update(DialogueState::CategoriesIdle).await?;
        }
        DialogueState::CategoriesAddingName { kind } => {
            handlers::budgeting::categories::add(text, kind, bot, msg, &db.categories()).await?;

            dialogue.update(DialogueState::CategoriesIdle).await?;
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
