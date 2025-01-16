use axum::{
    Router,
    routing::{get, post},
    Extension,
    Json,
    extract::Path,
};
use database::Database;
use shared::{
    models::{User, ApiResponse},
    routes::UserRoute,
};
use crate::{error::AppError, middleware::auth::require_auth};

pub fn create_router() -> Router {
    Router::new()
        .route(UserRoute::PATH, get(get_user))
        .route_layer(require_auth())
}

async fn get_user(
    Extension(database): Extension<Database>,
    Path(user_id): Path<i32>,
) -> Result<Json<ApiResponse<User>>, AppError> {
    let user = database
        .user_repository()
        .find_by_id(user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(ApiResponse {
        data: Some(user),
        error: None,
    }))
}