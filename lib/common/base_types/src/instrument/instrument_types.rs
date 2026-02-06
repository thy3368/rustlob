//! 金融产品类型定义
//!
//! 定义交易所支持的产品类别，用于费率差异化、结算流程区分、风控规则分类和订单处理路由

use std::fmt;

/// 金融产品类型（Instrument Type）
///
/// 定义交易所支持的产品类别，用于：
/// - 费率差异化配置
/// - 结算流程区分
/// - 风控规则分类
/// - 订单处理路由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstrumentType {
    /// 现货交易（Spot Trading）
    /// - 即时交割
    /// - 全额保证金（1:1）
    /// - 无资金费率
    Spot = 0,

    /// 永续合约（Perpetual Swap）
    /// - 无到期日
    /// - 杠杆交易（1x-125x）
    /// - 资金费率机制
    Perpetual = 1,

    /// 交割合约（Futures）
    /// - 固定到期日
    /// - 杠杆交易
    /// - 到期实物或现金交割
    Futures = 2,

    /// 期权（Options）
    /// - 看涨/看跌期权
    /// - 到期日行权
    /// - 权利金支付
    Options = 3,
}

impl InstrumentType {
    /// 判断是否为衍生品
    #[inline]
    pub fn is_derivative(&self) -> bool {
        matches!(
            self,
            InstrumentType::Perpetual | InstrumentType::Futures | InstrumentType::Options
        )
    }

    /// 判断是否支持杠杆
    #[inline]
    pub fn supports_leverage(&self) -> bool {
        matches!(self, InstrumentType::Perpetual | InstrumentType::Futures)
    }

    /// 判断是否需要资金费率
    #[inline]
    pub fn requires_funding_rate(&self) -> bool {
        matches!(self, InstrumentType::Perpetual)
    }

    /// 判断是否有到期日
    #[inline]
    pub fn has_expiry(&self) -> bool {
        matches!(self, InstrumentType::Futures | InstrumentType::Options)
    }
}

impl Default for InstrumentType {
    fn default() -> Self {
        InstrumentType::Spot
    }
}

impl fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrumentType::Spot => write!(f, "Spot"),
            InstrumentType::Perpetual => write!(f, "Perpetual"),
            InstrumentType::Futures => write!(f, "Futures"),
            InstrumentType::Options => write!(f, "Options"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrument_type_properties() {
        // 现货测试
        assert!(!InstrumentType::Spot.is_derivative());
        assert!(!InstrumentType::Spot.supports_leverage());
        assert!(!InstrumentType::Spot.requires_funding_rate());
        assert!(!InstrumentType::Spot.has_expiry());

        // 永续合约测试
        assert!(InstrumentType::Perpetual.is_derivative());
        assert!(InstrumentType::Perpetual.supports_leverage());
        assert!(InstrumentType::Perpetual.requires_funding_rate());
        assert!(!InstrumentType::Perpetual.has_expiry());

        // 交割合约测试
        assert!(InstrumentType::Futures.is_derivative());
        assert!(InstrumentType::Futures.supports_leverage());
        assert!(!InstrumentType::Futures.requires_funding_rate());
        assert!(InstrumentType::Futures.has_expiry());

        // 期权测试
        assert!(InstrumentType::Options.is_derivative());
        assert!(!InstrumentType::Options.supports_leverage());
        assert!(!InstrumentType::Options.requires_funding_rate());
        assert!(InstrumentType::Options.has_expiry());
    }

    #[test]
    fn test_instrument_type_display() {
        assert_eq!(InstrumentType::Spot.to_string(), "Spot");
        assert_eq!(InstrumentType::Perpetual.to_string(), "Perpetual");
        assert_eq!(InstrumentType::Futures.to_string(), "Futures");
        assert_eq!(InstrumentType::Options.to_string(), "Options");
    }

    #[test]
    fn test_instrument_type_default() {
        assert_eq!(InstrumentType::default(), InstrumentType::Spot);
    }
}
