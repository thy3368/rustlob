#![forbid(unsafe_code)]

pub mod handler;
pub mod server;
pub mod errors;
pub mod buffer;

pub use handler::{TradeMessageHandler, TradeMessageProcessor, LoggingTradeHandler};
pub use server::WebSocketServer;
pub use errors::{WsError, WsResult};
pub use buffer::ZeroCopyBuffer;

pub const DEFAULT_WS_ADDR: &str = "127.0.0.1:9001";
