use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use shared::error::ApiError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Access forbidden")]
    Forbidden,
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Database error: {0}")]
    Database(#[from] database::error::DbError),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}