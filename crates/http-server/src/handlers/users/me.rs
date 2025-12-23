use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

#[derive(Serialize)]
pub struct Response {
    id: i64,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
) -> HttpResult<Json<Response>> {
    let user = repositories::users::find_by_id(&db, claims.id)
        .await?
        .ok_or(HttpException::Internal("user not found".into()))?;

    let response = Response { id: user.id };

    Ok(Json(response))
}
