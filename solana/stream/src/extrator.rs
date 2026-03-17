use fastwebsockets::{Frame, OpCode, WebSocketError};
use serde::Deserialize;
use serde_json::Value;
use solana_client::rpc_response::{Response, RpcLogsResponse};
use tracing::instrument;
use ws_client::FrameCollector;

#[derive(Deserialize)]
struct IncomingLogResultMsg {
    params: LogResult,
}

#[derive(Deserialize)]
struct LogResult {
    result: Value,
}

/// Extracts and deserializes log responses from WebSocket frames
#[instrument(skip_all)]
pub async fn extract_frame(
    ws: &mut FrameCollector,
    frame: Frame<'_>,
) -> Result<Option<Response<RpcLogsResponse>>, WebSocketError> {
    match frame.opcode {
        OpCode::Text => {
            let log = serde_json::from_slice::<IncomingLogResultMsg>(frame.payload.as_ref())
                .ok()
                .and_then(|incoming| serde_json::from_value(incoming.params.result).ok());

            Ok(log)
        }
        OpCode::Ping => {
            ws.write_frame(Frame::pong(frame.payload)).await?;
            Ok(None)
        }
        OpCode::Close => Err(WebSocketError::ConnectionClosed),
        _ => Ok(None),
    }
}
