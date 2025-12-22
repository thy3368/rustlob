use base_types::{OrderId, Price, Quantity, Side, TradingPair};
use crate::proc::trading_prep_order_proc::{OrderStatus, OrderType};

/// 内部订单状态（扩展字段用于撮合引擎）
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "order_id")]
pub struct InternalOrder {
    pub order_id: OrderId,
    pub trading_pair: TradingPair,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: Quantity,
    pub price: Option<Price>,
    pub filled_quantity: Quantity,
    pub status: OrderStatus,
    pub created_at: u64,
    /// 冻结的保证金金额（用于订单取消时归还）
    pub frozen_margin: Price,

    
}

/// 实现 Order trait 以适配 LOB 仓储
impl lob_repo::core::symbol_lob_repo::Order for InternalOrder {
    fn order_id(&self) -> base_types::OrderId {
        self.order_id
    }

    fn price(&self) -> Price {
        self.price.unwrap_or_else(|| Price::from_raw(0))
    }

    fn quantity(&self) -> Quantity {
        self.quantity
    }

    fn filled_quantity(&self) -> Quantity {
        self.filled_quantity
    }

    fn side(&self) -> base_types::Side {
        self.side
    }

    fn symbol(&self) -> TradingPair {
        self.trading_pair
    }
}
