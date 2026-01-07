mod trade;

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use futures::{SinkExt, StreamExt};
use ws_gateway::{
    WebSocketServer, TradeMessageHandler, WsResult,
};
use trade::Trade;

/// Example handler that collects trade statistics
struct StatsHandler {
    total_trades: std::sync::atomic::AtomicU64,
    total_volume: std::sync::Mutex<f64>,
}

impl StatsHandler {
    fn new() -> Self {
        Self {
            total_trades: std::sync::atomic::AtomicU64::new(0),
            total_volume: std::sync::Mutex::new(0.0),
        }
    }

    fn stats(&self) -> (u64, f64) {
        let trades = self.total_trades.load(std::sync::atomic::Ordering::Relaxed);
        let volume = *self.total_volume.lock().unwrap();
        (trades, volume)
    }
}

impl TradeMessageHandler for StatsHandler {
    fn on_trade(&self, data: &[u8]) -> WsResult<()> {
        if let Some(trade) = Trade::decode(data) {
            self.total_trades.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let mut volume = self.total_volume.lock().unwrap();
            *volume += trade.notional();

            tracing::info!(
                "Trade received: id={}, symbol={}, price={}, qty={}",
                trade.trade_id,
                trade.symbol_char(),
                trade.price,
                trade.quantity
            );
        }
        Ok(())
    }

    fn on_connect(&self) -> WsResult<()> {
        tracing::info!("Client connected");
        Ok(())
    }

    fn on_disconnect(&self) -> WsResult<()> {
        let (trades, volume) = self.stats();
        tracing::info!(
            "Client disconnected. Total trades: {}, Total volume: {}",
            trades,
            volume
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create handler
    let handler = Arc::new(StatsHandler::new());

    // Create and spawn server
    let server = Arc::new(WebSocketServer::new("127.0.0.1:9001", handler.clone()));
    let server_task = server.clone().spawn();

    tracing::info!("Trade WebSocket server started on {}", server.addr());

    // Spawn example client
    let client_task = tokio::spawn(example_client());

    // Wait for either server or client to finish
    tokio::select! {
        res = server_task => {
            tracing::error!("Server error: {:?}", res);
        }
        res = client_task => {
            tracing::info!("Client finished: {:?}", res);
        }
    }

    Ok(())
}

/// Example client that sends trade messages
async fn example_client() {
    // Give server time to start
    sleep(Duration::from_millis(500)).await;

    tracing::info!("Connecting to WebSocket server...");

    match tokio_tungstenite::connect_async("ws://127.0.0.1:9001").await {
        Ok((ws_stream, _)) => {
            tracing::info!("Connected to server");

            let (mut write, _read) = ws_stream.split();

            // Send some individual trade messages
            tracing::info!("Sending 5 individual trade messages...");
            for i in 0..5 {
                let trade = Trade::new(
                    1000 + i as u64,
                    if i % 2 == 0 { b'A' } else { b'B' },
                    100.0 + (i as f64),
                    (i + 1) * 100,
                );

                let encoded = trade.encode();
                let msg = tokio_tungstenite::tungstenite::Message::Binary(encoded);

                if let Err(e) = write.send(msg).await {
                    tracing::error!("Failed to send message: {}", e);
                    break;
                }
                tracing::info!(
                    "Sent trade: id={}, symbol={}, price={}, qty={}",
                    trade.trade_id,
                    trade.symbol_char(),
                    trade.price,
                    trade.quantity
                );

                sleep(Duration::from_millis(100)).await;
            }

            // Send batch of trades
            tracing::info!("Sending batch of 3 trades...");
            let trades = vec![
                Trade::new(2000, b'C', 200.0, 500),
                Trade::new(2001, b'D', 250.0, 750),
                Trade::new(2002, b'E', 300.0, 1000),
            ];

            let msg_size = Trade::encoded_size();
            let mut batch = vec![0u8; msg_size * trades.len()];
            let encoded_len = Trade::encode_to_buffer(&trades, &mut batch);

            let batch_msg = tokio_tungstenite::tungstenite::Message::Binary(batch[..encoded_len].to_vec());
            if let Err(e) = write.send(batch_msg).await {
                tracing::error!("Failed to send batch: {}", e);
            } else {
                tracing::info!(
                    "Sent batch of {} trades, total notional: {:.2}",
                    trades.len(),
                    trades.iter().map(|t| t.notional()).sum::<f64>()
                );
            }

            // Close connection
            sleep(Duration::from_secs(1)).await;
            let _ = write.close().await;
            tracing::info!("Client closed connection");
        }
        Err(e) => {
            tracing::error!("Failed to connect to server: {}", e);
        }
    }
}
