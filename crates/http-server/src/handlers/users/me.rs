use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

#[derive(Serialize)]
pub struct Response {
    address: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
) -> HttpResult<Json<Response>> {
    let _ = repositories::users::find_by_wallet_address(&db, claims.address)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    let response = Response {
        address: claims.address.to_string(),
    };

    Ok(Json(response))
}
