use std::fmt;

#[derive(Debug)]
pub enum WsError {
    /// Network I/O error
    IoError(String),
    /// WebSocket protocol error
    ProtocolError(String),
    /// SBE codec encoding error
    EncodeError(String),
    /// SBE codec decoding error
    DecodeError(String),
    /// Invalid message format
    InvalidMessage(String),
    /// Buffer overflow
    BufferOverflow,
    /// Connection closed
    ConnectionClosed,
}

impl fmt::Display for WsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WsError::IoError(msg) => write!(f, "IO error: {}", msg),
            WsError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            WsError::EncodeError(msg) => write!(f, "Encode error: {}", msg),
            WsError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            WsError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            WsError::BufferOverflow => write!(f, "Buffer overflow"),
            WsError::ConnectionClosed => write!(f, "Connection closed"),
        }
    }
}

impl std::error::Error for WsError {}

impl From<std::io::Error> for WsError {
    fn from(err: std::io::Error) -> Self {
        WsError::IoError(err.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for WsError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        WsError::ProtocolError(err.to_string())
    }
}

pub type WsResult<T> = Result<T, WsError>;
