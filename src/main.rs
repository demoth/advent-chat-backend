mod auth;
mod chat;
mod models;
mod storage;
mod websocket;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tracing::Level;
use tracing_subscriber::{fmt};

#[tokio::main]
async fn main() {
    // Initialize tracing with environment filter for dynamic log level control
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO) // Set default log level from environment variable
        .with_target(true) // Include the target in logs
        .pretty() // Format logs to be more human-readable (formatted output)
        .init();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Define routes
    let app = Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/ws", get(websocket::handler))
        .layer(cors);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}