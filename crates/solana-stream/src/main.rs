use std::{
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use database::sea_orm::DatabaseConnection;
use fastwebsockets::{Frame, OpCode, Payload, WebSocketError};
use hyper::Uri;
use shared::env::Env;
use solana_client::rpc_config::{
    CommitmentConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
};
use solana_stream::{extrator, handler::handle_response_log};
use tokio::time::sleep;
use tracing::{error, info};

/// Commitment level for transaction logs
const COMMITMENT: CommitmentConfig = CommitmentConfig::confirmed();
/// Ping interval to keep WebSocket connection alive
const PING_INTERVAL_SECS: u64 = 30;
/// Polling interval to track zombie connection
const ZOMBIE_INTERVAL_POLLING: u64 = 300;

const DELAY_RECONNECT: u64 = 3;

static ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() {
    shared::env::load();
    shared::tracing::subscribe();

    let db_url = shared::env::read(Env::DatabaseUrl);
    let ws_rpc = shared::env::read(Env::SolanaWsRpc);

    let uri = Uri::from_str(&ws_rpc).unwrap_or_else(|_| panic!("invalid ws rpc {}", ws_rpc));

    let db = database::establish_connection(&db_url)
        .await
        .unwrap_or_else(|error| panic!("❌ Database connection error: {}", error));

    loop {
        if let Err(err) = bootstrap(&db, &uri).await {
            error!(error = %err, "❌ WebSocket connection error, reconnecting...");
        }
        sleep(Duration::from_secs(DELAY_RECONNECT)).await;
    }
}

async fn bootstrap(db: &DatabaseConnection, uri: &Uri) -> Result<(), WebSocketError> {
    let mut ping_clock = tokio::time::interval(Duration::from_secs(PING_INTERVAL_SECS));
    let mut zombie_conn_detector =
        tokio::time::interval(Duration::from_secs(ZOMBIE_INTERVAL_POLLING));
    zombie_conn_detector.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    ping_clock.tick().await;
    zombie_conn_detector.tick().await;

    let mut ws = ws_client::connect(uri).await?;
    info!("✅ WebSocket connected {}", uri);

    let filter =
        RpcTransactionLogsFilter::Mentions(vec![solana::bo::program::PROGRAM_ID.to_string()]);

    let config = RpcTransactionLogsConfig {
        commitment: Some(COMMITMENT),
    };

    let msg_subscribe = serde_json::json!({
        "id": ID.fetch_add(1, Ordering::Relaxed),
        "jsonrpc": "2.0",
        "method": "logsSubscribe",
        "params": [filter, config]
    });

    let payload_subscribe = Payload::Owned(
        serde_json::to_vec(&msg_subscribe).expect("Failed to serialize subscription message"),
    );

    ws.write_frame(Frame::text(payload_subscribe)).await?;

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                if let Some(res) = extrator::extract_frame(&mut ws, frame?).await? {
                    zombie_conn_detector.reset();
                    match handle_response_log(db, res).await {
                        Ok(Some(signature)) => info!("✅ Processed transaction {}", signature),
                        Ok(None) => {},
                        Err(error) => error!(error = %error, "❌ Failed to handle log"),
                    }
                }
            }
            _ = ping_clock.tick() => {
                ws.write_frame(Frame::new(true, OpCode::Ping, None, Payload::Borrowed(b"ping"))).await?;
            },
            _ = zombie_conn_detector.tick() => {
                tracing::info!("no message received for {} seconds, reconnecting...", ZOMBIE_INTERVAL_POLLING);
                return Ok(());
            }
        }
    }
}
