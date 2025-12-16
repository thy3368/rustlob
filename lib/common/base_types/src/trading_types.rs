//! 交易相关基础类型定义
//!
//! 包含用户ID、账户ID、资产ID、交易对等核心类型

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}
