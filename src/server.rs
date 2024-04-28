use crate::{
    commands::{maintainer_commands, public_commands},
    env::ENV,
    types::{ChatHistories, ConfigParameters, MaintainerCommands, PublicCommands},
};
use async_openai::{config::OpenAIConfig, Client};
use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

    let mut commands = Vec::new();

    for public_command in PublicCommands::bot_commands().iter() {
        commands.push(public_command.clone());
    }

    for maintainer_command in MaintainerCommands::bot_commands().iter() {
        commands.push(maintainer_command.clone());
    }

    match bot.set_my_commands(commands).await {
        Ok(_) => info!("Commands loaded"),
        Err(_) => error!("Commands loading error"),
    }

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<PublicCommands>()
                .endpoint(public_commands),
        )
        .branch(
            dptree::filter(|cfg: ConfigParameters, msg: Message| {
                msg.from()
                    .map(|user| user.id == cfg.bot_maintainer)
                    .unwrap_or_default()
            })
            .filter_command::<MaintainerCommands>()
            .endpoint(maintainer_commands),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![client, state, parameters])
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
