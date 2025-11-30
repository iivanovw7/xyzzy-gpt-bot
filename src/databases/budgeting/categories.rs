use sqlx::sqlite::SqliteQueryResult;

use crate::types::{common::TransactionKind, databases::CategoriesDb, models::CategoryRow};

impl CategoriesDb {
    pub async fn list(&self, kind: TransactionKind) -> Vec<CategoryRow> {
        let kind_string: &str = kind.into();

        sqlx::query_as!(
            CategoryRow,
            r#"
            SELECT 
                id as "id!: _",
                name,
                kind as "kind!: _"
            FROM categories
            WHERE kind = ?
            "#,
            kind_string
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn add(&self, name: &str, kind: TransactionKind) -> SqliteQueryResult {
        let kind_string: &str = kind.into();

        sqlx::query!(
            "INSERT OR IGNORE INTO categories (name, kind) VALUES (?, ?)",
            name,
            kind_string
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
