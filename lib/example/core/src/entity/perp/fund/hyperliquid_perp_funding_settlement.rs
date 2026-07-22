use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype,
};

use crate::entity::{BalanceLedgerEntry, BalanceLedgerEntryV2Error, BalanceLedgerReason};

const HYPERLIQUID_PERP_FUNDING_SETTLEMENT_ENTITY_TYPE: u8 = 13;

/// 一次 Hyperliquid perp 仓位级资金费结算事实。
///
/// 这是 funding use case 组内部的主业务真相，承接
/// `position + oracle_price + funding_rate -> funding settlement` 的可回放、可审计结果。
/// 它对齐 Hyperliquid `userFunding` 的 `coin`、`delta.szi`、`delta.usdc`、
/// `delta.nSamples` 与 `hash` 账本事实，属于
/// `Moment-Interval + AppendOnlyRecord + AggregateRoot + BusinessVoucher`。
///
/// 该实体不表达 payer-to-receiver 对手方配对转账腿，也不直接持有账本流水引用。
/// 保证金余额变化与 `BalanceLedgerEntry` 由资金费结算 use case 另外产出。
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
    /// Hyperliquid `userFunding.delta.coin` 合约名。
    pub coin: String,
    /// 本次 funding 真相对应的时间锚点。
    pub funding_time: u64,
    /// Hyperliquid `userFunding.delta.szi` 有符号仓位数量，long 为正、short 为负。
    pub signed_size: i128,
    /// 本次资金费使用的 oracle 价格。
    pub oracle_price: u64,
    /// signed 1e8 刻度资金费率；`0.01% = 10000`。
    pub funding_rate_e8: i64,
    /// Hyperliquid `userFunding.delta.usdc` 有符号 USDC 变动；正数表示收款，负数表示付款。
    pub signed_usdc_delta: i128,
    /// Hyperliquid `userFunding.hash`，从官方账本导入时用于对账。
    pub source_hash: Option<String>,
    /// Hyperliquid `userFunding.delta.nSamples`，从官方账本导入时用于对账。
    pub n_samples: Option<u32>,
}

impl HyperliquidPerpFundingSettlement {
    /// 从已经推导完成或回放出的资金费结算事实组装记录。
    ///
    /// 该构造器不承担业务校验，也不推导资金费方向与金额；这些规则应由 use case
    /// 或上游已校验的业务流程完成后再传入。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        funding_settlement_id: String,
        funding_batch_id: String,
        account_id: String,
        position_id: String,
        coin: String,
        funding_time: u64,
        signed_size: i128,
        oracle_price: u64,
        funding_rate_e8: i64,
        signed_usdc_delta: i128,
        source_hash: Option<String>,
        n_samples: Option<u32>,
    ) -> Self {
        Self {
            funding_settlement_id,
            funding_batch_id,
            account_id,
            position_id,
            coin,
            funding_time,
            signed_size,
            oracle_price,
            funding_rate_e8,
            signed_usdc_delta,
            source_hash,
            n_samples,
        }
    }

    /// 返回本次 funding USDC 变动的绝对金额。
    pub fn funding_fee_abs(&self) -> Option<u64> {
        u64::try_from(self.signed_usdc_delta.unsigned_abs()).ok()
    }

    /// 返回本账户本次 funding 是否付款。
    pub fn is_payment(&self) -> bool {
        self.signed_usdc_delta < 0
    }

    /// 可 BDD 规格化的聚合根行为：从单条 funding settlement 派生保证金余额流水。
    ///
    /// 该方法只表达 `FundingSettlement -> BalanceLedgerEntry` 的直接下游单据链：
    /// 本账户应付 funding 时扣减可用保证金，本账户应收 funding 时增加可用保证金。
    /// 它不应用余额、不聚合多条 settlement，也不负责持久化或发布。
    pub fn derive_margin_balance_ledger_entry(
        &self,
        entry_id: String,
        margin_asset_id: String,
        balance_entity_id: String,
    ) -> Result<BalanceLedgerEntry, BalanceLedgerEntryV2Error> {
        let reason = BalanceLedgerReason::SettlePerpFunding {
            funding_batch_id: self.funding_batch_id.clone(),
            settlement_ids: vec![self.funding_settlement_id.clone()],
            position_ids: vec![self.position_id.clone()],
        };

        let funding_fee = self.funding_fee_abs().ok_or(BalanceLedgerEntryV2Error::InvalidAmount)?;
        if self.is_payment() {
            BalanceLedgerEntry::debit_available(
                entry_id,
                self.account_id.clone(),
                margin_asset_id,
                balance_entity_id,
                funding_fee,
                reason,
            )
        } else {
            BalanceLedgerEntry::credit_available(
                entry_id,
                self.account_id.clone(),
                margin_asset_id,
                balance_entity_id,
                funding_fee,
                reason,
            )
        }
    }
}

impl FieldDiff for HyperliquidPerpFundingSettlement {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("funding_settlement_id", "", self.funding_settlement_id.clone()),
            EntityFieldChange::new("funding_batch_id", "", self.funding_batch_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("position_id", "", self.position_id.clone()),
            EntityFieldChange::new("coin", "", self.coin.clone()),
            EntityFieldChange::new("funding_time", "", self.funding_time.to_string()),
            EntityFieldChange::new("signed_size", "", self.signed_size.to_string()),
            EntityFieldChange::new("oracle_price", "", self.oracle_price.to_string()),
            EntityFieldChange::new("funding_rate_e8", "", self.funding_rate_e8.to_string()),
            EntityFieldChange::new("signed_usdc_delta", "", self.signed_usdc_delta.to_string()),
            EntityFieldChange::new("source_hash", "", self.source_hash.clone().unwrap_or_default()),
            EntityFieldChange::new(
                "n_samples",
                "",
                self.n_samples.map(|n_samples| n_samples.to_string()).unwrap_or_default(),
            ),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
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
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "funding_settlement_id"
            | "funding_batch_id"
            | "account_id"
            | "position_id"
            | "coin"
            | "source_hash" => 0,
            "funding_time" | "signed_size" | "oracle_price" | "funding_rate_e8"
            | "signed_usdc_delta" | "n_samples" => 1,
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
            "BTC-PERP".to_string(),
            1_717_977_600_000,
            2,
            50_000,
            10_000,
            -10,
            Some("0xabc".to_string()),
            Some(8),
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
            change.field_name_as_str().ok() == Some("coin")
                && change.new_value_bytes() == b"BTC-PERP"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("signed_size")
                && change.new_value_bytes() == b"2"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("signed_usdc_delta")
                && change.new_value_bytes() == b"-10"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("source_hash")
                && change.new_value_bytes() == b"0xabc"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("n_samples") && change.new_value_bytes() == b"8"
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
