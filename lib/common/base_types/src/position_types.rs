//! 持仓相关类型定义
//!
//! 包含持仓信息、价格、数量、交易对符号等核心类型

use std::fmt;

use super::trading_types::{Timestamp, TradingPair, UserId, AssetId};
use crate::{AccountId, OrderId, Side};

// ============================================================================
// 持仓相关类型定义
// ============================================================================

/// 持仓ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionId(pub u64);

impl PositionId {
    /// 生成新的持仓ID（简化实现，实际应使用雪花算法等）
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        Self(nanos)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PositionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for PositionId {
    fn default() -> Self {
        Self(0)
    }
}

/// 持仓方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionSide {
    /// 多头（做多）
    Long,
    /// 空头（做空）
    Short,
    /// 双向持仓模式
    Both
}

impl fmt::Display for PositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSide::Long => write!(f, "LONG"),
            PositionSide::Short => write!(f, "SHORT"),
            PositionSide::Both => write!(f, "BOTH")
        }
    }
}

impl Default for PositionSide {
    fn default() -> Self {
        PositionSide::Long
    }
}

/// 价格（内部使用 i64 存储，8 位小数精度）
///
/// 加密货币标准：
/// - 8 位小数精度（与 Bitcoin Satoshi 一致）
/// - 例如：50000.12345678 USDT
/// - 最小单位：0.00000001
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price(i64);

impl Price {
    const DECIMALS: i64 = 100_000_000; // 8 位小数（加密货币标准）

    pub fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> i64 {
        self.0
    }

    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::DECIMALS as f64) as i64)
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / Self::DECIMALS as f64
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// 与数量相乘：Price * Quantity -> Price
    /// 或与价格相乘（表示名义价值）：Price * Price -> Price
    /// 用于计算名义价值（notional）
    pub fn checked_mul(&self, qty: impl Into<i128>) -> Option<Price> {
        let price_raw = self.0 as i128;
        let qty_raw = qty.into();
        let result = price_raw.checked_mul(qty_raw)?;
        // 需要除以 DECIMALS 因为两个都有 8 位小数
        let normalized = result.checked_div(Self::DECIMALS as i128)?;
        i64::try_from(normalized).ok().map(Price)
    }

    /// 与 Quantity 相乘
    pub fn mul_quantity(&self, qty: Quantity) -> Option<Price> {
        self.checked_mul(qty.raw())
    }

    /// 与另一个 Price 相乘（定点数乘法）
    pub fn mul_price(&self, other: Price) -> Option<Price> {
        self.checked_mul(other.raw())
    }
}

impl std::ops::Add for Price {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Price {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Default for Price {
    fn default() -> Self {
        Self(0)
    }
}

impl From<Price> for i128 {
    fn from(p: Price) -> Self {
        p.0 as i128
    }
}

/// 数量（内部使用 i64 存储，假设 8 位小数精度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity(i64);

impl Quantity {
    const DECIMALS: i64 = 100_000_000; // 8 位小数

    pub fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> i64 {
        self.0
    }

    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::DECIMALS as f64) as i64)
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / Self::DECIMALS as f64
    }

    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Default for Quantity {
    fn default() -> Self {
        Self(0)
    }
}

// ============================================================================
// 持仓信息结构体
// ============================================================================

/// 持仓信息
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "position_id")]
pub struct PrepPosition {
    /// 所属用户ID
    pub user_id: UserId,
    /// 持仓ID
    pub position_id: PositionId,
    /// 账户ID
    pub account_id: AccountId,

    /// 交易对（包含 base_asset 和 quote_asset）
    pub trading_pair: TradingPair,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 持仓数量（正数表示多头，负数表示空头）
    pub quantity: Quantity,
    /// 持仓均价
    pub entry_price: Price,
    /// 标记价格（用于计算未实现盈亏）
    pub mark_price: Price,
    /// 未实现盈亏
    pub unrealized_pnl: Price,
    /// 已实现盈亏
    pub realized_pnl: Price,
    /// 杠杆倍数
    pub leverage: u8,
    /// 保证金
    pub margin: Price,
    /// 强平价格
    pub liquidation_price: Option<Price>,
    /// 更新时间戳（纳秒）
    pub updated_at: Timestamp
}

impl PrepPosition {
    /// 创建空持仓
    pub fn empty(trading_pair: TradingPair, position_side: PositionSide) -> Self {
        Self {
            user_id: UserId(0),
            position_id: PositionId::generate(),
            account_id: AccountId(0),
            trading_pair,
            position_side,
            quantity: Quantity::from_raw(0),
            entry_price: Price::from_raw(0),
            mark_price: Price::from_raw(0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 1,
            margin: Price::from_raw(0),
            liquidation_price: None,
            updated_at: current_timestamp()
        }
    }

    /// 是否有持仓
    pub fn has_position(&self) -> bool {
        self.quantity.is_positive()
    }

    /// 是否为多头
    pub fn is_long(&self) -> bool {
        self.position_side == PositionSide::Long && self.quantity.is_positive()
    }

    /// 是否为空头
    pub fn is_short(&self) -> bool {
        self.position_side == PositionSide::Short && self.quantity.is_positive()
    }

    /// 计算下次资金费用
    pub fn calculate_next_funding_fee(&self, funding_rate: Price) -> Price {
        if !self.has_position() {
            return Price::from_raw(0);
        }

        let notional = self.mark_price.to_f64() * self.quantity.to_f64();
        let base_fee = notional * funding_rate.to_f64();

        let fee = if self.position_side == PositionSide::Long {
            -base_fee
        } else {
            base_fee
        };

        Price::from_f64(fee)
    }

    fn calculate_liquidation_price(&self, position: &PrepPosition) -> Option<Price> {
        if !position.has_position() {
            return None;
        }

        const MAINTENANCE_MARGIN_RATE: f64 = 0.004; // 0.4% 维持保证金率
        let entry = position.entry_price.to_f64();
        let leverage = position.leverage as f64;

        let liq_price = match position.position_side {
            PositionSide::Long => {
                // 多仓：价格下跌到此价格时强平
                entry * (1.0 - 1.0 / leverage + MAINTENANCE_MARGIN_RATE)
            }
            PositionSide::Short => {
                // 空仓：价格上涨到此价格时强平
                entry * (1.0 + 1.0 / leverage - MAINTENANCE_MARGIN_RATE)
            }
            PositionSide::Both => {
                // 单向模式，暂时按多仓处理
                entry * (1.0 - 1.0 / leverage + MAINTENANCE_MARGIN_RATE)
            }
        };

        Some(Price::from_f64(liq_price.max(0.0)))
    }


    fn calculate_unrealized_pnl(&self, position: &PrepPosition) -> Price {
        if !position.has_position() {
            return Price::from_raw(0);
        }

        let entry = position.entry_price.to_f64();
        let mark = position.mark_price.to_f64();
        let qty = position.quantity.to_f64();

        let pnl = match position.position_side {
            PositionSide::Long => (mark - entry) * qty,
            PositionSide::Short => (entry - mark) * qty,
            PositionSide::Both => {
                // 单向持仓模式，根据数量符号判断
                (mark - entry) * qty
            }
        };

        Price::from_f64(pnl)
    }

    /// 更新持仓数量、均价、杠杆和相关计算字段
    ///
    /// # 参数
    /// - `new_quantity`: 新成交数量
    /// - `new_price`: 新成交价格
    /// - `leverage`: 杠杆倍数
    /// - `side`: 订单方向
    /// - `position_side`: 持仓方向
    pub fn add(&mut self, new_quantity: Quantity, new_price: Price, leverage: u8, _side: crate::Side, _position_side: crate::PositionSide) {
        // 计算新的持仓数量和均价（加权平均）
        let old_qty = self.quantity.to_f64();
        let old_price = self.entry_price.to_f64();
        let new_qty_val = new_quantity.to_f64();
        let new_price_val = new_price.to_f64();

        let total_cost = old_qty * old_price + new_qty_val * new_price_val;
        let total_qty = old_qty + new_qty_val;

        // 更新持仓数量和均价
        self.quantity = Quantity::from_f64(total_qty);
        self.entry_price = if total_qty > 0.0 {
            Price::from_f64(total_cost / total_qty)
        } else {
            Price::from_raw(0)
        };

        // 更新标记价格
        self.mark_price = new_price;

        // 更新杠杆
        self.leverage = leverage;

        // 计算保证金 = (持仓价值) / 杠杆倍数
        let notional = self.entry_price.to_f64() * self.quantity.to_f64();
        self.margin = Price::from_f64(notional / leverage as f64);

        // 计算未实现盈亏
        self.unrealized_pnl = self.calculate_unrealized_pnl_value();

        // 计算强平价格
        self.liquidation_price = self.calculate_liquidation_price_value();

        // 更新时间戳
        self.updated_at = current_timestamp_ms();
    }

    /// 计算未实现盈亏值
    fn calculate_unrealized_pnl_value(&self) -> Price {
        if !self.has_position() {
            return Price::from_raw(0);
        }

        let entry = self.entry_price.to_f64();
        let mark = self.mark_price.to_f64();
        let qty = self.quantity.to_f64();

        let pnl = match self.position_side {
            PositionSide::Long => (mark - entry) * qty,
            PositionSide::Short => (entry - mark) * qty,
            PositionSide::Both => {
                // 单向持仓模式，根据数量符号判断
                (mark - entry) * qty
            }
        };

        Price::from_f64(pnl)
    }

    /// 计算强平价格值
    fn calculate_liquidation_price_value(&self) -> Option<Price> {
        if !self.has_position() {
            return None;
        }

        const MAINTENANCE_MARGIN_RATE: f64 = 0.004; // 0.4% 维持保证金率
        let entry = self.entry_price.to_f64();
        let leverage = self.leverage as f64;

        let liq_price = match self.position_side {
            PositionSide::Long => {
                // 多仓：价格下跌到此价格时强平
                entry * (1.0 - 1.0 / leverage + MAINTENANCE_MARGIN_RATE)
            }
            PositionSide::Short => {
                // 空仓：价格上涨到此价格时强平
                entry * (1.0 + 1.0 / leverage - MAINTENANCE_MARGIN_RATE)
            }
            PositionSide::Both => {
                // 单向模式，暂时按多仓处理
                entry * (1.0 - 1.0 / leverage + MAINTENANCE_MARGIN_RATE)
            }
        };

        Some(Price::from_f64(liq_price.max(0.0)))
    }

    /// 更新已实现盈亏
    ///
    /// # 参数
    /// - `pnl`: 盈亏金额
    pub fn update_realized_pnl(&mut self, pnl: Price) {
        self.realized_pnl = self.realized_pnl + pnl;
    }
}

/// 获取当前时间戳（纳秒）
fn current_timestamp() -> Timestamp {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
}

/// 获取当前时间戳（毫秒）
fn current_timestamp_ms() -> Timestamp {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

// ============================================================================
// 成交相关类型定义
// ============================================================================

/// 成交ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TradeId(String);

impl TradeId {
    /// 创建新的成交ID
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    /// 生成随机成交ID
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        Self(format!("TRD-{}", timestamp))
    }

    /// 获取字符串表示
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for TradeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

/// 成交记录（单次撮合成交）
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "trade_id")]
pub struct PrepTrade {
    /// 成交ID
    pub trade_id: TradeId,
    /// 关联订单ID
    pub ask_order_id: OrderId,
    /// 关联订单ID
    pub bid_order_id: OrderId,
    /// 交易对
    pub symbol: TradingPair,
    /// 订单方向
    pub side: Side,
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// 手续费
    pub fee: Price,
    /// 手续费资产（通常是USDT）
    pub fee_asset: AssetId,
    /// 是否为Maker（流动性提供方）
    pub is_maker: bool,
    /// 成交时间戳（毫秒）
    pub timestamp: u64
}

impl PrepTrade {
    /// 创建新的成交记录
    pub fn new(
        trade_id: TradeId, ask_order_id: OrderId,bid_order_id: OrderId, symbol: TradingPair, side: Side, price: Price, quantity: Quantity, fee: Price,
        fee_asset: AssetId, is_maker: bool
    ) -> Self {
        Self {
            trade_id,
            ask_order_id,
            bid_order_id,
            symbol,
            side,
            price,
            quantity,
            fee,
            fee_asset,
            is_maker,
            timestamp: current_timestamp_ms()
        }
    }

    /// 计算成交金额（价格 * 数量）
    pub fn notional(&self) -> Price {
        // 简化计算：使用浮点数计算后转回定点数
        let value = self.price.to_f64() * self.quantity.to_f64();
        Price::from_f64(value)
    }
}
