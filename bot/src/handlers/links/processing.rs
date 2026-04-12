use crate::{
    config::CONFIG,
    types::{
        common::{BotDialogue, DialogueState, HandleResult, LinkDraft},
        databases::Database,
    },
    utils::scraper::scrape_metadata,
};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use serde::Deserialize;
use std::sync::Arc;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};
use tracing::error;

#[derive(Deserialize, Debug)]
struct GptLinkAnalysis {
    category: String,
    tags: Vec<String>,
}

pub fn extract_url(text: &str) -> Option<String> {
    text.split_whitespace()
        .find(|word| word.starts_with("http://") || word.starts_with("https://"))
        .map(|s| s.to_string())
}

pub async fn process_link(
    url: String,
    bot: Bot,
    client: Client<OpenAIConfig>,
    msg: Message,
    dialogue: BotDialogue,
    db: Arc<Database>,
) -> HandleResult {
    let wait_msg = bot
        .send_message(msg.chat.id, "🔗 Link detected. Scraping metadata...")
        .reply_to(msg.id)
        .await?;

    let user_id = msg.chat.id.0;

    if let Ok(Some(_)) = db.links().get_link_by_url(user_id, &url).await {
        bot.edit_message_text(
            msg.chat.id,
            wait_msg.id,
            "This link is already in your database.",
        )
        .await?;
        return Ok(());
    }

    let metadata = match scrape_metadata(&url).await {
        Ok(m) => m,
        Err(e) => {
            error!("Failed to scrape link {}: {}", url, e);
            bot.edit_message_text(
                msg.chat.id,
                wait_msg.id,
                format!("Failed to scrape link: {}", e),
            )
            .await?;

            return Ok(());
        }
    };

    bot.edit_message_text(msg.chat.id, wait_msg.id, "🧠 Analyzing content...")
        .await?;

    let title = metadata.title.as_deref().unwrap_or("No title");
    let description = metadata.description.as_deref().unwrap_or("No description");

    let analysis: GptLinkAnalysis = if (title == url || title == "No title") && description == "No description" {
        GptLinkAnalysis {
            category: "Uncategorized".to_string(),
            tags: vec![],
        }
    } else {
        let existing_categories = db
            .links()
            .list_categories(user_id)
            .await
            .unwrap_or_default();
        let category_names: Vec<String> = existing_categories.into_iter().map(|c| c.name).collect();
        let categories_str = if category_names.is_empty() {
            "None".to_string()
        } else {
            category_names.join(", ")
        };

        let existing_tags = db.links().list_tags(user_id).await.unwrap_or_default();
        let tags_str_context = if existing_tags.is_empty() {
            "None".to_string()
        } else {
            existing_tags.join(", ")
        };

        let prompt = format!(
            "Analyze the following link and provide a single category and 3-4 short tags.\n\
            URL: {}\n\
            Title: {}\n\
            Description: {}\n\n\
            Existing Categories: {}\n\
            Existing Tags: {}\n\n\
            Please strongly prefer using the existing categories and tags if they accurately describe the link. Only create new ones if absolutely necessary.",
            url, title, description, categories_str, tags_str_context
        );

        let request = match CreateChatCompletionRequestArgs::default()
            .model(&CONFIG.open_ai.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a helpful assistant. Output ONLY valid JSON in the exact format: {\"category\": \"string\", \"tags\": [\"string\", \"string\", \"string\"]}")
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt)
                    .build()?
                    .into(),
            ])
            .build() {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to build request: {}", e);
                    bot.edit_message_text(msg.chat.id, wait_msg.id, "Failed to build AI request.")
                        .await?;
                    return Ok(());
                }
            };

        let response = match client.chat().create(request).await {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to call OpenAI: {}", e);
                bot.edit_message_text(
                    msg.chat.id,
                    wait_msg.id,
                    format!("AI Analysis failed: {}", e),
                )
                .await?;
                return Ok(());
            }
        };

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let content = content
            .trim_start_matches("```json\n")
            .trim_start_matches("```\n")
            .trim_end_matches("\n```")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str(content) {
            Ok(a) => a,
            Err(e) => {
                error!("Failed to parse response: {} \nResponse: {}", e, content);
                bot.edit_message_text(
                    msg.chat.id,
                    wait_msg.id,
                    format!("Failed to analyze link via AI: {}", e),
                )
                .await?;
                return Ok(());
            }
        }
    };

    let draft = LinkDraft {
        url: url.clone(),
        title: metadata.title.clone(),
        description: metadata.description.clone(),
        thumbnail_url: metadata.thumbnail_url.clone(),
        category: analysis.category.clone(),
        tags: analysis.tags.clone(),
    };

    dialogue
        .update(DialogueState::WaitingForLinkConfirmation(draft))
        .await?;

    let message_text = format!(
        "🔗 *Link Draft*\n\n*URL:* {}\n*Title:* {}\n*Category:* {}\n*Tags:* {}\n\nSave this link?",
        crate::utils::markdown::escape_markdown_v2(&url),
        crate::utils::markdown::escape_markdown_v2(title),
        crate::utils::markdown::escape_markdown_v2(&analysis.category),
        crate::utils::markdown::escape_markdown_v2(&analysis.tags.join(", "))
    );

    let keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("✅ Save", "link:save"),
        InlineKeyboardButton::callback("❌ Ignore", "link:ignore"),
    ]]);

    bot.edit_message_text(msg.chat.id, wait_msg.id, message_text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
