use axum::Router;

pub mod auth;
pub mod users;

pub fn create_router() -> Router {
    Router::new()
        .merge(auth::create_router())
        .merge(users::create_router())
}

