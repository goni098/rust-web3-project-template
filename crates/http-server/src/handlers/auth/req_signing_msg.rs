use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::{Deserialize, Serialize};
use shared::UnionAddress;
use validator::Validate;

use crate::{exception::HttpResult, extractors::validator::ValidatedPayload};

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(custom(function = "shared::validators::validate_union_address"))]
    address: String,
}

#[derive(Serialize)]
pub struct Response {
    msg: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    ValidatedPayload(Payload { address }): ValidatedPayload<Payload>,
) -> HttpResult<Json<Response>> {
    let address = address.parse::<UnionAddress>()?;
    let msg = format!("Welcome {}", address);
    repositories::signing_messages::allocate(&db, address, msg.clone()).await?;

    let response = Response { msg };

    Ok(Json(response))
}
