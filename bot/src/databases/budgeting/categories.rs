use sqlx::sqlite::SqliteQueryResult;

use crate::types::{
    common::{AppError, TransactionKind},
    databases::CategoriesDb,
    models::CategoryRow,
};

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

    pub async fn add_many(
        &self,
        names: Vec<String>,
        kind: TransactionKind,
    ) -> Result<u64, AppError> {
        let kind_string: &str = kind.into();

        let mut transaction = self.pool.begin().await?;
        let mut total_rows_affected: u64 = 0;

        for name in names {
            let result = sqlx::query!(
                "INSERT OR IGNORE INTO categories (name, kind) VALUES (?, ?)",
                name,
                kind_string
            )
            .execute(&mut *transaction)
            .await?;

            total_rows_affected += result.rows_affected();
        }

        transaction.commit().await?;

        Ok(total_rows_affected)
    }

    pub async fn remove(&self, id: i64) -> Result<SqliteQueryResult, AppError> {
        let result = sqlx::query!("DELETE FROM categories WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::InternalError(format!(
                "Category with id:{} not found.",
                id
            )));
        }

        Ok(result)
    }

    pub async fn has(&self, id: i64) -> bool {
        let row = sqlx::query!("SELECT id FROM categories WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        row.is_some()
    }
}
