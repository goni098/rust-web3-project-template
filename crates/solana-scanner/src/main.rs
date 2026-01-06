use std::time::Duration;

use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};
use solana::PROGRAM_ID;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_config::RpcTransactionConfig, rpc_response::OptionSerializer,
};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use tracing::instrument;

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

    let mut cursor = repositories::settings::get(&db, Setting::SolCurrentScannedSignature)
        .await
        .expect("fail to get solana_current_scanned_signature setting")
        .unwrap_or_else(|| Signature::default().to_string());

    tracing::info!("🦀 Events scanner started, program_id: {}", PROGRAM_ID);
    tracing::info!("Starting from signature: {:?}", cursor);

    loop {
        match scan(&db, &client, &mut cursor).await {
            Ok(_) => {
                tracing::info!("Scan cycle completed successfully");
            }
            Err(error) => {
                tracing::error!("scan error: {:#?}", error);
            }
        }

        tokio::time::sleep(Duration::from_secs(6)).await;
    }
}

#[instrument(skip(db, client))]
async fn scan(db: &DatabaseConnection, client: &RpcClient, cursor: &mut String) -> Rs<()> {
    let mut before: Option<Signature> = None;
    let mut is_reached_cursor = false;
    let mut newest_processed: Option<String> = None;

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            commitment: Some(CommitmentConfig::finalized()),
            before,
            until: None,
            limit: None,
        };

        let page = client
            .get_signatures_for_address_with_config(&PROGRAM_ID, config)
            .await?;

        if page.is_empty() {
            break;
        }

        if newest_processed.is_none() {
            newest_processed = Some(page.first().unwrap().signature.clone());
        }

        for tx in &page {
            if tx.signature == *cursor {
                is_reached_cursor = true;
                break;
            }

            proceed_signature(client, &tx.signature).await?;
        }

        if is_reached_cursor {
            break;
        }

        before = Some(page.last().unwrap().signature.parse()?);
    }

    if let Some(new_cursor) = newest_processed {
        *cursor = new_cursor;

        repositories::settings::set(db, Setting::SolCurrentScannedSignature, cursor.clone())
            .await?;

        tracing::info!("cursor updated to {}", cursor);
    } else {
        tracing::info!("no new signatures found");
    }

    Ok(())
}

async fn proceed_signature(client: &RpcClient, signature: &str) -> Rs<()> {
    let sig = signature.parse()?;

    let config = RpcTransactionConfig {
        max_supported_transaction_version: Some(0),
        commitment: Some(CommitmentConfig::finalized()),
        encoding: None,
    };

    let tx = client.get_transaction_with_config(&sig, config).await?;

    if let Some(meta) = tx.transaction.meta
        && let OptionSerializer::Some(_logs) = meta.log_messages
    {
        let timestamp = tx.block_time.unwrap_or_default();
        tracing::info!("timestamp {}", timestamp);
    }

    Ok(())
}
