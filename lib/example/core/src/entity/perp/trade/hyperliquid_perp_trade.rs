use std::cmp::Ordering;

use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff};

use crate::HyperliquidPerpOrderSide;
use crate::entity::{
    SettlementKind, SettlementTransferLeg, SettlementTransferPurpose, SettlementTransferVoucher,
};

const HYPERLIQUID_PERP_TRADE_ENTITY_TYPE: u8 = 10;

/// 已完成撮合的一笔 Hyperliquid perp 成交事实。
///
/// 该实体只记录订单撮合结果，不表达仓位、保证金、手续费、PnL 或资金费。
/// 构造器假定输入已经由撮合 use case 校验。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpTrade {
    /// 本系统稳定成交 ID。
    pub trade_id: String,
    /// 一次撮合批次 ID。
    pub match_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 主动吃单订单 ID。
    pub taker_order_id: String,
    /// 被动挂单订单 ID。
    pub maker_order_id: String,
    /// taker 所属账户 ID。
    pub taker_account_id: String,
    /// maker 所属账户 ID。
    pub maker_account_id: String,
    /// taker 买卖方向。
    pub taker_side: HyperliquidPerpOrderSide,
    /// 成交价格，取 maker 限价。
    pub price: u64,
    /// 成交数量。
    pub qty: u64,
    /// 成交发生时间，单位毫秒。
    pub executed_at_ms: u64,
}

impl HyperliquidPerpTrade {
    /// 从已经校验过的撮合事实构造成交实体。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trade_id: String,
        match_id: String,
        asset: u32,
        symbol: String,
        taker_order_id: String,
        maker_order_id: String,
        taker_account_id: String,
        maker_account_id: String,
        taker_side: HyperliquidPerpOrderSide,
        price: u64,
        qty: u64,
        executed_at_ms: u64,
    ) -> Self {
        Self {
            trade_id,
            match_id,
            asset,
            symbol,
            taker_order_id,
            maker_order_id,
            taker_account_id,
            maker_account_id,
            taker_side,
            price,
            qty,
            executed_at_ms,
        }
    }

    /// 返回成交 quote 名义价值；乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.price.checked_mul(self.qty)
    }

    /// 从永续成交事实与 settlement outcome 派生清结算转账凭证。
    ///
    /// 该凭证只承接“账户间转账”语义：
    /// - 一正一负对冲的 realized pnl，生成 1 条账户间 PnL transfer leg
    /// - taker / maker fee 各自向 `fee_account_id` 支付手续费
    /// - 同账户内 `available / frozen` 的保证金重分类，不在这里建模成 transfer leg
    ///
    /// fee 与 realized pnl 来自 settlement outcome，不属于原始撮合事实，因此由调用方传入。
    /// 若 realized pnl 正负绝对值不一致，或 `i64` 绝对值转换失败，则返回 `None`。
    #[allow(clippy::too_many_arguments)]
    pub fn derive_perp_settlement_transfer_voucher(
        &self,
        voucher_id: String,
        settlement_id: String,
        margin_asset_id: &str,
        fee_account_id: String,
        taker_fee: u64,
        maker_fee: u64,
        taker_realized_pnl: i64,
        maker_realized_pnl: i64,
    ) -> Option<SettlementTransferVoucher> {
        let mut legs = Vec::new();

        match (taker_realized_pnl.cmp(&0), maker_realized_pnl.cmp(&0)) {
            (Ordering::Greater, Ordering::Less) => {
                let profit = u64::try_from(taker_realized_pnl).ok()?;
                let loss = abs_i64_to_u64(maker_realized_pnl)?;
                if profit != loss {
                    return None;
                }
                legs.push(SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.maker_account_id,
                        self.taker_account_id
                    ),
                    self.maker_account_id.clone(),
                    self.taker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    format!(
                        "balance-ledger:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.maker_account_id,
                        self.taker_account_id
                    ),
                ));
            }
            (Ordering::Less, Ordering::Greater) => {
                let profit = u64::try_from(maker_realized_pnl).ok()?;
                let loss = abs_i64_to_u64(taker_realized_pnl)?;
                if profit != loss {
                    return None;
                }
                legs.push(SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.taker_account_id,
                        self.maker_account_id
                    ),
                    self.taker_account_id.clone(),
                    self.maker_account_id.clone(),
                    margin_asset_id.to_string(),
                    profit,
                    SettlementTransferPurpose::PerpRealizedPnlTransfer,
                    format!(
                        "balance-ledger:{}:{}:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::PerpRealizedPnlTransfer.as_str(),
                        self.taker_account_id,
                        self.maker_account_id
                    ),
                ));
            }
            _ => {}
        }

        if taker_fee > 0 {
            legs.push(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:taker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
                self.taker_account_id.clone(),
                fee_account_id.clone(),
                margin_asset_id.to_string(),
                taker_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:taker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
            ));
        }

        if maker_fee > 0 {
            legs.push(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:maker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
                self.maker_account_id.clone(),
                fee_account_id.clone(),
                margin_asset_id.to_string(),
                maker_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:maker",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str()
                ),
            ));
        }

        Some(SettlementTransferVoucher::new(
            voucher_id,
            SettlementKind::Perp,
            settlement_id,
            self.trade_id.clone(),
            Some(self.match_id.clone()),
            fee_account_id,
            legs,
        ))
    }
}

impl FieldDiff for HyperliquidPerpTrade {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("trade_id", "", self.trade_id.clone()),
            EntityFieldChange::new("match_id", "", self.match_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("taker_order_id", "", self.taker_order_id.clone()),
            EntityFieldChange::new("maker_order_id", "", self.maker_order_id.clone()),
            EntityFieldChange::new("taker_account_id", "", self.taker_account_id.clone()),
            EntityFieldChange::new("maker_account_id", "", self.maker_account_id.clone()),
            EntityFieldChange::new("taker_side", "", self.taker_side.as_str()),
            EntityFieldChange::new("price", "", self.price.to_string()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("executed_at_ms", "", self.executed_at_ms.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for HyperliquidPerpTrade {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.trade_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_TRADE_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "trade_id" | "match_id" | "symbol" | "taker_order_id" | "maker_order_id"
            | "taker_account_id" | "maker_account_id" | "taker_side" => 0,
            "asset" | "price" | "qty" | "executed_at_ms" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.trade_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn abs_i64_to_u64(value: i64) -> Option<u64> {
    value.checked_abs().and_then(|abs| u64::try_from(abs).ok())
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn trade() -> HyperliquidPerpTrade {
        HyperliquidPerpTrade::new(
            "match-1-1".to_string(),
            "match-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            "taker-1".to_string(),
            "maker-1".to_string(),
            "winner".to_string(),
            "loser".to_string(),
            HyperliquidPerpOrderSide::Buy,
            100,
            2,
            1_717_171_717_000,
        )
    }

    #[test]
    fn create_event_contains_trade_fields() {
        let trade = trade();

        let event = trade.track_create_event().unwrap();

        assert!(event.is_created());
        assert_eq!(trade.notional_quote(), Some(200));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("trade_id")
                && change.new_value_bytes() == b"match-1-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("executed_at_ms")
                && change.new_value_bytes() == b"1717171717000"
        }));
    }

    #[test]
    fn derive_perp_settlement_transfer_voucher_creates_pnl_and_fee_legs() {
        let voucher = trade()
            .derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                3,
                2,
                25,
                -25,
            )
            .unwrap();

        let pnl_legs =
            voucher.transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer);
        let fee_legs = voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee);

        assert_eq!(voucher.settlement_kind(), SettlementKind::Perp);
        assert_eq!(pnl_legs.len(), 1);
        assert_eq!(pnl_legs[0].from_account_id(), "loser");
        assert_eq!(pnl_legs[0].to_account_id(), "winner");
        assert_eq!(pnl_legs[0].amount(), 25);
        assert_eq!(fee_legs.len(), 2);
        assert_eq!(voucher.fee_amount_paid_by("winner"), Some(3));
        assert_eq!(voucher.fee_amount_paid_by("loser"), Some(2));
    }

    #[test]
    fn derive_perp_settlement_transfer_voucher_skips_zero_pnl_and_fee() {
        let voucher = trade()
            .derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                0,
                0,
                0,
                0,
            )
            .unwrap();

        assert!(
            voucher
                .transfers_for_purpose(SettlementTransferPurpose::PerpRealizedPnlTransfer)
                .is_empty()
        );
        assert!(voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee).is_empty());
    }

    #[test]
    fn derive_perp_settlement_transfer_voucher_returns_none_for_bad_pnl_pair() {
        assert_eq!(
            trade().derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                0,
                0,
                25,
                -24,
            ),
            None
        );
        assert_eq!(
            trade().derive_perp_settlement_transfer_voucher(
                "voucher-1".to_string(),
                "settle-1".to_string(),
                "USDC",
                "fee-account".to_string(),
                0,
                0,
                25,
                i64::MIN,
            ),
            None
        );
    }
}
