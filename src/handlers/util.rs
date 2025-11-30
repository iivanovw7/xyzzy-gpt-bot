use teloxide::{prelude::Requester, Bot};

use crate::types::common::AppError;

pub async fn parse_positive_i64(
    bot: &Bot,
    user_id: String,
    input: &str,
    field_name: &str,
) -> Result<i64, AppError> {
    match input.trim().parse::<i64>() {
        Ok(n) if n > 0 => Ok(n),
        _ => {
            bot.send_message(user_id, format!("⚠️ Invalid {}.", field_name))
                .await?;
            Err(AppError::InternalError("invalid input".into()))
        }
    }
}
