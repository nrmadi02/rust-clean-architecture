use std::collections::HashMap;
use actix_web::HttpResponse;
use serde::Serialize;
use crate::domain::dto::{ApiErrorResponse, ApiResponse};

pub fn success_response<T: Serialize>(message: &str, data:T) -> ApiResponse<T> {
    ApiResponse {
        status: "success".to_string(),
        message: message.to_string(),
        data: Some(data),
    }
}

pub fn error_response(message: &str, errors: Option<HashMap<String, Vec<String>>>) -> ApiErrorResponse {
    ApiErrorResponse {
        status: "error".to_string(),
        message: message.to_string(),
        errors,
    }
}

pub fn map_app_error_to_response(e: crate::errors::AppError) -> HttpResponse {
    match e {
        crate::errors::AppError::NotFound => {
            let error = error_response("User not found", None);
            HttpResponse::NotFound().json(error)
        },
        crate::errors::AppError::DatabaseError(msg) => {
            let mut errors = HashMap::new();
            errors.insert("database".to_string(), vec![msg]);
            let error = error_response("Database error occurred", Some(errors));
            HttpResponse::InternalServerError().json(error)
        },
        crate::errors::AppError::InternalServerError => {
            let error = error_response("Internal server error", None);
            HttpResponse::InternalServerError().json(error)
        },
        crate::errors::AppError::InvalidInput(errors_map) => {
            let error = error_response("Invalid input provided", Some(errors_map));
            HttpResponse::BadRequest().json(error)
        },
    }
}