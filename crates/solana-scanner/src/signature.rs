use shared::result::Rs;
use solana::PROGRAM_ID;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_sdk::signature::Signature;
use tracing::instrument;

use crate::COMMITMENT;

#[instrument(skip_all)]
pub async fn retrieve_txs(
    client: &RpcClient,
    cursor: &Signature,
) -> Rs<Vec<RpcConfirmedTransactionStatusWithSignature>> {
    let mut page = client
        .get_signatures_for_address_with_config(
            &PROGRAM_ID,
            GetConfirmedSignaturesForAddress2Config {
                commitment: Some(COMMITMENT),
                until: Some(*cursor),
                before: None,
                limit: None,
            },
        )
        .await?;

    if page.is_empty() {
        tracing::info!("No new tx found");
        return Ok(page);
    }

    let mut before = page.last().and_then(|tx| tx.signature.parse().ok());

    loop {
        let order_page = client
            .get_signatures_for_address_with_config(
                &PROGRAM_ID,
                GetConfirmedSignaturesForAddress2Config {
                    commitment: Some(COMMITMENT),
                    until: Some(*cursor),
                    before,
                    limit: None,
                },
            )
            .await?;

        if order_page.is_empty() {
            return Ok(page);
        }

        before = order_page.last().and_then(|tx| tx.signature.parse().ok());
        page.extend(order_page);
    }
}

#[instrument(skip_all)]
pub async fn get_the_first_signature(client: &RpcClient) -> Rs<Option<Signature>> {
    let mut before: Option<Signature> = None;
    let mut first_signature: Option<Signature> = None;

    loop {
        let page = client
            .get_signatures_for_address_with_config(
                &PROGRAM_ID,
                GetConfirmedSignaturesForAddress2Config {
                    commitment: Some(COMMITMENT),
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
