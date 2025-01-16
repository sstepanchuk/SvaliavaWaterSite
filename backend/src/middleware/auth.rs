use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::auth::AuthenticationLayer;
use tower::ServiceBuilder;
use crate::error::AppError;

pub fn require_auth<B>() -> ServiceBuilder<AuthenticationLayer<B>> {
    ServiceBuilder::new().layer(AuthenticationLayer::new(auth_middleware))
}

async fn auth_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth) if auth.starts_with("Bearer ") => {
            let token = &auth[7..];
            // Тут має бути валідація токена
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}