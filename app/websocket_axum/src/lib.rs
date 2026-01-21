pub mod md_gw;
pub mod ud_gw;

pub mod trade_gw;
// User Data Streams
use axum::{
    extract::WebSocketUpgrade,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use simd_json::json;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tokio::sync::broadcast;
use simd_json::owned::Value;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub text: String,
}

/// WebSocket 事件数据类型
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct WebSocketEvent {
    pub r#type: String,
    pub data: Value,
}

/// WebSocket 连接处理器
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    tx: broadcast::Sender<WebSocketEvent>,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        println!("New WebSocket connection established");

        // 发送欢迎消息
        let welcome_msg = json!({
            "type": "welcome",
            "message": "Hello from Axum WebSocket!"
        });
        if socket.send(axum::extract::ws::Message::Text(simd_json::to_string(&welcome_msg).unwrap())).await.is_err() {
            return;
        }

        // 订阅事件广播
        let mut rx = tx.subscribe();

        // 发送事件
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(msg) => {
                            let event_msg = json!({
                                "type": msg.r#type,
                                "data": msg.data
                            });
                            if socket.send(axum::extract::ws::Message::Text(
                                simd_json::to_string(&event_msg).unwrap()
                            )).await.is_err() {
                                break;
                            }
                        },
                        Err(_) => break,
                    }
                },
                msg = socket.recv() => {
                    match msg {
                        Some(Ok(msg)) => match msg {
                            axum::extract::ws::Message::Text(text) => {
                                println!("Received message: {}", text);
                            },
                            axum::extract::ws::Message::Close(_) => {
                                println!("Client closed the connection");
                                break;
                            },
                            _ => {},
                        },
                        _ => break,
                    }
                }
            }
        }

        println!("WebSocket connection closed");
    })
}

/// 创建包含 WebSocket 路由的 Axum 应用
pub fn create_app(tx: broadcast::Sender<WebSocketEvent>) -> Router {
    Router::new()
        .route("/ws", get(move |ws| websocket_handler(ws, tx.clone())))
        .nest_service("/", ServeDir::new("."))
}

/// 启动 WebSocket 服务器
pub async fn start_server(
    port: u16,
    tx: broadcast::Sender<WebSocketEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(tx);

    println!("WebSocket server starting on http://localhost:{}", port);
    println!("Open index.html in your browser to test");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}