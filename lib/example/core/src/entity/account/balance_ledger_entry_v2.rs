use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel, FourColorArchetype,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::balance::Balance;
use super::balance_ledger_entry::{BalanceLedgerReason, SpotSettlementLeg};

const BALANCE_LEDGER_ENTRY_ENTITY_TYPE: u8 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BalanceLedgerOperation {
    Freeze,
    Unfreeze,
    CreditAvailable,
    DebitAvailable,
    DebitFrozen,
}

impl BalanceLedgerOperation {
    fn label(self) -> &'static str {
        match self {
            Self::Freeze => "freeze",
            Self::Unfreeze => "unfreeze",
            Self::CreditAvailable => "credit_available",
            Self::DebitAvailable => "debit_available",
            Self::DebitFrozen => "debit_frozen",
        }
    }
}

/// 无状态机版余额流水错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceLedgerEntryV2Error {
    /// 金额必须大于零。
    #[error("balance ledger amount must be greater than zero")]
    InvalidAmount,
    /// 可用余额不足。
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    /// 冻结余额不足。
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    /// 余额状态计算发生整数溢出。
    #[error("arithmetic overflow while deriving balance ledger transition")]
    ArithmeticOverflow,
}

/// 一条 create-only 的余额流水审计事实。
///
/// 该实体记录某条 `Balance` 在单次业务动作前后的快照差异，不维护额外状态机。
/// 在聚合边界上，它自身作为一条独立的审计事实聚合根存在。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerEntryV2 {
    /// 流水记录 ID。
    pub entry_id: String,
    /// 账户 ID。
    pub account_id: String,
    /// 资产 ID。
    pub asset_id: String,
    /// 对应余额快照实体 ID。
    pub balance_entity_id: String,
    /// 变更前可用余额。
    pub before_available: u64,
    /// 变更前冻结余额。
    pub before_frozen: u64,
    /// 变更后可用余额。
    pub after_available: u64,
    /// 变更后冻结余额。
    pub after_frozen: u64,
    /// 余额变更原因。
    pub reason: BalanceLedgerReason,
}

impl BalanceLedgerEntryV2 {
    /// 冻结可用余额并返回已应用的流水事实。
    pub fn freeze(
        entry_id: String,
        balance: &mut Balance,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::apply(entry_id, balance, amount, reason, BalanceLedgerOperation::Freeze)
    }

    /// 解冻冻结余额并返回已应用的流水事实。
    pub fn unfreeze(
        entry_id: String,
        balance: &mut Balance,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::apply(entry_id, balance, amount, reason, BalanceLedgerOperation::Unfreeze)
    }

    /// 增加可用余额并返回已应用的流水事实。
    pub fn credit_available(
        entry_id: String,
        balance: &mut Balance,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::apply(entry_id, balance, amount, reason, BalanceLedgerOperation::CreditAvailable)
    }

    /// 扣减可用余额并返回已应用的流水事实。
    pub fn debit_available(
        entry_id: String,
        balance: &mut Balance,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::apply(entry_id, balance, amount, reason, BalanceLedgerOperation::DebitAvailable)
    }

    /// 扣减冻结余额并返回已应用的流水事实。
    pub fn debit_frozen(
        entry_id: String,
        balance: &mut Balance,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::apply(entry_id, balance, amount, reason, BalanceLedgerOperation::DebitFrozen)
    }

    fn apply(
        entry_id: String,
        balance: &mut Balance,
        amount: u64,
        reason: BalanceLedgerReason,
        operation: BalanceLedgerOperation,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        if amount == 0 {
            return Err(BalanceLedgerEntryV2Error::InvalidAmount);
        }

        let before_available = balance.available;
        let before_frozen = balance.frozen;
        let next_version =
            balance.version.checked_add(1).ok_or(BalanceLedgerEntryV2Error::ArithmeticOverflow)?;

        let (after_available, after_frozen) = match operation {
            BalanceLedgerOperation::Freeze => balance
                .reserve_after(amount)
                .ok_or(BalanceLedgerEntryV2Error::InsufficientAvailableBalance)?,
            BalanceLedgerOperation::Unfreeze => balance
                .release_after(amount)
                .ok_or(BalanceLedgerEntryV2Error::InsufficientFrozenBalance)?,
            BalanceLedgerOperation::CreditAvailable => (
                balance
                    .credit_available_after(amount)
                    .ok_or(BalanceLedgerEntryV2Error::ArithmeticOverflow)?,
                balance.frozen,
            ),
            BalanceLedgerOperation::DebitAvailable => (
                balance
                    .debit_available_after(amount)
                    .ok_or(BalanceLedgerEntryV2Error::InsufficientAvailableBalance)?,
                balance.frozen,
            ),
            BalanceLedgerOperation::DebitFrozen => (
                balance.available,
                balance
                    .debit_frozen_after(amount)
                    .ok_or(BalanceLedgerEntryV2Error::InsufficientFrozenBalance)?,
            ),
        };

        balance.apply_after(after_available, after_frozen, next_version);

        Ok(Self {
            entry_id,
            account_id: balance.account_id.clone(),
            asset_id: balance.asset_id.clone(),
            balance_entity_id: balance.entity_id(),
            before_available,
            before_frozen,
            after_available,
            after_frozen,
            reason,
        })
    }

    fn inferred_command_and_amount(&self) -> Option<(BalanceLedgerOperation, u64)> {
        let available_delta = i128::from(self.after_available) - i128::from(self.before_available);
        let frozen_delta = i128::from(self.after_frozen) - i128::from(self.before_frozen);

        let positive_abs = |delta: i128| -> Option<u64> {
            if delta > 0 { u64::try_from(delta).ok() } else { None }
        };
        let negative_abs = |delta: i128| -> Option<u64> {
            if delta < 0 {
                delta.checked_neg().and_then(|value| u64::try_from(value).ok())
            } else {
                None
            }
        };

        match (
            negative_abs(available_delta),
            positive_abs(available_delta),
            negative_abs(frozen_delta),
            positive_abs(frozen_delta),
        ) {
            (Some(amount), None, None, Some(frozen_amount)) if amount == frozen_amount => {
                Some((BalanceLedgerOperation::Freeze, amount))
            }
            (None, Some(amount), Some(frozen_amount), None) if amount == frozen_amount => {
                Some((BalanceLedgerOperation::Unfreeze, amount))
            }
            (None, Some(amount), None, None) => {
                Some((BalanceLedgerOperation::CreditAvailable, amount))
            }
            (Some(amount), None, None, None) => {
                Some((BalanceLedgerOperation::DebitAvailable, amount))
            }
            (None, None, Some(amount), None) => Some((BalanceLedgerOperation::DebitFrozen, amount)),
            _ => None,
        }
    }

    fn reason_settlement_leg(&self) -> Option<SpotSettlementLeg> {
        self.reason.settlement_leg()
    }
}

impl Entity for BalanceLedgerEntryV2 {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.entry_id.clone()
    }

    fn entity_type() -> u8 {
        BALANCE_LEDGER_ENTRY_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }

    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::AppendOnlyRecord
    }

    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let inferred = self.inferred_command_and_amount();
        let command = inferred.map(|(operation, _)| operation.label()).unwrap_or_default();
        let command_amount = inferred.map(|(_, amount)| amount).unwrap_or(0).to_string();

        vec![
            EntityFieldChange::new("entry_id", "", self.entry_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("balance_entity_id", "", self.balance_entity_id.clone()),
            EntityFieldChange::new("command", "", command),
            EntityFieldChange::new("command_amount", "", command_amount),
            EntityFieldChange::new("before_available", "", self.before_available.to_string()),
            EntityFieldChange::new("before_frozen", "", self.before_frozen.to_string()),
            EntityFieldChange::new("after_available", "", self.after_available.to_string()),
            EntityFieldChange::new("after_frozen", "", self.after_frozen.to_string()),
            EntityFieldChange::new("reason", "", self.reason.as_str()),
            EntityFieldChange::new(
                "reason_order_id",
                "",
                self.reason.order_id().unwrap_or_default(),
            ),
            EntityFieldChange::new("reason_trade_ids", "", self.reason.trade_ids().join(",")),
            EntityFieldChange::new(
                "reason_settlement_ids",
                "",
                self.reason.settlement_ids().join(","),
            ),
            EntityFieldChange::new(
                "reason_funding_batch_id",
                "",
                self.reason.funding_batch_id().unwrap_or_default(),
            ),
            EntityFieldChange::new(
                "reason_settlement_batch_id",
                "",
                self.reason.settlement_batch_id().unwrap_or_default(),
            ),
            EntityFieldChange::new(
                "reason_settlement_leg",
                "",
                self.reason_settlement_leg().map(|leg| leg.as_str()).unwrap_or_default(),
            ),
            EntityFieldChange::new("reason_position_ids", "", self.reason.position_ids().join(",")),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "entry_id"
            | "account_id"
            | "asset_id"
            | "balance_entity_id"
            | "command"
            | "command_amount"
            | "reason"
            | "reason_order_id"
            | "reason_trade_ids"
            | "reason_settlement_ids"
            | "reason_funding_batch_id"
            | "reason_settlement_batch_id"
            | "reason_settlement_leg"
            | "reason_position_ids" => 0,
            "before_available" | "before_frozen" | "after_available" | "after_frozen" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_balance_ledger_entry_id(&self.entry_id))
    }
}

fn stable_balance_ledger_entry_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn order_reason() -> BalanceLedgerReason {
        BalanceLedgerReason::FreezeForOrder { order_id: "order-1".to_string() }
    }

    #[test]
    fn freeze_updates_balance_and_returns_ledger_entry() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 100, 3);

        let entry =
            BalanceLedgerEntryV2::freeze("ledger-1".to_string(), &mut balance, 200, order_reason())
                .unwrap();

        assert_eq!(BalanceLedgerEntryV2::aggregate_role(), AggregateRole::AggregateRoot);
        assert_eq!(balance.available, 800);
        assert_eq!(balance.frozen, 300);
        assert_eq!(balance.version, 4);
        assert_eq!(entry.before_available, 1_000);
        assert_eq!(entry.before_frozen, 100);
        assert_eq!(entry.after_available, 800);
        assert_eq!(entry.after_frozen, 300);
    }

    #[test]
    fn unfreeze_updates_balance_and_returns_ledger_entry() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 500, 400, 7);

        let entry = BalanceLedgerEntryV2::unfreeze(
            "ledger-2".to_string(),
            &mut balance,
            150,
            BalanceLedgerReason::UnfreezeForCancel { order_id: "order-2".to_string() },
        )
        .unwrap();

        assert_eq!(balance.available, 650);
        assert_eq!(balance.frozen, 250);
        assert_eq!(balance.version, 8);
        assert_eq!(entry.after_available, 650);
        assert_eq!(entry.after_frozen, 250);
    }

    #[test]
    fn credit_available_updates_balance_and_returns_ledger_entry() {
        let mut balance = Balance::new("trader-1".to_string(), "BTC".to_string(), 10, 2, 1);

        let entry = BalanceLedgerEntryV2::credit_available(
            "ledger-3".to_string(),
            &mut balance,
            5,
            BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                trade_ids: vec!["trade-1".to_string()],
                settlement_ids: vec!["settlement-1".to_string()],
            },
        )
        .unwrap();

        assert_eq!(balance.available, 15);
        assert_eq!(balance.frozen, 2);
        assert_eq!(balance.version, 2);
        assert_eq!(entry.after_available, 15);
        assert_eq!(entry.after_frozen, 2);
    }

    #[test]
    fn debit_available_updates_balance_and_returns_ledger_entry() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 900, 20, 11);

        let entry = BalanceLedgerEntryV2::debit_available(
            "ledger-4".to_string(),
            &mut balance,
            300,
            BalanceLedgerReason::SettleSpotTradeSellerReceiveQuote {
                trade_ids: vec!["trade-2".to_string()],
                settlement_ids: vec!["settlement-2".to_string()],
            },
        )
        .unwrap();

        assert_eq!(balance.available, 600);
        assert_eq!(balance.frozen, 20);
        assert_eq!(balance.version, 12);
        assert_eq!(entry.after_available, 600);
    }

    #[test]
    fn debit_frozen_updates_balance_and_returns_ledger_entry() {
        let mut balance = Balance::new("trader-1".to_string(), "BTC".to_string(), 5, 7, 4);

        let entry = BalanceLedgerEntryV2::debit_frozen(
            "ledger-5".to_string(),
            &mut balance,
            3,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: "trade-3".to_string(),
                match_id: "match-3".to_string(),
                settlement_batch_id: "batch-3".to_string(),
                leg: SpotSettlementLeg::SellerDebitFrozenBase,
            },
        )
        .unwrap();

        assert_eq!(balance.available, 5);
        assert_eq!(balance.frozen, 4);
        assert_eq!(balance.version, 5);
        assert_eq!(entry.after_frozen, 4);
    }

    #[test]
    fn zero_amount_is_rejected() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 1);

        let result =
            BalanceLedgerEntryV2::freeze("ledger-6".to_string(), &mut balance, 0, order_reason());

        assert_eq!(result, Err(BalanceLedgerEntryV2Error::InvalidAmount));
    }

    #[test]
    fn insufficient_available_balance_is_rejected() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 100, 0, 1);

        let result =
            BalanceLedgerEntryV2::freeze("ledger-7".to_string(), &mut balance, 200, order_reason());

        assert_eq!(result, Err(BalanceLedgerEntryV2Error::InsufficientAvailableBalance));
    }

    #[test]
    fn insufficient_frozen_balance_is_rejected() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 100, 50, 1);

        let result = BalanceLedgerEntryV2::debit_frozen(
            "ledger-8".to_string(),
            &mut balance,
            80,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: "trade-8".to_string(),
                match_id: "match-8".to_string(),
                settlement_batch_id: "batch-8".to_string(),
                leg: SpotSettlementLeg::BuyerDebitFrozenQuote,
            },
        );

        assert_eq!(result, Err(BalanceLedgerEntryV2Error::InsufficientFrozenBalance));
    }

    #[test]
    fn version_overflow_is_rejected() {
        let mut balance =
            Balance::new("trader-1".to_string(), "USDT".to_string(), 100, 50, u64::MAX);

        let result = BalanceLedgerEntryV2::debit_available(
            "ledger-9".to_string(),
            &mut balance,
            10,
            order_reason(),
        );

        assert_eq!(result, Err(BalanceLedgerEntryV2Error::ArithmeticOverflow));
    }

    #[test]
    fn create_event_contains_command_amount_and_reason() {
        let mut balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);
        let entry = BalanceLedgerEntryV2::freeze(
            "ledger-10".to_string(),
            &mut balance,
            200,
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-10".to_string() },
        )
        .unwrap();

        let event = entry.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("command")
                && change.new_value_bytes() == b"freeze"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("command_amount")
                && change.new_value_bytes() == b"200"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason")
                && change.new_value_bytes() == b"reserve_for_immediate_order"
        }));
    }
}
