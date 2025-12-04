use strum::EnumProperty;
use teloxide::prelude::*;

use crate::{
    types::{
        common::{BotDialogue, DialogueState, HandleResult, TransactionKind},
        databases::{CategoriesDb, TransactionsDb},
    },
    utils::markdown::escape_markdown_v2,
};

pub async fn list(bot: Bot, msg: Message, categories_db: &CategoriesDb) -> HandleResult {
    let kinds: Vec<TransactionKind> = vec![TransactionKind::Income, TransactionKind::Spending];

    let mut output = Vec::new();

    for kind in kinds.iter() {
        let mut categories = categories_db.list(*kind).await;

        categories.sort_by(|a, b| a.id.cmp(&b.id));

        let list_text = categories
            .iter()
            .map(|category| format!("{} - {}", category.id, &category.name))
            .collect::<Vec<_>>()
            .join("\n");

        output.push(kind.get_str("label").unwrap().to_string());
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
    kind: TransactionKind,
    bot: Bot,
    msg: Message,
    categories_db: &CategoriesDb,
) -> HandleResult {
    let trimmed_category = category.trim();

    if trimmed_category.is_empty() {
        bot.send_message(msg.chat.id, "❌ Category name cannot be empty")
            .await?;
        return Ok(());
    }

    if trimmed_category.starts_with('[') && trimmed_category.ends_with(']') {
        let list_content = &trimmed_category[1..trimmed_category.len() - 1];

        let names: Vec<String> = list_content
            .split(',')
            .filter_map(|s| {
                let name = s.trim().to_string();
                if name.is_empty() {
                    None
                } else {
                    Some(name)
                }
            })
            .collect();

        if !names.is_empty() {
            let num_to_add = names.len();

            let result = categories_db.add_many(names, kind).await;

            match result {
                Ok(inserted_count) => {
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "✅ Added {} of {} categories for {} ({} already existed).",
                            inserted_count,
                            num_to_add,
                            kind,
                            num_to_add - inserted_count as usize
                        ),
                    )
                    .await?;
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Database error while adding categories: {}", e),
                    )
                    .await?;
                }
            }

            return Ok(());
        }
    }

    let result = categories_db.add(trimmed_category, kind).await;

    if result.rows_affected() == 0 {
        bot.send_message(
            msg.chat.id,
            format!(
                "⚠️ The category '{}' already exists for {}",
                trimmed_category, kind
            ),
        )
        .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            format!("✅ Category {} added for {}", trimmed_category, kind),
        )
        .await?;
    }

    Ok(())
}

pub async fn remove(
    text: String,
    categories_db: &CategoriesDb,
    transactions_db: &TransactionsDb,
    bot: Bot,
    chat_id: String,
) -> HandleResult {
    let id: i64 = match text.trim().parse() {
        Ok(n) if n > 0 => n,
        _ => {
            bot.send_message(chat_id, "⚠️ Please provide a valid category id to remove.")
                .await?;

            return Ok(());
        }
    };

    if !categories_db.has(id).await {
        bot.send_message(chat_id, "⚠️ The category does not exist")
            .await?;

        return Ok(());
    }

    let has_transactions = transactions_db.has_transactions_for_category(id).await;

    if has_transactions {
        bot.send_message(
            chat_id.clone(),
            "⚠️ Cannot remove this category because it has transactions.",
        )
        .await?;

        return Ok(());
    }

    categories_db.remove(id).await?;

    bot.send_message(chat_id, format!("🗑 Removed category with id: {}", id))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

pub async fn add_kind(
    kind: TransactionKind,
    dialogue: BotDialogue,
    bot: Bot,
    chat_id: String,
) -> HandleResult {
    dialogue
        .update(DialogueState::WaitingForNewCategoryName { kind })
        .await?;

    let kind_string: &str = kind.into();

    bot.send_message(
        chat_id,
        format!(
            "✏️ Please enter the new category name for a new {} category ([a,b,c] - to add many).",
            kind_string
        ),
    )
    .await?;

    Ok(())
}
