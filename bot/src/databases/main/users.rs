use teloxide::types::User as TgUser;

use crate::types::databases::UsersDb;

impl UsersDb {
    pub async fn upsert_user(&self, telegram_id: i64, username: Option<String>) -> i64 {
        sqlx::query!(
            "INSERT INTO users (telegram_id, username) VALUES (?, ?) \
             ON CONFLICT(telegram_id) DO UPDATE SET username = excluded.username",
            telegram_id,
            username,
        )
        .execute(&self.pool)
        .await
        .unwrap();

        let row = sqlx::query!("SELECT id FROM users WHERE telegram_id = ?", telegram_id)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        row.id.expect("User inserted but id is NULL")
    }

    pub async fn ensure_user(&self, tg: &TgUser) -> i64 {
        let id = tg.id.0 as i64;
        let name = tg.username.clone().or_else(|| {
            let mut full_name = tg.first_name.clone();
            if let Some(last_name) = &tg.last_name {
                full_name.push(' ');
                full_name.push_str(last_name);
            }
            Some(full_name)
        });

        self.upsert_user(id, name).await
    }

    pub async fn get_user_id(&self, telegram_id: i64) -> Option<i64> {
        sqlx::query!("SELECT id FROM users WHERE telegram_id = ?", telegram_id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
            .and_then(|r| r.id)
    }

    pub async fn get_user_by_telegram_id(&self, telegram_id: i64) -> Option<crate::types::models::User> {
        sqlx::query_as::<_, crate::types::models::User>(
            "SELECT id, telegram_id, username FROM users WHERE telegram_id = ?"
        )
        .bind(telegram_id)
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }
}
