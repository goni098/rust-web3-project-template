use std::time::Duration;

use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};
use solana::PROGRAM_ID;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
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
async fn scan(_db: &DatabaseConnection, client: &RpcClient, cursor: &mut Signature) -> Rs<()> {
    let txns = retrieve_signatures(client, cursor).await?;

    dbg!(txns.len());

    if let Some(txn) = txns.first() {
        *cursor = txn.signature.parse()?;
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

    let mut last = first_page.last().unwrap().signature.parse::<Signature>()?;

    loop {
        let next_page = client
            .get_signatures_for_address_with_config(
                &PROGRAM_ID,
                GetConfirmedSignaturesForAddress2Config {
                    commitment: Some(CommitmentConfig::finalized()),
                    until: Some(*cursor),
                    before: Some(last),
                    limit: None,
                },
            )
            .await?;

        if next_page.is_empty() {
            return Ok(first_page);
        }

        last = next_page.last().unwrap().signature.parse()?;
        first_page.extend(next_page);
    }
}

#[instrument(skip_all)]
async fn _proceed_signature(
    client: &RpcClient,
    db: &DatabaseConnection,
    signature: &str,
) -> Rs<()> {
    tracing::info!("processing signature {}", signature);

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
        repositories::signatures::upsert(db, signature.to_string(), timestamp).await?;
    }

    Ok(())
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

        let last = page.last().unwrap().signature.parse()?;
        before = Some(last);
        first_signature = Some(last);
    }
}
