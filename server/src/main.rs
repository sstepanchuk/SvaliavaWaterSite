mod shared;

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use shared::{file_and_error_handler, AppState};
use tower_http::timeout::TimeoutLayer;
use tower_http::compression::{CompressionLayer, CompressionLevel};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use std::time::Duration;
use leptos_image::*;
use tracing::{info, error};
use tracing_subscriber::{fmt, filter::EnvFilter, prelude::*};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() {
    // Initialize tracing
    //LogTracer::init().expect("Failed to set log tracer");

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let conf = get_configuration(Some("./Cargo.toml")).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Log application start
    info!("Starting application with configuration");

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Create a middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        )
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Best)
                .gzip(true)
                .br(true)
                .zstd(true),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .into_inner();

    // Optimizer
    let optimizer = ImageOptimizer::new(
        "/__cache/image",
        leptos_options.site_root.to_string(),
        1,
    );

    // State
    let state = AppState { 
        leptos_options: leptos_options.clone(), 
        optimizer: optimizer,
    };
    
    let app = Router::new()
        .leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), {
            let state = state.clone();
            move || shell(state.leptos_options.clone())
        })
        .image_cache_route(&state)
        /* .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })*/
        .fallback(file_and_error_handler(shell))
        .layer(middleware_stack)
        .with_state(state);

    // Run the app with hyper
    info!("Listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("Server encountered an error: {}", e);
    }
}
