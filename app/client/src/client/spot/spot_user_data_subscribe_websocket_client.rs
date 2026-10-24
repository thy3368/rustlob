use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::info;

/// Spot 用户数据 WebSocket 流客户端
///
/// 负责连接到 `/ws/user_data` 端点，接收并处理用户数据消息
#[derive(Clone)]
pub struct SpotUserDataWebSocketStreamClient {
    /// WebSocket 服务器地址
    server_addr: String,
    /// 连接状态（使用 Mutex 确保线程安全）
    connection: Arc<Mutex<Option<ConnectionState>>>,
}

/// WebSocket 连接状态
struct ConnectionState {
    sender: mpsc::UnboundedSender<Message>,
}

impl SpotUserDataWebSocketStreamClient {
    /// 创建新的用户数据 WebSocket 客户端实例
    ///
    /// # Arguments
    /// * `server_addr` - WebSocket 服务器地址，例如 "ws://localhost:8084"
    ///
    /// # Example
    /// ```
    /// let client = SpotUserDataWebSocketStreamClient::new("ws://localhost:8084");
    /// ```
    pub fn new(server_addr: impl Into<String>) -> Self {
        Self { server_addr: server_addr.into(), connection: Arc::new(Mutex::new(None)) }
    }

    /// 连接到用户数据 WebSocket 服务器
    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/ws/user_data", self.server_addr);
        info!("Connecting to Spot User Data WebSocket at {}", url);

        let (ws_stream, _) = connect_async(url).await?;
        info!("Successfully connected to Spot User Data WebSocket");

        let (mut sender, mut receiver) = ws_stream.split();

        // 创建通道用于发送消息
        let (tx, mut rx) = mpsc::unbounded_channel();

        // 保存连接状态
        *self.connection.lock().await = Some(ConnectionState { sender: tx.clone() });

        // 处理接收消息的任务
        let connection_clone = self.connection.clone();
        tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        info!("Received Spot User Data message: {}", text);
                        println!("Received Spot User Data message: {}", text);
                    }
                    Ok(Message::Binary(data)) => {
                        info!("Received Spot User Data binary message: {:?}", data);
                        println!("Received Spot User Data binary message: {:?}", data);
                    }
                    Ok(Message::Close(frame)) => {
                        info!("Spot User Data WebSocket connection closed: {:?}", frame);
                        println!("Spot User Data WebSocket connection closed: {:?}", frame);
                        break;
                    }
                    Err(e) => {
                        info!("Spot User Data WebSocket error: {}", e);
                        println!("Spot User Data WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // 连接关闭后清理状态
            *connection_clone.lock().await = None;
            info!("Spot User Data WebSocket connection cleaned up");
        });

        // 处理发送消息的任务
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(msg) => {
                        if sender.send(msg).await.is_err() {
                            info!("Failed to send message to Spot User Data WebSocket");
                            println!("Failed to send message to Spot User Data WebSocket");
                            break;
                        }
                    }
                    None => break,
                }
            }
        });

        Ok(())
    }

    /// 发送文本消息到服务器
    pub async fn send_text(
        &self,
        text: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.connection.lock().await;
        if let Some(state) = &mut *guard {
            state
                .sender
                .send(Message::Text(text.into().into()))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(())
        } else {
            Err("WebSocket connection not established".into())
        }
    }

    /// 发送二进制消息到服务器
    pub async fn send_binary(&self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.connection.lock().await;
        if let Some(state) = &mut *guard {
            state
                .sender
                .send(Message::Binary(data.into()))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(())
        } else {
            Err("WebSocket connection not established".into())
        }
    }

    /// 检查连接是否已建立
    pub async fn is_connected(&self) -> bool {
        self.connection.lock().await.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_data_websocket_connection() {
        // 注意：需要先启动服务端 (axum_server)
        let client = SpotUserDataWebSocketStreamClient::new("ws://localhost:8084");

        match client.connect().await {
            Ok(_) => {
                println!("Successfully connected to Spot User Data WebSocket");

                // 保持连接一段时间以接收消息
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

                assert!(client.is_connected().await);
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                // 如果服务端未启动，测试可能会失败，但不应该导致panic
                eprintln!("Warning: Service may not be running: {}", e);
            }
        }
    }
}
