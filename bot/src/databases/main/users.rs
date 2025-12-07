use teloxide::types::User as TgUser;

use crate::types::databases::UsersDb;

impl UsersDb {
    pub async fn ensure_user(&self, tg: &TgUser) -> i64 {
        let id = tg.id.0 as i64;
        let name = tg.username.clone();

        sqlx::query!(
            "INSERT OR IGNORE INTO users (telegram_id, username) VALUES (?, ?)",
            id,
            name,
        )
        .execute(&self.pool)
        .await
        .unwrap();

        let row = sqlx::query!("SELECT id FROM users WHERE telegram_id = ?", id)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        row.id.expect("User inserted but id is NULL")
    }

    pub async fn get_user_id(&self, telegram_id: i64) -> Option<i64> {
        sqlx::query!("SELECT id FROM users WHERE telegram_id = ?", telegram_id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
            .and_then(|r| r.id)
    }
}
