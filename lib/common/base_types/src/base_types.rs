//! 基础数值类型定义
//!
//! 包含价格、数量等核心数值类型

use std::fmt;
use decimal::Decimal;

// ============================================================================
// 类型别名：为了语义清晰，保留 Price 和 Quantity 作为类型别名
// ============================================================================

/// 价格（语义别名，实际使用 Decimal）
pub type Price = Decimal;

/// 数量（语义别名，实际使用 Decimal）
pub type Quantity = Decimal;


/// 时间戳（纳秒）
pub type Timestamp = u64;

/// 用户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct UserId(pub u64);

impl From<u64> for UserId {
    #[inline]
    fn from(id: u64) -> Self {
        Self(id)
    }
}

/// 账户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AccountId(pub u64);

impl From<u64> for AccountId {
    #[inline]
    fn from(id: u64) -> Self {
        Self(id)
    }
}

/// 资产ID（使用 u32 高性能）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AssetId(pub u32);

impl AssetId {
    pub const USDT: AssetId = AssetId(1);
    pub const BTC: AssetId = AssetId(2);
    pub const ETH: AssetId = AssetId(3);
}

impl From<u32> for AssetId {
    #[inline]
    fn from(id: u32) -> Self {
        Self(id)
    }
}

/// 交易对
///
/// 定义基础资产和计价资产的关系
/// 例如：BTC/USDT 交易对
/// - base_asset = BTC (基础资产，卖出时检查)
/// - quote_asset = USDT (计价资产，买入时检查)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TradingPair {
    /// 基础资产（如 BTC）
    pub base_asset: AssetId,
    /// 计价资产（如 USDT）
    pub quote_asset: AssetId
}

impl TradingPair {
    #[inline]
    pub const fn new(base_asset: AssetId, quote_asset: AssetId) -> Self {
        Self {
            base_asset,
            quote_asset
        }
    }

    /// 从交易对符号字符串创建 TradingPair
    ///
    /// # 说明
    /// - 假设格式为 `{BASE_ASSET}{QUOTE_ASSET}`，例如 "BTCUSDT"
    /// - 假设 quote_asset 总是 USDT（4字符）
    /// - base_asset 是其余部分（通常 3-4 个字符）
    ///
    /// # 示例
    /// ```ignore
    /// let pair = TradingPair::from_symbol_str("BTCUSDT");
    /// assert_eq!(pair.base_asset, AssetId::BTC);
    /// assert_eq!(pair.quote_asset, AssetId::USDT);
    /// ```
    pub fn from_symbol_str(symbol: &str) -> Option<Self> {
        if symbol.len() < 7 {
            // 最少需要 3 (base) + 4 (USDT) = 7 个字符
            return None;
        }

        let quote_str = &symbol[symbol.len() - 4..];
        let base_str = &symbol[..symbol.len() - 4];

        let base_asset = match base_str.to_uppercase().as_str() {
            "BTC" => AssetId::BTC,
            "ETH" => AssetId::ETH,
            // 可以继续添加其他资产
            _ => return None,
        };

        let quote_asset = match quote_str.to_uppercase().as_str() {
            "USDT" => AssetId::USDT,
            // 可以继续添加其他计价资产
            _ => return None,
        };

        Some(Self {
            base_asset,
            quote_asset,
        })
    }

    /// 生成交易对符号字符串
    ///
    /// # 说明
    /// - 生成格式为 `{BASE_ASSET}{QUOTE_ASSET}` 的符号
    /// - 例如：BTC/USDT -> "BTCUSDT"
    ///
    /// # 示例
    /// ```ignore
    /// let pair = TradingPair::BTC_USDT;
    /// assert_eq!(pair.to_symbol_string(), "BTCUSDT");
    /// ```
    pub fn to_symbol_string(&self) -> String {
        let base_str = match self.base_asset.0 {
            2 => "BTC",
            3 => "ETH",
            _ => "UNKNOWN",
        };

        let quote_str = match self.quote_asset.0 {
            1 => "USDT",
            _ => "UNKNOWN",
        };

        format!("{}{}", base_str, quote_str)
    }

    /// BTC/USDT 交易对
    pub const BTC_USDT: TradingPair = TradingPair {
        base_asset: AssetId::BTC,
        quote_asset: AssetId::USDT
    };

    /// ETH/USDT 交易对
    pub const ETH_USDT: TradingPair = TradingPair {
        base_asset: AssetId::ETH,
        quote_asset: AssetId::USDT
    };

    /// USDT/USDT（用于费用资产表示）
    pub const USDT_USDT: TradingPair = TradingPair {
        base_asset: AssetId::USDT,
        quote_asset: AssetId::USDT
    };
}


/// 订单ID
pub type OrderId = u64;

/// 买卖方向
///
/// 定义交易的买卖方向，供 LOB、Account 等模块共享使用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    Buy = 0,
    Sell = 1
}

impl Side {
    /// 获取相反方向
    #[inline]
    pub fn opposite(&self) -> Side {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}

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

    pub fn as_u64(&self) -> u64 { self.0 }
}

impl fmt::Display for PositionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl Default for PositionId {
    fn default() -> Self { Self(0) }
}

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

