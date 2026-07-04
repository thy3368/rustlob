use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel,
    FinancialClassification, FourColorArchetype,
};

use crate::entity::HyperliquidPerpPositionSide;

const HYPERLIQUID_PERP_FUNDING_SETTLEMENT_ENTITY_TYPE: u8 = 13;

/// 一次 Hyperliquid perp 资金费结算产生的账户级事实。
///
/// 该实体只记录资金费事实本身；保证金余额变化由资金费结算 use case 另外发出
/// `Balance` update event。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpFundingSettlement {
    /// 本系统稳定资金费结算记录 ID。
    pub funding_settlement_id: String,
    /// 资金费结算批次 ID。
    pub funding_batch_id: String,
    /// 被结算账户 ID。
    pub account_id: String,
    /// 被结算仓位 ID。
    pub position_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 本次 funding 真相对应的时间锚点。
    pub funding_time: u64,
    /// 被结算仓位方向。
    pub side: HyperliquidPerpPositionSide,
    /// 被结算仓位数量。
    pub qty: u64,
    /// 本次资金费使用的 oracle 价格。
    pub oracle_price: u64,
    /// 本次资金费使用的名义价值。
    pub notional: u64,
    /// signed 1e8 刻度资金费率；`0.01% = 10000`。
    pub funding_rate_e8: i64,
    /// 本账户本次应收或应付的资金费绝对金额。
    pub funding_fee: u64,
    /// `true` 表示本账户付款，`false` 表示本账户收款。
    pub is_payment: bool,
}

impl HyperliquidPerpFundingSettlement {
    /// 从已经校验过的资金费结算事实构造记录。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        funding_settlement_id: String,
        funding_batch_id: String,
        account_id: String,
        position_id: String,
        asset: u32,
        symbol: String,
        funding_time: u64,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        oracle_price: u64,
        notional: u64,
        funding_rate_e8: i64,
        funding_fee: u64,
        is_payment: bool,
    ) -> Self {
        Self {
            funding_settlement_id,
            funding_batch_id,
            account_id,
            position_id,
            asset,
            symbol,
            funding_time,
            side,
            qty,
            oracle_price,
            notional,
            funding_rate_e8,
            funding_fee,
            is_payment,
        }
    }
}

impl Entity for HyperliquidPerpFundingSettlement {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.funding_settlement_id.clone()
    }

    fn entity_type() -> u8 {
        HYPERLIQUID_PERP_FUNDING_SETTLEMENT_ENTITY_TYPE
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

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("funding_settlement_id", "", self.funding_settlement_id.clone()),
            EntityFieldChange::new("funding_batch_id", "", self.funding_batch_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("asset", "", self.asset.to_string()),
            EntityFieldChange::new("symbol", "", self.symbol.clone()),
            EntityFieldChange::new("funding_time", "", self.funding_time.to_string()),
            EntityFieldChange::new("side", "", self.side.as_str()),
            EntityFieldChange::new("qty", "", self.qty.to_string()),
            EntityFieldChange::new("oracle_price", "", self.oracle_price.to_string()),
            EntityFieldChange::new("notional", "", self.notional.to_string()),
            EntityFieldChange::new("funding_rate_e8", "", self.funding_rate_e8.to_string()),
            EntityFieldChange::new("funding_fee", "", self.funding_fee.to_string()),
            EntityFieldChange::new("is_payment", "", self.is_payment.to_string()),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "funding_settlement_id"
            | "funding_batch_id"
            | "account_id"
            | "position_id"
            | "symbol"
            | "side"
            | "is_payment" => 0,
            "asset" | "funding_time" | "qty" | "oracle_price" | "notional" | "funding_rate_e8"
            | "funding_fee" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_entity_id(&self.funding_settlement_id))
    }
}

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    #[test]
    fn create_event_contains_funding_fields() {
        let settlement = HyperliquidPerpFundingSettlement::new(
            "funding-1-position-1".to_string(),
            "funding-1".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            1_717_977_600_000,
            HyperliquidPerpPositionSide::Long,
            2,
            50_000,
            100_000,
            10_000,
            10,
            true,
        );

        let event = settlement.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("funding_settlement_id")
                && change.new_value_bytes() == b"funding-1-position-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("funding_rate_e8")
                && change.new_value_bytes() == b"10000"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("oracle_price")
                && change.new_value_bytes() == b"50000"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("funding_time")
                && change.new_value_bytes() == b"1717977600000"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("is_payment")
                && change.new_value_bytes() == b"true"
        }));
    }

    #[test]
    fn funding_settlement_declares_business_voucher_metadata() {
        assert_eq!(
            HyperliquidPerpFundingSettlement::four_color_archetype(),
            FourColorArchetype::MomentInterval
        );
        assert_eq!(
            HyperliquidPerpFundingSettlement::mutation_model(),
            EntityMutationModel::AppendOnlyRecord
        );
        assert_eq!(
            HyperliquidPerpFundingSettlement::aggregate_role(),
            AggregateRole::AggregateRoot
        );
        assert_eq!(
            HyperliquidPerpFundingSettlement::financial_classification(),
            FinancialClassification::BusinessVoucher
        );
        assert!(!HyperliquidPerpFundingSettlement::is_mi_chain_root());
    }
}
