use std::time::Duration;

use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};
use solana::bo::program::PROGRAM_ID;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use tracing::{instrument, info, error};

use crate::cursor::load_or_init_cursor;
use crate::handler::consume_txs;
use crate::signature::retrieve_txs;

/// Maximum concurrent signature processing
const CONCURRENCY_SIGNATURE: usize = 30;
/// Commitment level for transaction finality
const COMMITMENT: CommitmentConfig = CommitmentConfig::finalized();
/// Interval between scan cycles
const SCAN_INTERVAL_SECS: u64 = 6;

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

    info!(program_id = %PROGRAM_ID, "🚀 Events scanner started");
    info!(signature = %cursor, "📍 Starting from signature");

    loop {
        if let Err(error) = scan(&db, &client, &mut cursor).await {
            error!(error = %error, "❌ Scan error occurred");
        }

        tokio::time::sleep(Duration::from_secs(SCAN_INTERVAL_SECS)).await;
    }
}

#[instrument(skip_all)]
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
