use std::time::Duration;

use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use futures_util::TryFutureExt;
use futures_util::future::join_all;
use shared::{env::Env, result::Rs};
use solana::PROGRAM_ID;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_config::RpcTransactionConfig,
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

    let mut cursor = find_or_init_cursor(&db, &client)
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
    let signatures = retrieve_signatures(client, cursor).await?;
    let next_curor = signatures.first().and_then(|tx| tx.signature.parse().ok());

    proceed_signatures(db, client, signatures).await;

    if let Some(next_curor) = next_curor {
        *cursor = next_curor;
        repositories::settings::set(db, Setting::SolCurrentScannedSignature, cursor.to_string())
            .await?;
    }

    Ok(())
}

#[instrument(skip_all)]
async fn retrieve_signatures(
    client: &RpcClient,
    cursor: &Signature,
) -> Rs<Vec<RpcConfirmedTransactionStatusWithSignature>> {
    let mut first_page = client
        .get_signatures_for_address_with_config(
            &PROGRAM_ID,
            GetConfirmedSignaturesForAddress2Config {
                commitment: Some(CommitmentConfig::finalized()),
                until: Some(*cursor),
                before: None,
                limit: None,
            },
        )
        .await?;

    if first_page.is_empty() {
        tracing::info!("No new tx found");
        return Ok(first_page);
    }

    let mut before = first_page.last().and_then(|tx| tx.signature.parse().ok());

    loop {
        let next_page = client
            .get_signatures_for_address_with_config(
                &PROGRAM_ID,
                GetConfirmedSignaturesForAddress2Config {
                    commitment: Some(CommitmentConfig::finalized()),
                    until: Some(*cursor),
                    before,
                    limit: None,
                },
            )
            .await?;

        if next_page.is_empty() {
            return Ok(first_page);
        }

        before = next_page.last().and_then(|tx| tx.signature.parse().ok());
        first_page.extend(next_page);
    }
}

#[instrument(skip_all)]
async fn find_or_init_cursor(db: &DatabaseConnection, client: &RpcClient) -> Rs<Signature> {
    if let Some(sig) = repositories::settings::get(db, Setting::SolCurrentScannedSignature).await? {
        Ok(sig.parse()?)
    } else {
        tracing::info!("Finding the first signature of program...");

        let sig = get_the_first_signature(client)
            .await?
            .expect("not found the first signature");

        repositories::settings::insert(db, Setting::SolCurrentScannedSignature, sig.to_string())
            .await?;

        Ok(sig)
    }
}

#[instrument(skip_all)]
async fn get_the_first_signature(client: &RpcClient) -> Rs<Option<Signature>> {
    let mut before: Option<Signature> = None;
    let mut first_signature: Option<Signature> = None;

    loop {
        let page = client
            .get_signatures_for_address_with_config(
                &PROGRAM_ID,
                GetConfirmedSignaturesForAddress2Config {
                    commitment: Some(CommitmentConfig::finalized()),
                    before,
                    until: None,
                    limit: None,
                },
            )
            .await?;

        if page.is_empty() {
            return Ok(first_signature);
        }

        let last = page.last().and_then(|tx| tx.signature.parse().ok());
        before = last;
        first_signature = last;
    }
}

#[instrument(skip_all)]
async fn proceed_signatures(
    db: &DatabaseConnection,
    client: &RpcClient,
    mut signatures: Vec<RpcConfirmedTransactionStatusWithSignature>,
) {
    while !signatures.is_empty() {
        let tasks = signatures.iter().take(10).map(|tx| {
            proceed_signle_signature(client, db, &tx.signature)
                .map_err(move |error| (&tx.signature, error))
        });

        let mut succeeded = Vec::with_capacity(10);

        join_all(tasks)
            .await
            .into_iter()
            .for_each(|result| match result {
                Err((signature, err)) => {
                    tracing::warn!("proceed {} error {}", signature, err);
                }
                Ok(signature) => {
                    succeeded.push(signature);
                }
            });

        signatures.retain(|tx| !succeeded.iter().any(|sig| sig.to_string() == tx.signature));
    }
}

#[instrument(skip_all)]
async fn proceed_signle_signature(
    client: &RpcClient,
    db: &DatabaseConnection,
    signature: &str,
) -> Rs<Signature> {
    tracing::info!("processing signature {}", signature);
    let signature = signature.parse()?;

    let config = RpcTransactionConfig {
        max_supported_transaction_version: Some(0),
        commitment: Some(CommitmentConfig::finalized()),
        encoding: None,
    };

    let tx = client
        .get_transaction_with_config(&signature, config)
        .await?;

    let timestamp = tx.block_time.unwrap_or_default();
    repositories::signatures::upsert(db, signature.to_string(), timestamp).await?;

    // if let Some(meta) = tx.transaction.meta
    //     && let OptionSerializer::Some(_logs) = meta.log_messages
    // {
    //     let timestamp = tx.block_time.unwrap_or_default();
    //     repositories::signatures::upsert(db, sig.to_string(), timestamp).await?;
    // }

    Ok(signature)
}
