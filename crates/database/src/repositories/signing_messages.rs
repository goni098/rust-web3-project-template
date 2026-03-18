use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::OnConflict,
};
use shared::{
    UnionAddress,
    result::{AppErr, Rs},
};

use crate::entities::signing_message;

pub async fn allocate<A>(db: &DatabaseConnection, address: A, message: String) -> Rs<()>
where
    A: Into<UnionAddress>,
{
    if message.len() > 98 {
        return Err(AppErr::custom(
            "signing message's lenght can not greater than 98",
        ));
    }

    signing_message::Entity::insert(signing_message::ActiveModel {
        address: Set(address.into().to_string()),
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

pub async fn revoke<A>(db: &DatabaseConnection, address: A) -> Rs<()>
where
    A: Into<UnionAddress>,
{
    signing_message::Entity::delete_many()
        .filter(signing_message::Column::Address.eq(address.into().to_string()))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn get<A>(db: &DatabaseConnection, address: A) -> Rs<Option<String>>
where
    A: Into<UnionAddress>,
{
    let message = signing_message::Entity::find()
        .filter(signing_message::Column::Address.eq(address.into().to_string()))
        .one(db)
        .await?
        .map(|row| row.message);

    Ok(message)
}
