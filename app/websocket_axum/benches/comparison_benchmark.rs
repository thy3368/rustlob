use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::runtime::Runtime;
use tungstenite::protocol::Message;

// 测试配置
const MESSAGE_COUNT: usize = 1000;
const CONCURRENT_CONNECTIONS: usize = 100;
const LATENCY_MESSAGE: &str = r#"{"text": "latency_test"}"#;
const THROUGHPUT_MESSAGE: &str = r#"{"text": "throughput_test_message"}"#;

// 测试 Axum 服务器
const AXUM_SERVER_ADDR: &str = "ws://127.0.0.1:8080/ws";
// 测试 Sockudo 服务器
const SOCKUDO_SERVER_ADDR: &str = "ws://127.0.0.1:8081";

// 单连接延迟测试
async fn single_connection_latency(server_addr: &str) -> Result<std::time::Duration, String> {
    let (mut socket, _) = tokio_tungstenite::connect_async(server_addr)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    let start = std::time::Instant::now();

    socket.send(Message::Text(LATENCY_MESSAGE.to_string()))
        .await
        .map_err(|e| format!("Send failed: {}", e))?;

    if let Some(Ok(_)) = socket.next().await {
        let duration = start.elapsed();
        socket.close(None).await.map_err(|e| format!("Close failed: {}", e))?;
        Ok(duration)
    } else {
        Err("No response received".to_string())
    }
}

// 发送多条消息的吞吐量测试
async fn send_multiple_messages(server_addr: &str, connection_id: usize, num_messages: usize) -> Result<(), String> {
    let (mut socket, _) = tokio_tungstenite::connect_async(server_addr)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    for i in 0..num_messages {
        let msg = format!(
            r#"{{"text": "Hello from connection {} - message {}"}}"#,
            connection_id, i
        );
        socket.send(Message::Text(msg))
            .await
            .map_err(|e| format!("Send failed: {}", e))?;

        if let Some(Ok(_)) = socket.next().await {
            // 处理响应
        }
    }

    socket.close(None)
        .await
        .map_err(|e| format!("Close failed: {}", e))?;
    Ok(())
}

// 并发连接测试
async fn concurrent_connections_test(server_addr: &str, num_connections: usize, messages_per_connection: usize) -> Result<(), String> {
    let mut handles = Vec::with_capacity(num_connections);

    for i in 0..num_connections {
        let addr = server_addr.to_string();
        let handle = tokio::spawn(async move {
            send_multiple_messages(&addr, i, messages_per_connection).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await
            .map_err(|e| format!("Task failed: {}", e))?
            .map_err(|e| format!("Connection failed: {}", e))?;
    }

    Ok(())
}

fn websocket_comparison_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    // 延迟测试
    let mut latency_group = c.benchmark_group("WebSocket Latency");
    latency_group.sample_size(1000);
    latency_group.measurement_time(std::time::Duration::from_secs(10));

    // 测试 Axum 服务器延迟
    latency_group.bench_with_input(BenchmarkId::new("axum_single_message_latency", 1), &1, |b, &_| {
        b.iter(|| {
            rt.block_on(single_connection_latency(AXUM_SERVER_ADDR)).unwrap()
        });
    });

    // 测试 Sockudo 服务器延迟
    latency_group.bench_with_input(BenchmarkId::new("sockudo_single_message_latency", 1), &1, |b, &_| {
        b.iter(|| {
            rt.block_on(single_connection_latency(SOCKUDO_SERVER_ADDR)).unwrap()
        });
    });

    latency_group.finish();

    // 吞吐量测试
    let mut throughput_group = c.benchmark_group("WebSocket Throughput");
    throughput_group.sample_size(20);
    throughput_group.measurement_time(std::time::Duration::from_secs(30));

    // 测试 Axum 服务器吞吐量
    throughput_group.bench_with_input(
        BenchmarkId::new("axum_throughput", format!("{}_connections_{}_messages", CONCURRENT_CONNECTIONS, MESSAGE_COUNT)),
        &(CONCURRENT_CONNECTIONS, MESSAGE_COUNT),
        |b, &(connections, messages)| {
            b.iter(|| {
                rt.block_on(concurrent_connections_test(AXUM_SERVER_ADDR, connections, messages)).unwrap();
            });
        },
    );

    // 测试 Sockudo 服务器吞吐量
    throughput_group.bench_with_input(
        BenchmarkId::new("sockudo_throughput", format!("{}_connections_{}_messages", CONCURRENT_CONNECTIONS, MESSAGE_COUNT)),
        &(CONCURRENT_CONNECTIONS, MESSAGE_COUNT),
        |b, &(connections, messages)| {
            b.iter(|| {
                rt.block_on(concurrent_connections_test(SOCKUDO_SERVER_ADDR, connections, messages)).unwrap();
            });
        },
    );

    throughput_group.finish();
}

criterion_group!(benches, websocket_comparison_benchmark);
criterion_main!(benches);