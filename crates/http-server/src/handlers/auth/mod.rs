use axum::{Router, routing};

use crate::extractors::state::AppState;

mod req_signing_msg;
mod sign_in_evm;
mod sign_in_sol;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/signing-msg", routing::post(req_signing_msg::handler))
        .route("/auth/sign-in-sol", routing::post(sign_in_sol::handler))
        .route("/auth/sign-in-evm", routing::post(sign_in_evm::handler))
}
