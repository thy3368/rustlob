use std::sync::Arc;

use spot_behavior::proc::behavior::v2::spot_market_data_sse_behavior::SpotMarketDataStreamAny;
use tokio::sync::broadcast;

use crate::interfaces::spot::{
    http_server::HttpServer,
    websocket_server::WebSocketServer
};
use crate::interfaces::spot::websocket::connection_types::ConnectionRepo;

/// Spot 模块启动器
pub struct SpotStarter;


impl SpotStarter {
    /// 启动 Spot 模块的 HTTP 和 WebSocket 服务器
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting Spot module...");
        println!("⚠️  Running in MOCK mode (no database connection)");

        // ==================== HTTP 服务器启动 ====================
        println!("📡 Starting Spot HTTP API server...");
        HttpServer::start().await?;

        // ==================== WebSocket 服务器启动 ====================
        println!("🔌 Starting Spot WebSocket server...");

        // 创建事件广播通道（仅用于市场数据，用户数据使用定向推送）
        let (md_tx, _) = broadcast::channel(1024);

        // 初始化连接管理器
        let connection_manager = Arc::new(ConnectionRepo::new());

        // 启动 WebSocket 服务器
        WebSocketServer::start(md_tx.clone(), connection_manager.clone()).await?;

        println!("✅ Spot module started successfully");

        Ok(())
    }
}

/// 便捷函数：启动 Spot 模块
pub async fn start_spot_module() -> Result<(), Box<dyn std::error::Error>> { SpotStarter::start().await }
