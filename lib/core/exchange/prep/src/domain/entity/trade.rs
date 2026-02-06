//! 成交实体

use super::types::{OrderId, PositionSide, Price, Quantity, Side, Timestamp, TradeId};

/// 成交实体 - 不可变的交易记录
///
/// Trade 是 Entity 而非 Value Object：
/// - 有唯一标识 (TradeId)
/// - 需要持久化和查询
/// - 有独立的业务生命周期
/// - 是审计和对账的核心记录
#[derive(Debug, Clone)]
pub struct Trade {
    /// 成交ID
    id: TradeId,
    /// 订单ID
    order_id: OrderId,
    /// 成交价格
    price: Price,
    /// 成交数量
    quantity: Quantity,
    /// 方向
    side: Side,
    /// 持仓方向
    position_side: PositionSide,
    /// 时间戳
    timestamp: Timestamp,
    /// 手续费
    fee: u64,
    /// 已实现盈亏
    realized_pnl: i64,
    /// 是否Maker
    is_maker: bool,
}

impl Trade {
    /// 创建新的成交记录
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: TradeId,
        order_id: OrderId,
        price: Price,
        quantity: Quantity,
        side: Side,
        position_side: PositionSide,
        timestamp: Timestamp,
        fee: u64,
        realized_pnl: i64,
        is_maker: bool,
    ) -> Self {
        Self {
            id,
            order_id,
            price,
            quantity,
            side,
            position_side,
            timestamp,
            fee,
            realized_pnl,
            is_maker,
        }
    }

    // ========== Getters (不可变，只有读取方法) ==========

    /// 成交ID
    pub fn id(&self) -> TradeId {
        self.id
    }

    /// 订单ID
    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    /// 成交价格
    pub fn price(&self) -> Price {
        self.price
    }

    /// 成交数量
    pub fn quantity(&self) -> Quantity {
        self.quantity
    }

    /// 方向
    pub fn side(&self) -> Side {
        self.side
    }

    /// 持仓方向
    pub fn position_side(&self) -> PositionSide {
        self.position_side
    }

    /// 时间戳
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// 手续费
    pub fn fee(&self) -> u64 {
        self.fee
    }

    /// 已实现盈亏
    pub fn realized_pnl(&self) -> i64 {
        self.realized_pnl
    }

    /// 是否Maker
    pub fn is_maker(&self) -> bool {
        self.is_maker
    }

    // ========== 业务方法 ==========

    /// 计算成交金额 (notional value)
    pub fn notional(&self) -> u64 {
        self.price * self.quantity
    }

    /// 计算净收益（卖出）或净支出（买入）
    pub fn net_value(&self) -> i64 {
        let gross = self.notional() as i64;
        match self.side {
            Side::Buy => gross + self.fee as i64,  // 买入：支出
            Side::Sell => gross - self.fee as i64, // 卖出：收入
        }
    }

    /// 是否为开仓成交
    pub fn is_open(&self) -> bool {
        match (self.side, self.position_side) {
            (Side::Buy, PositionSide::Long) => true,
            (Side::Sell, PositionSide::Short) => true,
            (Side::Buy, PositionSide::Both) => true, // 单向模式买入开多
            _ => false,
        }
    }

    /// 是否为平仓成交
    pub fn is_close(&self) -> bool {
        !self.is_open()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_creation() {
        let trade = Trade::new(
            1,                  // id
            100,                // order_id
            50000_00,           // price
            10,                 // quantity
            Side::Buy,          // side
            PositionSide::Long, // position_side
            1700000000000,      // timestamp
            5,                  // fee
            0,                  // realized_pnl
            true,               // is_maker
        );

        assert_eq!(trade.id(), 1);
        assert_eq!(trade.order_id(), 100);
        assert_eq!(trade.price(), 50000_00);
        assert_eq!(trade.quantity(), 10);
        assert!(trade.is_maker());
    }

    #[test]
    fn test_notional_value() {
        let trade =
            Trade::new(1, 100, 50000, 10, Side::Buy, PositionSide::Long, 1700000000000, 5, 0, true);

        assert_eq!(trade.notional(), 500000); // 50000 * 10
    }

    #[test]
    fn test_is_open_close() {
        // 买入开多 = 开仓
        let open_long =
            Trade::new(1, 100, 50000, 10, Side::Buy, PositionSide::Long, 1700000000000, 5, 0, true);
        assert!(open_long.is_open());
        assert!(!open_long.is_close());

        // 卖出平多 = 平仓
        let close_long = Trade::new(
            2,
            101,
            51000,
            10,
            Side::Sell,
            PositionSide::Long,
            1700000000000,
            5,
            10000,
            false,
        );
        assert!(!close_long.is_open());
        assert!(close_long.is_close());

        // 卖出开空 = 开仓
        let open_short = Trade::new(
            3,
            102,
            50000,
            10,
            Side::Sell,
            PositionSide::Short,
            1700000000000,
            5,
            0,
            true,
        );
        assert!(open_short.is_open());
    }
}
