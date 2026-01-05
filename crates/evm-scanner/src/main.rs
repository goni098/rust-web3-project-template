use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    providers::Provider,
    rpc::types::{Filter, FilterBlockOption},
};
use database::repositories::settings::Setting;
use database::{repositories, sea_orm::DatabaseConnection};
use evm::client::{PublicClient, create_public_client};
use futures_util::future::try_join_all;
use shared::{env::Env, result::Rs};
use tokio::time::sleep;

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
            client.get_block_number().await?
        }
    };

    let adddresses = vec![];

    let mut filter = Filter::new()
        .address(adddresses)
        .from_block(BlockNumberOrTag::Number(current_scanned_block));

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

async fn scan(
    client: &PublicClient,
    db: &DatabaseConnection,
    chain: u64,
    filter: &mut Filter,
) -> Rs<BlockNumberOrTag> {
    let latest_block = client.get_block_number().await?;
    let from_block = filter.get_from_block().unwrap_or(latest_block);
    let to_block = (latest_block - 2).min(from_block + block_range_by_chain(chain));

    filter.block_option = FilterBlockOption::Range {
        from_block: Some(BlockNumberOrTag::Number(from_block)),
        to_block: Some(BlockNumberOrTag::Number(to_block)),
    };

    let logs = client.get_logs(filter).await?;
    let catched_len = logs.len();

    let tasks = logs.into_iter().map(|log| evm_stream::proceed_log(db, log));

    try_join_all(tasks).await?;

    repositories::settings::set(db, Setting::EvmScannedBlock(chain), to_block.to_string()).await?;

    tracing::info!(
        "scanned from {} to {} successfully with {} logs",
        from_block,
        to_block,
        catched_len
    );

    Ok(BlockNumberOrTag::Number(to_block + 1))
}

fn block_range_by_chain(_chain: u64) -> u64 {
    4999
}
