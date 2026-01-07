use crate::errors::{WsError, WsResult};
use bytes::Bytes;
use std::sync::Arc;

/// Trade message handler trait
///
/// Implement this trait to handle binary trade messages in your application.
/// The handler receives raw bytes that contain SBE-encoded trade data.
pub trait TradeMessageHandler: Send + Sync {
    /// Handle a binary trade message (SBE-encoded)
    ///
    /// The bytes parameter contains a single SBE-encoded trade message.
    /// Decode it using your SBE codec.
    fn on_trade(&self, data: &[u8]) -> WsResult<()>;

    /// Handle a batch of trades decoded from a single WebSocket message
    ///
    /// Each message in the batch is msg_size bytes.
    fn on_trades(&self, data: &[u8], msg_size: usize) -> WsResult<()> {
        let msg_count = data.len() / msg_size;
        for i in 0..msg_count {
            let offset = i * msg_size;
            self.on_trade(&data[offset..offset + msg_size])?;
        }
        Ok(())
    }

    /// Called when connection is established
    fn on_connect(&self) -> WsResult<()> {
        Ok(())
    }

    /// Called when connection is closed
    fn on_disconnect(&self) -> WsResult<()> {
        Ok(())
    }

    /// Called on any error
    fn on_error(&self, error: &WsError) {
        tracing::error!("Trade handler error: {}", error);
    }
}

/// Default trade message handler - logs binary messages
pub struct LoggingTradeHandler;

impl TradeMessageHandler for LoggingTradeHandler {
    fn on_trade(&self, data: &[u8]) -> WsResult<()> {
        tracing::info!("Received trade message: {} bytes", data.len());
        Ok(())
    }

    fn on_connect(&self) -> WsResult<()> {
        tracing::info!("Trade handler connected");
        Ok(())
    }

    fn on_disconnect(&self) -> WsResult<()> {
        tracing::info!("Trade handler disconnected");
        Ok(())
    }
}

/// Message processor for handling WebSocket frames containing trade data (zero-copy)
///
/// This processor directly passes binary message data to the handler.
/// The handler is responsible for decoding SBE-encoded messages.
pub struct TradeMessageProcessor {
    handler: Arc<dyn TradeMessageHandler>,
}

impl TradeMessageProcessor {
    /// Create a new processor with a handler
    pub fn new(handler: Arc<dyn TradeMessageHandler>) -> Self {
        Self { handler }
    }

    /// Process a single binary WebSocket message (zero-copy)
    ///
    /// The message is passed directly to the handler as raw bytes.
    /// No copying or intermediate buffers are created.
    pub async fn process_message(&self, data: Bytes) -> WsResult<()> {
        // Zero-copy: Pass raw bytes directly to handler
        // Handler is responsible for SBE decoding
        self.handler.on_trade(&data)
    }

    /// Process text messages (not supported)
    pub async fn process_text(&self, text: String) -> WsResult<()> {
        Err(WsError::InvalidMessage(format!(
            "Text messages not supported, received: {}",
            text
        )))
    }

    /// Get reference to the handler
    pub fn handler(&self) -> &Arc<dyn TradeMessageHandler> {
        &self.handler
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        message_count: std::sync::atomic::AtomicUsize,
    }

    impl TradeMessageHandler for TestHandler {
        fn on_trade(&self, _data: &[u8]) -> WsResult<()> {
            self.message_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        }
    }

    #[test]
    fn test_handler_creation() {
        let handler = Arc::new(TestHandler {
            message_count: std::sync::atomic::AtomicUsize::new(0),
        });
        assert_eq!(handler.message_count.load(std::sync::atomic::Ordering::Relaxed), 0);
    }
}
