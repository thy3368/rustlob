//! 基础数值类型定义
//!
//! 包含价格、数量等核心数值类型

use std::fmt;

use decimal::Decimal;
// ============================================================================
// 类型别名：为了语义清晰，保留 Price 和 Quantity 作为类型别名
// ============================================================================

// todo 这里定义的类型 都是  #[immutable] 都不应该可变
/// 价格（语义别名，实际使用 Decimal）

pub type Price = Decimal;

/// 数量（语义别名，实际使用 Decimal）
pub type Quantity = Decimal;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[immutable]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn now_as_nanos() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);

        Self(timestamp)
    }
}


/// 订单ID
/// todo 要改
pub type OrderId = u64;


/// 用户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserId(pub u64);

impl From<u64> for UserId {
    #[inline]
    fn from(id: u64) -> Self { Self(id) }
}

/// 账户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct AccountId(pub u64);

impl From<u64> for AccountId {
    #[inline]
    fn from(id: u64) -> Self { Self(id) }
}


/// 持仓ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// 交易员标识符（8字节固定长度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(align(8))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// todo 币安是怎么定义


pub struct TraderId([u8; 8]);

impl Default for TraderId {
    fn default() -> Self { Self([0; 8]) }
}

impl TraderId {
    /// 创建新的交易员ID
    #[inline]
    pub fn new(bytes: [u8; 8]) -> Self { Self(bytes) }
}


/// 成交ID
/// todo 要改
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// 资产类型枚举
///
/// 提供类型安全的资产定义，支持与 u32 之间的转换
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetId {
    /// USDT（泰达币）- 稳定币
    Usdt = 1,
    /// BTC（比特币）- 主要加密货币
    Btc = 2,
    /// ETH（以太坊）- 智能合约平台
    Eth = 3
}

impl AssetId {
    /// 将资产ID转换为原始数值
    pub const fn as_u32(self) -> u32 { self as u32 }

    /// 将资产ID转换为字符串表示
    pub const fn as_str(self) -> &'static str {
        match self {
            AssetId::Usdt => "USDT",
            AssetId::Btc => "BTC",
            AssetId::Eth => "ETH"
        }
    }

    /// 从字符串表示转换为资产ID（如果匹配）
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "USDT" => Some(AssetId::Usdt),
            "BTC" => Some(AssetId::Btc),
            "ETH" => Some(AssetId::Eth),
            _ => None
        }
    }
}

impl Default for AssetId {
    #[inline]
    fn default() -> Self { AssetId::Usdt }
}

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

impl From<AssetId> for u32 {
    #[inline]
    fn from(asset: AssetId) -> Self { asset as u32 }
}

impl TryFrom<u32> for AssetId {
    type Error = ();

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AssetId::Usdt),
            2 => Ok(AssetId::Btc),
            3 => Ok(AssetId::Eth),
            _ => Err(())
        }
    }
}

/// 交易对
///
/// 定义基础资产和计价资产的关系
/// 例如：BTC/USDT 交易对
/// - base_asset = BTC (基础资产，卖出时检查)
/// - quote_asset = USDT (计价资产，买入时检查)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]

pub enum TradingPair {
    /// BTC/USDT 交易对
    BtcUsdt = 1,
    /// ETH/USDT 交易对
    EthUsdt = 2,
    /// BTC/ETH 交易对
    BtcEth = 3,
    /// USDT/USDT（用于费用资产表示）
    UsdtUsdt = 4
}

impl TradingPair {
    /// 获取基础资产
    pub const fn base_asset(self) -> AssetId {
        match self {
            TradingPair::BtcUsdt => AssetId::Btc,
            TradingPair::EthUsdt => AssetId::Eth,
            TradingPair::BtcEth => AssetId::Btc,
            TradingPair::UsdtUsdt => AssetId::Usdt
        }
    }

    /// 获取计价资产
    pub const fn quote_asset(self) -> AssetId {
        match self {
            TradingPair::BtcUsdt => AssetId::Usdt,
            TradingPair::EthUsdt => AssetId::Usdt,
            TradingPair::BtcEth => AssetId::Eth,
            TradingPair::UsdtUsdt => AssetId::Usdt
        }
    }

    /// 从交易对符号字符串创建 TradingPair
    ///
    /// # 说明
    /// 支持多种格式：
    /// - `{BASE}{QUOTE}` 例如 "BTCUSDT"
    /// - `{BASE}_{QUOTE}` 例如 "BTC_USDT"
    /// - `{BASE}/{QUOTE}` 例如 "BTC/USDT"
    /// - `{BASE}-{QUOTE}` 例如 "BTC-USDT"
    ///
    /// # 示例
    /// ```ignore
    /// let pair1 = TradingPair::from_symbol_str("BTCUSDT");
    /// let pair2 = TradingPair::from_symbol_str("BTC/USDT");
    /// assert_eq!(pair1, pair2);
    /// ```
    pub fn from_symbol_str(symbol: &str) -> Option<Self> {
        let normalized = symbol.replace(['_', '/', '-'], "").to_uppercase();
        match normalized.as_str() {
            "BTCUSDT" => Some(TradingPair::BtcUsdt),
            "ETHUSDT" => Some(TradingPair::EthUsdt),
            "BTCETH" => Some(TradingPair::BtcEth),
            "USDTUSDT" => Some(TradingPair::UsdtUsdt),
            _ => None
        }
    }

    /// 生成交易对符号字符串
    ///
    /// # 说明
    /// - 生成格式为 `{BASE_ASSET}{QUOTE_ASSET}` 的符号
    /// - 例如：BTC/USDT -> "BTCUSDT"
    ///
    /// # 示例
    /// ```ignore
    /// let pair = TradingPair::BtcUsdt;
    /// assert_eq!(pair.to_symbol_string(), "BTCUSDT");
    /// ```
    pub const fn to_symbol_string(self) -> &'static str {
        match self {
            TradingPair::BtcUsdt => "BTCUSDT",
            TradingPair::EthUsdt => "ETHUSDT",
            TradingPair::BtcEth => "BTCETH",
            TradingPair::UsdtUsdt => "USDTUSDT"
        }
    }
}

impl Default for TradingPair {
    #[inline]
    fn default() -> Self { TradingPair::BtcUsdt }
}

impl fmt::Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.to_symbol_string()) }
}


/// 买卖方向
///
/// 定义交易的买卖方向，供 LOB、Account 等模块共享使用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum OrderSide {
    Buy = 0,
    Sell = 1
}

impl OrderSide {
    /// 获取相反方向
    #[inline]
    pub fn opposite(&self) -> OrderSide {
        match self {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy
        }
    }
}

impl Default for OrderSide {
    fn default() -> Self { OrderSide::Buy }
}
