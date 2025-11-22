use async_openai::types::ChatCompletionRequestUserMessageArgs;

use teloxide::{prelude::*, sugar::request::RequestReplyExt};
use tracing::info;

use crate::types::main::{ChatHistoryState, HandleResult};

pub async fn set(prompt: String, bot: Bot, state: ChatHistoryState, msg: Message) -> HandleResult {
    info!("Set prompt, user: {}, prompt: {}", msg.chat.id, prompt);

    {
        let mut guard = state.lock().unwrap();
        let messages = guard.entry(msg.chat.id).or_default();

        messages.clear();
        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );
    }

    bot.send_message(msg.chat.id, "Prompt set.")
        .reply_to(msg.id)
        .await?;

    Ok(())
}
