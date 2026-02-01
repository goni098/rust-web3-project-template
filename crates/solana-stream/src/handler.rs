use database::sea_orm::{DatabaseConnection, sea_query::prelude::Utc};
use shared::result::Rs;
use solana::bo::program::BoEvent;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::signature::Signature;
use tracing::{debug, info, instrument, warn};

/// Processes a Solana log response and extracts/handles events
#[instrument(skip_all)]
pub async fn handle_response_log(
    db: &DatabaseConnection,
    res: Response<RpcLogsResponse>,
) -> Rs<Option<String>> {
    let signature = res.value.signature;

    // Early return for default/invalid signatures
    if signature == Signature::default().to_string() {
        debug!("Skipping default signature");
        return Ok(None);
    }

    // Skip failed transactions
    if res.value.err.is_some() {
        warn!("Skipping failed transaction,signature {}", signature);
        return Ok(None);
    }

    // Parse events from logs
    let events = BoEvent::from_logs(&res.value.logs);

    if events.is_empty() {
        debug!("No events found in transaction, signature {}", signature);
        return Ok(None);
    }

    debug!("Found {} events: {:#?}", events.len(), events);

    // Process events
    handle_events(db, &signature, Utc::now().timestamp(), events).await?;

    Ok(Some(signature))
}

/// Processes individual blockchain events
#[instrument(skip_all)]
async fn handle_events(
    _db: &DatabaseConnection,
    _signature: &str,
    _timestamp: i64,
    events: Vec<BoEvent>,
) -> Rs<()> {
    info!("events: {}", events.len());

    Ok(())
}
