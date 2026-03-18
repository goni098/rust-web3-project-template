use database::sea_orm::{DatabaseConnection, sea_query::prelude::Utc};
use shared::result::Rs;
use sol_lib::pumpfun;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::signature::Signature;

/// Processes a Solana log response and extracts/handles events
pub async fn handle_log_from_ws(
    db: &DatabaseConnection,
    res: Response<RpcLogsResponse>,
) -> Rs<Option<Signature>> {
    let signature = res.value.signature.parse()?;

    if signature == Signature::default() {
        return Ok(None);
    }

    if res.value.err.is_some() {
        tracing::trace!("Skipping failed transaction, signature {}", signature);
        return Ok(None);
    }

    let events = pumpfun::utils::Event::from_logs(&res.value.logs);

    solana_stream::handle_events(db, signature, Utc::now().timestamp(), events).await?;

    Ok(Some(signature))
}
