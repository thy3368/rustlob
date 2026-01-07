use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tracing::{info, error, warn};

use crate::handler::{TradeMessageHandler, TradeMessageProcessor};
use crate::errors::WsResult;

/// WebSocket server for real-time trade data with zero-copy processing
///
/// This server uses tungstenite for WebSocket handling and SBE codec for
/// efficient binary message encoding/decoding. All data is processed with
/// zero-copy semantics using Bytes and reference counting.
pub struct WebSocketServer {
    addr: String,
    handler: Arc<dyn TradeMessageHandler>,
}

impl WebSocketServer {
    /// Create a new WebSocket server
    pub fn new(addr: impl Into<String>, handler: Arc<dyn TradeMessageHandler>) -> Self {
        Self {
            addr: addr.into(),
            handler,
        }
    }

    /// Start the WebSocket server and listen for connections
    ///
    /// This blocks and runs the server indefinitely. Use spawn_server() for non-blocking.
    pub async fn run(&self) -> WsResult<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        info!("WebSocket server listening on {}", self.addr);

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            let handler = self.handler.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, handler).await {
                    error!("Error handling connection from {}: {}", peer_addr, e);
                }
            });
        }
    }

    /// Spawn the server in a background task
    pub fn spawn(self: Arc<Self>) -> tokio::task::JoinHandle<WsResult<()>> {
        let self_clone = Arc::clone(&self);
        tokio::spawn(async move { self_clone.run().await })
    }

    /// Handle a single WebSocket connection
    async fn handle_connection(
        stream: TcpStream,
        handler: Arc<dyn TradeMessageHandler>,
    ) -> WsResult<()> {
        let peer_addr = stream.peer_addr()?;
        info!("New WebSocket connection from {}", peer_addr);

        // Accept WebSocket upgrade
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .map_err(|e| crate::errors::WsError::ProtocolError(e.to_string()))?;

        info!("WebSocket handshake successful with {}", peer_addr);

        // Notify handler of connection
        if let Err(e) = handler.on_connect() {
            handler.on_error(&e);
            return Err(e);
        }

        // Create message processor
        let processor = TradeMessageProcessor::new(handler.clone());

        // Split the WebSocket stream
        let (mut write, mut read) = ws_stream.split();

        // Process incoming messages
        let result = loop {
            match read.next().await {
                Some(Ok(msg)) => {
                    if let Err(e) = Self::process_ws_message(msg, &processor).await {
                        warn!("Error processing message from {}: {}", peer_addr, e);
                        handler.on_error(&e);
                    }
                }
                Some(Err(e)) => {
                    error!("WebSocket error from {}: {}", peer_addr, e);
                    break Err(crate::errors::WsError::ProtocolError(e.to_string()));
                }
                None => {
                    info!("WebSocket connection closed by client: {}", peer_addr);
                    break Ok(());
                }
            }
        };

        // Notify handler of disconnection
        if let Err(e) = handler.on_disconnect() {
            handler.on_error(&e);
        }

        // Close the connection
        let _ = write.close().await;

        result
    }

    /// Process a single WebSocket message (zero-copy)
    async fn process_ws_message(
        msg: tokio_tungstenite::tungstenite::Message,
        processor: &TradeMessageProcessor,
    ) -> WsResult<()> {
        use tokio_tungstenite::tungstenite::Message;

        match msg {
            Message::Binary(data) => {
                // Convert to Bytes for zero-copy processing
                let bytes = bytes::Bytes::from(data);
                processor.process_message(bytes).await
            }
            Message::Text(text) => processor.process_text(text).await,
            Message::Close(frame) => {
                info!(
                    "Close frame received: {:?}",
                    frame.map(|f| f.reason.to_string())
                );
                Err(crate::errors::WsError::ConnectionClosed)
            }
            Message::Ping(data) => {
                tracing::debug!("Ping received: {} bytes", data.len());
                Ok(())
            }
            Message::Pong(data) => {
                tracing::debug!("Pong received: {} bytes", data.len());
                Ok(())
            }
            Message::Frame(_) => {
                Err(crate::errors::WsError::ProtocolError(
                    "Frame messages not supported".to_string(),
                ))
            }
        }
    }

    /// Get the server address
    pub fn addr(&self) -> &str {
        &self.addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::LoggingTradeHandler;

    #[tokio::test]
    async fn test_server_creation() {
        let handler = Arc::new(LoggingTradeHandler);
        let server = WebSocketServer::new("127.0.0.1:0", handler);
        assert_eq!(server.addr(), "127.0.0.1:0");
    }
}
