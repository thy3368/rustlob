use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype, MiCausalRelation, MiCausalSourceMetadata,
};
use serde::{Deserialize, Serialize};

use crate::entity::{
    SettlementKind, SettlementTransferLeg, SettlementTransferPurpose, SettlementTransferVoucher,
};
use crate::{SpotOrderSide, SpotTradeFeeRole};

#[cfg(test)]
mod spot_trade_bdd_notional_quote;
#[cfg(test)]
mod spot_trade_bdd_settlement_transfer_voucher;

const SPOT_TRADE_ENTITY_TYPE: u8 = 5;

/// 已完成撮合的一笔现货成交事实。
///
/// `SpotTrade` 只记录订单撮合结果和已确定的撮合角色手续费金额，不表达账户清算或资产划转。
/// 构造器假定输入已经由撮合 use case 校验。
/// 在聚合边界上，它自身作为一条独立的成交事实聚合根存在。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotTrade {
    /// 本系统稳定成交 ID。
    pub trade_id: String,
    /// 一次撮合批次 ID。
    pub match_id: String,
    /// Hyperliquid 现货资产编号。
    pub asset: u32,
    /// 交易对，例如 `BTCUSDT`。
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
    pub taker_side: SpotOrderSide,
    /// 成交价格，取 maker 限价。
    pub price: u64,
    /// 成交数量，以 base asset 计价。
    pub qty: u64,
    /// taker 为本笔成交支付的 quote 手续费金额。
    pub taker_fee: u64,
    /// maker 为本笔成交支付的 quote 手续费金额。
    pub maker_fee: u64,
}

impl SpotTrade {
    /// 从已经校验过的撮合事实构造成交实体。
    pub fn new(
        trade_id: String,
        match_id: String,
        asset: u32,
        symbol: String,
        taker_order_id: String,
        maker_order_id: String,
        taker_account_id: String,
        maker_account_id: String,
        taker_side: SpotOrderSide,
        price: u64,
        qty: u64,
        taker_fee: u64,
        maker_fee: u64,
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
            taker_fee,
            maker_fee,
        }
    }

    /// 返回成交 quote 名义价值；乘法溢出时返回 `None`。
    pub fn notional_quote(&self) -> Option<u64> {
        self.price.checked_mul(self.qty)
    }

    /// 返回清结算中的买方账户 ID。
    pub fn buyer_account_id(&self) -> &str {
        match self.taker_side {
            SpotOrderSide::Buy => self.taker_account_id.as_str(),
            SpotOrderSide::Sell => self.maker_account_id.as_str(),
        }
    }

    /// 返回清结算中的卖方账户 ID。
    pub fn seller_account_id(&self) -> &str {
        match self.taker_side {
            SpotOrderSide::Buy => self.maker_account_id.as_str(),
            SpotOrderSide::Sell => self.taker_account_id.as_str(),
        }
    }

    /// 返回买方订单 ID。
    pub fn buyer_order_id(&self) -> &str {
        match self.taker_side {
            SpotOrderSide::Buy => self.taker_order_id.as_str(),
            SpotOrderSide::Sell => self.maker_order_id.as_str(),
        }
    }

    /// 返回卖方订单 ID。
    pub fn seller_order_id(&self) -> &str {
        match self.taker_side {
            SpotOrderSide::Buy => self.maker_order_id.as_str(),
            SpotOrderSide::Sell => self.taker_order_id.as_str(),
        }
    }

    /// 返回给定账户在该成交中的真实 fee 角色。
    pub fn fee_role_for_account(&self, account_id: &str) -> Option<SpotTradeFeeRole> {
        if self.taker_account_id == account_id {
            Some(SpotTradeFeeRole::Taker)
        } else if self.maker_account_id == account_id {
            Some(SpotTradeFeeRole::Maker)
        } else {
            None
        }
    }

    /// 返回买方在该成交中的真实 fee 角色。
    pub fn buyer_fee_role(&self) -> SpotTradeFeeRole {
        match self.taker_side {
            SpotOrderSide::Buy => SpotTradeFeeRole::Taker,
            SpotOrderSide::Sell => SpotTradeFeeRole::Maker,
        }
    }

    /// 返回卖方在该成交中的真实 fee 角色。
    pub fn seller_fee_role(&self) -> SpotTradeFeeRole {
        match self.taker_side {
            SpotOrderSide::Buy => SpotTradeFeeRole::Maker,
            SpotOrderSide::Sell => SpotTradeFeeRole::Taker,
        }
    }

    /// 返回买方在本笔成交中支付的 quote 手续费金额。
    pub fn buyer_fee(&self) -> u64 {
        match self.taker_side {
            SpotOrderSide::Buy => self.taker_fee,
            SpotOrderSide::Sell => self.maker_fee,
        }
    }

    /// 返回卖方在本笔成交中支付的 quote 手续费金额。
    pub fn seller_fee(&self) -> u64 {
        match self.taker_side {
            SpotOrderSide::Buy => self.maker_fee,
            SpotOrderSide::Sell => self.taker_fee,
        }
    }

    /// 从现货成交事实派生 principal-only settlement transfer voucher。
    ///
    /// 当前只生成 4 条 principal legs，不包含任何手续费腿。
    /// 若 `price * qty` 溢出，则返回 `None`。
    pub fn derive_spot_principal_settlement_transfer_voucher(
        &self,
        voucher_id: String,
        settlement_id: String,
        base_asset_id: &str,
        quote_asset_id: &str,
        fee_account_id: String,
    ) -> Option<SettlementTransferVoucher> {
        let quote_amount = self.notional_quote()?;
        let buyer_account_id = self.buyer_account_id();
        let seller_account_id = self.seller_account_id();

        Some(SettlementTransferVoucher::new(
            voucher_id,
            SettlementKind::Spot,
            settlement_id.clone(),
            self.trade_id.clone(),
            Some(self.match_id.clone()),
            fee_account_id,
            vec![
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerReceiveBase.as_str()
                    ),
                    seller_account_id.to_string(),
                    buyer_account_id.to_string(),
                    base_asset_id.to_string(),
                    self.qty,
                    SettlementTransferPurpose::SpotBuyerReceiveBase,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerReceiveBase.as_str()
                    ),
                ),
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerPayQuote.as_str()
                    ),
                    buyer_account_id.to_string(),
                    seller_account_id.to_string(),
                    quote_asset_id.to_string(),
                    quote_amount,
                    SettlementTransferPurpose::SpotBuyerPayQuote,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotBuyerPayQuote.as_str()
                    ),
                ),
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerReceiveQuote.as_str()
                    ),
                    buyer_account_id.to_string(),
                    seller_account_id.to_string(),
                    quote_asset_id.to_string(),
                    quote_amount,
                    SettlementTransferPurpose::SpotSellerReceiveQuote,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerReceiveQuote.as_str()
                    ),
                ),
                SettlementTransferLeg::new(
                    format!(
                        "settlement-leg:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerDeliverBase.as_str()
                    ),
                    seller_account_id.to_string(),
                    buyer_account_id.to_string(),
                    base_asset_id.to_string(),
                    self.qty,
                    SettlementTransferPurpose::SpotSellerDeliverBase,
                    format!(
                        "balance-ledger:{}:{}",
                        settlement_id,
                        SettlementTransferPurpose::SpotSellerDeliverBase.as_str()
                    ),
                ),
            ],
        ))
    }

    /// 从现货成交事实派生包含 principal 与 fee 的 settlement transfer voucher。
    ///
    /// fee 统一以 quote 资产计价，且买卖双方各自向 `fee_account_id` 支付自己真实角色下的 fee。
    /// fee 金额来自本成交事实的 `buyer_fee()` / `seller_fee()`。
    /// 若 quote notional 溢出则返回 `None`。
    pub fn derive_spot_settlement_transfer_voucher_with_fees(
        &self,
        voucher_id: String,
        settlement_id: String,
        base_asset_id: &str,
        quote_asset_id: &str,
        fee_account_id: String,
    ) -> Option<SettlementTransferVoucher> {
        let mut voucher = self.derive_spot_principal_settlement_transfer_voucher(
            voucher_id,
            settlement_id.clone(),
            base_asset_id,
            quote_asset_id,
            fee_account_id.clone(),
        )?;

        let buyer_fee = self.buyer_fee();
        if buyer_fee > 0 {
            voucher.push_leg(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    self.buyer_fee_role().as_str()
                ),
                self.buyer_account_id().to_string(),
                fee_account_id.clone(),
                quote_asset_id.to_string(),
                buyer_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    self.buyer_fee_role().as_str()
                ),
            ));
        }

        let seller_fee = self.seller_fee();
        if seller_fee > 0 {
            voucher.push_leg(SettlementTransferLeg::new(
                format!(
                    "settlement-leg:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    self.seller_fee_role().as_str()
                ),
                self.seller_account_id().to_string(),
                fee_account_id,
                quote_asset_id.to_string(),
                seller_fee,
                SettlementTransferPurpose::TradingFee,
                format!(
                    "balance-ledger:{}:{}:{}",
                    settlement_id,
                    SettlementTransferPurpose::TradingFee.as_str(),
                    self.seller_fee_role().as_str()
                ),
            ));
        }

        Some(voucher)
    }
}

impl FieldDiff for SpotTrade {
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
            EntityFieldChange::new("taker_fee", "", self.taker_fee.to_string()),
            EntityFieldChange::new("maker_fee", "", self.maker_fee.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for SpotTrade {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.trade_id.clone()
    }

    fn entity_type() -> u8 {
        SPOT_TRADE_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel
    where
        Self: Sized,
    {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::BusinessVoucher
    }

    fn mi_causal_sources() -> &'static [MiCausalSourceMetadata]
    where
        Self: Sized,
    {
        &[
            MiCausalSourceMetadata {
                source_fact_type: "spot_order",
                relation: MiCausalRelation::CausedBy,
                source_role: "taker_order",
            },
            MiCausalSourceMetadata {
                source_fact_type: "spot_order",
                relation: MiCausalRelation::DueTo,
                source_role: "maker_order",
            },
        ]
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "trade_id" | "match_id" | "symbol" | "taker_order_id" | "maker_order_id"
            | "taker_account_id" | "maker_account_id" | "taker_side" => 0,
            "asset" | "price" | "qty" | "taker_fee" | "maker_fee" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_trade_entity_id(&self.trade_id))
    }
}

fn stable_trade_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn trade() -> SpotTrade {
        SpotTrade::new(
            "match-1-1".to_string(),
            "match-1".to_string(),
            10_001,
            "BTCUSDT".to_string(),
            "taker-1".to_string(),
            "maker-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            SpotOrderSide::Buy,
            100,
            2,
            2,
            1,
        )
    }

    #[test]
    fn constructor_stores_trade_facts() {
        let trade = trade();

        assert_eq!(trade.trade_id, "match-1-1");
        assert_eq!(trade.match_id, "match-1");
        assert_eq!(trade.taker_side, SpotOrderSide::Buy);
        assert_eq!(trade.notional_quote(), Some(200));
        assert_eq!(trade.buyer_account_id(), "buyer");
        assert_eq!(trade.seller_account_id(), "seller");
        assert_eq!(trade.buyer_fee_role(), SpotTradeFeeRole::Taker);
        assert_eq!(trade.seller_fee_role(), SpotTradeFeeRole::Maker);
        assert_eq!(trade.taker_fee, 2);
        assert_eq!(trade.maker_fee, 1);
        assert_eq!(trade.buyer_fee(), 2);
        assert_eq!(trade.seller_fee(), 1);
        assert_eq!(trade.fee_role_for_account("buyer"), Some(SpotTradeFeeRole::Taker));
        assert_eq!(trade.fee_role_for_account("seller"), Some(SpotTradeFeeRole::Maker));
        assert_eq!(trade.fee_role_for_account("outsider"), None);
    }

    #[test]
    fn spot_trade_declares_mi_causal_source_metadata() {
        assert_eq!(SpotTrade::four_color_archetype(), FourColorArchetype::MomentInterval);
        assert_eq!(SpotTrade::mutation_model(), EntityMutationModel::AppendOnlyRecord);
        assert_eq!(SpotTrade::aggregate_role(), AggregateRole::AggregateRoot);
        assert_eq!(SpotTrade::financial_classification(), FinancialClassification::BusinessVoucher);
        assert!(!SpotTrade::is_mi_chain_root());
        assert_eq!(
            SpotTrade::mi_causal_sources(),
            &[
                MiCausalSourceMetadata {
                    source_fact_type: "spot_order",
                    relation: MiCausalRelation::CausedBy,
                    source_role: "taker_order",
                },
                MiCausalSourceMetadata {
                    source_fact_type: "spot_order",
                    relation: MiCausalRelation::DueTo,
                    source_role: "maker_order",
                },
            ]
        );
    }

    #[test]
    fn buyer_and_seller_account_ids_follow_taker_side() {
        let buy_taker = trade();
        assert_eq!(buy_taker.buyer_account_id(), "buyer");
        assert_eq!(buy_taker.seller_account_id(), "seller");

        let sell_taker = SpotTrade::new(
            "match-2-1".to_string(),
            "match-2".to_string(),
            10_001,
            "BTCUSDT".to_string(),
            "taker-2".to_string(),
            "maker-2".to_string(),
            "seller".to_string(),
            "buyer".to_string(),
            SpotOrderSide::Sell,
            100,
            2,
            2,
            1,
        );
        assert_eq!(sell_taker.buyer_account_id(), "buyer");
        assert_eq!(sell_taker.seller_account_id(), "seller");
        assert_eq!(sell_taker.buyer_fee(), 1);
        assert_eq!(sell_taker.seller_fee(), 2);
    }

    #[test]
    fn create_event_contains_trade_fields() {
        let event = trade().track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("trade_id")
                && change.new_value_bytes() == b"match-1-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("price") && change.new_value_bytes() == b"100"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("taker_fee") && change.new_value_bytes() == b"2"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("maker_fee") && change.new_value_bytes() == b"1"
        }));
    }
}
