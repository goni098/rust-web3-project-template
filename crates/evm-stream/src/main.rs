use std::{
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use alloy::{primitives::address, rpc::types::Filter};
use database::sea_orm::DatabaseConnection;
use evm::uniswap_v3::UniswapPoolV3::UniswapPoolV3Events;
use fastwebsockets::{Frame, OpCode, Payload, WebSocketError};
use hyper::Uri;
use shared::env::Env;
use tokio::time::sleep;

mod extrator;

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

    let chain = shared::arg::parse_chain_arg();
    let db_url = shared::env::read(Env::DatabaseUrl);
    let ws_rpc = shared::env::read(Env::EvmWsRpc(chain));

    let uri = Uri::from_str(&ws_rpc).unwrap_or_else(|_| panic!("invalid ws rpc chain {}", chain));

    let db = database::establish_connection(&db_url)
        .await
        .unwrap_or_else(|error| panic!("Db error {}", error));

    loop {
        if let Err(err) = bootstrap(&db, &uri).await {
            tracing::error!("WebSocketError >> {}", err);
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
    tracing::info!("✅ WebSocket connected {}", uri);

    let filter = Filter::new()
        .address(address!("0x4e68ccd3e89f51c3074ca5072bbac773960dfa36"))
        .events(UniswapPoolV3Events::SIGNATURES);

    let msg_subcribe = serde_json::json!({
          "id": ID.fetch_add(1, Ordering::Relaxed),
          "jsonrpc": "2.0",
          "method": "eth_subscribe",
          "params": ["logs", filter]
    });

    let payload_subscribe = Payload::Owned(
        serde_json::to_vec(&msg_subcribe).unwrap_or_else(|_| panic!("invalid msg_subcribe")),
    );

    ws.write_frame(Frame::text(payload_subscribe)).await?;

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                if let Some(log) = extrator::extract_frame(frame?, &mut ws).await? {
                    zombie_conn_detector.reset();

                    evm_stream::handle_log(db, log)
                        .await
                        .unwrap_or_else(|error| {
                            tracing::error!("handle log error {}", error);
                        });
                }
            }
            _ = ping_clock.tick() => {
                ws.write_frame(Frame::new(true, OpCode::Ping, None, Payload::Borrowed(b"ping"))).await?;
            }
            _ = zombie_conn_detector.tick() => {
                tracing::info!("no message received for {} seconds, reconnecting...", ZOMBIE_INTERVAL_POLLING);
                return Ok(());
            }
        }
    }
}
