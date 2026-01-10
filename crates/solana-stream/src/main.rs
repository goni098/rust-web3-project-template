use std::{str::FromStr, time::Duration};

use database::sea_orm::DatabaseConnection;
use fastwebsockets::{Frame, OpCode, Payload, WebSocketError};
use hyper::Uri;
use shared::env::Env;
use solana_client::rpc_config::{
    CommitmentConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
};
use solana_stream::{extrator, handler::handle_response_log};
use tokio::time::Interval;

const COMMITMENT: CommitmentConfig = CommitmentConfig::confirmed();

#[tokio::main]
async fn main() {
    shared::env::load();
    shared::tracing::subscribe();

    let db_url = shared::env::read(Env::DatabaseUrl);
    let ws_rpc = shared::env::read(Env::SolanaWsRpc);

    let uri = Uri::from_str(&ws_rpc).unwrap_or_else(|_| panic!("invalid ws rpc {}", ws_rpc));

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
    tracing::info!("connected to {}", uri);

    let filter =
        RpcTransactionLogsFilter::Mentions(vec![solana::bo::program::PROGRAM_ID.to_string()]);

    let config = RpcTransactionLogsConfig {
        commitment: Some(COMMITMENT),
    };

    let msg_subscribe = serde_json::json!({
          "id": 1,
          "jsonrpc": "2.0",
          "method": "logsSubscribe",
          "params": [filter, config]
    });

    let payload_subscribe = Payload::Owned(
        serde_json::to_vec(&msg_subscribe).unwrap_or_else(|_| panic!("invalid msg_subcribe")),
    );

    ws.write_frame(Frame::text(payload_subscribe)).await?;

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                if let Some(res) = extrator::extract_frame(&mut ws, frame?).await? {
                    handle_response_log(db, res)
                        .await
                        .inspect(|signature| {
                            if let Some(signature) = signature {
                                tracing::info!("catched {}", signature);
                            }
                        })
                        .unwrap_or_else(|error| {
                            tracing::error!("handle incoming error {}", error);
                            None
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
