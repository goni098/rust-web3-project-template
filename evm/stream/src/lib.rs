use alloy::{primitives::Log, rpc::types::Log as RpcLog, sol_types::SolEventInterface};
use database::{repositories::log_memos, sea_orm::DatabaseConnection};
use evm_lib::{
    uniswap_v2::{UniswapPoolV2::UniswapPoolV2Events, WETH_USDC_V2_POOL},
    uniswap_v3::{UniswapPoolV3::UniswapPoolV3Events, WETH_USDC_V3_POOL, WETH_USDT_V3_POOL},
};
use shared::result::{AppErr, Rs};

enum Event {
    UniswapV3(Log<UniswapPoolV3Events>),
    UniswapV2(Log<UniswapPoolV2Events>),
}

impl Event {
    fn decode_log(log: &RpcLog) -> Rs<Self> {
        let address = log.address();

        let event = if address == WETH_USDT_V3_POOL || address == WETH_USDC_V3_POOL {
            Self::UniswapV3(UniswapPoolV3Events::decode_log(&log.inner)?)
        } else if address == WETH_USDC_V2_POOL {
            Self::UniswapV2(UniswapPoolV2Events::decode_log(&log.inner)?)
        } else {
            return Err(AppErr::custom("unknown log address"));
        };

        Ok(event)
    }
}

pub async fn handle_log(db: &DatabaseConnection, log: &RpcLog) -> Rs<()> {
    let hash = log.transaction_hash.unwrap_or_default();
    let log_ix = log.log_index.unwrap_or_default() as i32;

    if log_memos::is_existed(db, hash, log_ix).await? {
        return Ok(());
    }

    let event = Event::decode_log(log)?;

    match event {
        Event::UniswapV2(event) => {
            tracing::info!("uniswap pool v2 event: {:#?}", event);
        }
        Event::UniswapV3(event) => {
            tracing::info!("uniswap pool v3 event: {:#?}", event);
        }
    }

    log_memos::save(
        db,
        hash,
        log_ix,
        log.block_timestamp.unwrap_or_default() as i64,
    )
    .await?;

    Ok(())
}
