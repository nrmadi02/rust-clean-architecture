use utoipa::OpenApi;
use crate::adapters::api::__path_create_user;
use crate::adapters::api::__path_get_user;
use crate::domain::User;
use crate::domain::dto::{ ApiResponse, ApiErrorResponse};
use crate::adapters::api::CreateUserRequest;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Clean Architecture API",
        description = "API backend sederhana yang dibangun dengan Rust, mengikuti prinsip Clean Architecture. API ini memungkinkan Anda untuk membuat dan mengambil data pengguna.",
        version = "1.0.0",
    ),
    paths(
        get_user,
        create_user,
    ),
    components(
        schemas(
            User,
            ApiResponse<User>,
            ApiErrorResponse,
            CreateUserRequest,
        )
    ),
    tags(
        (name = "User", description = "User management endpoints.")
    )
)]
pub struct ApiDoc;
