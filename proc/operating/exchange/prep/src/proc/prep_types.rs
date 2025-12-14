use account::{Price, Quantity, Symbol};

use crate::proc::trading_prep_order_proc::{OrderId, OrderStatus, OrderType, Side};

/// 内部订单状态（扩展字段用于撮合引擎）
#[derive(Debug, Clone)]
pub struct InternalOrder {
    pub order_id: OrderId,
    pub symbol: Symbol,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: Quantity,
    pub price: Option<Price>,
    pub filled_quantity: Quantity,
    pub status: OrderStatus,
    pub created_at: u64,
    /// 冻结的保证金金额（用于订单取消时归还）
    pub frozen_margin: Price
}
