use axum::{
    extract::{WebSocketUpgrade, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Deserialize, Debug)]
struct Message {
    text: String,
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        println!("New WebSocket connection established");

        // 发送欢迎消息
        let welcome_msg = serde_json::json!({
            "type": "welcome",
            "message": "Hello from Axum WebSocket!"
        });
        if socket.send(axum::extract::ws::Message::Text(serde_json::to_string(&welcome_msg).unwrap())).await.is_err() {
            return;
        }

        // 接收并处理消息
        while let Some(Ok(msg)) = socket.recv().await {
            match msg {
                axum::extract::ws::Message::Text(text) => {
                    println!("Received message: {}", text);

                    match serde_json::from_str::<Message>(&text) {
                        Ok(msg) => {
                            // 回复消息
                            let response = serde_json::json!({
                                "type": "response",
                                "message": format!("You said: {}", msg.text)
                            });
                            if socket.send(axum::extract::ws::Message::Text(serde_json::to_string(&response).unwrap())).await.is_err() {
                                break;
                            }

                            // 如果收到 "hello" 消息，发送特殊响应
                            if msg.text.to_lowercase().contains("hello") {
                                let special_response = serde_json::json!({
                                    "type": "special",
                                    "message": "Hello World! 👋"
                                });
                                if socket.send(axum::extract::ws::Message::Text(serde_json::to_string(&special_response).unwrap())).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error parsing message: {}", e);
                            let error_response = serde_json::json!({
                                "type": "error",
                                "message": "Invalid message format"
                            });
                            if socket.send(axum::extract::ws::Message::Text(serde_json::to_string(&error_response).unwrap())).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                axum::extract::ws::Message::Binary(_) => {
                    println!("Received binary message");
                }
                axum::extract::ws::Message::Close(_) => {
                    println!("Client closed the connection");
                    break;
                }
                _ => {
                    println!("Received other message type");
                }
            }
        }

        println!("WebSocket connection closed");
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .nest_service("/", ServeDir::new("."));

    println!("WebSocket server starting on http://localhost:8080");
    println!("Open index.html in your browser to test");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
