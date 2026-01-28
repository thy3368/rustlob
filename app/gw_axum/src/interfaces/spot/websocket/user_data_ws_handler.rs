use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, WebSocketUpgrade},
    response::IntoResponse
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;

use crate::interfaces::spot::websocket::connection_types::{ConnectionInfo, ConnectionRepo};

/// 用户数据 WebSocket 连接处理器
pub async fn user_data_websocket_handler(
    ws: WebSocketUpgrade, ConnectInfo(client_addr): ConnectInfo<std::net::SocketAddr>,
    connection_repo: Arc<ConnectionRepo>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        println!("New Spot User Data WebSocket connection established from {}", client_addr);

        let (mut sender, mut receiver) = socket.split();

        // 创建 mpsc 通道用于服务端推送消息
        let (tx, mut rx) = mpsc::unbounded_channel();

        let timestamp: i64 = chrono::Utc::now().timestamp_millis();
        // 创建连接信息
        let conn_info = ConnectionInfo {
            user_id: None, // 初始为 None，需要用户认证后设置
            client_addr,
            connected_at: timestamp,
            last_active_at: timestamp,
            sender: tx
        };

        // 添加到连接管理器（使用 mpsc Sender）
        connection_repo.add_connection(conn_info).await;


        // 处理客户端消息和服务端主动推送，有多少个连接便有多少个这个
        loop {
            tokio::select! {
                // 处理服务端主动推送消息
                Some(msg) = rx.recv() => {
                    if sender.send(msg).await.is_err() {
                        println!("Failed to send message to WebSocket: {}", client_addr);
                        break;
                    }
                }

                // 处理客户端消息
                msg = receiver.next() => {

                    //todo 是否处理订阅命令？
                    match msg {
                        Some(Ok(msg)) => match msg {
                            axum::extract::ws::Message::Text(text) => {
                                println!("Received User Data WebSocket message from {}: {}", client_addr, text);

                            },
                            axum::extract::ws::Message::Close(_) => {
                                println!("Spot User Data WebSocket client closed the connection: {}", client_addr);
                                break;
                            },
                            _ => {
                                // connection_repo.update_last_active(client_addr).await;
                            },
                        },
                        _ => break,
                    }
                }
            }
        }

        // 连接关闭时移除连接信息
        connection_repo.remove_connection(client_addr).await;
        println!("Spot User Data WebSocket connection closed: {}", client_addr);
    })
}
