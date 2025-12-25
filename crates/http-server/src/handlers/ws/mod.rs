use axum::{Router, routing::get};

use crate::extractors::state::AppState;

mod random_u64;

pub fn routes() -> Router<AppState> {
    Router::new().route("/random-u64", get(random_u64::handler))
}
