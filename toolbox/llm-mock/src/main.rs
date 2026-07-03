mod config;
mod error;
pub mod models;
pub mod routes;

use axum::{routing::{get, post}, Router};
use config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("config.toml")?;
    let response_content = config.load_response_content()?;

    let app_state = Arc::new(AppState {
        config,
        response_content,
    });

    let app = Router::new()
        .route("/v1/models", get(routes::models::list_models))
        .route("/v1/embeddings", post(routes::embeddings::create_embedding))
        .route("/v1/chat/completions", post(routes::chat::chat_completions))
        .with_state(app_state.clone());

    let addr = format!(
        "{}:{}",
        app_state.config.server.host, app_state.config.server.port
    );
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

struct AppState {
    config: Config,
    response_content: String,
}