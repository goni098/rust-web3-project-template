use alloy::{primitives::Log, rpc::types::Log as RpcLog, sol_types::SolEventInterface};
use database::{repositories::log_memos, sea_orm::DatabaseConnection};
use evm_lib::{
    uniswap_v2::UniswapPoolV2::UniswapPoolV2Events, uniswap_v3::UniswapPoolV3::UniswapPoolV3Events,
};
use shared::result::Rs;

#[derive(Debug)]
enum Event {
    UniswapV3(Log<UniswapPoolV3Events>),
    UniswapV2(Log<UniswapPoolV2Events>),
}

impl Event {
    fn decode_log(log: &RpcLog) -> Rs<Option<Self>> {
        let Some(signature) = log.topic0() else {
            return Ok(None);
        };

        let event = if UniswapPoolV3Events::SELECTORS.contains(signature) {
            let event = Self::UniswapV3(UniswapPoolV3Events::decode_log(&log.inner)?);
            Some(event)
        } else if UniswapPoolV2Events::SELECTORS.contains(signature) {
            let event = Self::UniswapV2(UniswapPoolV2Events::decode_log(&log.inner)?);
            Some(event)
        } else {
            None
        };

        Ok(event)
    }
}

pub async fn handle_log(db: &DatabaseConnection, log: &RpcLog) -> Rs<()> {
    let hash = log.transaction_hash.unwrap_or_default();
    let log_ix = log.log_index.unwrap_or_default() as i32;
    let timestamp = log.block_timestamp.unwrap_or_default() as i64;

    if log_memos::is_existed(db, hash, log_ix).await? {
        return Ok(());
    }

    if let Some(event) = Event::decode_log(log)? {
        match event {
            Event::UniswapV2(_event) => {}
            Event::UniswapV3(_event) => {}
        };
    }

    log_memos::save(db, hash, log_ix, timestamp).await?;

    Ok(())
}
