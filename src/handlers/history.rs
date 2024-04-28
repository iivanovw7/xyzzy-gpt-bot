use crate::types::{HandleResult, State};
use async_openai::types::ChatCompletionRequestMessage;
use teloxide::prelude::*;

fn print_msg(msg: &ChatCompletionRequestMessage) -> String {
    match msg {
        ChatCompletionRequestMessage::User(f) => {
            format!("[{}]: {:?}", f.role, f.content.to_owned())
        }
        ChatCompletionRequestMessage::Tool(f) => {
            format!("[{}]: {}", f.role, f.content)
        }
        ChatCompletionRequestMessage::System(f) => {
            format!("[{}]: {}", f.role, f.content)
        }
        ChatCompletionRequestMessage::Function(f) => {
            format!("[{}]: {:?}", f.role, f.content)
        }
        ChatCompletionRequestMessage::Assistant(f) => {
            format!("[{}]: {:?}", f.role, f.content.to_owned())
        }
    }
}

pub async fn view(bot: Bot, state: State, msg: Message) -> HandleResult {
    let content = {
        let mut guard = state.lock().unwrap();
        let messages = guard.entry(msg.chat.id).or_default();

        if messages.is_empty() {
            "Empty chat history.".to_owned()
        } else {
            messages
                .iter()
                .map(print_msg)
                .collect::<Vec<String>>()
                .join("\n\n")
        }
    };

    bot.send_message(msg.chat.id, content)
        .reply_to_message_id(msg.id)
        .await?;

    Ok(())
}

pub async fn clear(bot: Bot, state: State, msg: Message) -> HandleResult {
    {
        let mut guard = state.lock().unwrap();
        let messages = guard.entry(msg.chat.id).or_default();

        messages.clear();
    }

    bot.send_message(msg.chat.id, "Chat history cleared.")
        .reply_to_message_id(msg.id)
        .await?;

    Ok(())
}
