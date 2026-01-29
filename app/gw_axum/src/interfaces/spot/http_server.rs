use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router
};
use base_types::{
    account::balance::Balance,
    exchange::spot::spot_types::{SpotOrder, SpotTrade}
};
use db_repo::MySqlDbRepo;
use lob_repo::adapter::{distributed_lob_repo::DistributedLobRepo, embedded_lob_repo::EmbeddedLobRepo};
use spot_behavior::proc::{
    behavior::v2::{
        spot_market_data_behavior::{SpotMarketDataCmdAny, SpotMarketDataResAny},
        spot_trade_behavior_v2::{SpotTradeCmdAny, SpotTradeResAny},
        spot_user_data_behavior::{SpotUserDataCmdAny, SpotUserDataResAny}
    },
    v2::{
        spot_market_data::SpotMarketDataImpl, spot_trade_v2::SpotTradeBehaviorV2Impl, spot_user_data::SpotUserDataImpl
    }
};

use crate::interfaces::{
    common::http_handler_util::handle_generic,
    spot::http::{trade_handler, trade_handler::TradeService}
};

/// HTTP 服务器启动器
pub struct HttpServer;
impl HttpServer {
    pub async fn start_4_ds() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）
        let trade_service = Arc::new(TradeService::new());

        // 初始化 SpotTradeBehaviorV2Impl - 使用 mock 数据库和 EmbeddedLobRepo

        let (balance_repo, trade_repo, order_repo, user_data_repo, market_data_repo) = (
            MySqlDbRepo::<Balance>::new_mock(),
            MySqlDbRepo::<SpotTrade>::new_mock(),
            MySqlDbRepo::<SpotOrder>::new_mock(),
            MySqlDbRepo::<SpotOrder>::new_mock(),
            MySqlDbRepo::<SpotOrder>::new_mock()
        );


        let ds_lob_repo = DistributedLobRepo::<SpotOrder>::new(vec![]);
        let trade_behavior = SpotTradeBehaviorV2Impl::new(
            balance_repo,
            trade_repo,
            order_repo,
            user_data_repo,
            market_data_repo,
            ds_lob_repo
        );


        let trade_v2_service = Arc::new(trade_behavior);
        let market_data_service = Arc::new(SpotMarketDataImpl::new());
        let user_data_service = Arc::new(SpotUserDataImpl::new());

        // 创建路由，注入服务依赖
        let order_routes =
            Router::new().route("/api/spot/order/", post(trade_handler::handle)).with_state(trade_service);


        let trade_v2_routes = Router::new()
            .route(
                "/api/spot/v2/",
                post(
                    handle_generic::<
                        SpotTradeBehaviorV2Impl<DistributedLobRepo<SpotOrder>>,
                        SpotTradeCmdAny,
                        SpotTradeResAny
                    >
                )
            )
            .with_state(trade_v2_service);

        let market_data_routes = Router::new()
            .route(
                "/api/spot/market/data",
                post(handle_generic::<SpotMarketDataImpl, SpotMarketDataCmdAny, SpotMarketDataResAny>)
            )
            .with_state(market_data_service);

        let user_data_routes = Router::new()
            .route(
                "/api/spot/user/data",
                post(handle_generic::<SpotUserDataImpl, SpotUserDataCmdAny, SpotUserDataResAny>)
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
            axum::serve(http_listener, http_app.into_make_service()).await.expect("Spot HTTP server failed to start");
        });

        Ok(())
    }
}


impl HttpServer {
    /// 启动 Spot HTTP 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // 创建应用服务（单例，全局共享）
        let trade_service = Arc::new(TradeService::new());

        // 初始化 SpotTradeBehaviorV2Impl - 使用 mock 数据库和 EmbeddedLobRepo

        let (balance_repo, trade_repo, order_repo, user_data_repo, market_data_repo) = (
            MySqlDbRepo::<Balance>::new_mock(),
            MySqlDbRepo::<SpotTrade>::new_mock(),
            MySqlDbRepo::<SpotOrder>::new_mock(),
            MySqlDbRepo::<SpotOrder>::new_mock(),
            MySqlDbRepo::<SpotOrder>::new_mock()
        );


        let lob_repo = EmbeddedLobRepo::<SpotOrder>::new(vec![]);

        let trade_behavior = SpotTradeBehaviorV2Impl::new(
            balance_repo,
            trade_repo,
            order_repo,
            user_data_repo,
            market_data_repo,
            lob_repo
        );


        let trade_v2_service = Arc::new(trade_behavior);
        let market_data_service = Arc::new(SpotMarketDataImpl::new());
        let user_data_service = Arc::new(SpotUserDataImpl::new());

        // 创建路由，注入服务依赖
        let order_routes =
            Router::new().route("/api/spot/order/", post(trade_handler::handle)).with_state(trade_service);


        let trade_v2_routes = Router::new()
            .route(
                "/api/spot/v1/v2/",
                post(
                    handle_generic::<
                        SpotTradeBehaviorV2Impl<EmbeddedLobRepo<SpotOrder>>,
                        SpotTradeCmdAny,
                        SpotTradeResAny
                    >
                )
            )
            .with_state(trade_v2_service);

        let market_data_routes = Router::new()
            .route(
                "/api/spot/market/data",
                post(handle_generic::<SpotMarketDataImpl, SpotMarketDataCmdAny, SpotMarketDataResAny>)
            )
            .with_state(market_data_service);

        let user_data_routes = Router::new()
            .route(
                "/api/spot/user/data",
                post(handle_generic::<SpotUserDataImpl, SpotUserDataCmdAny, SpotUserDataResAny>)
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
            axum::serve(http_listener, http_app.into_make_service()).await.expect("Spot HTTP server failed to start");
        });

        Ok(())
    }

    /// 健康检查
    pub async fn health_check() -> &'static str { "OK" }
}
