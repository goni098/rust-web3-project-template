use fastwebsockets::{Frame, OpCode, Payload, WebSocketError};
use serde::Deserialize;
use serde_json::Value;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use tracing::instrument;
use ws_client::FrameCollector;

#[derive(Deserialize)]
struct InCommingLogResutMsg {
    params: LogResult,
}

#[derive(Deserialize)]
struct LogResult {
    result: Value,
}

#[instrument(skip_all)]
pub async fn extract_frame(
    ws: &mut FrameCollector,
    frame: Frame<'_>,
) -> Result<Option<Response<RpcLogsResponse>>, WebSocketError> {
    match frame.opcode {
        OpCode::Text => {
            let log = serde_json::from_slice::<InCommingLogResutMsg>(frame.payload.as_ref())
                .ok()
                .and_then(|incomming| serde_json::from_value(incomming.params.result).ok());

            Ok(log)
        }
        OpCode::Ping => {
            ws.write_frame(Frame::pong(Payload::Borrowed(&[]))).await?;
            tracing::info!("pong!");
            Ok(None)
        }
        OpCode::Pong => {
            tracing::info!("->pong");
            Ok(None)
        }
        _ => Ok(None),
    }
}
