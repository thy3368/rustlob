use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router
};
use spot_behavior::proc::trade_v2::{
    spot_market_data::SpotMarketDataImpl, spot_trade_v2::SpotTradeBehaviorV2Impl, spot_user_data::SpotUserDataImpl
};

use crate::interfaces::spot::http::{
    md_handler, trade_handler, trade_handler::TradeService, trade_v2_handler, ud_handler
};

/// HTTP 服务器启动器
pub struct HttpServer;

impl HttpServer {
    /// 启动 Spot HTTP 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）
        let trade_service = Arc::new(TradeService::new());
        let trade_v2_service = Arc::new(SpotTradeBehaviorV2Impl::new());
        let market_data_service = Arc::new(SpotMarketDataImpl::new());
        let user_data_service = Arc::new(SpotUserDataImpl::new());

        // 创建路由，注入服务依赖
        let order_routes =
            Router::new().route("/api/spot/order/", post(trade_handler::handle)).with_state(trade_service);

        let trade_v2_routes =
            Router::new().route("/api/spot/trade/v2/", post(trade_v2_handler::handle)).with_state(trade_v2_service);

        let market_data_routes =
            Router::new().route("/api/spot/market/data", post(md_handler::handle)).with_state(market_data_service);

        let user_data_routes =
            Router::new().route("/api/spot/user/data", post(ud_handler::handle)).with_state(user_data_service);

        use axum::extract::connect_info::MockConnectInfo;
        use std::net::SocketAddr;

        let http_app = Router::new()
            .route("/api/spot/health", get(Self::health_check))
            .merge(order_routes)
            .merge(trade_v2_routes)
            .merge(market_data_routes)
            .merge(user_data_routes)
            .layer(MockConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080))));

        // 启动 HTTP 服务器（在后台运行）
        let http_listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
        tracing::info!("🚀 Spot HTTP server started at http://localhost:3001");
        tracing::info!("📊 Spot health check: GET /api/spot/health");
        tracing::info!("💹 Spot trade: POST /api/spot/order/ (JSON)");
        tracing::info!("💹 Spot trade v2: POST /api/spot/trade/v2/ (JSON)");
        tracing::info!("📈 Spot market data: POST /api/spot/market/data (JSON)");
        tracing::info!("👤 Spot user data: POST /api/spot/user/data (JSON)");

        tokio::spawn(async move {
            let make_service = http_app.into_make_service_with_connect_info::<std::net::SocketAddr>();
            axum::serve(http_listener, make_service).await.expect("Spot HTTP server failed to start");
        });

        Ok(())
    }

    /// 健康检查
    pub async fn health_check() -> &'static str { "OK" }
}
