//! RPC 请求/响应数据模型

use serde::{Deserialize, Serialize};

/// JSON-RPC 请求参数：限价单
#[derive(Debug, Deserialize)]
pub struct LimitOrderRequest {
    pub trader_id: String,
    pub side: String,      // "BUY" or "SELL"
    pub price: u32,
    pub quantity: u32,
}

/// JSON-RPC 请求参数：市价单
#[derive(Debug, Deserialize)]
pub struct MarketOrderRequest {
    pub trader_id: String,
    pub side: String,
    pub quantity: u32,
}

/// JSON-RPC 请求参数：冰山单
#[derive(Debug, Deserialize)]
pub struct IcebergOrderRequest {
    pub trader_id: String,
    pub side: String,
    pub price: u32,
    pub total_quantity: u32,
    pub display_quantity: u32,
}

/// JSON-RPC 请求参数：取消订单
#[derive(Debug, Deserialize)]
pub struct CancelOrderRequest {
    pub order_id: u64,
}

/// 交易信息
#[derive(Debug, Serialize)]
pub struct TradeInfo {
    pub buyer: String,
    pub seller: String,
    pub price: u32,
    pub quantity: u32,
}

/// RPC 服务配置
#[derive(Debug, Clone)]
pub struct RpcServiceConfig {
    /// 监听地址
    pub listen_addr: String,
    /// 工作线程数
    pub threads: usize,
    /// 订单容量
    pub order_capacity: usize,
    /// 价格范围
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
