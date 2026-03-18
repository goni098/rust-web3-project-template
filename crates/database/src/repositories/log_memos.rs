use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use shared::{UnionTxHash, result::Rs};

use crate::entities::log_memo;

pub async fn save<H: Into<UnionTxHash>>(
    db: &DatabaseConnection,
    hash: H,
    log_ix: i32,
    timestamp: i64,
) -> Rs<()> {
    log_memo::Entity::insert(log_memo::ActiveModel {
        hash: Set(hash.into().to_string()),
        log_ix: Set(log_ix),
        timestamp: Set(timestamp),
    })
    .exec(db)
    .await?;

    Ok(())
}

pub async fn is_existed<H: Into<UnionTxHash>>(
    db: &DatabaseConnection,
    hash: H,
    log_ix: i32,
) -> Rs<bool> {
    let is_existed = log_memo::Entity::find_by_id((hash.into().to_string(), log_ix))
        .one(db)
        .await?
        .is_some();

    Ok(is_existed)
}
