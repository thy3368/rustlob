use std::sync::Arc;

// use push::push::connection_types::ConnectionRepo;
use tokio::sync::broadcast;
use tracing;

use crate::interfaces::spot::http_server::HttpServer;
use crate::interfaces::spot::websocket_server::WebSocketServer;

/// Spot 模块启动器
pub struct SpotStarter;

impl SpotStarter {
    /// 启动 Spot 模块的 HTTP 和 WebSocket 服务器
    pub async fn start(ds: bool) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🚀 Starting Spot module...");
        tracing::warn!("⚠️  Running in MOCK mode (no database connection)");

        // ==================== HTTP 服务器启动 ====================
        tracing::info!("📡 Starting Spot HTTP API server...");

        // todo K_line服务/push服务 怎么启动？
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


        // 启动 WebSocket 服务器
        WebSocketServer::start().await?;

        tracing::info!("✅ Spot module started successfully");

        Ok(())
    }
}

/// 便捷函数：启动 Spot 模块单机怎么
pub async fn start_spot_module(ds: bool) -> Result<(), Box<dyn std::error::Error>> {
    SpotStarter::start(ds).await
}
