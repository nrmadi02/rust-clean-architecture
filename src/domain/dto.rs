
use serde::Serialize;
use utoipa::ToSchema;
use std::collections::HashMap;

#[derive(Serialize, ToSchema)]
#[schema(description = "Standard API response structure.")]
pub struct ApiResponse<T> {
    pub status: String,

    pub message: String,

    #[schema(nullable)]
    pub data: Option<T>,
}

#[derive(Serialize, ToSchema)]
#[schema(description = "Standard API error response structure.")]
pub struct ApiErrorResponse {
    pub status: String,

    pub message: String,

    #[schema(nullable)]
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, ToSchema)]
#[schema(description = "Detail information about an error.")]
pub struct ErrorDetail {
    pub code: String,

    pub message: String,
}
