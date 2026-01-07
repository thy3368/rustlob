use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::{SinkExt, StreamExt};
use sockudo_ws::{Config, Message, WebSocketStream};
use tokio::net::TcpStream;
use websocket_sockudo::start_server;

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
                    if let Ok(text) = String::from_utf8(text_bytes.to_vec()) {
                        response_count += 1;

                        // 期望至少收到一个响应
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
                    // 只等待第一个响应
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

fn websocket_latency_benchmark(c: &mut Criterion) {
    // 启动服务器（在后台）
    let server_handle = std::thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let _ = start_server().await;
            });
    });

    // 等待服务器启动
    std::thread::sleep(std::time::Duration::from_millis(500));

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("WebSocket Latency");
    group.sample_size(100); // 增加样本大小以提高准确性
    group.measurement_time(std::time::Duration::from_secs(5));

    group.bench_function("single_message_latency", |b| {
        b.iter(|| runtime.block_on(connect_and_send_message()));
    });

    group.bench_function("10_connections", |b| {
        b.iter(|| runtime.block_on(multiple_connections_test(10)));
    });

    group.bench_function("100_connections", |b| {
        b.iter(|| runtime.block_on(multiple_connections_test(100)));
    });

    group.bench_function("1000_connections", |b| {
        b.iter(|| runtime.block_on(multiple_connections_test(1000)));
    });

    group.finish();

    // 停止服务器（通过向标准输入发送换行符）
    // 注意：这种方法可能不够完美，但对于基准测试来说足够了
    drop(server_handle);
}

criterion_group!(benches, websocket_latency_benchmark);
criterion_main!(benches);