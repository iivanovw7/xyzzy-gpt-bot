use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryRow {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRow {
    pub id: i64,
    pub amount: i64,
    pub date: NaiveDate,
    pub category: String,
    pub description: String,
}
