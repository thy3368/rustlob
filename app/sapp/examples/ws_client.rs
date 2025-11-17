//! WebSocket客户端示例
//!
//! 演示如何连接到WebSocket服务并进行交易
//!
//! 运行: cargo run --example ws_client

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    Subscribe {
        channels: Vec<String>,
    },
    LimitOrder {
        trader_id: String,
        side: String,
        price: u32,
        quantity: u32,
    },
    MarketOrder {
        trader_id: String,
        side: String,
        quantity: u32,
    },
    CancelOrder {
        order_id: u64,
    },
    Ping,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMessage {
    Pong {
        timestamp: u64,
    },
    Subscribed {
        channels: Vec<String>,
    },
    OrderAck {
        order_id: u64,
        status: String,
        latency_us: u128,
    },
    Trade {
        trade_id: u64,
        buyer: String,
        seller: String,
        price: u32,
        quantity: u32,
        timestamp: u64,
    },
    BookUpdate {
        best_bid: Option<u32>,
        best_ask: Option<u32>,
        spread: Option<u32>,
    },
    Error {
        code: String,
        message: String,
    },
}

#[tokio::main]
async fn main() {
    let url = "ws://localhost:9090/ws";
    println!("连接到 WebSocket 服务器: {}", url);

    // 连接WebSocket
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("无法连接到服务器");

    println!("✓ 已连接到服务器");

    let (mut write, mut read) = ws_stream.split();

    // 启动接收任务
    let recv_handle = tokio::spawn(async move {
        println!("\n[开始监听服务器消息]\n");
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                        match server_msg {
                            ServerMessage::Pong { timestamp } => {
                                println!("❤ Pong - 时间戳: {}", timestamp);
                            }
                            ServerMessage::Subscribed { channels } => {
                                println!("✓ 已订阅频道: {:?}", channels);
                            }
                            ServerMessage::OrderAck {
                                order_id,
                                status,
                                latency_us,
                            } => {
                                println!(
                                    "✓ 订单确认 - ID: {}, 状态: {}, 延迟: {}μs",
                                    order_id, status, latency_us
                                );
                            }
                            ServerMessage::Trade {
                                trade_id,
                                buyer,
                                seller,
                                price,
                                quantity,
                                timestamp,
                            } => {
                                println!(
                                    "🔥 成交 - ID: {}, 买: {}, 卖: {}, 价格: {}, 数量: {}, 时间: {}",
                                    trade_id, buyer, seller, price, quantity, timestamp
                                );
                            }
                            ServerMessage::BookUpdate {
                                best_bid,
                                best_ask,
                                spread,
                            } => {
                                println!(
                                    "📊 订单簿更新 - 最佳买价: {:?}, 最佳卖价: {:?}, 价差: {:?}",
                                    best_bid, best_ask, spread
                                );
                            }
                            ServerMessage::Error { code, message } => {
                                println!("❌ 错误 - 代码: {}, 消息: {}", code, message);
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    println!("连接已关闭");
                    break;
                }
                Err(e) => {
                    eprintln!("接收消息错误: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // 演示各种操作
    println!("\n=== 演示交易操作 ===\n");

    // 1. 订阅频道
    println!("[1] 订阅市场数据...");
    let subscribe = ClientMessage::Subscribe {
        channels: vec!["trades".to_string(), "book".to_string()],
    };
    write
        .send(Message::Text(serde_json::to_string(&subscribe).unwrap()))
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 2. 发送Ping
    println!("\n[2] 发送心跳...");
    let ping = ClientMessage::Ping;
    write
        .send(Message::Text(serde_json::to_string(&ping).unwrap()))
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 3. 下限价卖单
    println!("\n[3] 下限价卖单 (价格: 50100, 数量: 5)...");
    let sell_order = ClientMessage::LimitOrder {
        trader_id: "alice".to_string(),
        side: "sell".to_string(),
        price: 50100,
        quantity: 5,
    };
    write
        .send(Message::Text(serde_json::to_string(&sell_order).unwrap()))
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 4. 下限价买单
    println!("\n[4] 下限价买单 (价格: 49900, 数量: 3)...");
    let buy_order = ClientMessage::LimitOrder {
        trader_id: "bob".to_string(),
        side: "buy".to_string(),
        price: 49900,
        quantity: 3,
    };
    write
        .send(Message::Text(serde_json::to_string(&buy_order).unwrap()))
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 5. 下匹配订单触发成交
    println!("\n[5] 下买单触发成交 (价格: 50100, 数量: 2)...");
    let matching_order = ClientMessage::LimitOrder {
        trader_id: "charlie".to_string(),
        side: "buy".to_string(),
        price: 50100,
        quantity: 2,
    };
    write
        .send(Message::Text(
            serde_json::to_string(&matching_order).unwrap(),
        ))
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // 6. 下市价单
    println!("\n[6] 下市价买单 (数量: 1)...");
    let market_order = ClientMessage::MarketOrder {
        trader_id: "david".to_string(),
        side: "buy".to_string(),
        quantity: 1,
    };
    write
        .send(Message::Text(
            serde_json::to_string(&market_order).unwrap(),
        ))
        .await
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    println!("\n[演示完成，保持连接监听...]");
    println!("按 Ctrl+C 退出\n");

    // 等待接收任务
    recv_handle.await.unwrap();
}
