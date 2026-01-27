use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router
};
use spot_behavior::proc::trade_v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

use crate::interfaces::spot::http::{
    md_handler, md_handler::MarketDataService, trade_handler, trade_handler::TradeService,
    trade_v2_controller, ud_handler, ud_handler::UserDataService
};

/// HTTP 服务器启动器
pub struct HttpServer;

impl HttpServer {
    /// 启动 Spot HTTP 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）
        let trade_service = Arc::new(TradeService::new());
        let trade_v2_service = Arc::new(SpotTradeBehaviorV2Impl::new());
        let market_data_service = Arc::new(MarketDataService::new());
        let user_data_service = Arc::new(UserDataService::new());

        // 创建路由，注入服务依赖
        let order_routes =
            Router::new().route("/api/spot/order/", post(trade_handler::handle)).with_state(trade_service);

        let trade_v2_routes =
            Router::new().route("/api/spot/trade/v2/", post(trade_v2_controller::handle)).with_state(trade_v2_service);

        let market_data_routes =
            Router::new().route("/api/spot/market/data", post(md_handler::handle)).with_state(market_data_service);

        let user_data_routes =
            Router::new().route("/api/spot/user/data", post(ud_handler::handle)).with_state(user_data_service);

        let http_app = Router::new()
            .route("/api/spot/health", get(Self::health_check))
            .merge(order_routes)
            .merge(trade_v2_routes)
            .merge(market_data_routes)
            .merge(user_data_routes);

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
    pub async fn health_check() -> &'static str { "OK" }
}
