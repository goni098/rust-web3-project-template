use sea_orm::{DatabaseConnection, EntityTrait};
use shared::result::Rs;
use tracing::instrument;

use crate::entities::user;

#[instrument]
pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Rs<Option<user::Model>> {
    let user = user::Entity::find_by_id(id).one(db).await?;
    Ok(user)
}
