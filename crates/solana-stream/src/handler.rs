use database::sea_orm::{DatabaseConnection, sea_query::prelude::Utc};
use shared::result::Rs;
use solana::bo::program::BoEvent;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::signature::Signature;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn handle_response_log(
    db: &DatabaseConnection,
    res: Response<RpcLogsResponse>,
) -> Rs<Option<String>> {
    let signature = res.value.signature;

    if signature == Signature::default().to_string() {
        return Ok(None);
    }

    if res.value.err.is_some() {
        tracing::info!("skip err transaction {}", signature);
        return Ok(None);
    }

    let events = solana::bo::program::BoEvent::from_logs(&res.value.logs);

    dbg!(&events);

    handle_events(db, &signature, Utc::now().timestamp(), events).await?;

    Ok(Some(signature))
}

#[instrument(skip_all)]
async fn handle_events(
    _db: &DatabaseConnection,
    _signature: &str,
    _timestamp: i64,
    events: Vec<BoEvent>,
) -> Rs<()> {
    for log in events {
        match log {
            BoEvent::OpenPosition(event) => {
                tracing::info!("OpenPosition event: {:#?}", event);
            }
            BoEvent::SettlePosition(event) => {
                tracing::info!("SettlePosition event: {:#?}", event);
            }
        };
    }

    Ok(())
}
