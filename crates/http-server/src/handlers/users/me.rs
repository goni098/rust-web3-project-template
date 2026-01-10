use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;
use tracing::instrument;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

#[derive(Serialize)]
pub struct Response {
    id: i64,
}

#[instrument(skip_all)]
pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
) -> HttpResult<Json<Response>> {
    let user = repositories::users::find_by_id(&db, claims.id)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    let response = Response { id: user.id };

    Ok(Json(response))
}
