mod entities;
pub mod repositories;
pub use sea_orm;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

/// Establishes a connection to the database with optimized settings
/// 
/// # Arguments
/// * `db_url` - Database connection URL
/// 
/// # Returns
/// A database connection or an error if connection fails
pub async fn establish_connection(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false)
        .max_connections(100)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(30));

    Database::connect(opt).await
}
