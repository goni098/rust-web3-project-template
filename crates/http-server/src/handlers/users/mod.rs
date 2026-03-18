use axum::{Router, routing};

use crate::extractors::state::AppState;

mod me;

pub fn routes() -> Router<AppState> {
    Router::new().route("/users/me", routing::get(me::handler))
}
