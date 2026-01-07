use futures_util::{SinkExt, StreamExt};
use sockudo_ws::{Config, Message, WebSocketStream};
use tokio::net::TcpListener;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MessageData {
    text: String,
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sockudo WebSocket Hello World Server");
    println!("Server starting on 0.0.0.0:8080");
    println!("Open index.html in your browser to test");

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr).await {
                println!("Connection error {}: {}", addr, e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut websocket = WebSocketStream::server(stream, Config::default());

    // 发送欢迎消息
    let welcome_msg = serde_json::json!({
        "type": "welcome",
        "message": "Hello from Sockudo WebSocket!"
    });
    websocket.send(Message::text(serde_json::to_string(&welcome_msg).unwrap())).await?;

    // 连接处理循环
    while let Some(msg) = websocket.next().await {
        match msg? {
            Message::Text(text_bytes) => {
                let text = String::from_utf8(text_bytes.to_vec())?;
                println!("Received text message from {}: {}", addr, text);

                if let Ok(msg_data) = serde_json::from_str::<MessageData>(&text) {
                    let response = serde_json::json!({
                        "type": "response",
                        "message": format!("You said: {}", msg_data.text)
                    });
                    websocket.send(Message::text(serde_json::to_string(&response).unwrap())).await?;

                    if msg_data.text.to_lowercase().contains("hello") {
                        let special_response = serde_json::json!({
                            "type": "special",
                            "message": "Hello World! 👋"
                        });
                        websocket.send(Message::text(serde_json::to_string(&special_response).unwrap())).await?;
                    }
                } else {
                    let error_response = serde_json::json!({
                        "type": "error",
                        "message": "Invalid message format"
                    });
                    websocket.send(Message::text(serde_json::to_string(&error_response).unwrap())).await?;
                }
            }
            Message::Binary(data) => {
                println!("Received binary message from {}: {} bytes", addr, data.len());
            }
            Message::Close(_) => {
                println!("Client {} closed the connection", addr);
                break;
            }
            Message::Ping(data) => {
                websocket.send(Message::pong(data)).await?;
            }
            Message::Pong(_) => {
                // 心跳响应
            }
        }
    }

    Ok(())
}