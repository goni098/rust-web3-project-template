use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption},
};
use database::repositories::settings::Setting;
use database::{repositories, sea_orm::DatabaseConnection};
use evm_lib::{
    SupportedChain,
    client::{PublicClient, create_public_client},
    uniswap_v2::UniswapPoolV2::UniswapPoolV2Events,
    uniswap_v3::UniswapPoolV3::UniswapPoolV3Events,
};
use futures_util::future::try_join_all;
use shared::{env::Env, result::Rs};
use tokio::time::sleep;

const SCAN_FREQUENCY: Duration = Duration::from_millis(6_000);

#[tokio::main]
async fn main() {
    shared::env::load();
    shared::tracing::subscribe();
    let chain_id = shared::arg::parse_chain_id_arg();
    bootstrap(chain_id).await.unwrap();
}

async fn bootstrap(chain_id: u64) -> Rs<()> {
    let db_url = shared::env::read(Env::DatabaseUrl)?;
    let db = database::establish_connection(&db_url).await?;

    let chain = SupportedChain::try_from(chain_id)?;
    let client = create_public_client(chain);

    let current_scanned_block = {
        let scanned_block =
            repositories::settings::get(&db, Setting::EvmScannedBlock(chain_id)).await?;

        if let Some(scanned_block) = scanned_block {
            scanned_block.parse()?
        } else {
            let latest_block = client.get_block_number().await?;
            repositories::settings::insert(
                &db,
                Setting::EvmScannedBlock(chain_id),
                latest_block.to_string(),
            )
            .await?;
            latest_block
        }
    };

    let mut filter = Filter::new()
        .address(
            [
                chain.usdt_weth_pool_v2_address(),
                chain.usdt_weth_pool_v3_address(),
            ]
            .to_vec(),
        )
        .events(
            [
                UniswapPoolV3Events::SIGNATURES,
                UniswapPoolV2Events::SIGNATURES,
            ]
            .concat(),
        )
        .from_block(BlockNumberOrTag::Number(current_scanned_block));

    tracing::info!("starting scanner from block {}", current_scanned_block);

    loop {
        match scan(&client, &db, chain, &mut filter).await {
            Ok(next) => {
                filter = filter.from_block(next);
            }
            Err(error) => {
                error.trace("Scan failed");
            }
        };

        sleep(SCAN_FREQUENCY).await;
    }
}

async fn scan(
    client: &PublicClient,
    db: &DatabaseConnection,
    chain: SupportedChain,
    filter: &mut Filter,
) -> Rs<u64> {
    let latest_block = client.get_block_number().await?;
    let from_block = filter.get_from_block().unwrap_or(latest_block);

    if from_block > latest_block {
        return Ok(from_block);
    }

    let to_block = (latest_block - 2).min(from_block + block_range_by_chain(chain));

    if to_block < from_block {
        return Ok(from_block);
    }

    filter.block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumberOrTag::Number(from_block)),
        to_block: Some(BlockNumberOrTag::Number(to_block)),
    };

    let logs = client.get_logs(filter).await?;

    let tasks = logs.iter().map(|log| evm_stream::handle_log(db, log));

    try_join_all(tasks).await?;

    tracing::trace!("scanned from {} to {} successfully", from_block, to_block,);

    let next_block = to_block + 1;

    repositories::settings::set(
        db,
        Setting::EvmScannedBlock(chain.to_chain_id()),
        next_block.to_string(),
    )
    .await?;

    Ok(next_block)
}

fn block_range_by_chain(chain: SupportedChain) -> u64 {
    match chain {
        SupportedChain::Bsc => 1998,
        SupportedChain::Mainet => 2000,
    }
}
