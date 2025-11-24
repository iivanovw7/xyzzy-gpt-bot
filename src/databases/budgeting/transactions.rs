use crate::types::databases::TransactionsDb;

impl TransactionsDb {
    pub async fn add(
        &self,
        amount: i64,
        description: Option<String>,
        user_id: i64,
        category_id: i64,
    ) {
        sqlx::query!(
            "INSERT INTO transactions (amount, description, user_id, category_id)
             VALUES (?, ?, ?, ?)",
            amount,
            description,
            user_id,
            category_id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn list(&self, user_id: i64) -> Vec<(i64, i64, String)> {
        sqlx::query!(
            "SELECT t.amount, t.date, c.name as category
             FROM transactions t
             JOIN categories c ON c.id = t.category_id
             WHERE t.user_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
        .into_iter()
        .map(|r| (r.amount, r.date, r.category))
        .collect()
    }
}
