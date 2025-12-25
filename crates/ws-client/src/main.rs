use std::time::Duration;

use fastwebsockets::Frame;
use fastwebsockets::OpCode;
use fastwebsockets::Payload;
use fastwebsockets::WebSocketError;
use hyper::Uri;
use tokio::time::Interval;

use crate::frame::handle_frame;

mod frame;
mod handshake;
mod tls;

const WS_ENDPOINT: &str = "ws://localhost:8080/random-u64";

#[tokio::main]
async fn main() {
    let uri = Uri::from_static(WS_ENDPOINT);

    let mut clock = tokio::time::interval(Duration::from_secs(10));

    loop {
        if let Err(err) = bootstrap(&uri, &mut clock).await {
            println!("error {}", err);
        }
    }
}

async fn bootstrap(uri: &Uri, ping_clock: &mut Interval) -> Result<(), WebSocketError> {
    let mut ws = handshake::connect(uri).await?;

    loop {
        tokio::select! {
            frame = ws.read_frame() => {
                handle_frame(frame?, &mut ws).await;
            },
            _ = ping_clock.tick() => {
                    println!("ping");
                    ws.write_frame(Frame::new(true, OpCode::Ping, None, Payload::Owned(vec![])))
                        .await?;
            }
        }
    }
}
