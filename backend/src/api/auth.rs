use axum::{
    Router,
    routing::post,
    Extension,
    Json,
};
use database::Database;
use shared::{
    models::{LoginRequest, ApiResponse, AuthResponse},
    routes::LoginRoute,
};
use crate::error::AppError;
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn create_router() -> Router {
    Router::new()
        .route(LoginRoute::PATH, post(login))
}

async fn login(
    Extension(database): Extension<Database>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let user = database
        .user_repository()
        .find_by_username(&login_req.username)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Тут має бути перевірка пароля

    let token = create_token(&user.id.to_string())?;

    Ok(Json(ApiResponse {
        data: Some(AuthResponse { token }),
        error: None,
    }))
}

fn create_token(user_id: &str) -> Result<String, AppError> {
    // Логіка створення JWT токена
    Ok("token".to_string())
}