use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Request failed: {0}")]
    RequestFailed(String),
}