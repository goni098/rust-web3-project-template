use database::sea_orm::{DatabaseConnection, sea_query::prelude::Utc};
use shared::result::Rs;
use solana::bo::program::BoEvent;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::signature::Signature;
use tracing::{debug, info, instrument, warn};

/// Processes a Solana log response and extracts/handles events
#[instrument(skip(db), fields(signature = %res.value.signature))]
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
    if let Some(ref err) = res.value.err {
        warn!(
            signature = %signature,
            error = ?err,
            "Skipping failed transaction"
        );
        return Ok(None);
    }

    // Parse events from logs
    let events = BoEvent::from_logs(&res.value.logs);

    if events.is_empty() {
        debug!(signature = %signature, "No events found in transaction");
        return Ok(None);
    }

    debug!(
        signature = %signature,
        event_count = events.len(),
        "Found events: {:#?}", events
    );

    // Process events
    handle_events(db, &signature, Utc::now().timestamp(), events).await?;

    Ok(Some(signature))
}

/// Processes individual blockchain events
#[instrument(skip(_db), fields(signature = %signature, event_count = events.len()))]
async fn handle_events(
    _db: &DatabaseConnection,
    signature: &str,
    timestamp: i64,
    events: Vec<BoEvent>,
) -> Rs<()> {
    for event in events {
        match event {
            BoEvent::OpenPosition(ref data) => {
                info!(
                    signature = %signature,
                    timestamp = timestamp,
                    "Processing OpenPosition: {:#?}",
                    data
                );
                // TODO: Persist to database
                // save_open_position(db, signature, timestamp, data).await?;
            }
            BoEvent::SettlePosition(ref data) => {
                info!(
                    signature = %signature,
                    timestamp = timestamp,
                    "Processing SettlePosition: {:#?}",
                    data
                );
                // TODO: Persist to database
                // save_settle_position(db, signature, timestamp, data).await?;
            }
        }
    }

    Ok(())
}

// Future database persistence functions
// #[instrument(skip(db))]
// async fn save_open_position(
//     db: &DatabaseConnection,
//     signature: &str,
//     timestamp: i64,
//     event: &OpenPositionEvent,
// ) -> Rs<()> {
//     // Implementation here
//     Ok(())
// }

// #[instrument(skip(db))]
// async fn save_settle_position(
//     db: &DatabaseConnection,
//     signature: &str,
//     timestamp: i64,
//     event: &SettlePositionEvent,
// ) -> Rs<()> {
//     // Implementation here
//     Ok(())
// }
