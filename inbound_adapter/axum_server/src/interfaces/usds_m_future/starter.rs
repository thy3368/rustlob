use std::sync::Arc;

use axum::Router;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use tokio::sync::broadcast;
use tracing_subscriber;

use crate::interfaces::usds_m_future::http::md_controller::MarketDataService;
use crate::interfaces::usds_m_future::http::trade_controller::TradeService;
use crate::interfaces::usds_m_future::http::ud_controller::UserDataService;
use crate::interfaces::usds_m_future::http::{md_controller, trade_controller, ud_controller};
use crate::interfaces::usds_m_future::websocket::md_sse_controller::UsdsMFutureMarketDataSSEImpl;
use crate::interfaces::usds_m_future::websocket::ud_sse_controller::UsdsMFutureUserDataSSEImpl;

/// USDS-M Future 模块启动器
pub struct UsdsMFutureStarter;

impl UsdsMFutureStarter {
    /// 启动 USDS-M Future 模块的 HTTP 和 WebSocket 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting USDS-M Future module...");
        println!("⚠️  Running in MOCK mode (no database connection)");

        // ==================== HTTP 服务器启动 ====================
        println!("📡 Starting USDS-M Future HTTP API server...");

        // 创建应用服务（单例，全局共享）
        let trade_service = Arc::new(TradeService::new());
        let market_data_service = Arc::new(MarketDataService::new());
        let user_data_service = Arc::new(UserDataService::new());

        // 创建路由，注入服务依赖
        let order_routes = Router::new()
            .route("/api/usds-m-future/order/", post(trade_controller::handle))
            .with_state(trade_service);

        let market_data_routes = Router::new()
            .route("/api/usds-m-future/market/data", post(md_controller::handle))
            .with_state(market_data_service);

        let user_data_routes = Router::new()
            .route("/api/usds-m-future/user/data", post(ud_controller::handle))
            .with_state(user_data_service);

        let http_app = Router::new()
            .route("/api/usds-m-future/health", get(Self::health_check))
            .nest("/", order_routes)
            .nest("/", market_data_routes)
            .nest("/", user_data_routes);

        // 启动 HTTP 服务器（在后台运行）
        let http_listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
        println!("🚀 USDS-M Future HTTP server started at http://localhost:3002");
        println!("📊 USDS-M Future health check: GET /api/usds-m-future/health");
        println!("💹 USDS-M Future v1: POST /api/usds-m-future/order/ (JSON)");
        println!("📈 USDS-M Future market data: POST /api/usds-m-future/market/data (JSON)");
        println!("👤 USDS-M Future user data: POST /api/usds-m-future/user/data (JSON)");

        tokio::spawn(async move {
            axum::serve(http_listener, http_app)
                .await
                .expect("USDS-M Future HTTP server failed to start");
        });

        // ==================== WebSocket 服务器启动 ====================
        println!("🔌 Starting USDS-M Future WebSocket server...");

        // 创建事件广播通道
        let (tx, _) = broadcast::channel(1024);

        // 发布 MarketDataSSEImpl
        let market_data_sse = UsdsMFutureMarketDataSSEImpl::new();
        println!("UsdsMFutureMarketDataSSEImpl published successfully");

        // 发布 UserDataSSEImpl
        let user_data_sse = UsdsMFutureUserDataSSEImpl::new();
        println!("UsdsMFutureUserDataSSEImpl published successfully");

        // 创建 WebSocket 应用
        let ws_app = Self::create_websocket_app(tx.clone());

        // 启动 WebSocket 服务器（在后台运行）
        let ws_listener = tokio::net::TcpListener::bind("0.0.0.0:8085").await?;
        println!("🚀 USDS-M Future WebSocket server started at ws://localhost:8085/ws");

        tokio::spawn(async move {
            axum::serve(ws_listener, ws_app)
                .await
                .expect("USDS-M Future WebSocket server failed to start");
        });

        println!("✅ USDS-M Future module started successfully");

        Ok(())
    }

    /// 创建 WebSocket 应用
    fn create_websocket_app(tx: broadcast::Sender<()>) -> Router {
        use axum::extract::WebSocketUpgrade;
        use axum::routing::get;
        use serde_json::json;
        use tower_http::services::ServeDir;

        // WebSocket 连接处理器
        async fn websocket_handler(
            ws: WebSocketUpgrade,
            tx: broadcast::Sender<()>,
        ) -> impl IntoResponse {
            ws.on_upgrade(|mut socket| async move {
                println!("New USDS-M Future WebSocket connection established");

                // 创建 MarketDataSSEImpl 和 UserDataSSEImpl 实例
                let mut market_data_sse = UsdsMFutureMarketDataSSEImpl::new();
                let mut user_data_sse = UsdsMFutureUserDataSSEImpl::new();

                // 发送欢迎消息
                let welcome_msg = json!({
                    "type": "welcome",
                    "message": "Hello from USDS-M Future WebSocket!"
                });
                if socket.send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&welcome_msg).unwrap().into()
                )).await.is_err() {
                    return;
                }

                // 订阅事件广播
                let mut rx = tx.subscribe();

                // 发送事件
                loop {
                    tokio::select! {
                        msg = rx.recv() => {
                            match msg {
                                Ok(_) => {
                                    // 这里可以添加事件处理逻辑
                                },
                                Err(_) => break,
                            }
                        },
                        msg = socket.recv() => {
                            match msg {
                                Some(Ok(msg)) => match msg {
                                    axum::extract::ws::Message::Text(text) => {
                                        println!("Received USDS-M Future WebSocket message: {}", text);

                                        // 这里可以添加消息处理逻辑
                                    },
                                    axum::extract::ws::Message::Close(_) => {
                                        println!("USDS-M Future WebSocket client closed the connection");
                                        break;
                                    },
                                    _ => {},
                                },
                                _ => break,
                            }
                        }
                    }
                }

                println!("USDS-M Future WebSocket connection closed");
            })
        }

        Router::new()
            .route("/ws", get(move |ws| websocket_handler(ws, tx.clone())))
            .nest_service("/", ServeDir::new("../../../../.."))
    }

    /// 健康检查
    async fn health_check() -> &'static str {
        "OK"
    }
}

/// 便捷函数：启动 USDS-M Future 模块
pub async fn start_usds_m_future_module() -> Result<(), Box<dyn std::error::Error>> {
    UsdsMFutureStarter::start().await
}
