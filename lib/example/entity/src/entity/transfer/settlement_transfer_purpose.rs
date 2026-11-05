use serde::{Deserialize, Serialize};

/// 结算余额流水的业务用途。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementTransferPurpose {
    /// 现货买方收到 base。
    SpotBuyerReceiveBase,
    /// 现货买方支付 quote。
    SpotBuyerPayQuote,
    /// 现货卖方交付 base。
    SpotSellerDeliverBase,
    /// 现货卖方收到 quote。
    SpotSellerReceiveQuote,
    /// 永续保证金划转。
    PerpMarginTransfer,
    /// 永续已实现盈亏划转。
    PerpRealizedPnlTransfer,
    /// 交易手续费划转。
    TradingFee,
}

impl SettlementTransferPurpose {
    /// 返回稳定的业务标签。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpotBuyerReceiveBase => "spot_buyer_receive_base",
            Self::SpotBuyerPayQuote => "spot_buyer_pay_quote",
            Self::SpotSellerDeliverBase => "spot_seller_deliver_base",
            Self::SpotSellerReceiveQuote => "spot_seller_receive_quote",
            Self::PerpMarginTransfer => "perp_margin_transfer",
            Self::PerpRealizedPnlTransfer => "perp_realized_pnl_transfer",
            Self::TradingFee => "trading_fee",
        }
    }

    /// 返回该用途是否属于手续费流水。
    pub const fn is_fee(self) -> bool {
        matches!(self, Self::TradingFee)
    }
}
