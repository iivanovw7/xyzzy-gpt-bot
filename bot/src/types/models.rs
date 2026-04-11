use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
    pub date: NaiveDateTime,
    pub category_id: i64,
    pub category_name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkRow {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub tags: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkCategoryRow {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
}
