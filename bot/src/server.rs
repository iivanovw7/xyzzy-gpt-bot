use crate::{
    commands::commands,
    config::CONFIG,
    env::ENV,
    handlers, keyboard,
    types::{
        auth::AuthState,
        common::{ChatHistories, Commands, DialogueState},
        databases::Database,
    },
};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use async_openai::{config::OpenAIConfig, Client};
use dotenv::dotenv;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
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

    let open_ai_config = OpenAIConfig::new().with_api_key(&ENV.open_api_key);
    let bot = Bot::new(&ENV.token);
    let client: Client<OpenAIConfig> = Client::with_config(open_ai_config);
    let state = Arc::new(Mutex::new(ChatHistories::new()));
    let auth_state: AuthState = web::Data::new(Arc::new(Mutex::new(HashMap::new())));
    let db = Arc::new(Database::new().await);

    let dialogue_storage = InMemStorage::<DialogueState>::new();
    let jwt_secret = ENV.jwt_secret.clone();
    let is_dev = cfg!(debug_assertions);
    let web_auth_state = auth_state.clone();
    let web_db = db.clone();

    let api_server = tokio::spawn(async move {
        HttpServer::new(move || {
            let mut cors = Cors::default()
                .allowed_origin(&CONFIG.web.url)
                .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                    actix_web::http::header::CONTENT_TYPE,
                ])
                .supports_credentials();

            if is_dev {
                cors = cors.allowed_origin(&format!("http://localhost:{}", &CONFIG.web.port));
            }

            App::new()
                .wrap(cors)
                .app_data(web_auth_state.clone())
                .app_data(web::Data::new(jwt_secret.clone()))
                .app_data(web::Data::new(Arc::new(ENV.clone())))
                .app_data(web::Data::new(Arc::new(CONFIG.clone())))
                .app_data(web::Data::new(web_db.clone()))
                .route("/api/user", web::get().to(handlers::web::user::get))
                .route("/api/auth/login", web::get().to(handlers::web::auth::login))
                .route(
                    "/api/auth/refresh",
                    web::post().to(handlers::web::auth::refresh),
                )
                .route(
                    "/api/budgeting/overview",
                    web::get().to(handlers::web::budgeting::overview::get),
                )
                .route(
                    "/api/budgeting/transactions",
                    web::get().to(handlers::web::budgeting::transactions::get),
                )
        })
        .bind(("0.0.0.0", CONFIG.api.port))
        .unwrap()
        .run()
        .await
        .unwrap();
    });

    let web_server = if !is_dev {
        Some(tokio::spawn(async move {
            HttpServer::new(move || {
                App::new()
                    .service(Files::new("/", CONFIG.web.dist.clone()).index_file("index.html"))
                    .default_service(web::to(|| async {
                        actix_files::NamedFile::open_async(format!(
                            "{}/index.html",
                            CONFIG.web.dist
                        ))
                        .await
                    }))
            })
            .bind(("0.0.0.0", CONFIG.web.port))
            .unwrap_or_else(|_| {
                panic!("Failed to bind frontend server to port {}", CONFIG.web.port)
            })
            .run()
            .await
            .unwrap()
        }))
    } else {
        info!("DEV mode: web server is disabled");
        None
    };

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

    let is_authorized_cb = dptree::filter(|q: CallbackQuery| q.from.id == UserId(ENV.user_id));
    let is_unathorized_cb = dptree::filter(|q: CallbackQuery| q.from.id != UserId(ENV.user_id));

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
                .branch(is_unathorized_cb.endpoint(handlers::auth::bot::unauthorized_access_cb))
                .branch(is_authorized_cb.endpoint(keyboard::core::callback)),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<DialogueState>, DialogueState>()
                .branch(
                    is_unauthorized
                        .clone()
                        .endpoint(handlers::auth::bot::unauthorized_access),
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

    let bot_auth_state = auth_state.clone();

    let mut bot_dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![
            client,
            state,
            bot_auth_state,
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
        .build();

    let dispatcher = bot_dispatcher.dispatch();

    let _ = dispatcher.await;
    let api_result = api_server.await;

    match api_result {
        Err(e) => error!("API server task failed: {:?}", e),
        Ok(_) => {
            info!("API server task completed.");
        }
    }

    if let Some(web_server) = web_server {
        match web_server.await {
            Err(e) => error!("Web app server task failed: {:?}", e),
            Ok(_) => info!("Web app server task completed."),
        }
    }
}
