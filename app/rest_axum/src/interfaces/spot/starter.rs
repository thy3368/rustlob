use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing_subscriber;

use crate::interfaces::spot::http::{
    trade_controller::TradeService,
    trade_v2_controller::TradeV2Service,
    md_controller::MarketDataService,
    ud_controller::UserDataService,
};
use crate::interfaces::spot::http::{
    trade_controller,
    trade_v2_controller,
    md_controller,
    ud_controller,
};
use crate::interfaces::spot::websocket::md_sse_controller::SpotMarketDataSSEImpl;
use crate::interfaces::spot::websocket::spot_market_data_pusher;
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::SpotMarketDataStreamAny;

/// Spot 模块启动器
pub struct SpotStarter;

impl SpotStarter {
    /// 启动 Spot 模块的 HTTP 和 WebSocket 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting Spot module...");
        println!("⚠️  Running in MOCK mode (no database connection)");

        // ==================== HTTP 服务器启动 ====================
        println!("📡 Starting Spot HTTP API server...");
        Self::start_http_server().await?;

        // ==================== WebSocket 服务器启动 ====================
        println!("🔌 Starting Spot WebSocket server...");

        // 创建事件广播通道
        let (tx, _) = broadcast::channel(1024);

        // 发布 SpotMarketDataSSEImpl
        let market_data_sse = SpotMarketDataSSEImpl::new();
        println!("SpotMarketDataSSEImpl published successfully");

        // 启动 SpotMarketDataPusher
        let pusher = spot_market_data_pusher::SpotMarketDataPusher::new(tx.clone())
            .with_interval(5); // 每5秒推送一次
        pusher.start();
        println!("SpotMarketDataPusher started successfully");

        // 创建 WebSocket 应用
        let ws_app = Self::create_websocket_app(tx.clone());

        //todo 下面代码移到 create_websocket_app里面？
        // 启动 WebSocket 服务器（在后台运行）
        let ws_listener = tokio::net::TcpListener::bind("0.0.0.0:8084").await?;
        println!("🚀 Spot WebSocket server started at ws://localhost:8084/ws");

        tokio::spawn(async move {
            axum::serve(ws_listener, ws_app).await.expect("Spot WebSocket server failed to start");
        });

        println!("✅ Spot module started successfully");

        Ok(())
    }

    /// 创建 WebSocket 应用
    fn create_websocket_app(tx: broadcast::Sender<SpotMarketDataStreamAny>) -> Router {
        use axum::extract::WebSocketUpgrade;
        use axum::routing::get;
        use serde_json::json;
        use tower_http::services::ServeDir;

        // WebSocket 连接处理器
        //todo user data怎么处理？
        async fn websocket_handler(
            ws: WebSocketUpgrade,
            tx: broadcast::Sender<SpotMarketDataStreamAny>,
        ) -> impl IntoResponse {
            ws.on_upgrade(|mut socket| async move {
                println!("New Spot WebSocket connection established");

                // 创建 SpotMarketDataSSEImpl 实例
                let mut market_data_sse = SpotMarketDataSSEImpl::new();

                // 发送欢迎消息
                let welcome_msg = json!({
                    "type": "welcome",
                    "message": "Hello from Spot WebSocket!"
                });
                if socket.send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&welcome_msg).unwrap()
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
                                        println!("Received Spot WebSocket message: {}", text);

                                        //todo market_data_sse处理订阅
                                        // 这里可以添加消息处理逻辑
                                        // 例如解析 MarketDataSubscriptionCmdAny 等
                                    },
                                    axum::extract::ws::Message::Close(_) => {
                                        println!("Spot WebSocket client closed the connection");
                                        break;
                                    },
                                    _ => {},
                                },
                                _ => break,
                            }
                        }
                    }
                }

                println!("Spot WebSocket connection closed");
            })
        }

        Router::new()
            .route("/ws", get(move |ws| websocket_handler(ws, tx.clone())))
            .nest_service("/", ServeDir::new("."))
    }

    /// 启动 HTTP 服务器
    async fn start_http_server() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）
        let trade_service = Arc::new(TradeService::new());
        let trade_v2_service = Arc::new(TradeV2Service::new());
        let market_data_service = Arc::new(MarketDataService::new());
        let user_data_service = Arc::new(UserDataService::new());

        // 创建路由，注入服务依赖
        let order_routes = Router::new()
            .route("/api/spot/order/", post(trade_controller::handle))
            .with_state(trade_service);

        let trade_v2_routes = Router::new()
            .route("/api/spot/trade/v2/", post(trade_v2_controller::handle))
            .with_state(trade_v2_service);

        let market_data_routes = Router::new()
            .route("/api/spot/market/data", post(md_controller::handle))
            .with_state(market_data_service);

        let user_data_routes = Router::new()
            .route("/api/spot/user/data", post(ud_controller::handle))
            .with_state(user_data_service);

        let http_app = Router::new()
            .route("/api/spot/health", get(Self::health_check))
            .nest("/", order_routes)
            .nest("/", trade_v2_routes)
            .nest("/", market_data_routes)
            .nest("/", user_data_routes);

        // 启动 HTTP 服务器（在后台运行）
        let http_listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
        println!("🚀 Spot HTTP server started at http://localhost:3001");
        println!("📊 Spot health check: GET /api/spot/health");
        println!("💹 Spot trade: POST /api/spot/order/ (JSON)");
        println!("💹 Spot trade v2: POST /api/spot/trade/v2/ (JSON)");
        println!("📈 Spot market data: POST /api/spot/market/data (JSON)");
        println!("👤 Spot user data: POST /api/spot/user/data (JSON)");

        tokio::spawn(async move {
            axum::serve(http_listener, http_app).await.expect("Spot HTTP server failed to start");
        });

        Ok(())
    }

    /// 健康检查
    async fn health_check() -> &'static str {
        "OK"
    }
}

/// 便捷函数：启动 Spot 模块
pub async fn start_spot_module() -> Result<(), Box<dyn std::error::Error>> {
    SpotStarter::start().await
}
