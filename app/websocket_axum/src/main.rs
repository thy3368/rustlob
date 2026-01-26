use tokio::sync::broadcast;
use crate::interfaces::spot_websocket::md_sse_controller::SpotMarketDataSSEImpl;
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::{
    SpotMarketDataStreamAny, AggregateTradeStream, TradeStream, MiniTickerStream,
    MarketDataSubscriptionCmdAny, SpotMarketDataSSEBehavior,
};
use serde_json::json;
use axum::{extract::WebSocketUpgrade, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use serde::Deserialize;

// 模块声明
pub mod interfaces {
    pub mod spot_websocket {
        pub mod md_sse_controller;
        pub mod ud_sse_controller;
    }

    pub mod usds_m_future_websocket {
        pub mod md_sse_controller;
        pub mod ud_sse_controller;
    }

    pub mod coin_m_future_websocket {
        pub mod md_sse_controller;
        pub mod ud_sse_controller;
    }

    pub mod option_websocket {
        pub mod md_sse_controller;
        pub mod ud_sse_controller;
    }
}

/// WebSocket 事件数据类型
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct WebSocketEvent {
    pub r#type: String,
    pub data: serde_json::Value
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub text: String
}

/// WebSocket 连接处理器
async fn websocket_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<WebSocketEvent>) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        println!("New WebSocket connection established");

        // 创建 SpotMarketDataSSEImpl 实例
        let mut market_data_sse = SpotMarketDataSSEImpl::new();

        // 发送欢迎消息
        let welcome_msg = json!({
            "type": "welcome",
            "message": "Hello from Axum WebSocket!"
        });
        if socket.send(axum::extract::ws::Message::Text(serde_json::to_string(&welcome_msg).unwrap())).await.is_err() {
            return;
        }

        // 订阅事件广播
        let mut rx = tx.subscribe();

        // 发送事件
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(msg) => {
                            let event_msg = json!({
                                "type": msg.r#type,
                                "data": msg.data
                            });
                            if socket.send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&event_msg).unwrap()
                            )).await.is_err() {
                                break;
                            }
                        },
                        Err(_) => break,
                    }
                },
                msg = socket.recv() => {
                    match msg {
                        Some(Ok(msg)) => match msg {
                            axum::extract::ws::Message::Text(text) => {
                                println!("Received message: {}", text);

                                // 尝试解析为 MarketDataSubscriptionCmdAny
                                if let Ok(cmd) = serde_json::from_str::<MarketDataSubscriptionCmdAny>(&text) {
                                    println!("Parsed MarketDataSubscriptionCmdAny: {:?}", cmd);

                                    // 处理订阅命令
                                    match market_data_sse.handle_subscription(cmd) {
                                        Ok(resp) => {
                                            println!("Subscription response: {:?}", resp);
                                            // 发送响应
                                            let resp_text = serde_json::to_string(&resp.result).unwrap();
                                            if socket.send(axum::extract::ws::Message::Text(resp_text)).await.is_err() {
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            println!("Subscription error: {:?}", e);
                                            // 发送错误响应
                                            let error_msg = json!({
                                                "type": "error",
                                                "message": format!("{}", e)
                                            });
                                            if socket.send(axum::extract::ws::Message::Text(
                                                serde_json::to_string(&error_msg).unwrap()
                                            )).await.is_err() {
                                                break;
                                            }
                                        }
                                    }
                                }
                            },
                            axum::extract::ws::Message::Close(_) => {
                                println!("Client closed the connection");
                                break;
                            },
                            _ => {},
                        },
                        _ => break,
                    }
                }
            }
        }

        println!("WebSocket connection closed");
    })
}

/// 创建包含 WebSocket 路由的 Axum 应用
fn create_app(tx: broadcast::Sender<WebSocketEvent>) -> Router {
    Router::new().route("/ws", get(move |ws| websocket_handler(ws, tx.clone()))).nest_service("/", ServeDir::new("."))
}

/// 启动 WebSocket 服务器
async fn start_server(port: u16, tx: broadcast::Sender<WebSocketEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app(tx);

    println!("WebSocket server starting on http://localhost:{}", port);
    println!("Open index.html in your browser to test");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建事件广播通道
    let (tx, _) = broadcast::channel(1024);

    // 启动服务器（在后台运行）
    let server_tx = tx.clone();

    // 发布 SpotMarketDataSSEImpl
    let market_data_sse = SpotMarketDataSSEImpl::new();
    println!("SpotMarketDataSSEImpl published successfully");

    // 模拟获取并推送 SpotMarketDataStreamAny 消息
    let mut market_data_sse_clone = market_data_sse.clone();
    let push_tx = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        let mut counter = 0;

        loop {
            interval.tick().await;
            counter += 1;

            // 模拟生成不同类型的市场数据消息
            let stream_msg: SpotMarketDataStreamAny = if counter % 3 == 0 {
                SpotMarketDataStreamAny::AggregateTrade(AggregateTradeStream {
                    event_type: "aggTrade".to_string(),
                    event_time: chrono::Utc::now().timestamp_millis(),
                    symbol: "BTCUSDT".to_string(),
                    agg_trade_id: counter as i64,
                    price: format!("{:.2}", 45000.0 + counter as f64 * 0.1),
                    quantity: format!("{:.4}", 0.001 + counter as f64 * 0.0001),
                    first_trade_id: counter as i64,
                    last_trade_id: counter as i64,
                    trade_time: chrono::Utc::now().timestamp_millis(),
                    is_buyer_maker: counter % 2 == 0,
                    ignore: false,
                })
            } else if counter % 3 == 1 {
                SpotMarketDataStreamAny::Trade(TradeStream {
                    event_type: "trade".to_string(),
                    event_time: chrono::Utc::now().timestamp_millis(),
                    symbol: "ETHUSDT".to_string(),
                    trade_id: counter as i64,
                    price: format!("{:.2}", 2500.0 + counter as f64 * 0.05),
                    quantity: format!("{:.4}", 0.01 + counter as f64 * 0.001),
                    trade_time: chrono::Utc::now().timestamp_millis(),
                    is_buyer_maker: counter % 2 == 1,
                    ignore: false,
                })
            } else {
                SpotMarketDataStreamAny::MiniTicker(MiniTickerStream {
                    event_type: "24hrMiniTicker".to_string(),
                    event_time: chrono::Utc::now().timestamp_millis(),
                    symbol: "ADAUSDT".to_string(),
                    close_price: format!("{:.4}", 0.45 + counter as f64 * 0.001),
                    open_price: format!("{:.4}", 0.44 + counter as f64 * 0.0005),
                    high_price: format!("{:.4}", 0.46 + counter as f64 * 0.0015),
                    low_price: format!("{:.4}", 0.43 + counter as f64 * 0.0005),
                    base_volume: format!("{:.0}", 1000000.0 + counter as f64 * 1000.0),
                    quote_volume: format!("{:.0}", 450000.0 + counter as f64 * 500.0),
                })
            };

            // 处理流数据
            if let Err(e) = market_data_sse_clone.handle_stream_data(stream_msg.clone()) {
                eprintln!("Error handling stream data: {}", e);
            }

            // 向 WebSocket 客户端推送消息
            let event = WebSocketEvent {
                r#type: "market_data".to_string(),
                data: json!(stream_msg),
            };

            let _ = push_tx.send(event);
        }
    });

    tokio::spawn(async move {
        if let Err(e) = start_server(8083, server_tx).await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    println!("WebSocket server started. Press Ctrl+C to exit.");

    // 等待用户中断
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}
