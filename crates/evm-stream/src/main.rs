use std::{str::FromStr, time::Duration};

use alloy::rpc::types::Filter;
use database::sea_orm::DatabaseConnection;
use fastwebsockets::{Frame, OpCode, Payload, WebSocketError};
use hyper::Uri;
use shared::env::Env;
use tokio::time::Interval;

mod extrator;

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

    let mut clock = tokio::time::interval(Duration::from_secs(12));

    loop {
        if let Err(err) = bootstrap(&db, &uri, &mut clock).await {
            tracing::error!("WebSocketError >> {}", err);
        }
    }
}

async fn bootstrap(
    db: &DatabaseConnection,
    uri: &Uri,
    ping_clock: &mut Interval,
) -> Result<(), WebSocketError> {
    let mut ws = ws_client::connect(uri).await?;

    let filter = Filter::new().address(vec![]);

    let msg_subcribe = serde_json::json!({
          "id": 0,
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
                    evm_stream::handle_log(db, log)
                        .await
                        .unwrap_or_else(|error| {
                            tracing::error!("handle log error {}", error);
                        });
                }
            },
            _ = ping_clock.tick() => {
                ws.write_frame(Frame::new(true, OpCode::Ping, None, Payload::Borrowed(&[]))).await?;
                tracing::info!("ping!");
            }
        }
    }
}
