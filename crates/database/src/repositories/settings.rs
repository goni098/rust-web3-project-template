use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    sea_query::Expr,
};
use tracing::instrument;

use crate::entities::setting;

#[derive(Clone, Copy, Debug)]
pub enum Setting {
    Something,
}

#[instrument(skip(db))]
pub async fn get(db: &DatabaseConnection, key: Setting) -> Result<Option<String>, DbErr> {
    let val = setting::Entity::find_by_id(key.to_str_key())
        .one(db)
        .await?
        .map(|record| record.value);

    Ok(val)
}

#[instrument(skip(db))]
pub async fn set(db: &DatabaseConnection, key: Setting, value: String) -> Result<(), DbErr> {
    setting::Entity::update_many()
        .col_expr(setting::Column::Value, Expr::value(value))
        .filter(setting::Column::Key.eq(key.to_str_key()))
        .exec(db)
        .await?;

    Ok(())
}

#[instrument(skip(db))]
pub async fn insert(db: &DatabaseConnection, key: Setting, value: String) -> Result<(), DbErr> {
    setting::Entity::insert(setting::ActiveModel {
        key: Set(key.to_str_key().to_string()),
        value: Set(value),
    })
    .exec(db)
    .await?;

    Ok(())
}

impl Setting {
    fn to_str_key(self) -> &'static str {
        match self {
            Self::Something => "some_thing",
        }
    }
}
