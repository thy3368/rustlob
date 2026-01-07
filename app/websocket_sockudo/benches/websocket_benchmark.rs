use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::{SinkExt, StreamExt};
use sockudo_ws::{Config, Message, WebSocketStream};
use tokio::net::TcpStream;

// 简单的Echo服务器处理函数（用于基准测试）
async fn echo_server(stream: tokio::net::TcpStream) -> Result<(), String> {
    let config = Config::builder()
        .compression(sockudo_ws::Compression::Disabled)
        .build();

    let mut websocket = WebSocketStream::server(stream, config);

    while let Some(msg) = websocket.next().await {
        match msg.map_err(|e| format!("WebSocket error: {}", e))? {
            Message::Text(text) => {
                websocket.send(Message::text(String::from_utf8_lossy(&text).to_string())).await
                    .map_err(|e| format!("Send failed: {}", e))?;
            }
            Message::Binary(data) => {
                websocket.send(Message::binary(data)).await
                    .map_err(|e| format!("Send failed: {}", e))?;
            }
            _ => {}
        }
    }

    Ok(())
}

// 启动基准测试服务器
async fn start_benchmark_server() -> u16 {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(echo_server(stream));
        }
    });

    port
}

async fn connect_and_send_small_message(port: u16) {
    if let Ok(stream) = TcpStream::connect(("127.0.0.1", port)).await {
        let config = Config::builder()
            .compression(sockudo_ws::Compression::Disabled)
            .build();

        let mut websocket = WebSocketStream::client(stream, config);

        // 发送小消息
        let msg = Message::text("Hello");
        if websocket.send(msg).await.is_ok() {
            // 接收响应
            if let Some(_) = websocket.next().await {
                // 不处理响应内容，只关心延迟
            }
        }
    }
}

async fn multiple_small_messages_test(port: u16, num_messages: usize) {
    if let Ok(stream) = TcpStream::connect(("127.0.0.1", port)).await {
        let config = Config::builder()
            .compression(sockudo_ws::Compression::Disabled)
            .build();

        let mut websocket = WebSocketStream::client(stream, config);

        for _ in 0..num_messages {
            let msg = Message::text("Hello");
            if websocket.send(msg).await.is_ok() {
                if let Some(_) = websocket.next().await {
                    // 不处理响应内容，只关心延迟
                }
            }
        }
    }
}

async fn connect_and_send_binary_message(port: u16, size: usize) {
    if let Ok(stream) = TcpStream::connect(("127.0.0.1", port)).await {
        let config = Config::builder()
            .compression(sockudo_ws::Compression::Disabled)
            .build();

        let mut websocket = WebSocketStream::client(stream, config);

        // 发送二进制消息
        let data = vec![0u8; size];
        let msg = Message::binary(data);
        if websocket.send(msg).await.is_ok() {
            // 接收响应
            if let Some(_) = websocket.next().await {
                // 不处理响应内容，只关心延迟
            }
        }
    }
}

fn websocket_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // 启动基准测试服务器
    let port = runtime.block_on(start_benchmark_server());

    let mut group = c.benchmark_group("WebSocket Performance");
    group.sample_size(200);
    group.measurement_time(std::time::Duration::from_secs(10));

    group.bench_function("small_message_roundtrip", |b| {
        b.iter(|| runtime.block_on(connect_and_send_small_message(port)));
    });

    group.bench_function("10_small_messages", |b| {
        b.iter(|| runtime.block_on(multiple_small_messages_test(port, 10)));
    });

    group.bench_function("64byte_binary_message", |b| {
        b.iter(|| runtime.block_on(connect_and_send_binary_message(port, 64)));
    });

    group.bench_function("256byte_binary_message", |b| {
        b.iter(|| runtime.block_on(connect_and_send_binary_message(port, 256)));
    });

    group.bench_function("1KB_binary_message", |b| {
        b.iter(|| runtime.block_on(connect_and_send_binary_message(port, 1024)));
    });

    group.finish();
}

criterion_group!(benches, websocket_benchmark);
criterion_main!(benches);