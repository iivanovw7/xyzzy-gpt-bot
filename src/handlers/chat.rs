use crate::env::ENV;
use crate::types::{HandleResult, State};
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::StreamExt;
use teloxide::prelude::*;
use tracing::info;

pub async fn message(
    content: String,
    bot: Bot,
    client: Client<OpenAIConfig>,
    state: State,
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
                .role(Role::User)
                .content(content)
                .build()?
                .into(),
        );

        hists = messages.clone();
    }

    let response = bot
        .send_message(msg.chat.id, "💭")
        .reply_to_message_id(msg.id)
        .await?;

    let msg_id = response.id;
    let request = CreateChatCompletionRequestArgs::default()
        .model(env.model.clone())
        .messages(hists)
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;
    let mut chunks = Vec::new();
    let mut count = 0;

    while let Some(result) = stream.next().await {
        if let Some(ref content) = result?.choices.get(0).unwrap().delta.content {
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
            .role(Role::Assistant)
            .content(chunks.join(""))
            .build()?
            .into(),
    );

    Ok(())
}
