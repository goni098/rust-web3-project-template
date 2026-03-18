use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shared::{UnionAddr, result::Rs};

use crate::entities::user;

pub async fn find_by_wallet_address<A: UnionAddr>(
    db: &DatabaseConnection,
    address: A,
) -> Rs<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::WalletAddress.eq(address.to_string()))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn save<A: UnionAddr>(db: &DatabaseConnection, address: A) -> Rs<()> {
    let user = user::ActiveModel {
        wallet_address: Set(address.to_string()),
    };

    user::Entity::insert(user)
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

    Ok(())
}
