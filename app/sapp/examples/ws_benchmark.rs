//! WebSocket性能基准测试
//!
//! 测量订单处理延迟和吞吐量
//!
//! 运行: cargo run --example ws_benchmark --release


use std::time::{Duration, Instant};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

// ============ 消息定义 ============

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    LimitOrder { trader_id: String, side: String, price: u32, quantity: u32 },
    Ping
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMessage {
    Pong { timestamp: u64 },
    OrderAck { order_id: u64, status: String, latency_us: u128 },
    Trade { trade_id: u64, buyer: String, seller: String, price: u32, quantity: u32, timestamp: u64 },
    BookUpdate { best_bid: Option<u32>, best_ask: Option<u32>, spread: Option<u32> }
}

// ============ 性能统计 ============

#[derive(Debug, Default)]
struct LatencyStats {
    samples: Vec<u128>,
    total_latency: u128,
    min: u128,
    max: u128
}

impl LatencyStats {
    fn new() -> Self {
        Self {
            samples: Vec::new(),
            total_latency: 0,
            min: u128::MAX,
            max: 0
        }
    }

    fn record(&mut self, latency_us: u128) {
        self.samples.push(latency_us);
        self.total_latency += latency_us;
        self.min = self.min.min(latency_us);
        self.max = self.max.max(latency_us);
    }

    fn avg(&self) -> u128 {
        if self.samples.is_empty() {
            0
        } else {
            self.total_latency / self.samples.len() as u128
        }
    }

    fn percentile(&mut self, p: f64) -> u128 {
        if self.samples.is_empty() {
            return 0;
        }
        self.samples.sort_unstable();
        let index = ((self.samples.len() as f64) * p) as usize;
        self.samples[index.min(self.samples.len() - 1)]
    }

    fn print_report(&mut self, test_name: &str) {
        println!("\n=== {} 性能报告 ===", test_name);
        println!("样本数量: {}", self.samples.len());
        println!("平均延迟: {} μs", self.avg());
        println!("最小延迟: {} μs", self.min);
        println!("最大延迟: {} μs", self.max);
        println!("P50 延迟: {} μs", self.percentile(0.50));
        println!("P95 延迟: {} μs", self.percentile(0.95));
        println!("P99 延迟: {} μs", self.percentile(0.99));
        println!("P99.9 延迟: {} μs", self.percentile(0.999));
    }
}

// ============ 测试工具 ============

async fn connect_ws(url: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to WebSocket");
    ws_stream
}

async fn send_message(ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, msg: &ClientMessage) {
    let json = serde_json::to_string(msg).unwrap();
    ws.send(Message::Text(json)).await.unwrap();
}

async fn recv_message(ws: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) -> ServerMessage {
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(text))) => {
                if let Ok(msg) = serde_json::from_str::<ServerMessage>(&text) {
                    return msg;
                }
            }
            Some(Err(e)) => panic!("WebSocket error: {}", e),
            None => panic!("Connection closed"),
            _ => {}
        }
    }
}

// ============ 测试用例 ============

/// 测试1: Ping/Pong延迟
async fn test_ping_latency(url: &str, count: usize) {
    println!("\n[测试1] Ping/Pong 延迟测试 (样本: {})", count);

    let mut ws = connect_ws(url).await;
    let mut stats = LatencyStats::new();

    for _ in 0..count {
        let start = Instant::now();
        send_message(&mut ws, &ClientMessage::Ping).await;

        match recv_message(&mut ws).await {
            ServerMessage::Pong {
                ..
            } => {
                let latency = start.elapsed().as_micros();
                stats.record(latency);
            }
            _ => panic!("Expected Pong")
        }
    }

    stats.print_report("Ping/Pong");
}

/// 测试2: 限价单处理延迟
async fn test_order_latency(url: &str, count: usize) {
    println!("\n[测试2] 限价单处理延迟测试 (样本: {})", count);

    let mut ws = connect_ws(url).await;
    let mut stats = LatencyStats::new();
    let mut price: u32 = 50000;

    for i in 0..count {
        let side = if i % 2 == 0 { "buy" } else { "sell" };
        // 交替价格避免成交
        if side == "buy" {
            price = price.saturating_sub(10);
        } else {
            price += 10;
        }

        let start = Instant::now();
        send_message(&mut ws, &ClientMessage::LimitOrder {
            trader_id: format!("trader{}", i),
            side: side.to_string(),
            price,
            quantity: 1
        })
        .await;

        // 等待订单确认
        loop {
            match recv_message(&mut ws).await {
                ServerMessage::OrderAck {
                    ..
                } => {
                    let latency = start.elapsed().as_micros();
                    stats.record(latency);
                    break;
                }
                _ => {} // 忽略其他消息
            }
        }
    }

    stats.print_report("限价单处理");
}

/// 测试3: 撮合延迟（买卖匹配）
async fn test_matching_latency(url: &str, count: usize) {
    println!("\n[测试3] 订单撮合延迟测试 (样本: {})", count);

    let mut ws = connect_ws(url).await;
    let mut stats = LatencyStats::new();

    for i in 0..count {
        let price = 50000;

        // 先下卖单
        send_message(&mut ws, &ClientMessage::LimitOrder {
            trader_id: format!("seller{}", i),
            side: "sell".to_string(),
            price,
            quantity: 1
        })
        .await;

        // 等待卖单确认
        loop {
            if let ServerMessage::OrderAck {
                ..
            } = recv_message(&mut ws).await
            {
                break;
            }
        }

        // 下买单触发撮合
        let start = Instant::now();
        send_message(&mut ws, &ClientMessage::LimitOrder {
            trader_id: format!("buyer{}", i),
            side: "buy".to_string(),
            price,
            quantity: 1
        })
        .await;

        // 等待成交通知
        loop {
            match recv_message(&mut ws).await {
                ServerMessage::Trade {
                    ..
                } => {
                    let latency = start.elapsed().as_micros();
                    stats.record(latency);
                    break;
                }
                _ => {} // 忽略订单确认和订单簿更新
            }
        }
    }

    stats.print_report("订单撮合");
}

/// 测试4: 吞吐量测试
async fn test_throughput(url: &str, duration_secs: u64) {
    println!("\n[测试4] 吞吐量测试 (持续时间: {}秒)", duration_secs);

    let mut ws = connect_ws(url).await;
    let start_time = Instant::now();
    let test_duration = Duration::from_secs(duration_secs);
    let mut order_count = 0;
    let mut price = 50000;

    while start_time.elapsed() < test_duration {
        // 快速下单（不等待响应）
        for _ in 0..10 {
            let side = if price % 2 == 0 { "buy" } else { "sell" };
            price += 10;

            send_message(&mut ws, &ClientMessage::LimitOrder {
                trader_id: format!("trader{}", order_count),
                side: side.to_string(),
                price,
                quantity: 1
            })
            .await;
            order_count += 1;
        }

        // 消费响应消息
        tokio::time::timeout(Duration::from_millis(10), async {
            while let Some(Ok(_)) = ws.next().await {
                // 丢弃消息
            }
        })
        .await
        .ok();
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let throughput = order_count as f64 / elapsed;

    println!("\n=== 吞吐量测试报告 ===");
    println!("总订单数: {}", order_count);
    println!("测试时长: {:.2} 秒", elapsed);
    println!("吞吐量: {:.0} 订单/秒", throughput);
}

/// 测试5: 并发连接测试
async fn test_concurrent_connections(url: &str, num_clients: usize, orders_per_client: usize) {
    println!("\n[测试5] 并发连接测试 (客户端: {}, 每客户端订单: {})", num_clients, orders_per_client);

    let start_time = Instant::now();
    let mut handles = Vec::new();

    for client_id in 0..num_clients {
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            let mut ws = connect_ws(&url).await;
            let mut price = 50000 + (client_id as u32 * 100);

            for i in 0..orders_per_client {
                let side = if i % 2 == 0 { "buy" } else { "sell" };
                price += 10;

                send_message(&mut ws, &ClientMessage::LimitOrder {
                    trader_id: format!("client{}trader{}", client_id, i),
                    side: side.to_string(),
                    price,
                    quantity: 1
                })
                .await;

                // 等待确认
                loop {
                    if let ServerMessage::OrderAck {
                        ..
                    } = recv_message(&mut ws).await
                    {
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // 等待所有客户端完成
    for handle in handles {
        handle.await.unwrap();
    }

    let elapsed = start_time.elapsed();
    let total_orders = num_clients * orders_per_client;
    let throughput = total_orders as f64 / elapsed.as_secs_f64();

    println!("\n=== 并发连接测试报告 ===");
    println!("客户端数: {}", num_clients);
    println!("总订单数: {}", total_orders);
    println!("总时长: {:.2} 秒", elapsed.as_secs_f64());
    println!("吞吐量: {:.0} 订单/秒", throughput);
}

// ============ 主测试 ============

#[tokio::main]
async fn main() {
    let url = "ws://localhost:9090/ws";

    println!("===================================");
    println!("  WebSocket 订单匹配服务 性能测试");
    println!("===================================");
    println!("目标服务器: {}", url);

    // 等待服务器启动
    println!("\n连接到服务器...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    // 运行测试套件
    test_ping_latency(url, 1000).await;
    test_order_latency(url, 1000).await;
    test_matching_latency(url, 500).await;
    test_throughput(url, 10).await;
    test_concurrent_connections(url, 10, 100).await;

    println!("\n===================================");
    println!("  所有测试完成");
    println!("===================================");
}
