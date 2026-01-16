use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::extractors::state::AppState;

mod exception;
mod extractors;
mod handlers;

/// Default server port
const SERVER_PORT: u16 = 8080;

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

    let addr = format!("0.0.0.0:{}", SERVER_PORT);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!(
        addr = %listener.local_addr()?,
        "🦀 Server is running"
    );

    axum::serve(listener, app).await?;

    Ok(())
}
