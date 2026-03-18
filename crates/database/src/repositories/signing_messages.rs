use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::OnConflict,
};
use shared::{
    UnionAddress,
    result::{AppErr, Rs},
};

use crate::entities::signing_message;

pub async fn allocate(db: &DatabaseConnection, address: UnionAddress, message: String) -> Rs<()> {
    if message.len() > 98 {
        return Err(AppErr::custom(
            "signing message's lenght can not greater than 98",
        ));
    }

    signing_message::Entity::insert(signing_message::ActiveModel {
        address: Set(address.to_string()),
        message: Set(message),
    })
    .on_conflict(
        OnConflict::column(signing_message::Column::Address)
            .update_column(signing_message::Column::Message)
            .to_owned(),
    )
    .exec(db)
    .await?;

    Ok(())
}

pub async fn revoke(db: &DatabaseConnection, address: UnionAddress) -> Rs<()> {
    signing_message::Entity::delete_many()
        .filter(signing_message::Column::Address.eq(address.to_string()))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn get(db: &DatabaseConnection, address: UnionAddress) -> Rs<Option<String>> {
    let message = signing_message::Entity::find()
        .filter(signing_message::Column::Address.eq(address.to_string()))
        .one(db)
        .await?
        .map(|row| row.message);

    Ok(message)
}
