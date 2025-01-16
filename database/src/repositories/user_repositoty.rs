use async_trait::async_trait;
use sqlx::SqlitePool;
use shared::models::User;
use crate::error::DbError;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<User, DbError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, DbError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DbError>;
}

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    // Реалізація методів
}