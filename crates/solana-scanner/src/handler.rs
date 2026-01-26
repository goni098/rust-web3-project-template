use database::repositories;
use database::sea_orm::DatabaseConnection;
use futures_util::future::join_all;
use shared::result::Rs;
use solana::bo::program::BoEvent;
use solana_client::rpc_response::{OptionSerializer, RpcConfirmedTransactionStatusWithSignature};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use tracing::instrument;

use crate::{COMMITMENT, CONCURRENCY_SIGNATURE};

#[instrument(skip_all)]
pub async fn consume_txs(
    db: &DatabaseConnection,
    client: &RpcClient,
    mut txs: Vec<RpcConfirmedTransactionStatusWithSignature>,
) {
    while !txs.is_empty() {
        let batch: Vec<_> = txs.drain(..txs.len().min(CONCURRENCY_SIGNATURE)).collect();

        let results = join_all(batch.iter().map(|tx| handle_tx(client, db, tx))).await;

        for (tx, result) in batch.into_iter().zip(results) {
            if let Err(err) = result {
                tracing::warn!("retry {} error {}", tx.signature, err);
                txs.push(tx);
            }
        }
    }
}

#[instrument(skip_all)]
async fn handle_tx(
    client: &RpcClient,
    db: &DatabaseConnection,
    tx: &RpcConfirmedTransactionStatusWithSignature,
) -> Rs<()> {
    tracing::info!("processing signature {}", tx.signature);
    let signature = tx.signature.parse()?;

    let config = RpcTransactionConfig {
        max_supported_transaction_version: Some(0),
        commitment: Some(COMMITMENT),
        encoding: None,
    };

    let txn = client
        .get_transaction_with_config(&signature, config)
        .await?;

    if let Some(meta) = txn.transaction.meta
        && let OptionSerializer::Some(logs) = meta.log_messages
    {
        let timestamp = txn.block_time.unwrap_or_default();

        let events = BoEvent::from_logs(logs);

        tracing::debug!(
            signature = %tx.signature,
            event_count = events.len(),
            "Found events: {:#?}", events
        );

        repositories::signatures::upsert(db, signature.to_string(), timestamp).await?;
    }

    Ok(())
}
