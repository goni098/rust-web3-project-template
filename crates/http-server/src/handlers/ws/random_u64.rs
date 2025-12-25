use std::time::Duration;

use axum::response::IntoResponse;
use fastwebsockets::Frame;
use fastwebsockets::OpCode;
use fastwebsockets::Payload;
use fastwebsockets::WebSocketError;
use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use crate::exception::HttpException;
use crate::exception::HttpResult;

pub async fn handler(req: IncomingUpgrade) -> HttpResult<impl IntoResponse> {
    let (response, fut) = req.upgrade().map_err(HttpException::internal)?;

    tokio::task::spawn(async move {
        if let Err(e) = handle_client(fut).await {
            tracing::error!("Error in websocket connection: {}", e);
        }
    });

    Ok(response)
}

async fn handle_client(fut: UpgradeFut) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Frame>();

    tokio::spawn(async move {
        loop {
            let rand = rand::random::<u64>().to_string();
            let payload = Payload::Owned(rand.as_bytes().to_vec());
            let _ = tx.send(Frame::binary(payload));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                let frame = match frame {
                    Ok(f) => f,
                    Err(WebSocketError::UnexpectedEOF) => {
                        tracing::info!("client disconnected");
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                };

                match frame.opcode {
                    OpCode::Close => {
                        ws.write_frame(Frame::close(1000, &frame.payload)).await?;
                    }
                    OpCode::Ping => ws.write_frame(Frame::pong(frame.payload)).await?,
                    OpCode::Binary | OpCode::Text => {
                        tracing::info!("received from client {:?}", frame.payload.as_ref());
                    }
                    _ => {}
                }
            },
            Some(frame) = rx.recv() => {
                ws.write_frame(frame).await?;
            }
        }
    }
}
