use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;
use crate::domain::{User};
use crate::errors::AppError;
use crate::use_case::UserUseCase;

#[derive(Deserialize, ToSchema, Validate)]
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
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found", body = crate::errors::ErrorResponse)
    ),
    tag = "User"
)]
pub async fn get_user(
    path: web::Path<i32>,
    user_use_case: web::Data<UserUseCase>
) -> Result<impl Responder, AppError> {
    let user_id = path.into_inner();
    let user = user_use_case.get_user(user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input", body = crate::errors::ErrorResponse)
    ),
    tag = "User"
)]
pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    user_use_case: web::Data<UserUseCase>
) -> Result<impl Responder, AppError> {
    user.validate().map_err(|e| AppError::InvalidInput(e.to_string()))?;

    let new_user = User {
        id: 0,
        username: user.username.clone(),
        email: user.email.clone(),
    };
    let created_user = user_use_case.create_user(new_user).await?;
    Ok(HttpResponse::Created().json(created_user))
}