use sqlx::SqlitePool;

use crate::{
    env::ENV,
    types::databases::{CategoriesDb, Database, TransactionsDb, UsersDb},
};

impl Database {
    pub async fn new() -> Self {
        let pool = SqlitePool::connect(&ENV.database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self { pool }
    }

    pub fn users(&self) -> UsersDb {
        UsersDb::new(&self.pool)
    }

    pub fn categories(&self) -> CategoriesDb {
        CategoriesDb::new(&self.pool)
    }

    pub fn transactions(&self) -> TransactionsDb {
        TransactionsDb::new(&self.pool)
    }
}
