use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::errors::AppError;

pub async fn get_db_pool() -> Result<sqlx::PgPool, AppError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|_| AppError::InternalServerError)?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(pool)
}