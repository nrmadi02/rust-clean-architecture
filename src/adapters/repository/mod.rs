use std::sync::Arc;
use sqlx::PgPool;
use async_trait::async_trait;
use crate::domain::{User, UserRepository};
use crate::errors::AppError;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool:  Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool)
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_user(&self, user_id: i32) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email FROM users WHERE id = $1",
            user_id
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(|_| AppError::NotFound)?;

        Ok(user)
    }

    async fn create_user(&self, user: User) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
            user.username,
            user.email
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }
}