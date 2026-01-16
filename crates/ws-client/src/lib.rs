//! WebSocket client utilities for secure connections
//! 
//! Provides TLS-enabled WebSocket connections with fragment collection

mod handshake;
mod tls;

pub use handshake::FrameCollector;
pub use handshake::connect;
