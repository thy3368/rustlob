//! RPC 请求/响应数据模型

use serde::{Deserialize, Serialize};

/// 统一命令请求
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    /// 幂等性 nonce（可选，如不提供则自动生成）
    pub nonce: Option<u64>,
    /// 命令类型: LimitOrder, MarketOrder, CancelOrder
    pub command: String,
    pub trader_id: Option<String>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub price: Option<u32>,
    pub quantity: Option<u32>,
    pub price_limit: Option<u32>,
    pub order_id: Option<u64>,
    pub client_order_id: Option<String>
}

/// 统一命令响应
#[derive(Debug, Serialize, Default)]
pub struct CommandResponse {
    /// 对应命令的 nonce
    pub nonce: Option<u64>,
    /// 是否为重复命令（幂等命中）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_duplicate: Option<bool>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>
}

/// 交易信息
#[derive(Debug, Serialize)]
pub struct TradeInfo {
    pub buyer: String,
    pub seller: String,
    pub price: u32,
    pub quantity: u32
}

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String
}

/// RPC 服务配置
#[derive(Debug, Clone)]
pub struct RpcServiceConfig {
    pub listen_addr: String,
    pub threads: usize,
    pub order_capacity: usize,
    pub price_range: usize
}

impl Default for RpcServiceConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:3030".to_string(),
            threads: 4,
            order_capacity: 100000,
            price_range: 1000000
        }
    }
}
