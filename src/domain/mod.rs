use serde::{Deserialize, Serialize};
use crate::errors::AppError;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "John Doe")]
    pub username: String,

    #[schema(example = "john.doe@example.com")]
    pub email: String,
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, id: i32) -> Result<User, AppError>;
    async fn create_user(&self, user: User) -> Result<User, AppError>;
}