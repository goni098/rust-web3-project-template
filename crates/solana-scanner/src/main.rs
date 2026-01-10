use std::time::Duration;

use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};
use solana::PROGRAM_ID;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use tracing::instrument;

use crate::cursor::load_or_init_cursor;
use crate::handler::consume_txs;
use crate::signature::retrieve_txs;

const CONCURRENCY_SIGNATURE: usize = 30;
const COMMITMENT: CommitmentConfig = CommitmentConfig::finalized();

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

    tracing::info!("Events scanner started, program_id: {}", PROGRAM_ID);
    tracing::info!("Starting from signature: {}", cursor);

    loop {
        match scan(&db, &client, &mut cursor).await {
            Ok(_) => {}
            Err(error) => {
                tracing::error!("scan error: {}", error);
            }
        }

        tokio::time::sleep(Duration::from_secs(6)).await;
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
