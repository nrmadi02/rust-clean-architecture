use std::sync::Arc;
use crate::errors::AppError;
use crate::domain::{User, UserRepository};

#[derive(Clone)]
pub struct UserUseCase{
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserUseCase{
    pub fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn get_user(&self, user_id: i32) -> Result<User, AppError> {
        self.repository.get_user(user_id).await
    }

    pub async fn create_user(&self, user: User) -> Result<User, AppError> {
        self.repository.create_user(user).await
    }
}