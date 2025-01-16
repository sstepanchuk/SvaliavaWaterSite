use axum::{
    Router,
    Extension,
    error_handling::HandleErrorLayer,
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use database::Database;
use crate::{config::Config, error::AppError};

pub async fn run(config: Config, database: Database) -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origins(config.cors_origins.iter().map(|origin| origin.parse().unwrap()))
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
        ])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .max_age(Duration::from_secs(3600));

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_| async { AppError::InternalServerError }))
        .layer(Extension(database))
        .layer(Extension(config.clone()))
        .layer(cors);

    let app = Router::new()
        .merge(crate::api::create_router())
        .layer(middleware_stack);

    let addr = config.server_addr.parse()?;
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}