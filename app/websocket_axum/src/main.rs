use tokio::sync::broadcast;
use websocket_axum::{start_server, WebSocketEvent};
use simd_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建事件广播通道
    let (tx, _) = broadcast::channel(1024);

    // 启动服务器（在后台运行）
    let server_tx = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(8083, server_tx).await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    // 示例：发送一些模拟事件
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        let mut counter = 0;

        loop {
            interval.tick().await;
            counter += 1;

            let event = WebSocketEvent {
                r#type: "test_event".to_string(),
                data: json!({
                    "message": format!("Test event {}", counter),
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "counter": counter
                }),
            };

            let _ = tx.send(event);
        }
    });

    println!("WebSocket server started. Press Ctrl+C to exit.");

    // 等待用户中断
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}
