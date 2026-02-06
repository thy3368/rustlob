//! 订单实体

use super::types::{
    OrderId, OrderStatus, PositionSide, Price, Quantity, Side, TimeInForce, Timestamp, TraderId,
};

/// 订单实体
#[derive(Debug, Clone)]
pub struct Order {
    /// 订单ID
    pub id: OrderId,
    /// 交易者ID
    pub trader: TraderId,
    /// 方向
    pub side: Side,
    /// 价格
    pub price: Price,
    /// 原始数量
    pub original_quantity: Quantity,
    /// 剩余数量
    pub remaining_quantity: Quantity,
    /// 已成交数量
    pub filled_quantity: Quantity,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 只减仓
    pub reduce_only: bool,
    /// 有效期
    pub time_in_force: TimeInForce,
    /// 状态
    pub status: OrderStatus,
    /// 创建时间
    pub created_at: Timestamp,
    /// 更新时间
    pub updated_at: Timestamp,
}

impl Order {
    /// 创建新订单
    pub fn new(
        id: OrderId,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        position_side: PositionSide,
        reduce_only: bool,
        time_in_force: TimeInForce,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            id,
            trader,
            side,
            price,
            original_quantity: quantity,
            remaining_quantity: quantity,
            filled_quantity: 0,
            position_side,
            reduce_only,
            time_in_force,
            status: OrderStatus::New,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    /// 成交（部分或全部）
    pub fn fill(&mut self, quantity: Quantity, timestamp: Timestamp) {
        let fill_qty = quantity.min(self.remaining_quantity);
        self.filled_quantity += fill_qty;
        self.remaining_quantity -= fill_qty;
        self.updated_at = timestamp;

        if self.remaining_quantity == 0 {
            self.status = OrderStatus::Filled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }
    }

    /// 取消订单
    pub fn cancel(&mut self, timestamp: Timestamp) {
        self.status = OrderStatus::Cancelled;
        self.updated_at = timestamp;
    }

    /// 是否可成交
    pub fn is_active(&self) -> bool {
        matches!(self.status, OrderStatus::New | OrderStatus::PartiallyFilled)
    }

    /// 是否为买单
    pub fn is_buy(&self) -> bool {
        matches!(self.side, Side::Buy)
    }

    /// 价格是否可匹配
    pub fn can_match(&self, other_price: Price) -> bool {
        match self.side {
            Side::Buy => self.price >= other_price,
            Side::Sell => self.price <= other_price,
        }
    }
}
