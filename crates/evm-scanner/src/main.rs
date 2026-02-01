use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    primitives::address,
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption},
};
use database::repositories::settings::Setting;
use database::{repositories, sea_orm::DatabaseConnection};
use evm::{
    client::{PublicClient, create_public_client},
    uniswap_v3::UniswapPoolV3::UniswapPoolV3Events,
};
use futures_util::future::try_join_all;
use shared::{env::Env, result::Rs};
use tokio::time::sleep;
use tracing::instrument;

#[tokio::main]
async fn main() {
    shared::env::load();
    shared::tracing::subscribe();
    bootstrap(shared::arg::parse_chain_arg()).await.unwrap();
}

async fn bootstrap(chain: u64) -> Rs<()> {
    let db_url = shared::env::read(Env::DatabaseUrl);
    let db = database::establish_connection(&db_url).await?;

    let client = create_public_client(chain);

    let current_scanned_block = {
        let scanned_block =
            repositories::settings::get(&db, Setting::EvmScannedBlock(chain)).await?;

        if let Some(scanned_block) = scanned_block {
            scanned_block.parse()?
        } else {
            let latest_block = client.get_block_number().await?;
            repositories::settings::insert(
                &db,
                Setting::EvmScannedBlock(chain),
                latest_block.to_string(),
            )
            .await?;
            latest_block
        }
    };

    let mut filter = Filter::new()
        .address(address!("0x4e68ccd3e89f51c3074ca5072bbac773960dfa36"))
        .events(UniswapPoolV3Events::SIGNATURES)
        .from_block(BlockNumberOrTag::Number(current_scanned_block));

    tracing::info!("starting scanner from block {}", current_scanned_block);

    loop {
        match scan(&client, &db, chain, &mut filter).await {
            Ok(next) => {
                filter = filter.from_block(next);
            }
            Err(error) => {
                tracing::error!(
                    "scan from {} to {} failed {:#?}",
                    filter.get_from_block().unwrap_or_default(),
                    filter.get_to_block().unwrap_or_default(),
                    error
                );
            }
        };

        sleep(Duration::from_secs(60)).await;
    }
}

#[instrument(skip_all)]
async fn scan(
    client: &PublicClient,
    db: &DatabaseConnection,
    chain: u64,
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

    let tasks = logs.into_iter().map(|log| evm_stream::handle_log(db, log));

    try_join_all(tasks).await?;

    tracing::info!("scanned from {} to {} successfully", from_block, to_block,);

    let next_block = to_block + 1;

    repositories::settings::set(db, Setting::EvmScannedBlock(chain), next_block.to_string())
        .await?;

    Ok(next_block)
}

fn block_range_by_chain(chain: u64) -> u64 {
    match chain {
        1 => 1998,
        _ => 2000,
    }
}
