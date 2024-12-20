use std::collections::HashMap;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;
use crate::domain::{User};
use crate::errors::AppError;
use crate::use_case::UserUseCase;
use crate::domain::dto::{ApiResponse, ApiErrorResponse};
use crate::domain::response_helpers::{map_app_error_to_response, success_response};

#[derive(Deserialize, ToSchema, Validate, Debug)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "ID of the user to retrieve")
    ),
    responses(
        (status = 200, description = "User found", body = ApiResponse<User>),
        (status = 404, description = "User not found", body = ApiErrorResponse)
    ),
    tag = "User"
)]
pub async fn get_user(
    path: web::Path<i32>,
    user_use_case: web::Data<UserUseCase>
) -> Result<impl Responder, actix_web::Error> {
    let user_id = path.into_inner();
    info!("Received request to get user with ID: {}", user_id);

    match user_use_case.get_user(user_id).await {
        Ok(user) => {
            info!("User found: {:#?}", user);
            let response = success_response("User found", user);
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            let response = map_app_error_to_response(e);
            Ok(response)
        },
    }
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = ApiResponse<User>),
        (status = 400, description = "Invalid input", body = ApiErrorResponse)
    ),
    tag = "User"
)]
pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    user_use_case: web::Data<UserUseCase>
) -> Result<impl Responder, actix_web::Error> {

    let mut validation_errors = HashMap::new();

    if let Err(e) = user.validate() {
        for (field, errors) in e.field_errors() {
            let messages: Vec<String> = errors.iter().map(|err| {
                if let Some(msg) = &err.message {
                    msg.to_string()
                } else {
                    "Invalid input".to_string()
                }
            }).collect();
            validation_errors.insert(field.to_string(), messages);
        }
    }


    if !validation_errors.is_empty() {
        error!("Invalid input for creating user: {:?}", validation_errors);
        let error_response = AppError::InvalidInput(validation_errors);
        let response = map_app_error_to_response(error_response);
        return Ok(response);
    }

    let user_data = user.into_inner();
    info!("Received request to create user: {:?}", user_data);
    let new_user = User {
        id: 0, // ID akan diisi oleh database
        username: user_data.username.clone(),
        email: user_data.email.clone(),
    };
    match user_use_case.create_user(new_user).await {
        Ok(created_user) => {
            info!("User created successfully: {:?}", created_user);
            let response = success_response("User created successfully", created_user);
            Ok(HttpResponse::Created().json(response))
        },
        Err(e) => {
            error!("Error creating user: {}", e);
            let response = map_app_error_to_response(e);
            Ok(response)
        }
    }
}