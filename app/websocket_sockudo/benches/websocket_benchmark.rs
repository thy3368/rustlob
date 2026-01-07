use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::{SinkExt, StreamExt};
use sockudo_ws::{Config, Message, WebSocketStream};
use tokio::net::TcpStream;

async fn connect_and_send_message() {
    // 连接到本地服务器
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080").await {
        let mut websocket = WebSocketStream::client(stream, Config::default());

        // 发送消息
        let msg = serde_json::json!({"text": "Hello World"}).to_string();
        if websocket.send(Message::text(msg)).await.is_ok() {
            // 接收响应
            let mut response_count = 0;
            while let Some(msg) = websocket.next().await {
                if let Ok(Message::Text(text_bytes)) = msg {
                    if let Ok(_text) = String::from_utf8(text_bytes.to_vec()) {
                        response_count += 1;
                        if response_count >= 1 {
                            break;
                        }
                    }
                }
            }
        }
    }
}

async fn multiple_connections_test(num_connections: usize) {
    let mut handles = Vec::with_capacity(num_connections);

    for i in 0..num_connections {
        let handle = tokio::spawn(async move {
            if let Ok(stream) = TcpStream::connect("127.0.0.1:8080").await {
                let mut websocket = WebSocketStream::client(stream, Config::default());

                let msg = serde_json::json!({"text": format!("Connection {}", i)}).to_string();
                if websocket.send(Message::text(msg)).await.is_ok() {
                    if let Some(msg) = websocket.next().await {
                        if let Ok(Message::Text(text_bytes)) = msg {
                            let _ = String::from_utf8(text_bytes.to_vec());
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

fn websocket_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("WebSocket Performance");
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));

    group.bench_function("single_message", |b| {
        b.iter(|| runtime.block_on(connect_and_send_message()));
    });

    group.bench_function("10_connections", |b| {
        b.iter(|| runtime.block_on(multiple_connections_test(10)));
    });

    group.bench_function("100_connections", |b| {
        b.iter(|| runtime.block_on(multiple_connections_test(100)));
    });

    group.finish();
}

criterion_group!(benches, websocket_benchmark);
criterion_main!(benches);