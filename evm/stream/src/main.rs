use std::{
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use alloy::rpc::types::Filter;
use database::sea_orm::DatabaseConnection;
use evm_lib::{
    SupportedChain, uniswap_v2::UniswapPoolV2::UniswapPoolV2Events,
    uniswap_v3::UniswapPoolV3::UniswapPoolV3Events,
};
use fastwebsockets::{Frame, OpCode, Payload, WebSocketError};
use hyper::Uri;
use shared::env::Env;
use tokio::time::sleep;

mod extractor;

const PING_INTERVAL_SECS: u64 = 30;

const DELAY_RECONNECT_SECS: u64 = 3;

static ID: AtomicU64 = AtomicU64::new(1);

#[tokio::main]
async fn main() {
    shared::env::load();
    shared::tracing::subscribe();

    let chain_id = shared::arg::parse_chain_id_arg();
    let chain = SupportedChain::try_from(chain_id).expect("invalid chain id");

    let db_url = shared::env::read(Env::DatabaseUrl);
    let ws_rpc = shared::env::read(Env::EvmWsRpc(chain_id));

    let uri =
        Uri::from_str(&ws_rpc).unwrap_or_else(|_| panic!("invalid ws rpc chain {}", chain_id));

    let db = database::establish_connection(&db_url)
        .await
        .unwrap_or_else(|error| panic!("Db error {}", error));

    loop {
        if let Err(err) = bootstrap(&db, &uri, chain).await {
            tracing::error!("WebSocketError >> {}", err);
        }

        sleep(Duration::from_secs(DELAY_RECONNECT_SECS)).await;
    }
}

async fn bootstrap(
    db: &DatabaseConnection,
    uri: &Uri,
    chain: SupportedChain,
) -> Result<(), WebSocketError> {
    let mut ping_clock = tokio::time::interval(Duration::from_secs(PING_INTERVAL_SECS));

    let mut ws = ws_client::connect(uri).await?;
    tracing::info!("WebSocket connected {}", uri);

    let filter = Filter::new()
        .address(vec![
            chain.usdt_weth_pool_v2_address(),
            chain.usdt_weth_pool_v3_address(),
        ])
        .events(
            [
                UniswapPoolV3Events::SIGNATURES,
                UniswapPoolV2Events::SIGNATURES,
            ]
            .concat(),
        );

    let msg_subscribe = serde_json::json!({
          "id": ID.fetch_add(1, Ordering::Relaxed),
          "jsonrpc": "2.0",
          "method": "eth_subscribe",
          "params": ["logs", filter]
    });

    let payload_subscribe = Payload::Owned(
        serde_json::to_vec(&msg_subscribe).unwrap_or_else(|_| panic!("invalid msg_subcribe")),
    );

    ws.write_frame(Frame::text(payload_subscribe)).await?;

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                if let Some(log) = extractor::extract_frame(frame?, &mut ws).await? {
                    evm_stream::handle_log(db, &log)
                        .await
                        .unwrap_or_else(|error| {
                            error.trace("handle log error");
                        });
                }
            }
            _ = ping_clock.tick() => {
                ws.write_frame(Frame::new(true, OpCode::Ping, None, Payload::Borrowed(b"ping"))).await?;
            }
        }
    }
}
