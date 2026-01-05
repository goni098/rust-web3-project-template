use alloy::rpc::types::Log;
use database::sea_orm::DatabaseConnection;
use shared::result::Rs;

pub async fn proceed_log(_db: &DatabaseConnection, _log: Log) -> Rs<()> {
    Ok(())
}
