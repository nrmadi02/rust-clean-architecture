
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
pub enum AppError {

    #[error("Not Found")]
    NotFound,

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Invalid Input: {0}")]
    InvalidInput(String),
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        match self {
            AppError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse { error: error_message })
            }
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse { error: error_message })
            }
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse { error: error_message })
            }
            AppError::InvalidInput(_) => {
                HttpResponse::BadRequest().json(ErrorResponse { error: error_message })
            }
        }
    }
}
