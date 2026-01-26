
// HTTP 接口层
pub mod interfaces {
    pub mod spot_http {
        pub mod trade_controller;
        pub mod trade_v2_controller;
        pub mod md_controller;
        pub mod ud_controller;
    }

    pub mod usds_m_future_http {
        pub mod trade_controller;

        pub mod md_controller;
        pub mod ud_controller;
    }

    pub mod coin_m_future_http {
        pub mod trade_controller;

        pub mod md_controller;
        pub mod ud_controller;
    }

    pub mod option_http {
        pub mod trade_controller;

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
    let trade_service = Arc::new(interfaces::spot_http::trade_controller::TradeService::new());
    let trade_v2_service = Arc::new(interfaces::spot_http::trade_v2_controller::TradeV2Service::new());
    let market_data_service = Arc::new(interfaces::spot_http::md_controller::MarketDataService::new());
    let user_data_service = Arc::new(interfaces::spot_http::ud_controller::UserDataService::new());

    // USDS-M期货服务
    let usds_m_future_trade_service = Arc::new(interfaces::usds_m_future_http::trade_controller::TradeService::new());
    let usds_m_future_md_service = Arc::new(interfaces::usds_m_future_http::md_controller::MarketDataService::new());
    let usds_m_future_ud_service = Arc::new(interfaces::usds_m_future_http::ud_controller::UserDataService::new());

    // 创建路由，注入服务依赖
    let order_routes = Router::new()
        .route("/api/spot/order/", post(interfaces::spot_http::trade_controller::handle))
        .with_state(trade_service);

    let trade_v2_routes = Router::new()
        .route("/api/spot/trade/v2/", post(interfaces::spot_http::trade_v2_controller::handle))
        .with_state(trade_v2_service);

    let market_data_routes = Router::new()
        .route("/api/spot/market/data", post(interfaces::spot_http::md_controller::handle))
        .with_state(market_data_service);

    let user_data_routes = Router::new()
        .route("/api/spot/user/data", post(interfaces::spot_http::ud_controller::handle))
        .with_state(user_data_service);

    // USDS-M期货路由
    let usds_m_future_trade_routes = Router::new()
        .route("/api/usds-m-future/order/", post(interfaces::usds_m_future_http::trade_controller::handle))
        .with_state(usds_m_future_trade_service);

    let usds_m_future_md_routes = Router::new()
        .route("/api/usds-m-future/market/data", post(interfaces::usds_m_future_http::md_controller::handle))
        .with_state(usds_m_future_md_service);

    let usds_m_future_ud_routes = Router::new()
        .route("/api/usds-m-future/user/data", post(interfaces::usds_m_future_http::ud_controller::handle))
        .with_state(usds_m_future_ud_service);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/", order_routes)
        .nest("/", trade_v2_routes)
        .nest("/", market_data_routes)
        .nest("/", user_data_routes)
        .nest("/", usds_m_future_trade_routes)
        .nest("/", usds_m_future_md_routes)
        .nest("/", usds_m_future_ud_routes);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind port");

    println!("🚀 Server started at http://localhost:3000");
    println!("📊 Health check: GET /health");
    println!("💹 Spot trade: POST /api/spot/order/ (JSON)");
    println!("💹 Spot trade v2: POST /api/spot/trade/v2/ (JSON)");
    println!("📈 Spot market data: POST /api/spot/market/data (JSON)");
    println!("👤 Spot user data: POST /api/spot/user/data (JSON)");
    println!("📉 USDS-M Future trade: POST /api/usds-m-future/order/ (JSON)");
    println!("📈 USDS-M Future market data: POST /api/usds-m-future/market/data (JSON)");
    println!("👤 USDS-M Future user data: POST /api/usds-m-future/user/data (JSON)");

    axum::serve(listener, app).await.expect("Server failed to start");
}

async fn health_check() -> &'static str {
    "OK"
}
