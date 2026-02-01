use alloy::{rpc::types::Log, sol_types::SolEventInterface};
use database::sea_orm::DatabaseConnection;
use evm::uniswap_v3::UniswapPoolV3::UniswapPoolV3Events;
use shared::result::Rs;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn handle_log(_db: &DatabaseConnection, log: Log) -> Rs<()> {
    let event = UniswapPoolV3Events::decode_log(&log.into_inner())?;
    tracing::info!("uniswap pool v3 event: {:#?}", event);

    Ok(())
}
