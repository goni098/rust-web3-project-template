use fastwebsockets::{Frame, OpCode};

use crate::handshake::FrameCollector;

pub async fn handle_frame(frame: Frame<'_>, ws: &mut FrameCollector) {
    match frame.opcode {
        OpCode::Text | OpCode::Binary => {
            let payload = frame.payload.as_ref();
            println!("received: {}", String::from_utf8_lossy(payload));
        }
        OpCode::Close => {
            println!("server closed");
        }
        OpCode::Ping => {
            ws.write_frame(Frame::pong(frame.payload)).await.unwrap();
        }
        _ => {
            println!("opcode: {:?}", frame.opcode);
        }
    }
}
