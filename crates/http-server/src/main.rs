use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;

use crate::extractors::state::AppState;

mod exception;
mod extractors;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    shared::tracing::subscribe();
    shared::env::load();

    let state = AppState::new().await;

    let app = Router::new()
        .route("/", get(async || "🦀 hello !"))
        .route(
            "/docs/openapi.yml",
            get(async || include_str!("../docs/openapi.yml")),
        )
        .route(
            "/docs",
            get(async || Html(include_str!("../docs/openapi.html"))),
        )
        .merge(handlers::users::routes())
        .merge(handlers::ws::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("🦀 server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
