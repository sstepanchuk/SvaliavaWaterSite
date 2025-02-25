use app::shared::state::{init_state, AppState};
use app::*;
use axum::body::Bytes;
use axum::extract::Host;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_image::*;
use tower_http::LatencyUnit;
use tracing::level_filters::LevelFilter;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::compression::{CompressionLayer, CompressionLevel};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    // load env vars
    match env::current_exe() {
        Ok(path) => info!("Current executable path: {}", path.display()),
        Err(e) => error!("Failed to get current executable path: {}", e),
    }

    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy()
        )
        .init();

    let conf = get_configuration(Some("./Cargo.toml")).unwrap();
    let mut addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let tls_config: Option<RustlsConfig> = get_tls_config().await.unwrap();

    let state = init_state(leptos_options).await.unwrap();

    // Run db migrations
    Migrator::up(&state.db, None).await.unwrap();

    // Log application start
    info!("Starting application with configuration");

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Create a middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Best)
        )
        .layer(
            TraceLayer::new_for_http()
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros))
        )
        .layer(
            TimeoutLayer::new(
                Duration::from_secs(30)
            )
        ).into_inner();

    let app = Router::new()
        .leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), {
            let state = state.clone();
            move || shell(state.leptos_options.clone())
        })
        .image_cache_route(&state)
        .fallback(
            leptos_axum::file_and_error_handler_with_context::<AppState, _>(
                state.optimizer.provide_context(), 
                shell
            )
        )
        .layer(middleware_stack)
        .with_state(state);

    // If we have TLS config, run HTTPS and HTTP servers
    if let Some(tls_config) = tls_config {
        tokio::spawn(start_http_redirect_server(addr.clone()));

        addr.set_port(addr.port() + 443);
        info!("Starting HTTPS server on port {}", addr);

        if let Err(e) = axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
        {
            warn!("HTTPS server failed: {}", e);
        }
    } else {
        info!("Starting HTTP server on port {}", addr.port());

        // Start HTTP server (without TLS)
        if let Err(e) = axum_server::bind(addr).serve(app.into_make_service()).await {
            warn!("HTTP server failed: {}", e);
        }
    }
}

async fn get_tls_config() -> Result<Option<RustlsConfig>> {
    // Retrieve base64-encoded certificate and key from ENV variables
    let cert_base64 = match env::var("TLS_CERT") {
        Ok(val) => val,
        Err(_) => {
            warn!("TLS_CERT environment variable not set");
            return Ok(None); // Return None if the certificate variable is not set
        }
    };

    let key_base64 = match env::var("TLS_KEY") {
        Ok(val) => val,
        Err(_) => {
            warn!("TLS_KEY environment variable not set");
            return Ok(None); // Return None if the key variable is not set
        }
    };

    debug!("Decoding base64-encoded certificate and key");

    // Decode the base64 strings into bytes
    let cert_bytes = match STANDARD.decode(&cert_base64) {
        Ok(bytes) => bytes,
        Err(_) => {
            warn!("Failed to decode certificate from base64");
            return Ok(None); // Return None if base64 decoding fails for certificate
        }
    };

    let key_bytes = match STANDARD.decode(&key_base64) {
        Ok(bytes) => bytes,
        Err(_) => {
            warn!("Failed to decode private key from base64");
            return Ok(None); // Return None if base64 decoding fails for private key
        }
    };

    debug!("Successfully decoded certificate and key");

    // Create and return the RustlsConfig directly from the decoded bytes
    match RustlsConfig::from_pem(cert_bytes, key_bytes).await {
        Ok(config) => {
            debug!("TLS configuration successfully created");
            Ok(Some(config))
        }
        Err(e) => {
            warn!("Failed to create TLS config: {}", e);
            Ok(None)
        }
    }
}

pub async fn start_http_redirect_server(addr: SocketAddr) {
    let app = Router::new().fallback(
        get(|Host(hostname): Host| async move {
            let redirect_url = format!("https://{}/", hostname);
            Redirect::permanent(&redirect_url)
        })
    );

    info!("Starting HTTP redirect server on {}", addr);

    if let Err(e) = axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
    {
        warn!("HTTP redirect server failed: {}", e);
    }
}