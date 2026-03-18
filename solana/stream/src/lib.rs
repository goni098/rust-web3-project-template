use database::{repositories::log_memos, sea_orm::DatabaseConnection};
use futures_util::future::try_join_all;
use shared::result::Rs;
use sol_lib::pumpfun;
use solana_sdk::signature::Signature;

pub async fn handle_events(
    db: &DatabaseConnection,
    signature: Signature,
    timestamp: i64,
    events: Vec<pumpfun::utils::Event>,
) -> Rs<()> {
    let iter = events
        .into_iter()
        .enumerate()
        .map(|(log_ix, event)| handle_event(db, signature, log_ix as i32, timestamp, event));

    try_join_all(iter).await?;

    Ok(())
}

async fn handle_event(
    db: &DatabaseConnection,
    signature: Signature,
    log_ix: i32,
    timestamp: i64,
    event: pumpfun::utils::Event,
) -> Rs<()> {
    if log_memos::is_existed(db, signature, log_ix).await? {
        return Ok(());
    }

    tracing::info!("event: {:#?}", event);

    log_memos::save(db, signature, log_ix, timestamp).await?;

    Ok(())
}
