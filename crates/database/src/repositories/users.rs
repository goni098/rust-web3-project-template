use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shared::{UnionAddress, result::Rs};

use crate::entities::user;

pub async fn find_by_wallet_address<A: Into<UnionAddress>>(
    db: &DatabaseConnection,
    address: A,
) -> Rs<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::WalletAddress.eq(address.into().to_string()))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn save<A: Into<UnionAddress>>(db: &DatabaseConnection, address: A) -> Rs<()> {
    let user = user::ActiveModel {
        wallet_address: Set(address.into().to_string()),
    };

    user::Entity::insert(user)
        .on_conflict_do_nothing()
        .exec(db)
        .await?;

    Ok(())
}
