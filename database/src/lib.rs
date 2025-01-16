pub mod schema;
pub mod models;
pub mod repositories;
pub mod error;

use sqlx::SqlitePool;
use error::DbError;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbError> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn user_repository(&self) -> repositories::UserRepository {
        repositories::UserRepository::new(self.pool.clone())
    }
}