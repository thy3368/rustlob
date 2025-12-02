//! RPC 请求/响应数据模型

use serde::{Deserialize, Serialize};

/// 统一命令请求
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    /// 命令类型: LimitOrder, MarketOrder, IcebergOrder, CancelOrder
    pub command: String,
    pub trader_id: Option<String>,
    pub side: Option<String>,
    pub price: Option<u32>,
    pub quantity: Option<u32>,
    pub total_quantity: Option<u32>,
    pub display_quantity: Option<u32>,
    pub order_id: Option<u64>,
}

/// 统一命令响应
#[derive(Debug, Serialize, Default)]
pub struct CommandResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_display: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 交易信息
#[derive(Debug, Serialize)]
pub struct TradeInfo {
    pub buyer: String,
    pub seller: String,
    pub price: u32,
    pub quantity: u32,
}

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}

/// RPC 服务配置
#[derive(Debug, Clone)]
pub struct RpcServiceConfig {
    pub listen_addr: String,
    pub threads: usize,
    pub order_capacity: usize,
    pub price_range: usize,
}

impl Default for RpcServiceConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:3030".to_string(),
            threads: 4,
            order_capacity: 100000,
            price_range: 1000000,
        }
    }
}
