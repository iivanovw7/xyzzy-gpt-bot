use chrono::NaiveDate;
use sqlx::Row;

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
        let mut query = "SELECT 
                        t.amount,
                        t.description,
                        DATE(t.date, 'unixepoch') AS date,
                        c.name as category
                     FROM transactions t
                     JOIN categories c ON c.id = t.category_id
                     WHERE t.user_id = ?"
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
                let amount: i64 = r.try_get("amount").unwrap();
                let date_str: String = r.try_get("date").unwrap();
                let category: String = r.try_get("category").unwrap();
                let description: String = r.try_get("description").unwrap();
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                TransactionRow {
                    amount,
                    date,
                    category,
                    description,
                }
            })
            .collect()
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
}
