use chrono::NaiveDate;
use sqlx::Row;
use std::string::String;

use crate::types::{common::DateFilter, databases::TransactionsDb, models::TransactionRow};

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

    pub async fn list_with_range(
        &self,
        user_id: i64,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Vec<TransactionRow> {
        let mut query = r#"SELECT 
                        t.id,
                        t.amount,
                        t.description,
                        DATE(t.date, 'unixepoch') AS date,
                        c.id as category_id,
                        c.name as category_name
                     FROM transactions t
                     JOIN categories c ON c.id = t.category_id
                     WHERE t.user_id = ?"#
            .to_string();

        if start_date.is_some() && end_date.is_some() {
            query.push_str(" AND DATE(t.date, 'unixepoch') BETWEEN ? AND ?");
        } else if start_date.is_some() {
            query.push_str(" AND DATE(t.date, 'unixepoch') >= ?");
        } else if end_date.is_some() {
            query.push_str(" AND DATE(t.date, 'unixepoch') <= ?");
        }

        let mut q = sqlx::query(&query).bind(user_id);

        if let Some(start) = start_date {
            q = q.bind(start.format("%Y-%m-%d").to_string());
        }
        if let Some(end) = end_date {
            q = q.bind(end.format("%Y-%m-%d").to_string());
        }

        let rows = q.fetch_all(&self.pool).await.unwrap();

        rows.into_iter()
            .map(|r| {
                let id: i64 = r.try_get("id").unwrap();
                let amount: i64 = r.try_get("amount").unwrap();
                let date_str: String = r.try_get("date").unwrap();
                let category_name: String = r.try_get("category_name").unwrap();
                let category_id: i64 = r.try_get("category_id").unwrap();
                let description: String = r.try_get("description").unwrap();
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                TransactionRow {
                    id,
                    amount,
                    date,
                    category_name,
                    category_id,
                    description,
                }
            })
            .collect()
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM transactions
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_last(&self, user_id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"
        DELETE FROM transactions
        WHERE id = (
            SELECT id FROM transactions
            WHERE user_id = ?
            ORDER BY date DESC, id DESC
            LIMIT 1
        )
        "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_last(&self, user_id: i64) -> Option<TransactionRow> {
        let row = sqlx::query!(
            r#"
            SELECT 
                t.id,
                t.amount,
                t.description,
                DATE(t.date, 'unixepoch') AS date,
                c.name AS "category_name!: String",
                c.id AS "category_id!: i64"
            FROM transactions t
            JOIN categories c ON c.id = t.category_id
            WHERE t.user_id = ?
            ORDER BY t.date DESC, t.id DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap();

        row.map(|r| {
            let date = NaiveDate::parse_from_str(&r.date.unwrap(), "%Y-%m-%d").unwrap();

            TransactionRow {
                id: r.id,
                amount: r.amount,
                date,
                category_name: r.category_name,
                category_id: r.category_id,
                description: r.description.unwrap(),
            }
        })
    }

    pub async fn list_filtered(&self, user_id: i64, filter: DateFilter) -> Vec<TransactionRow> {
        let (start, end) = filter.range();

        self.list_with_range(user_id, start, end).await
    }

    pub async fn has_transactions_for_category(&self, category_id: i64) -> bool {
        sqlx::query!(
            "SELECT 1 AS dummy FROM transactions WHERE category_id = ? LIMIT 1",
            category_id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
        .is_some()
    }

    pub async fn search_by_description(
        &self,
        user_id: i64,
        search: &str,
        limit: i64,
    ) -> Vec<TransactionRow> {
        let search_lc = search.to_lowercase();
        let like_pattern = format!("%{}%", search_lc);

        let rows = sqlx::query!(
            r#"
            SELECT
                t.id,
                t.amount,
                t.description,
                COALESCE(DATE(t.date, 'unixepoch'), '') AS "date!: String",
                c.name AS "category_name!: String",
                c.id AS "category_id!: i64"
            FROM transactions t
            JOIN categories c ON c.id = t.category_id
            WHERE t.user_id = ?
            AND LOWER(t.description) LIKE ?
            ORDER BY
                (COALESCE(INSTR(LOWER(t.description), ?), -1) = 1) DESC,
                COALESCE(INSTR(LOWER(t.description), ?), -1) ASC,
            LENGTH(t.description) ASC
            LIMIT ?
            "#,
            user_id,
            like_pattern,
            search_lc,
            search_lc,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        rows.into_iter()
            .map(|r| {
                let id: i64 = r.id.unwrap();
                let amount: i64 = r.amount;
                let date_str: String = r.date.clone();
                let category_id: i64 = r.category_id.clone();
                let category_name: String = r.category_name.clone();
                let description: String = r.description.unwrap();
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                TransactionRow {
                    id,
                    amount,
                    date,
                    category_name,
                    category_id,
                    description,
                }
            })
            .collect()
    }
}
