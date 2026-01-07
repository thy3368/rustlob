//! 仓位实体

use super::types::{Leverage, Margin, MarginMode, PositionId, PositionSide, Price, Quantity, Timestamp, TraderId};

/// 仓位实体
#[derive(Debug, Clone)]
pub struct Position {
    /// 仓位ID
    pub id: PositionId,
    /// 交易者ID
    pub trader: TraderId,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 持仓数量
    pub quantity: Quantity,
    /// 开仓均价
    pub entry_price: Price,
    /// 保证金模式
    pub margin_mode: MarginMode,
    /// 杠杆倍数
    pub leverage: Leverage,
    /// 保证金
    pub margin: Margin,
    /// 未实现盈亏
    pub unrealized_pnl: i64,
    /// 已实现盈亏
    pub realized_pnl: i64,
    /// 强平价格
    pub liquidation_price: Price,
    /// 创建时间
    pub created_at: Timestamp,
    /// 更新时间
    pub updated_at: Timestamp
}

impl Position {
    /// 创建新仓位
    pub fn new(
        id: PositionId, trader: TraderId, position_side: PositionSide, quantity: Quantity, entry_price: Price,
        margin_mode: MarginMode, leverage: Leverage, margin: Margin, timestamp: Timestamp
    ) -> Self {
        let liquidation_price = Self::calc_liquidation_price(entry_price, position_side, leverage, margin_mode);

        Self {
            id,
            trader,
            position_side,
            quantity,
            entry_price,
            margin_mode,
            leverage,
            margin,
            unrealized_pnl: 0,
            realized_pnl: 0,
            liquidation_price,
            created_at: timestamp,
            updated_at: timestamp
        }
    }

    /// 增加仓位
    pub fn add(&mut self, quantity: Quantity, price: Price, timestamp: Timestamp) {
        // 计算新均价
        let total_value = self.entry_price * self.quantity + price * quantity;
        let total_quantity = self.quantity + quantity;
        self.entry_price = total_value / total_quantity;
        self.quantity = total_quantity;
        self.updated_at = timestamp;
        self.update_liquidation_price();
    }

    /// 减少仓位，返回已实现盈亏
    pub fn reduce(&mut self, quantity: Quantity, price: Price, timestamp: Timestamp) -> i64 {
        let reduce_qty = quantity.min(self.quantity);
        let pnl = self.calc_pnl(reduce_qty, price);

        self.quantity -= reduce_qty;
        self.realized_pnl += pnl;
        self.updated_at = timestamp;

        if self.quantity > 0 {
            self.update_liquidation_price();
        }

        pnl
    }

    /// 计算盈亏
    fn calc_pnl(&self, quantity: Quantity, exit_price: Price) -> i64 {
        let entry = self.entry_price as i64;
        let exit = exit_price as i64;
        let qty = quantity as i64;

        match self.position_side {
            PositionSide::Long | PositionSide::Both => (exit - entry) * qty,
            PositionSide::Short => (entry - exit) * qty
        }
    }

    /// 更新未实现盈亏
    pub fn update_unrealized_pnl(&mut self, mark_price: Price) {
        self.unrealized_pnl = self.calc_pnl(self.quantity, mark_price);
    }

    /// 计算强平价格
    fn calc_liquidation_price(
        entry_price: Price, position_side: PositionSide, leverage: Leverage, _margin_mode: MarginMode
    ) -> Price {
        // 简化公式：强平价 = 开仓价 * (1 ± 1/杠杆 * 维持保证金率)
        // 维持保证金率假设为 0.5%
        let maintenance_margin_rate = 50u64; // 0.5% = 50/10000
        let leverage_factor = 10000 / leverage as u64;

        match position_side {
            PositionSide::Long | PositionSide::Both => {
                // 多头：价格下跌到强平价
                entry_price.saturating_sub(entry_price * (leverage_factor - maintenance_margin_rate) / 10000)
            }
            PositionSide::Short => {
                // 空头：价格上涨到强平价
                entry_price + entry_price * (leverage_factor - maintenance_margin_rate) / 10000
            }
        }
    }

    /// 更新强平价格
    fn update_liquidation_price(&mut self) {
        self.liquidation_price =
            Self::calc_liquidation_price(self.entry_price, self.position_side, self.leverage, self.margin_mode);
    }

    /// 是否空仓
    pub fn is_empty(&self) -> bool { self.quantity == 0 }

    /// 是否多头
    pub fn is_long(&self) -> bool { matches!(self.position_side, PositionSide::Long | PositionSide::Both) }
}
