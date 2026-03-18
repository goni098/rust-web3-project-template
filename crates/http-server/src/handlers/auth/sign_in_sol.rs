use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use validator::Validate;

use crate::{
    common,
    exception::{HttpException, HttpResult},
    extractors::validator::ValidatedPayload,
};

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(custom(function = "shared::validators::validate_solana_pubkey"))]
    address: String,
    message: String,
    #[validate(custom(function = "shared::validators::validate_solana_signature"))]
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
    let address = address.parse::<Pubkey>()?;
    let signature = signature.parse::<Signature>()?;

    let Some(msg) = repositories::signing_messages::get(&db, address).await? else {
        return Err(HttpException::unauthorized("msg was revoked"));
    };

    if msg != message {
        return Err(HttpException::unauthorized("invalid message"));
    }

    let is_valid_sig = signature.verify(address.as_array(), message.as_bytes());

    if !is_valid_sig {
        return Err(HttpException::unauthorized("invalid signature"));
    }

    let token = common::jwt::sign(address)?;

    let response = Response { token };

    Ok(Json(response))
}
