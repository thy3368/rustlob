use std::sync::Arc;

use axum::{
    routing::get,
    Router
};
use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::SpotMarketDataStreamAny;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::interfaces::spot::websocket::{
    connection_types::ConnectionRepo, md_sse_controller::SpotMarketDataSSEImpl, spot_market_data_pusher,
    spot_user_data_pusher, ud_sse_controller::SpotUserDataSSEImpl, user_data_ws_handler::user_data_websocket_handler
};

/// WebSocket 服务器启动器
pub struct WebSocketServer;

impl WebSocketServer {
    /// 启动 Spot WebSocket 服务器
    pub async fn start(
        md_tx: broadcast::Sender<SpotMarketDataStreamAny>, connection_repo: Arc<ConnectionRepo>
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 发布 SpotMarketDataSSEImpl
        let _market_data_sse = SpotMarketDataSSEImpl::new();
        println!("SpotMarketDataSSEImpl published successfully");

        // 发布 SpotUserDataSSEImpl
        let _user_data_sse = SpotUserDataSSEImpl::new();
        println!("SpotUserDataSSEImpl published successfully");

        // 启动 SpotMarketDataPusher
        let md_pusher = spot_market_data_pusher::SpotMarketDataPusher::new(md_tx.clone()).with_interval(5); // 每5秒推送一次
        md_pusher.start();
        println!("SpotMarketDataPusher started successfully");

        // 启动 SpotUserDataPusher（现在使用连接管理器而非广播通道）
        let ud_pusher = spot_user_data_pusher::SpotUserDataPusher::new(connection_repo.clone()).with_interval(8); // 每8秒推送一次
        ud_pusher.start();
        println!("SpotUserDataPusher started successfully");

        // 创建 WebSocket 应用
        // 路由分离：市场数据和用户数据使用不同的 WebSocket 端点

        //todo 检查一下websocket 路由信息,用“get"对不对
        let ws_app = Router::new()
            .route(
                "/ws/user_data",
                get(move |ws, conn_info| user_data_websocket_handler(ws, conn_info, connection_repo.clone()))
            )
            .fallback_service(ServeDir::new("."));


        // 启动 WebSocket 服务器（在后台运行）
        let ws_listener = tokio::net::TcpListener::bind("0.0.0.0:8084").await?;
        println!("🚀 Spot WebSocket server started at ws://localhost:8084");
        println!("📈 Market data stream: ws://localhost:8084/ws/market_data");
        println!("👤 User data stream: ws://localhost:8084/ws/user_data");

        tokio::spawn(async move {
            axum::serve(ws_listener, ws_app).await.expect("Spot WebSocket server failed to start");
        });

        Ok(())
    }

    /// 创建 WebSocket 应用
    fn create_websocket_app(
        md_tx: broadcast::Sender<SpotMarketDataStreamAny>, connection_repo: Arc<ConnectionRepo>
    ) -> Router {
        use axum::routing::get;
        use tower_http::services::ServeDir;

        // 路由分离：市场数据和用户数据使用不同的 WebSocket 端点
        Router::new()
            .route(
                "/ws/user_data",
                get(move |ws, conn_info| user_data_websocket_handler(ws, conn_info, connection_repo.clone()))
            )
            .fallback_service(ServeDir::new("."))
    }
}
