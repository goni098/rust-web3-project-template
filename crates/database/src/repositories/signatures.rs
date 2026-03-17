use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use shared::result::Rs;

use crate::entities::signature;

pub async fn upsert(db: &DatabaseConnection, signature: String, ts: i64) -> Rs<()> {
    signature::Entity::insert(signature::ActiveModel {
        id: Default::default(),
        sig: Set(signature),
        ts: Set(ts),
    })
    .on_conflict_do_nothing()
    .exec(db)
    .await?;

    Ok(())
}
