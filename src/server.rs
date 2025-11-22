use crate::{
    commands::{maintainer_commands, public_commands},
    env::ENV,
    handlers,
    menu::main::handle_menu_button,
    types::main::{
        ChatHistories, ConfigParameters, DialogueState, MaintainerCommands, PublicCommands,
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

    let parameters = ConfigParameters {
        bot_maintainer: UserId(ENV.user_id),
        maintainer_username: None,
    };

    let dialogue_storage = InMemStorage::<DialogueState>::new();

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

    let maintainer_filter = dptree::filter(|cfg: ConfigParameters, msg: Message| {
        msg.from
            .map(|user| user.id == cfg.bot_maintainer)
            .unwrap_or_default()
    });

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<DialogueState>, DialogueState>()
        .branch(
            dptree::case![DialogueState::InChatMode]
                .branch(
                    maintainer_filter
                        .clone()
                        .filter_command::<MaintainerCommands>()
                        .endpoint(maintainer_commands),
                )
                .branch(
                    dptree::filter(|msg: Message| msg.text().is_some())
                        .filter_map(|msg: Message| msg.text().map(ToOwned::to_owned))
                        .endpoint(handle_menu_button),
                )
                .branch(
                    dptree::filter(|msg: Message| msg.text().is_some())
                        .filter_map(|msg: Message| msg.text().map(ToOwned::to_owned))
                        .endpoint(handlers::gpt::chat::message_in_chat_mode),
                ),
        )
        .branch(
            dptree::entry()
                .filter_command::<PublicCommands>()
                .endpoint(public_commands),
        )
        .branch(
            maintainer_filter
                .clone()
                .filter_command::<MaintainerCommands>()
                .endpoint(maintainer_commands),
        )
        .branch(
            dptree::filter(|msg: Message| msg.text().is_some())
                .filter_map(|msg: Message| msg.text().map(ToOwned::to_owned))
                .endpoint(handle_menu_button),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![client, state, parameters, dialogue_storage])
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
