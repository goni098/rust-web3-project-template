use std::time::Duration;

use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};
use sol_lib::pumpfun;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;

use crate::cursor::load_or_init_cursor;
use crate::handler::consume_txs;
use crate::signature::retrieve_txs;

const CONCURRENCY_SIGNATURE: usize = 30;
const COMMITMENT: CommitmentConfig = CommitmentConfig::finalized();
const SCAN_FREQUENCY: Duration = Duration::from_millis(6_000);

mod cursor;
mod handler;
mod signature;

#[tokio::main]
async fn main() {
    shared::env::load();
    shared::tracing::subscribe();

    let rpc_url = shared::env::read(Env::SolanaRpc);
    let db_url = shared::env::read(Env::DatabaseUrl);

    let client = RpcClient::new(rpc_url);

    let db = database::establish_connection(&db_url)
        .await
        .unwrap_or_else(|error| panic!("Db error {}", error));

    let mut cursor = load_or_init_cursor(&db, &client)
        .await
        .unwrap_or_else(|error| panic!("find cursor error {}", error));

    tracing::info!("Event scanner started on {}", pumpfun::ID);

    tracing::info!("Starting from signature {}", cursor);

    loop {
        if let Err(error) = scan(&db, &client, &mut cursor).await {
            error.trace("Scan failed");
        }

        tokio::time::sleep(SCAN_FREQUENCY).await;
    }
}

async fn scan(db: &DatabaseConnection, client: &RpcClient, cursor: &mut Signature) -> Rs<()> {
    let sigs = retrieve_txs(client, cursor).await?;
    let next_curor = sigs.first().and_then(|tx| tx.signature.parse().ok());

    consume_txs(db, client, sigs).await;

    if let Some(next_curor) = next_curor {
        *cursor = next_curor;
        repositories::settings::set(db, Setting::SolCurrentScannedSignature, cursor.to_string())
            .await?;
    }

    Ok(())
}
