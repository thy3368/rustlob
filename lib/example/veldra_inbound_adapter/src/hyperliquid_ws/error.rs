use std::string::FromUtf8Error;

use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum HyperliquidWsError {
    #[error("WebSocket connect failed: {0}")]
    Connect(#[source] tungstenite::Error),
    #[error("WebSocket frame send/receive failed: {0}")]
    Frame(#[source] tungstenite::Error),
    #[error("WebSocket connection closed.")]
    ConnectionClosed,
    #[error("WebSocket binary frame was not valid UTF-8: {0}")]
    BinaryFrameNotUtf8(#[source] FromUtf8Error),
    #[error("WebSocket JSON decode failed: {0}")]
    Json(#[source] serde_json::Error),
    #[error("WebSocket protocol mismatch: {0}")]
    Protocol(String),
}
