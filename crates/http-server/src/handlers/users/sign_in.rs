use axum::Json;
use serde::Serialize;

use crate::exception::HttpResult;

#[derive(Serialize)]
pub struct Response {
    num: u64,
}

pub async fn handler() -> HttpResult<Json<Response>> {
    let num: u64 = "ab".parse()?;

    Ok(Json(Response { num }))
}
