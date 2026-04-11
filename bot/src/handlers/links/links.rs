use crate::types::common::HandleResult;
use crate::types::databases::Database;
use crate::utils::markdown::escape_markdown_v2;
use std::sync::Arc;
use teloxide::prelude::*;

pub async fn list_recent(bot: Bot, msg: Message, db: &Arc<Database>) -> HandleResult {
    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    let links = db.links().list_links(user_id, 10, 0).await?;

    if links.is_empty() {
        bot.send_message(msg.chat.id, "You haven't saved any links yet.")
            .await?;
        return Ok(());
    }

    let mut response = String::from("🔗 *Your Last 10 Links:*\n\n");
    for link in links {
        let title = link.title.unwrap_or_else(|| "Untitled".to_string());
        response.push_str(&format!(
            "• [{}]({})\n  _Category:_ {}\n\n",
            escape_markdown_v2(&title),
            escape_markdown_v2(&link.url),
            escape_markdown_v2(&link.category_name.unwrap_or_default())
        ));
    }

    bot.send_message(msg.chat.id, response)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .link_preview_options(teloxide::types::LinkPreviewOptions {
            is_disabled: true,
            url: None,
            prefer_small_media: false,
            prefer_large_media: false,
            show_above_text: false,
        })
        .await?;

    Ok(())
}
