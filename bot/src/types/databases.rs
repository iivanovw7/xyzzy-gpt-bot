use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

pub struct UsersDb {
    pub pool: SqlitePool,
}

pub struct CategoriesDb {
    pub pool: SqlitePool,
}

pub struct TransactionsDb {
    pub pool: SqlitePool,
}

impl UsersDb {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl CategoriesDb {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl TransactionsDb {
    pub fn new(pool: &SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }
}
