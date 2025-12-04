use crate::{
    commands::commands,
    env::ENV,
    handlers::{
        self,
        auth::{unauthorized_access_callback, unauthorized_access_command},
    },
    keyboard,
    types::{
        common::{ChatHistories, Commands, ConfigParameters, DialogueState},
        databases::Database,
    },
};
use async_openai::{config::OpenAIConfig, Client};
use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use teloxide::{dispatching::dialogue::InMemStorage, types::MenuButton};
use teloxide::{prelude::*, types::*};
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use url::Url;

pub async fn server() {
    match dotenv() {
        Ok(_) => info!("Loaded .env file"),
        Err(_) => error!("No .env file found. Falling back to environment variables"),
    }

    std::env::set_var("RUST_LOG", "debug");

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "xyzzy-gpt-bot=debug".into()))
        .with(fmt::layer())
        .init();

    info!("initializing..");

    let config = OpenAIConfig::new().with_api_key(&ENV.open_api_key);
    let bot = Bot::new(&ENV.token);
    let client: Client<OpenAIConfig> = Client::with_config(config);
    let state = Arc::new(Mutex::new(ChatHistories::new()));
    let db = Arc::new(Database::new().await);

    let parameters = ConfigParameters {
        bot_maintainer: UserId(ENV.user_id),
        maintainer_username: None,
    };

    let dialogue_storage = InMemStorage::<DialogueState>::new();

    // TODO: add stats and setting pages
    let web_app_url_str = "https://your-webapp-domain.com/index.html";
    let web_app_url = Url::parse(web_app_url_str).expect("Failed to parse Web App URL");
    let web_app_info = WebAppInfo { url: web_app_url };

    let menu_button_value = MenuButton::WebApp {
        text: "Open".to_string(),
        web_app: web_app_info,
    };

    bot.set_chat_menu_button()
        .menu_button(menu_button_value)
        .await
        .ok();

    let is_authorized = dptree::filter(|msg: Message| {
        msg.from
            .map(|user| user.id == UserId(ENV.user_id))
            .unwrap_or(false)
    });

    let is_unauthorized = dptree::filter(|msg: Message| {
        msg.from
            .map(|user| user.id != UserId(ENV.user_id))
            .unwrap_or(true)
    });

    let message_filter = Update::filter_message()
        .filter(|msg: Message| msg.text().is_some())
        .map(|msg: Message| msg.text().unwrap().to_string());

    let commands_handler = is_authorized
        .clone()
        .filter_command::<Commands>()
        .endpoint(commands);

    let handler = dptree::entry()
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, InMemStorage<DialogueState>, DialogueState>()
                .branch(
                    dptree::filter(|q: CallbackQuery| q.from.id != UserId(ENV.user_id))
                        .clone()
                        .endpoint(unauthorized_access_callback),
                )
                .branch(
                    dptree::filter(|q: CallbackQuery| q.from.id == UserId(ENV.user_id))
                        .clone()
                        .endpoint(keyboard::core::callback),
                ),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<DialogueState>, DialogueState>()
                .branch(
                    is_unauthorized
                        .clone()
                        .endpoint(unauthorized_access_command),
                )
                .branch(
                    is_authorized.clone().branch(
                        dptree::case![DialogueState::InChatMode]
                            .branch(commands_handler.clone())
                            .branch(
                                message_filter
                                    .clone()
                                    .endpoint(keyboard::core::handle_keyboard),
                            )
                            .branch(
                                message_filter
                                    .clone()
                                    .endpoint(handlers::gpt::chat::message_in_chat_mode),
                            ),
                    ),
                )
                .branch(is_authorized.clone().branch(commands_handler.clone()))
                .branch(
                    is_authorized
                        .clone()
                        .branch(message_filter.endpoint(keyboard::core::handle_keyboard)),
                ),
        );

    Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![
            client,
            state,
            parameters,
            dialogue_storage,
            db.clone(),
            bot.clone().get_me().await.unwrap()
        ])
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
