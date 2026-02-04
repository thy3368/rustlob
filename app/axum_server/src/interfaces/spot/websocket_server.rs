use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{routing::get, Router};
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::SpotMarketDataStreamAny;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::interfaces::{
    common::ins_repo,
    spot::websocket::{
        md_sse_controller::SpotMarketDataSSEImpl, spot_user_data_pusher, ud_sse_controller::SpotUserDataSSEImpl,
        user_data_ws_handler::user_data_websocket_handler
    }
};

/// WebSocket 服务器启动器
// #[stateless]
pub struct WebSocketServer;

impl WebSocketServer {
    /// 启动 Spot WebSocket 服务器
    ///
    /// todo 用tracing打日志
    pub async fn start(md_tx: broadcast::Sender<SpotMarketDataStreamAny>) -> Result<(), Box<dyn std::error::Error>> {
        // 使用 id_repo 中的单例服务
        let connection_repo = ins_repo::get_connection_repo();
        let push_service = ins_repo::get_push_service();
        let sub_service = ins_repo::get_subscription_service();

        // 使用 id_repo 中的单例服务
        let _market_data_sse = ins_repo::get_spot_market_data_sse_impl();
        tracing::info!("SpotMarketDataSSEImpl published successfully");

        // 使用 id_repo 中的单例服务
        let _user_data_sse = ins_repo::get_spot_user_data_sse_impl();
        tracing::info!("SpotUserDataSSEImpl published successfully");


        // 使用 100ms 轮询间隔启动后台任务
        //todo fix this
        push_service.start();

        tracing::info!("SubscriptionService started successfully");


        let ws_app = Router::new()
            .route(
                "/ws/user_data",
                get(move |ws, conn_info| user_data_websocket_handler(ws, conn_info, sub_service.clone()))
            )
            .fallback_service(ServeDir::new("."));


        // 启动 WebSocket 服务器（在后台运行）
        let ws_listener = tokio::net::TcpListener::bind("0.0.0.0:8084").await?;
        tracing::info!("🚀 Spot WebSocket server started at ws://localhost:8084");
        tracing::info!("📈 Market data stream: ws://localhost:8084/ws/market_data");
        tracing::info!("👤 User data stream: ws://localhost:8084/ws/user_data");

        tokio::spawn(async move {
            axum::serve(ws_listener, ws_app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
        });

        Ok(())
    }
}
