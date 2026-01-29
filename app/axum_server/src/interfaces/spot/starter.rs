use std::sync::Arc;

use tokio::sync::broadcast;
use tracing;

use crate::interfaces::spot::{
    http_server::HttpServer, websocket::connection_types::ConnectionRepo, websocket_server::WebSocketServer
};

/// Spot 模块启动器
pub struct SpotStarter;


impl SpotStarter {
    /// 启动 Spot 模块的 HTTP 和 WebSocket 服务器
    pub async fn start(ds: bool) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🚀 Starting Spot module...");
        tracing::warn!("⚠️  Running in MOCK mode (no database connection)");

        // ==================== HTTP 服务器启动 ====================
        tracing::info!("📡 Starting Spot HTTP API server...");

        match ds {
            true => {
                HttpServer::start_4_ds().await?;
            }
            false => {
                HttpServer::start().await?;
            }
        }


        // ==================== WebSocket 服务器启动 ====================
        tracing::info!("🔌 Starting Spot WebSocket server...");

        // 创建事件广播通道（仅用于市场数据，用户数据使用定向推送）
        let (md_tx, _) = broadcast::channel(1024);

        // 初始化连接管理器
        let connection_manager = Arc::new(ConnectionRepo::new());

        // 启动 WebSocket 服务器
        WebSocketServer::start(md_tx.clone(), connection_manager.clone()).await?;

        tracing::info!("✅ Spot module started successfully");

        Ok(())
    }
}

/// 便捷函数：启动 Spot 模块单机怎么
pub async fn start_spot_module(ds: bool) -> Result<(), Box<dyn std::error::Error>> { SpotStarter::start(ds).await }
