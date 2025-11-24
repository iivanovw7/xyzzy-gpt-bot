use teloxide::prelude::*;

use crate::{
    types::{
        common::{BotDialogue, DialogueState, HandleResult},
        databases::CategoriesDb,
    },
    utils::markdown::escape_markdown_v2,
};

pub async fn list(bot: Bot, msg: Message, categories_db: &CategoriesDb) -> HandleResult {
    let kinds = [("income", "💰 Income"), ("spending", "🛒 Spending")];

    let mut output = Vec::new();

    for (kind_key, kind_label) in kinds.iter() {
        let mut categories = categories_db.list(kind_key).await;

        categories.sort_by(|a, b| a.0.cmp(&b.0));

        let list_text = categories
            .iter()
            .map(|(id, name)| format!("{} - {}", id, escape_markdown_v2(name)))
            .collect::<Vec<_>>()
            .join("\n");

        output.push(kind_label.to_string());
        output.push(list_text.to_string());
    }

    let message = escape_markdown_v2(&output.join("\n\n"));

    bot.send_message(msg.chat.id, message)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

pub async fn add(
    category: String,
    kind: String,
    bot: Bot,
    msg: Message,
    categories_db: &CategoriesDb,
) -> HandleResult {
    let kind = kind.to_lowercase();

    if category.trim().is_empty() {
        bot.send_message(msg.chat.id, "❌ Category name cannot be empty")
            .await?;
        return Ok(());
    }

    if kind != "income" && kind != "spending" {
        bot.send_message(
            msg.chat.id,
            "❌ Invalid category type. Use income or spending",
        )
        .await?;
        return Ok(());
    }

    let result = categories_db.add(&category, &kind).await;

    if result.rows_affected() == 0 {
        bot.send_message(
            msg.chat.id,
            format!("⚠️ The category '{}' already exists for {}", category, kind),
        )
        .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            format!("✅ Category {} added for {}", category, kind),
        )
        .await?;
    }

    Ok(())
}

pub async fn remove(
    text: String,
    categories_db: &CategoriesDb,
    bot: Bot,
    msg: Message,
) -> HandleResult {
    let id: i64 = match text.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            bot.send_message(
                msg.chat.id,
                "⚠️ Please provide a valid category id to remove.",
            )
            .await?;
            return Ok(());
        }
    };

    if !categories_db.has(id).await {
        bot.send_message(msg.chat.id, "⚠️ The category does not exist")
            .await?;
        return Ok(());
    }

    categories_db.remove(id).await;

    bot.send_message(msg.chat.id, format!("🗑 Removed category with id: {}", id))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

pub async fn add_kind(text: String, dialogue: BotDialogue, bot: Bot, msg: Message) -> HandleResult {
    if text != "income" && text != "spending" {
        bot.send_message(msg.chat.id, "⚠️ Please type `income` or `spending`.")
            .await?;

        return Ok(());
    }

    dialogue
        .update(DialogueState::CategoriesAddingName { kind: text.clone() })
        .await?;

    bot.send_message(msg.chat.id, "✏️ Category name?").await?;

    Ok(())
}
