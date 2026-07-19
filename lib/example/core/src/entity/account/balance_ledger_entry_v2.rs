use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel, FieldDiff,
    FinancialClassification, FourColorArchetype,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::balance::{Balance, BalanceError};
use super::balance_ledger_reason::BalanceLedgerReason;
use super::settlement_transfer_voucher::SettlementTransferPurpose;

const BALANCE_LEDGER_ENTRY_ENTITY_TYPE: u8 = 8;

/// 余额流水的权威业务操作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BalanceLedgerOperation {
    Freeze,
    Unfreeze,
    CreditAvailable,
    DebitAvailable,
    DebitFrozen,
}

impl BalanceLedgerOperation {
    /// 返回稳定操作编码，供 replay event / 审计查询使用。
    pub const fn as_str(&self) -> &'static str {
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
    /// 流水身份与目标余额不一致。
    #[error("balance ledger entry identity does not match target balance")]
    BalanceIdentityMismatch,
    /// 流水已经应用过，不能重复落账。
    #[error("balance ledger entry has already been applied")]
    AlreadyApplied,
}

impl From<BalanceError> for BalanceLedgerEntryV2Error {
    fn from(error: BalanceError) -> Self {
        match error {
            BalanceError::InvalidAmount => Self::InvalidAmount,
            BalanceError::InsufficientAvailableBalance => Self::InsufficientAvailableBalance,
            BalanceError::InsufficientFrozenBalance => Self::InsufficientFrozenBalance,
            BalanceError::ArithmeticOverflow => Self::ArithmeticOverflow,
        }
    }
}

/// 一条 create-only 的余额流水审计事实。
///
/// 该实体先记录某条 `Balance` 的身份、业务操作与金额，成功应用到余额后再补齐 before/after 快照。
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
    /// 本条流水表达的余额操作。
    pub operation: BalanceLedgerOperation,
    /// 本条流水表达的金额。
    pub amount: u64,
    /// 变更前可用余额。
    pub before_available: Option<u64>,
    /// 变更前冻结余额。
    pub before_frozen: Option<u64>,
    /// 变更后可用余额。
    pub after_available: Option<u64>,
    /// 变更后冻结余额。
    pub after_frozen: Option<u64>,
    /// 余额变更原因。
    pub reason: BalanceLedgerReason,
}

impl BalanceLedgerEntryV2 {
    /// 构造一条冻结可用余额的流水事实。
    pub fn freeze(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::new_operation(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
            BalanceLedgerOperation::Freeze,
        )
    }

    /// 构造一条解冻冻结余额的流水事实。
    pub fn unfreeze(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::new_operation(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
            BalanceLedgerOperation::Unfreeze,
        )
    }

    /// 构造一条增加可用余额的流水事实。
    pub fn credit_available(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::new_operation(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
            BalanceLedgerOperation::CreditAvailable,
        )
    }

    /// 构造一条扣减可用余额的流水事实。
    pub fn debit_available(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::new_operation(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
            BalanceLedgerOperation::DebitAvailable,
        )
    }

    /// 构造一条扣减冻结余额的流水事实。
    pub fn debit_frozen(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        amount: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        Self::new_operation(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
            BalanceLedgerOperation::DebitFrozen,
        )
    }

    fn new_operation(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        amount: u64,
        reason: BalanceLedgerReason,
        operation: BalanceLedgerOperation,
    ) -> Result<Self, BalanceLedgerEntryV2Error> {
        if amount == 0 {
            return Err(BalanceLedgerEntryV2Error::InvalidAmount);
        }

        Ok(Self {
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            operation,
            amount,
            before_available: None,
            before_frozen: None,
            after_available: None,
            after_frozen: None,
            reason,
        })
    }

    /// 可 BDD 规格化的聚合根行为：把流水操作应用到对应余额并补齐审计快照。
    pub fn apply_to(&mut self, balance: &mut Balance) -> Result<(), BalanceLedgerEntryV2Error> {
        if self.has_any_snapshot() {
            return Err(BalanceLedgerEntryV2Error::AlreadyApplied);
        }
        if self.account_id != balance.account_id
            || self.asset_id != balance.asset_id
            || self.balance_entity_id != balance.entity_id()
        {
            return Err(BalanceLedgerEntryV2Error::BalanceIdentityMismatch);
        }

        let before_available = balance.available;
        let before_frozen = balance.frozen;

        match self.operation {
            BalanceLedgerOperation::Freeze => balance.reserve(self.amount),
            BalanceLedgerOperation::Unfreeze => balance.release(self.amount),
            BalanceLedgerOperation::CreditAvailable => balance.credit_available(self.amount),
            BalanceLedgerOperation::DebitAvailable => balance.debit_available(self.amount),
            BalanceLedgerOperation::DebitFrozen => balance.debit_frozen(self.amount),
        }
        .map_err(BalanceLedgerEntryV2Error::from)?;

        self.before_available = Some(before_available);
        self.before_frozen = Some(before_frozen);
        self.after_available = Some(balance.available);
        self.after_frozen = Some(balance.frozen);
        Ok(())
    }

    /// 返回这条流水是否已经成功应用到余额。
    pub fn is_applied(&self) -> bool {
        self.before_available.is_some()
            && self.before_frozen.is_some()
            && self.after_available.is_some()
            && self.after_frozen.is_some()
    }

    fn has_any_snapshot(&self) -> bool {
        self.before_available.is_some()
            || self.before_frozen.is_some()
            || self.after_available.is_some()
            || self.after_frozen.is_some()
    }

    /// 返回这条流水是否对应给定的余额 before/after 变化。
    pub fn matches_balance_update(&self, updated_balance: &UpdatedEntityPair<Balance>) -> bool {
        self.is_applied()
            && self.account_id == updated_balance.after.account_id
            && self.asset_id == updated_balance.after.asset_id
            && self.balance_entity_id == updated_balance.after.entity_id()
            && self.before_available == Some(updated_balance.before.available)
            && self.before_frozen == Some(updated_balance.before.frozen)
            && self.after_available == Some(updated_balance.after.available)
            && self.after_frozen == Some(updated_balance.after.frozen)
    }

    fn reason_settlement_purpose(&self) -> Option<SettlementTransferPurpose> {
        self.reason.settlement_purpose()
    }

    fn derived_available_delta(&self) -> i128 {
        match self.operation {
            BalanceLedgerOperation::Freeze => -(self.amount as i128),
            BalanceLedgerOperation::Unfreeze => self.amount as i128,
            BalanceLedgerOperation::CreditAvailable => self.amount as i128,
            BalanceLedgerOperation::DebitAvailable => -(self.amount as i128),
            BalanceLedgerOperation::DebitFrozen => 0,
        }
    }

    fn derived_frozen_delta(&self) -> i128 {
        match self.operation {
            BalanceLedgerOperation::Freeze => self.amount as i128,
            BalanceLedgerOperation::Unfreeze => -(self.amount as i128),
            BalanceLedgerOperation::CreditAvailable => 0,
            BalanceLedgerOperation::DebitAvailable => 0,
            BalanceLedgerOperation::DebitFrozen => -(self.amount as i128),
        }
    }
}

impl FieldDiff for BalanceLedgerEntryV2 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("entry_id", "", self.entry_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("balance_entity_id", "", self.balance_entity_id.clone()),
            EntityFieldChange::new("operation", "", self.operation.as_str()),
            EntityFieldChange::new("amount", "", self.amount.to_string()),
            EntityFieldChange::new(
                "available_delta",
                "",
                self.derived_available_delta().to_string(),
            ),
            EntityFieldChange::new("frozen_delta", "", self.derived_frozen_delta().to_string()),
            EntityFieldChange::new("command", "", self.operation.as_str()),
            EntityFieldChange::new("command_amount", "", self.amount.to_string()),
            EntityFieldChange::new("before_available", "", option_u64_value(self.before_available)),
            EntityFieldChange::new("before_frozen", "", option_u64_value(self.before_frozen)),
            EntityFieldChange::new("after_available", "", option_u64_value(self.after_available)),
            EntityFieldChange::new("after_frozen", "", option_u64_value(self.after_frozen)),
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
                self.reason_settlement_purpose()
                    .map(|purpose| purpose.as_str())
                    .unwrap_or_default(),
            ),
            EntityFieldChange::new("reason_position_ids", "", self.reason.position_ids().join(",")),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
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

    fn financial_classification() -> FinancialClassification {
        FinancialClassification::LedgerEntry
    }

    fn entity_version(&self) -> u64 {
        1
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "entry_id"
            | "account_id"
            | "asset_id"
            | "balance_entity_id"
            | "operation"
            | "command"
            | "reason"
            | "reason_order_id"
            | "reason_trade_ids"
            | "reason_settlement_ids"
            | "reason_funding_batch_id"
            | "reason_settlement_batch_id"
            | "reason_settlement_leg"
            | "reason_position_ids" => 0,
            "amount" | "command_amount" | "available_delta" | "frozen_delta"
            | "before_available" | "before_frozen" | "after_available" | "after_frozen" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_balance_ledger_entry_id(&self.entry_id))
    }
}

fn option_u64_value(value: Option<u64>) -> String {
    value.map_or_else(String::new, |value| value.to_string())
}

fn stable_balance_ledger_entry_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}
