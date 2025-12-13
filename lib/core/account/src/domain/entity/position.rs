//! 持仓实体定义
//!
//! 包含持仓信息、持仓方向等核心类型

use super::types::{Timestamp, UserId};
use std::fmt;

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
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
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

/// 交易对符号（如 BTCUSDT）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol([u8; 16]);

impl Symbol {
    /// 创建新的交易对符号
    pub fn new(s: &str) -> Self {
        let mut bytes = [0u8; 16];
        let src_bytes = s.as_bytes();
        let len = src_bytes.len().min(16);
        bytes[..len].copy_from_slice(&src_bytes[..len]);
        Self(bytes)
    }

    /// 转为字符串
    pub fn as_str(&self) -> &str {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(16);
        std::str::from_utf8(&self.0[..len]).unwrap_or("")
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
    Both,
}

impl fmt::Display for PositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSide::Long => write!(f, "LONG"),
            PositionSide::Short => write!(f, "SHORT"),
            PositionSide::Both => write!(f, "BOTH"),
        }
    }
}

/// 价格（内部使用 i64 存储，假设 8 位小数精度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(i64);

impl Price {
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

    pub fn is_negative(&self) -> bool {
        self.0 < 0
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

/// 数量（内部使用 i64 存储，假设 8 位小数精度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

// ============================================================================
// 持仓信息结构体
// ============================================================================

/// 持仓信息
#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// 所属用户ID
    pub user_id: UserId,
    /// 持仓ID
    pub position_id: PositionId,
    /// 交易对
    pub symbol: Symbol,
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
    pub updated_at: Timestamp,
}

impl PositionInfo {
    /// 创建空持仓
    pub fn empty(symbol: Symbol, position_side: PositionSide) -> Self {
        Self {
            user_id: UserId(0),
            position_id: PositionId::generate(),
            symbol,
            position_side,
            quantity: Quantity::from_raw(0),
            entry_price: Price::from_raw(0),
            mark_price: Price::from_raw(0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 1,
            margin: Price::from_raw(0),
            liquidation_price: None,
            updated_at: current_timestamp(),
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
}

/// 获取当前时间戳（纳秒）
fn current_timestamp() -> Timestamp {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

// ============================================================================
// 实现 Position trait（用于 PositionRepo）
// ============================================================================

impl crate::domain::repo::Position for PositionInfo {
    type Key = Symbol;

    fn key(&self) -> Self::Key {
        self.symbol
    }
}
