use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use crate::domain::dto::ApiErrorResponse;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User not found")]
    NotFound,

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Invalid Input")]
    InvalidInput(HashMap<String, Vec<String>>),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound => {
                let error = ApiErrorResponse {
                    status: "error".to_string(),
                    message: self.to_string(),
                    errors: None,
                };
                HttpResponse::NotFound().json(error)
            },
            AppError::DatabaseError(msg) => {
                let mut errors = HashMap::new();
                errors.insert("database".to_string(), vec![msg.clone()]);
                let error = ApiErrorResponse {
                    status: "error".to_string(),
                    message: "Database error occurred".to_string(),
                    errors: Some(errors),
                };
                HttpResponse::InternalServerError().json(error)
            },
            AppError::InternalServerError => {
                let error = ApiErrorResponse {
                    status: "error".to_string(),
                    message: self.to_string(),
                    errors: None,
                };
                HttpResponse::InternalServerError().json(error)
            },
            AppError::InvalidInput(errors_map) => {
                let error = ApiErrorResponse {
                    status: "error".to_string(),
                    message: "Invalid input provided".to_string(),
                    errors: Some(errors_map.clone()),
                };
                HttpResponse::BadRequest().json(error)
            },
        }
    }
}
