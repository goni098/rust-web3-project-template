use database::sea_orm::DatabaseConnection;
use futures_util::future::join_all;
use shared::result::Rs;
use sol_lib::pumpfun;
use solana_client::rpc_response::{OptionSerializer, RpcConfirmedTransactionStatusWithSignature};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};

use crate::{COMMITMENT, CONCURRENCY_SIGNATURE};

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

async fn handle_tx(
    client: &RpcClient,
    _db: &DatabaseConnection,
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
        let _timestamp = txn.block_time.unwrap_or_default();

        let _events = pumpfun::utils::Event::from_logs(logs);
    }

    Ok(())
}
