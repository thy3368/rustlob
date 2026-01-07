//! 领域基础类型
//!
//! 包含实体标识符、值类型别名和核心枚举

// ============================================================================
// 实体标识符
// ============================================================================

/// 交易者ID
pub type TraderId = u64;
/// 订单ID
pub type OrderId = u64;
/// 仓位ID
pub type PositionId = u64;
/// 成交ID
pub type TradeId = u64;

// ============================================================================
// 值类型别名
// ============================================================================

/// 价格（最小价格单位）
pub type Price = u64;
/// 数量（最小数量单位）
pub type Quantity = u64;
/// 杠杆倍数
pub type Leverage = u32;
/// 保证金金额
pub type Margin = u64;
/// 时间戳（Unix毫秒）
pub type Timestamp = u64;

// ============================================================================
// 核心枚举
// ============================================================================

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// 买入
    Buy,
    /// 卖出
    Sell
}

/// 持仓方向（双向持仓模式）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    /// 单向持仓或净仓位
    Both,
    /// 多头（双向持仓）
    Long,
    /// 空头（双向持仓）
    Short
}

/// 保证金模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginMode {
    /// 全仓
    Cross,
    /// 逐仓
    Isolated
}

/// 持仓模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionMode {
    /// 单向持仓
    OneWay,
    /// 双向持仓
    Hedge
}

/// 订单有效期
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    /// 成交为止
    GTC,
    /// 立即成交或取消
    IOC,
    /// 全部成交或取消
    FOK,
    /// 指定时间前有效
    GTD { expire_time: Timestamp },
    /// 只做Maker
    PostOnly
}

impl Default for TimeInForce {
    fn default() -> Self { Self::GTC }
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    /// 新订单
    New,
    /// 部分成交
    PartiallyFilled,
    /// 全部成交
    Filled,
    /// 已取消
    Cancelled,
    /// 已拒绝
    Rejected,
    /// 已过期
    Expired
}
