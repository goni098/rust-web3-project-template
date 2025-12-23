mod entities;
pub mod repositories;
pub use sea_orm;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn establish_connection(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);

    Database::connect(opt).await
}
