use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use base_types::account::balance::Balance;
use base_types::actor_x::ActorX;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use db_repo::MySqlDbRepo;
use immutable_derive::immutable;
use lob_repo::adapter::distributed_lob_repo::DistributedLobRepo;
use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
use spot_behavior::proc::behavior::v2::spot_market_data_behavior::{
    SpotMarketDataCmdAny, SpotMarketDataResAny,
};
use spot_behavior::proc::behavior::v2::spot_trade_behavior_v2::{SpotTradeCmdAny, SpotTradeResAny};
use spot_behavior::proc::behavior::v2::spot_user_data_behavior::{
    SpotUserDataCmdAny, SpotUserDataResAny,
};
use spot_behavior::proc::behavior::v2::spot_user_data_sse_behavior::{
    SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny,
};
use spot_behavior::proc::v2::spot_market_data::SpotMarketDataImpl;
use spot_behavior::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;
use spot_behavior::proc::v2::spot_user_data::SpotUserDataImpl;
use spot_behavior::proc::v2::spot_user_data_key::SpotUserDataListenKeyImpl;

use crate::interfaces::common::http_handler_util::handle_generic;
use crate::interfaces::common::ins_repo;
use crate::interfaces::spot::http::trade_handler;
use crate::interfaces::spot::http::trade_handler::TradeService;

// todo 认证： /api/spot/v2/；/api/spot/user/data
// todo 不认证： /api/spot/v2/；/api/spot/market/data
/// HTTP 服务器启动器

#[immutable]
pub struct HttpServer {}
impl HttpServer {
    pub async fn start_4_ds() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）- TradeService 依赖于 HTTP 框架，无法在 spot_behavior 中实例化
        let trade_service = Arc::new(TradeService::new());

        // 使用 id_repo 中的单例服务
        let trade_v2_service = ins_repo::get_spot_trade_behavior_v2_distributed();
        let market_data_service = ins_repo::get_spot_market_data_service();
        let user_data_service = ins_repo::get_spot_user_data_service();
        let listen_key_service = ins_repo::get_spot_user_data_listen_key_service();

        // 创建路由，注入服务依赖
        let order_routes = Router::new()
            .route("/api/spot/order/", post(trade_handler::handle))
            .with_state(trade_service);

        let trade_v2_routes = Router::new()
            .route(
                "/api/spot/v2/",
                post(
                    handle_generic::<
                        SpotTradeBehaviorV2Impl<Arc<DistributedLobRepo<SpotOrder>>>,
                        SpotTradeCmdAny,
                        SpotTradeResAny,
                    >,
                ),
            )
            .with_state(trade_v2_service);

        let market_data_routes =
            Router::new()
                .route(
                    "/api/spot/market/data",
                    post(
                        handle_generic::<
                            SpotMarketDataImpl,
                            SpotMarketDataCmdAny,
                            SpotMarketDataResAny,
                        >,
                    ),
                )
                .with_state(market_data_service);

        let user_data_routes = Router::new()
            .route(
                "/api/spot/user/data",
                post(handle_generic::<SpotUserDataImpl, SpotUserDataCmdAny, SpotUserDataResAny>),
            )
            .with_state(user_data_service);

        let user_key_routes = Router::new()
            .route(
                "/api/spot/user/listen_key",
                post(
                    handle_generic::<
                        SpotUserDataListenKeyImpl,
                        SpotUserDataListenKeyCmdAny,
                        SpotUserDataListenKeyResAny,
                    >,
                ),
            )
            .with_state(listen_key_service);

        let http_app = Router::new()
            .route("/api/spot/health", get(Self::health_check))
            .merge(order_routes)
            .merge(trade_v2_routes)
            .merge(market_data_routes)
            .merge(user_data_routes)
            .merge(user_key_routes);

        // 启动 HTTP 服务器（在后台运行）
        let http_listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
        tracing::info!("🚀 Spot HTTP server started at http://localhost:3001");
        tracing::info!("📊 Spot health check: GET /api/spot/health");
        tracing::info!("💹 Spot v1: POST /api/spot/order/ (JSON)");
        tracing::info!("💹 Spot v1 v2: POST /api/spot/v1/v2/ (JSON)");
        tracing::info!("📈 Spot market data: POST /api/spot/market/data (JSON)");
        tracing::info!("👤 Spot user data: POST /api/spot/user/data (JSON)");

        tokio::spawn(async move {
            axum::serve(http_listener, http_app.into_make_service())
                .await
                .expect("Spot HTTP server failed to start");
        });

        Ok(())
    }
}

impl HttpServer {
    /// 启动 Spot HTTP 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）- TradeService 依赖于 HTTP 框架，无法在 spot_behavior 中实例化
        let trade_service = Arc::new(TradeService::new());

        // 使用 id_repo 中的单例服务
        let trade_v2_service = ins_repo::get_spot_trade_behavior_v2_embedded();
        let market_data_service = ins_repo::get_spot_market_data_service();
        let user_data_service = ins_repo::get_spot_user_data_service();

        // 创建路由，注入服务依赖
        let order_routes = Router::new()
            .route("/api/spot/order/", post(trade_handler::handle))
            .with_state(trade_service);

        let trade_v2_routes = Router::new()
            .route(
                "/api/spot/v2/",
                post(
                    handle_generic::<
                        SpotTradeBehaviorV2Impl<Arc<EmbeddedLobRepo<SpotOrder>>>,
                        SpotTradeCmdAny,
                        SpotTradeResAny,
                    >,
                ),
            )
            .with_state(trade_v2_service);

        let market_data_routes =
            Router::new()
                .route(
                    "/api/spot/market/data",
                    post(
                        handle_generic::<
                            SpotMarketDataImpl,
                            SpotMarketDataCmdAny,
                            SpotMarketDataResAny,
                        >,
                    ),
                )
                .with_state(market_data_service);

        let user_data_routes = Router::new()
            .route(
                "/api/spot/user/data",
                post(handle_generic::<SpotUserDataImpl, SpotUserDataCmdAny, SpotUserDataResAny>),
            )
            .with_state(user_data_service);

        let http_app = Router::new()
            .route("/api/spot/health", get(Self::health_check))
            .merge(order_routes)
            .merge(trade_v2_routes)
            .merge(market_data_routes)
            .merge(user_data_routes);

        // 启动 HTTP 服务器（在后台运行）
        let http_listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
        tracing::info!("🚀 Spot HTTP server started at http://localhost:3001");
        tracing::info!("📊 Spot health check: GET /api/spot/health");
        tracing::info!("💹 Spot v1: POST /api/spot/order/ (JSON)");
        tracing::info!("💹 Spot v1 v2: POST /api/spot/v1/v2/ (JSON)");
        tracing::info!("📈 Spot market data: POST /api/spot/market/data (JSON)");
        tracing::info!("👤 Spot user data: POST /api/spot/user/data (JSON)");

        tokio::spawn(async move {
            axum::serve(http_listener, http_app.into_make_service())
                .await
                .expect("Spot HTTP server failed to start");
        });

        // 启动 K 线服务
        let kline_service = ins_repo::get_k_line_service();
        kline_service.start();
        tracing::info!("✅ K-Line service started");

        // 启动 Push 服务
        let push_service = ins_repo::get_push_service();
        push_service.start();
        tracing::info!("✅ Push service started");

        Ok(())
    }

    /// 健康检查
    pub async fn health_check() -> &'static str {
        "OK"
    }
}
