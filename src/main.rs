use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use hyper::Method;
use log::info;
use std::env;
use tower_http::cors::{Any, CorsLayer};

#[path = "util/resizer.rs"]
mod util_resizer;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init();
    info!("🚀 Rusty Salmon Server starting...");

    // Get environment variables
    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let app_host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or_else(|_| "9999".to_string());

    // Log environment variables
    info!("Server configured to accept connections on host {}...", app_host);
    info!("Server configured to listen connections on port {}...", app_port);

    // Log environment
    match app_environment.as_str() {
        "development" => info!("Running in development mode"),
        "production" => info!("Running in production mode"),
        _ => info!("Running in development mode"),
    }

    // create a new CorsLayer that allows all requests
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/health", get(|| async { "Hello, World!" }))
        .route("/resize", post(util_resizer::resize_image))
        .layer(cors);

    // run our app with hyper, listening globally on port
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", app_host, app_port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
