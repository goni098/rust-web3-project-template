use axum::{
    Router,
    routing::{get, post},
};

use crate::extractors::state::AppState;

mod me;
mod sign_in;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(me::handler))
        .route("/users/sign-in", post(sign_in::handler))
}
