use sqlx::sqlite::SqliteQueryResult;

use crate::types::databases::CategoriesDb;

impl CategoriesDb {
    pub async fn list(&self, kind: &str) -> Vec<(i64, String)> {
        sqlx::query!("SELECT id, name FROM categories WHERE kind = ?", kind)
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|r| (r.id.unwrap(), r.name))
            .collect()
    }

    pub async fn add(&self, name: &str, kind: &str) -> SqliteQueryResult {
        sqlx::query!(
            "INSERT OR IGNORE INTO categories (name, kind) VALUES (?, ?)",
            name,
            kind
        )
        .execute(&self.pool)
        .await
        .unwrap()
    }

    pub async fn remove(&self, id: i64) -> SqliteQueryResult {
        let cat = sqlx::query!("SELECT kind FROM categories WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        if cat.is_none() {
            return sqlx::query("SELECT 1").execute(&self.pool).await.unwrap();
        }

        let kind = cat.unwrap().kind;

        let fallback = sqlx::query!(
            "SELECT id FROM categories WHERE kind = ? AND name = 'unknown' LIMIT 1",
            kind
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        sqlx::query!(
            "UPDATE transactions SET category_id = ? WHERE category_id = ?",
            fallback.id,
            id
        )
        .execute(&self.pool)
        .await
        .unwrap();

        sqlx::query!("DELETE FROM categories WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .unwrap()
    }

    pub async fn has(&self, id: i64) -> bool {
        let row = sqlx::query!("SELECT id FROM categories WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        row.is_some()
    }
}
