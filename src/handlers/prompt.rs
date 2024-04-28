use crate::types::{HandleResult, State};
use async_openai::types::{ChatCompletionRequestUserMessageArgs, Role};

use teloxide::prelude::*;
use tracing::info;

pub async fn set(prompt: String, bot: Bot, state: State, msg: Message) -> HandleResult {
    info!("Set prompt, user: {}, prompt: {}", msg.chat.id, prompt);

    {
        let mut guard = state.lock().unwrap();
        let messages = guard.entry(msg.chat.id).or_default();

        messages.clear();
        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .role(Role::System)
                .content(prompt)
                .build()?
                .into(),
        );
    }

    bot.send_message(msg.chat.id, "Prompt set.")
        .reply_to_message_id(msg.id)
        .await?;

    Ok(())
}
