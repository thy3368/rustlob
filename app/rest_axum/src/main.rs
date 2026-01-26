pub mod md_gw;
pub mod trade_gw;
pub mod ud_gw;

pub mod spot_gw;

// HTTP 接口层
pub mod interfaces {
    pub mod http {
        pub mod trade_controller;
        pub mod trade_v2_controller;
        pub mod md_controller;
        pub mod ud_controller;

    }
}

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
#[hotpath::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("🚀 Starting REST API server...");
    println!("⚠️  Running in MOCK mode (no database connection)");

    // 创建应用服务（单例，全局共享）
    let trade_service = Arc::new(interfaces::http::trade_controller::TradeService::new());
    let market_data_service = Arc::new(interfaces::http::md_controller::MarketDataService::new());

    // 创建路由，注入服务依赖
    let order_routes = Router::new()
        .route("/api/spot/order/", post(interfaces::http::trade_controller::handle))
        .with_state(trade_service);

    let market_data_routes = Router::new()
        .route("/api/spot/market/data", post(interfaces::http::md_controller::handle))
        .with_state(market_data_service);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/", order_routes)
        .nest("/", market_data_routes);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind port");

    println!("🚀 Server started at http://localhost:3000");
    println!("📊 Health check: GET /health");
    println!("💹 Spot trade: POST /api/spot/order/ (JSON)");
    println!("📈 Spot market data: POST /api/spot/market/data (JSON)");

    axum::serve(listener, app).await.expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "OK"
}
