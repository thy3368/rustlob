use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{routing::get, Router};
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::SpotMarketDataStreamAny;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use db_repo::adapter::change_log_queue_repo::ChangeLogChannelQueueRepo;
use crate::interfaces::spot::websocket::{
    connection_types::ConnectionRepo, md_sse_controller::SpotMarketDataSSEImpl, spot_market_data_pusher,
    spot_user_data_pusher, subscription_service::SubscriptionService, ud_sse_controller::SpotUserDataSSEImpl,
    user_data_ws_handler::user_data_websocket_handler
};

/// WebSocket 服务器启动器
// #[stateless]
pub struct WebSocketServer;

impl WebSocketServer {
    /// 启动 Spot WebSocket 服务器
    ///
    /// todo 用tracing打日志
    pub async fn start(
        md_tx: broadcast::Sender<SpotMarketDataStreamAny>, connection_repo: Arc<ConnectionRepo>
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 发布 SpotMarketDataSSEImpl
        let _market_data_sse = SpotMarketDataSSEImpl::new();
        tracing::info!("SpotMarketDataSSEImpl published successfully");

        // 发布 SpotUserDataSSEImpl
        let _user_data_sse = SpotUserDataSSEImpl::new();
        tracing::info!("SpotUserDataSSEImpl published successfully");

        // 启动 SpotMarketDataPusher
        let md_pusher = spot_market_data_pusher::SpotMarketDataPusher::new(md_tx.clone()).with_interval(5); // 每5秒推送一次
        md_pusher.start();
        tracing::info!("SpotMarketDataPusher started successfully");

        // 启动 SpotUserDataPusher（现在使用连接管理器而非广播通道）
        // let ud_pusher =
        // spot_user_data_pusher::SpotUserDataPusher::new(connection_repo.clone()).
        // with_interval(8); // 每8秒推送一次 ud_pusher.start();

        // 启动订阅服务（无状态设计，不需要克隆）
        let change_log_repo = Arc::new(ChangeLogChannelQueueRepo::new());
        let sub_service = Arc::new(SubscriptionService::new(connection_repo.clone(), change_log_repo));

        // 使用 100ms 轮询间隔启动后台任务
        sub_service.start(Duration::from_millis(100));

        tracing::info!("SubscriptionService started successfully");


        let ws_app = Router::new()
            .route("/ws/user_data", get(move |ws, conn_info| user_data_websocket_handler(ws, conn_info, sub_service.clone())))
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
