use crate::{
    env::ENV,
    types::main::{BotDialogue, ChatHistoryState, DialogueState, HandleResult},
    utils::markdown::escape_markdown_v2,
};
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use futures::StreamExt;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{prelude::*, types::ParseMode};
use tracing::info;

pub async fn message(
    content: String,
    bot: Bot,
    client: Client<OpenAIConfig>,
    state: ChatHistoryState,
    msg: Message,
) -> HandleResult {
    info!("Complete chat, user: {}, content: {}", msg.chat.id, content);

    let env = &ENV;
    let hists;
    {
        let mut guard = state.lock().unwrap();
        let messages = guard.entry(msg.chat.id).or_default();

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()?
                .into(),
        );

        hists = messages.clone();
    }

    let response = bot.send_message(msg.chat.id, "💭").reply_to(msg.id).await?;

    let msg_id = response.id;
    let request = CreateChatCompletionRequestArgs::default()
        .model(env.model.clone())
        .messages(hists)
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;
    let mut chunks = Vec::new();
    let mut count = 0;

    while let Some(result) = stream.next().await {
        if let Some(ref content) = result?.choices.first().unwrap().delta.content {
            chunks.push(content.to_owned());

            if !content.trim().is_empty() {
                count += 1;
                if count % 20 == 0 {
                    bot.edit_message_text(msg.chat.id, msg_id, chunks.join(""))
                        .await?;
                }
            }
        }
    }

    bot.edit_message_text(msg.chat.id, msg_id, chunks.join(""))
        .await?;

    let mut guard = state.lock().unwrap();
    let messages = guard.entry(msg.chat.id).or_default();

    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(chunks.join(""))
            .build()?
            .into(),
    );

    Ok(())
}

pub async fn enter(bot: Bot, dialogue: BotDialogue, msg: Message) -> HandleResult {
    let dialogue_state = dialogue.get_or_default().await?;

    match dialogue_state {
        DialogueState::Start
        | DialogueState::WaitingForChatRequest
        | DialogueState::WaitingForNewPrompt => {
            dialogue.update(DialogueState::InChatMode).await?;

            if dialogue_state == DialogueState::Start {
                dialogue.update(DialogueState::InChatMode).await?;

                let message = escape_markdown_v2("🤖 **AI Chat Mode Activated**");

                bot.send_message(msg.chat.id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;
            } else {
                bot.send_message(msg.chat.id, "You are already in a AI Chat Mode.")
                    .await?;
            }
        }
        DialogueState::InChatMode => {
            bot.send_message(msg.chat.id, "You are already in AI Chat Mode.")
                .await?;
        }
    }

    Ok(())
}

pub async fn exit(bot: Bot, dialogue: BotDialogue, msg: Message) -> HandleResult {
    let dialogue_state = dialogue.get_or_default().await?;

    if dialogue_state == DialogueState::InChatMode {
        dialogue.update(DialogueState::Start).await?;

        let message = escape_markdown_v2("👋 **AI Chat Mode Deactivated**");

        bot.send_message(msg.chat.id, message)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
    } else {
        bot.send_message(msg.chat.id, "You are not currently in AI Chat Mode.")
            .await?;
    }

    Ok(())
}

pub async fn message_in_chat_mode(
    client: Client<OpenAIConfig>,
    state: ChatHistoryState,
    bot: Bot,
    msg: Message,
    text: String,
) -> HandleResult {
    message(text, bot, client, state, msg).await
}
