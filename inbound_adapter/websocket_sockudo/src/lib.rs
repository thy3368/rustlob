#![allow(warnings)]

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use simd_json::json;
use sockudo_ws::{Compression, Config, Message, WebSocketStream, handshake};
use tokio::net::TcpListener;

#[derive(Deserialize, Debug)]
struct MessageData {
    text: String,
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sockudo WebSocket Hello World Server");
    println!("Server starting on 0.0.0.0:8081");
    println!("Open index.html in your browser to test");

    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    let config = Config::builder().compression(Compression::Disabled).build();

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);

        // 处理WebSocket连接
        let config = config.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, config).await {
                println!("Connection error {}: {}", addr, e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // 执行HTTP握手
    let mut stream = stream;
    let _handshake = handshake::server_handshake(&mut stream).await?;

    // 创建服务器端WebSocket流
    let mut websocket = WebSocketStream::server(stream, config);

    // 发送欢迎消息
    let welcome_msg = json!({
        "type": "welcome",
        "message": "Hello from Sockudo WebSocket!"
    });
    websocket.send(Message::text(simd_json::to_string(&welcome_msg)?)).await?;

    // 连接处理循环
    while let Some(msg) = websocket.next().await {
        match msg? {
            Message::Text(text_bytes) => {
                let text = String::from_utf8(text_bytes.to_vec())?;
                println!("Received text message from {}: {}", addr, text);

                let mut text_mut = text.clone();
                if let Ok(msg_data) = unsafe { simd_json::from_str::<MessageData>(&mut text_mut) } {
                    let response = json!({
                        "type": "response",
                        "message": format!("You said: {}", msg_data.text)
                    });
                    websocket.send(Message::text(simd_json::to_string(&response)?)).await?;

                    if msg_data.text.to_lowercase().contains("hello") {
                        let special_response = json!({
                            "type": "special",
                            "message": "Hello World! 👋"
                        });
                        websocket
                            .send(Message::text(simd_json::to_string(&special_response)?))
                            .await?;
                    }
                } else {
                    let error_response = json!({
                        "type": "error",
                        "message": "Invalid message format"
                    });
                    websocket.send(Message::text(simd_json::to_string(&error_response)?)).await?;
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
