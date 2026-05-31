use crate::types::{common::AppError, databases::Database};
use std::sync::Arc;
use teloxide::prelude::*;

pub async fn remap_category_command(
    bot: Bot,
    message: Message,
    db: Arc<Database>,
    from_id: i64,
    to_id: i64,
) -> Result<(), AppError> {
    match db.transactions().remap_category(from_id, to_id).await {
        Ok(rows_affected) => {
            bot.send_message(
                message.chat.id,
                format!(
                    "Successfully remapped category. {} transactions updated.",
                    rows_affected
                ),
            )
            .await?;
        }
        Err(e) => {
            bot.send_message(
                message.chat.id,
                format!("Failed to remap category: {}", e),
            )
            .await?;
        }
    }
    Ok(())
}
