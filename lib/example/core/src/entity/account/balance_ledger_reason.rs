use serde::{Deserialize, Serialize};

use super::settlement_transfer_voucher::SettlementTransferPurpose;

/// 余额变更原因。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceLedgerReason {
    /// 订单下单冻结余额。
    FreezeForOrder {
        /// 触发本次余额冻结的订单 ID。
        order_id: String,
    },
    /// 撤单释放冻结余额。
    UnfreezeForCancel {
        /// 被撤销订单 ID。
        order_id: String,
    },
    /// 立即单冻结余额。
    ReserveForImmediateOrder {
        /// 触发本次余额冻结的订单 ID。
        order_id: String,
    },
    /// 现货撤单为买单释放冻结 quote。
    CancelSpotOrderReleaseQuote {
        /// 被撤销订单 ID。
        order_id: String,
    },
    /// 现货撤单为卖单释放冻结 base。
    CancelSpotOrderReleaseBase {
        /// 被撤销订单 ID。
        order_id: String,
    },
    /// 现货清结算为买方增加 base 可用余额。
    SettleSpotTradeBuyerReceiveBase {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货清结算为买方释放冻结 quote。
    SettleSpotTradeBuyerReleaseFrozenQuote {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货清结算为卖方增加 quote 可用余额。
    SettleSpotTradeSellerReceiveQuote {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货清结算为卖方释放冻结 base。
    SettleSpotTradeSellerReleaseFrozenBase {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货成交同步清结算的单条资金腿流水。
    SettleSpotTrade {
        /// 本次流水对应的成交 ID。
        trade_id: String,
        /// 本次流水对应的撮合批次 ID。
        match_id: String,
        /// 本次流水所属清结算批次 ID。
        settlement_batch_id: String,
        /// 本次流水表达的清结算资金腿用途。
        purpose: SettlementTransferPurpose,
    },
    /// perp funding 结算按账户聚合后的保证金余额流水。
    SettlePerpFunding {
        /// 资金费批次 ID。
        funding_batch_id: String,
        /// 本次余额变化对应的 funding settlement id 列表。
        settlement_ids: Vec<String>,
        /// 本次余额变化涉及的仓位 ID 列表。
        position_ids: Vec<String>,
    },
}

impl BalanceLedgerReason {
    /// 返回稳定原因编码，供 replay event / 审计查询使用。
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FreezeForOrder { .. } => "freeze_for_order",
            Self::UnfreezeForCancel { .. } => "unfreeze_for_cancel",
            Self::ReserveForImmediateOrder { .. } => "reserve_for_immediate_order",
            Self::CancelSpotOrderReleaseQuote { .. } => "cancel_spot_order_release_quote",
            Self::CancelSpotOrderReleaseBase { .. } => "cancel_spot_order_release_base",
            Self::SettleSpotTradeBuyerReceiveBase { .. } => "settle_spot_trade_buyer_receive_base",
            Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. } => {
                "settle_spot_trade_buyer_release_frozen_quote"
            }
            Self::SettleSpotTradeSellerReceiveQuote { .. } => {
                "settle_spot_trade_seller_receive_quote"
            }
            Self::SettleSpotTradeSellerReleaseFrozenBase { .. } => {
                "settle_spot_trade_seller_release_frozen_base"
            }
            Self::SettleSpotTrade { .. } => "settle_spot_trade",
            Self::SettlePerpFunding { .. } => "settle_perp_funding",
        }
    }

    /// 返回关联订单 ID；非下单冻结场景返回 `None`。
    pub fn order_id(&self) -> Option<&str> {
        match self {
            Self::FreezeForOrder { order_id }
            | Self::UnfreezeForCancel { order_id }
            | Self::ReserveForImmediateOrder { order_id }
            | Self::CancelSpotOrderReleaseQuote { order_id }
            | Self::CancelSpotOrderReleaseBase { order_id } => Some(order_id.as_str()),
            Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. }
            | Self::SettleSpotTrade { .. }
            | Self::SettlePerpFunding { .. } => None,
        }
    }

    /// 返回关联 trade id 列表；非成交清结算场景返回空切片。
    pub fn trade_ids(&self) -> &[String] {
        match self {
            Self::FreezeForOrder { .. }
            | Self::UnfreezeForCancel { .. }
            | Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettlePerpFunding { .. } => &[],
            Self::SettleSpotTrade { trade_id, .. } => std::slice::from_ref(trade_id),
            Self::SettleSpotTradeBuyerReceiveBase { trade_ids, .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { trade_ids, .. }
            | Self::SettleSpotTradeSellerReceiveQuote { trade_ids, .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { trade_ids, .. } => trade_ids,
        }
    }

    /// 返回关联 settlement id 列表；非成交清结算场景返回空切片。
    pub fn settlement_ids(&self) -> &[String] {
        match self {
            Self::FreezeForOrder { .. }
            | Self::UnfreezeForCancel { .. }
            | Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettleSpotTrade { .. } => &[],
            Self::SettleSpotTradeBuyerReceiveBase { settlement_ids, .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { settlement_ids, .. }
            | Self::SettleSpotTradeSellerReceiveQuote { settlement_ids, .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { settlement_ids, .. }
            | Self::SettlePerpFunding { settlement_ids, .. } => settlement_ids,
        }
    }

    /// 返回 funding batch id；非 funding 场景返回 `None`。
    pub fn funding_batch_id(&self) -> Option<&str> {
        match self {
            Self::SettlePerpFunding { funding_batch_id, .. } => Some(funding_batch_id.as_str()),
            Self::FreezeForOrder { .. }
            | Self::UnfreezeForCancel { .. }
            | Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. }
            | Self::SettleSpotTrade { .. } => None,
        }
    }

    /// 返回 settlement batch id；非现货成交资金腿场景返回 `None`。
    pub fn settlement_batch_id(&self) -> Option<&str> {
        match self {
            Self::SettleSpotTrade { settlement_batch_id, .. } => Some(settlement_batch_id.as_str()),
            _ => None,
        }
    }

    /// 返回结算资金腿用途；非现货成交资金腿场景返回 `None`。
    pub const fn settlement_purpose(&self) -> Option<SettlementTransferPurpose> {
        match self {
            Self::SettleSpotTrade { purpose, .. } => Some(*purpose),
            _ => None,
        }
    }

    /// 返回 funding 涉及的 position id 列表；非 funding 场景返回空切片。
    pub fn position_ids(&self) -> &[String] {
        match self {
            Self::SettlePerpFunding { position_ids, .. } => position_ids,
            Self::FreezeForOrder { .. }
            | Self::UnfreezeForCancel { .. }
            | Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. }
            | Self::SettleSpotTrade { .. } => &[],
        }
    }
}
