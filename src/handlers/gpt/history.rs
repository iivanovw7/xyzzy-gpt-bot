use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::ChatCompletionRequestToolMessageContent;
use teloxide::{prelude::*, sugar::request::RequestReplyExt};

use crate::types::common::{ChatHistoryState, HandleResult};

fn print_msg(msg: &ChatCompletionRequestMessage) -> String {
    match msg {
        ChatCompletionRequestMessage::User(f) => {
            format!("[User]: {:?}", f.content.to_owned())
        }
        ChatCompletionRequestMessage::Developer(f) => {
            format!("[Developer]: {:?}", f.content.to_owned())
        }
        ChatCompletionRequestMessage::Tool(f) => {
            let content_str = match f.content {
                ChatCompletionRequestToolMessageContent::Text(ref s) => s,
                _ => "No content or complex tool output (Tool response).",
            };

            format!("[Tool]: {}", content_str)
        }
        ChatCompletionRequestMessage::System(f) => {
            format!("[System]: {:?}", f.content)
        }
        ChatCompletionRequestMessage::Function(f) => {
            format!("[Function]: {:?}", f.content)
        }
        ChatCompletionRequestMessage::Assistant(f) => {
            format!("[Assistant]: {:?}", f.content.to_owned())
        }
    }
}

pub async fn view(bot: Bot, state: ChatHistoryState, msg: Message) -> HandleResult {
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
        .reply_to(msg.id)
        .await?;

    Ok(())
}

pub async fn clear(bot: Bot, state: ChatHistoryState, msg: Message) -> HandleResult {
    {
        let mut guard = state.lock().unwrap();
        let messages = guard.entry(msg.chat.id).or_default();

        messages.clear();
    }

    bot.send_message(msg.chat.id, "Chat history cleared.")
        .reply_to(msg.id)
        .await?;

    Ok(())
}
