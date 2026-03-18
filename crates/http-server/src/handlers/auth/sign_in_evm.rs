use alloy::{primitives::Address, signers::Signature};
use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    common,
    exception::{HttpException, HttpResult},
    extractors::validator::ValidatedPayload,
};

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(custom(function = "shared::validators::validate_evm_address"))]
    address: String,
    message: String,
    #[validate(custom(function = "shared::validators::validate_evm_signature"))]
    signature: String,
}

#[derive(Serialize)]
pub struct Response {
    token: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    ValidatedPayload(Payload {
        address,
        message,
        signature,
    }): ValidatedPayload<Payload>,
) -> HttpResult<Json<Response>> {
    let address = address.parse::<Address>()?;
    let signature = signature.parse::<Signature>()?;

    let Some(msg) = repositories::signing_messages::get(&db, address.into()).await? else {
        return Err(HttpException::unauthorized("msg was revoked"));
    };

    if msg != message {
        return Err(HttpException::unauthorized("invalid message"));
    }

    let recovered_address = signature
        .recover_address_from_msg(message.as_bytes())
        .map_err(|error| HttpException::unauthorized(error.to_string()))?;

    if recovered_address != address {
        return Err(HttpException::unauthorized("mismatch signature address"));
    }

    let token = common::jwt::sign(address.into())?;

    let response = Response { token };

    Ok(Json(response))
}
