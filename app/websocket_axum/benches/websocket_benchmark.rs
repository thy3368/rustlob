use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::runtime::Runtime;
use tungstenite::protocol::Message;

const SERVER_ADDR: &str = "ws://127.0.0.1:8080/ws";
const MESSAGE_COUNT: usize = 1000;
const CONCURRENT_CONNECTIONS: usize = 100;

async fn connect_and_send_messages(connection_id: usize) -> Result<(), String> {
    let (mut socket, _) = tokio_tungstenite::connect_async(SERVER_ADDR)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    // 发送消息
    for i in 0..MESSAGE_COUNT {
        let msg = format!(
            r#"{{"text": "Hello from connection {} - message {}"}}"#,
            connection_id, i
        );
        socket.send(Message::Text(msg))
            .await
            .map_err(|e| format!("Send failed: {}", e))?;

        // 接收响应
        if let Some(Ok(_)) = socket.next().await {
            // 处理响应
        }
    }

    socket.close(None)
        .await
        .map_err(|e| format!("Close failed: {}", e))?;
    Ok(())
}

async fn run_concurrent_connections() -> Result<(), String> {
    let mut handles = Vec::with_capacity(CONCURRENT_CONNECTIONS);

    for i in 0..CONCURRENT_CONNECTIONS {
        let handle = tokio::spawn(connect_and_send_messages(i));
        handles.push(handle);
    }

    for handle in handles {
        handle.await
            .map_err(|e| format!("Task failed: {}", e))?
            .map_err(|e| format!("Connection failed: {}", e))?;
    }

    Ok(())
}

fn websocket_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_with_input(
        BenchmarkId::new("websocket_throughput", CONCURRENT_CONNECTIONS),
        &CONCURRENT_CONNECTIONS,
        |b, &_| {
            b.iter(|| {
                rt.block_on(run_concurrent_connections()).unwrap();
            });
        },
    );

    // 单连接延迟测试
    c.bench_with_input(BenchmarkId::new("websocket_latency", 1), &1, |b, &_| {
        b.iter(|| {
            rt.block_on(async {
                let (mut socket, _) = tokio_tungstenite::connect_async(SERVER_ADDR).await.unwrap();
                let start = std::time::Instant::now();

                let msg = r#"{"text": "latency_test"}"#;
                socket.send(Message::Text(msg.to_string())).await.unwrap();

                if let Some(Ok(_)) = socket.next().await {
                    let _duration = start.elapsed();
                    // 记录延迟
                }

                socket.close(None).await.unwrap();
            });
        });
    });
}

criterion_group!(benches, websocket_benchmark);
criterion_main!(benches);