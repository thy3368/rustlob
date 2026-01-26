use axum::{extract::WebSocketUpgrade, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use serde_json::json;
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::{
    MarketDataSubscriptionCmdAny, SpotMarketDataSSEBehavior, SpotMarketDataStreamAny
};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::services::ServeDir;
use rest_axum::interfaces::spot::websocket::spot_market_data_pusher;
use rest_axum::interfaces::spot::websocket::md_sse_controller::SpotMarketDataSSEImpl;

// 模块声明
pub mod domain {}

pub mod interfaces {
    pub mod spot_websocket {}

    pub mod usds_m_future_websocket {}

    pub mod coin_m_future_websocket {
        pub mod md_sse_controller;
        pub mod ud_sse_controller;
    }

    pub mod option_websocket {
        pub mod md_sse_controller;
        pub mod ud_sse_controller;
    }
}



/// WebSocket 连接处理器
async fn websocket_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<SpotMarketDataStreamAny>) -> impl IntoResponse {
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

                            if socket.send(axum::extract::ws::Message::Text(
                                serde_json::to_string(&msg).unwrap()
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
fn create_app(tx: broadcast::Sender<SpotMarketDataStreamAny>) -> Router {
    Router::new().route("/ws", get(move |ws| websocket_handler(ws, tx.clone()))).nest_service("/", ServeDir::new("."))
}

/// 启动 WebSocket 服务器
async fn start_server(
    port: u16, tx: broadcast::Sender<SpotMarketDataStreamAny>
) -> Result<(), Box<dyn std::error::Error>> {
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

    // 启动 SpotMarketDataPusher
    let pusher = spot_market_data_pusher::SpotMarketDataPusher::new(tx.clone()).with_interval(5); // 每5秒推送一次
    pusher.start();
    println!("SpotMarketDataPusher started successfully");

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
