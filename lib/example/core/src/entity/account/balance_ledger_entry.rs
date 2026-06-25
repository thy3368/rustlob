use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityError, EntityFieldChange, EntityReplayableEvent, MiStateMachine,
    ReplayableChanges,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::balance::Balance;

const BALANCE_LEDGER_ENTRY_ENTITY_TYPE: u8 = 8;

/// 余额流水命令。
///
/// 余额流水的状态机只关心余额侧发生了哪类动作，而不是外部系统怎么触发。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceLedgerCommand {
    /// 冻结可用余额，`available` 减少，`frozen` 增加。
    Freeze { balance: Balance, amount: u64 },
    /// 解冻冻结余额，`available` 增加，`frozen` 减少。
    Unfreeze { balance: Balance, amount: u64 },
    /// 增加可用余额。
    CreditAvailable { balance: Balance, amount: u64 },
    /// 扣减可用余额。
    DebitAvailable { balance: Balance, amount: u64 },
}

fn balance_ledger_command_label(command: &BalanceLedgerCommand) -> &'static str {
    match command {
        BalanceLedgerCommand::Freeze { .. } => "freeze",
        BalanceLedgerCommand::Unfreeze { .. } => "unfreeze",
        BalanceLedgerCommand::CreditAvailable { .. } => "credit_available",
        BalanceLedgerCommand::DebitAvailable { .. } => "debit_available",
    }
}

fn balance_ledger_command_amount(command: &BalanceLedgerCommand) -> u64 {
    match command {
        BalanceLedgerCommand::Freeze { amount, .. }
        | BalanceLedgerCommand::Unfreeze { amount, .. }
        | BalanceLedgerCommand::CreditAvailable { amount, .. }
        | BalanceLedgerCommand::DebitAvailable { amount, .. } => *amount,
    }
}

fn balance_ledger_command_balance(command: &BalanceLedgerCommand) -> &Balance {
    match command {
        BalanceLedgerCommand::Freeze { balance, .. }
        | BalanceLedgerCommand::Unfreeze { balance, .. }
        | BalanceLedgerCommand::CreditAvailable { balance, .. }
        | BalanceLedgerCommand::DebitAvailable { balance, .. } => balance,
    }
}

fn same_balance_business_snapshot(lhs: &Balance, rhs: &Balance) -> bool {
    lhs.account_id == rhs.account_id
        && lhs.asset_id == rhs.asset_id
        && lhs.available == rhs.available
        && lhs.frozen == rhs.frozen
}

fn apply_balance_ledger_command(
    command: &BalanceLedgerCommand,
) -> Result<Balance, BalanceLedgerEntryError> {
    let balance = balance_ledger_command_balance(command);
    let amount = balance_ledger_command_amount(command);
    if amount == 0 {
        return Err(BalanceLedgerEntryError::InvalidAmount);
    }

    let (next_available, next_frozen) = match command {
        BalanceLedgerCommand::Freeze { .. } => {
            let next_available = balance
                .available
                .checked_sub(amount)
                .ok_or(BalanceLedgerEntryError::InsufficientAvailableBalance)?;
            let next_frozen = balance
                .frozen
                .checked_add(amount)
                .ok_or(BalanceLedgerEntryError::ArithmeticOverflow)?;
            (next_available, next_frozen)
        }
        BalanceLedgerCommand::Unfreeze { .. } => {
            let next_available = balance
                .available
                .checked_add(amount)
                .ok_or(BalanceLedgerEntryError::ArithmeticOverflow)?;
            let next_frozen = balance
                .frozen
                .checked_sub(amount)
                .ok_or(BalanceLedgerEntryError::InsufficientFrozenBalance)?;
            (next_available, next_frozen)
        }
        BalanceLedgerCommand::CreditAvailable { .. } => {
            let next_available = balance
                .credit_available_after(amount)
                .ok_or(BalanceLedgerEntryError::ArithmeticOverflow)?;
            (next_available, balance.frozen)
        }
        BalanceLedgerCommand::DebitAvailable { .. } => {
            let next_available = balance
                .debit_available_after(amount)
                .ok_or(BalanceLedgerEntryError::InsufficientAvailableBalance)?;
            (next_available, balance.frozen)
        }
    };

    let mut next_balance = balance.clone();
    let next_version =
        next_balance.version.checked_add(1).ok_or(BalanceLedgerEntryError::ArithmeticOverflow)?;
    next_balance.apply_after(next_available, next_frozen, next_version);
    Ok(next_balance)
}

fn infer_balance_ledger_command(
    before: &Balance,
    after: &Balance,
) -> Result<BalanceLedgerCommand, BalanceLedgerEntryError> {
    if before.account_id != after.account_id || before.asset_id != after.asset_id {
        return Err(BalanceLedgerEntryError::SnapshotMismatch);
    }

    if before.available == after.available && before.frozen == after.frozen {
        return Err(BalanceLedgerEntryError::InvalidAmount);
    }

    if after.available < before.available && after.frozen > before.frozen {
        let available_delta = before.available - after.available;
        let frozen_delta = after.frozen - before.frozen;
        if available_delta == frozen_delta {
            return Ok(BalanceLedgerCommand::Freeze {
                balance: before.clone(),
                amount: available_delta,
            });
        }
    }

    if after.available > before.available && after.frozen < before.frozen {
        let available_delta = after.available - before.available;
        let frozen_delta = before.frozen - after.frozen;
        if available_delta == frozen_delta {
            return Ok(BalanceLedgerCommand::Unfreeze {
                balance: before.clone(),
                amount: available_delta,
            });
        }
    }

    if after.available > before.available && after.frozen == before.frozen {
        return Ok(BalanceLedgerCommand::CreditAvailable {
            balance: before.clone(),
            amount: after.available - before.available,
        });
    }

    if after.available < before.available && after.frozen == before.frozen {
        return Ok(BalanceLedgerCommand::DebitAvailable {
            balance: before.clone(),
            amount: before.available - after.available,
        });
    }

    Err(BalanceLedgerEntryError::SnapshotMismatch)
}

fn inferred_balance_ledger_command(entry: &BalanceLedgerEntry) -> Option<BalanceLedgerCommand> {
    let before = Balance::new(
        entry.account_id.clone(),
        entry.asset_id.clone(),
        entry.before_available,
        entry.before_frozen,
        0,
    );
    let after = Balance::new(
        entry.account_id.clone(),
        entry.asset_id.clone(),
        entry.after_available,
        entry.after_frozen,
        0,
    );
    infer_balance_ledger_command(&before, &after).ok()
}

/// 余额流水状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceLedgerEntryStatus {
    /// 已创建但尚未应用。
    Draft,
    /// 已完成并可作为审计事实落地。
    Applied,
}

impl BalanceLedgerEntryStatus {
    /// 返回稳定状态编码。
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Applied => "applied",
        }
    }
}

/// 余额流水状态机错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BalanceLedgerEntryError {
    /// 金额必须大于零。
    #[error("balance ledger amount must be greater than zero")]
    InvalidAmount,
    /// 可用余额不足。
    #[error("available balance is insufficient")]
    InsufficientAvailableBalance,
    /// 冻结余额不足。
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    /// 余额快照和命令不一致。
    #[error("balance snapshot does not match the declared command")]
    SnapshotMismatch,
    /// 流水已经应用过一次。
    #[error("balance ledger entry is already applied")]
    AlreadyApplied,
    /// 非 Draft 状态不能执行应用。
    #[error("balance ledger entry is not in draft status")]
    InvalidStatusTransition,
    /// 传入命令与流水自身声明的命令不一致。
    #[error("balance ledger command does not match entry command")]
    CommandMismatch,
    /// 余额状态计算发生整数溢出。
    #[error("arithmetic overflow while deriving balance ledger transition")]
    ArithmeticOverflow,
}

/// 余额流水状态机的一次业务变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceLedgerEntryMiChanges {
    /// 本次流水状态迁移的 before/after。
    pub updated_entry: UpdatedEntityPair<BalanceLedgerEntry>,
    /// 本次流水触发的余额变化。
    pub updated_balance: UpdatedEntityPair<Balance>,
}

impl ReplayableChanges for BalanceLedgerEntryMiChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.updated_balance.after.track_update_event_from(&self.updated_balance.before)?,
            self.updated_entry.after.track_create_event()?,
        ])
    }
}

/// 余额变更原因。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceLedgerReason {
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
            Self::SettlePerpFunding { .. } => "settle_perp_funding",
        }
    }

    /// 返回关联订单 ID；非下单冻结场景返回 `None`。
    pub fn order_id(&self) -> Option<&str> {
        match self {
            Self::ReserveForImmediateOrder { order_id }
            | Self::CancelSpotOrderReleaseQuote { order_id }
            | Self::CancelSpotOrderReleaseBase { order_id } => Some(order_id.as_str()),
            Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. }
            | Self::SettlePerpFunding { .. } => None,
        }
    }

    /// 返回关联 trade id 列表；非成交清结算场景返回空切片。
    pub fn trade_ids(&self) -> &[String] {
        match self {
            Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettlePerpFunding { .. } => &[],
            Self::SettleSpotTradeBuyerReceiveBase { trade_ids, .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { trade_ids, .. }
            | Self::SettleSpotTradeSellerReceiveQuote { trade_ids, .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { trade_ids, .. } => trade_ids,
        }
    }

    /// 返回关联 settlement id 列表；非成交清结算场景返回空切片。
    pub fn settlement_ids(&self) -> &[String] {
        match self {
            Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. } => &[],
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
            Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. } => None,
        }
    }

    /// 返回 funding 涉及的 position id 列表；非 funding 场景返回空切片。
    pub fn position_ids(&self) -> &[String] {
        match self {
            Self::SettlePerpFunding { position_ids, .. } => position_ids,
            Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. }
            | Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. } => &[],
        }
    }
}

/// 一条余额变更流水事实。
///
/// 该实体是 create-only 审计记录：它说明哪条余额快照发生了什么变更，以及变更原因。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerEntry {
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
    /// 流水状态。
    pub status: BalanceLedgerEntryStatus,
    /// 余额变更原因。
    pub reason: BalanceLedgerReason,
}

impl BalanceLedgerEntry {
    /// 从已校验的余额前后快照、业务原因和命令推导流水记录。
    pub fn from_transition(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        before_available: u64,
        before_frozen: u64,
        after_available: u64,
        after_frozen: u64,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryError> {
        let before_balance =
            Balance::new(account_id.clone(), asset_id.clone(), before_available, before_frozen, 0);
        let after_balance =
            Balance::new(account_id.clone(), asset_id.clone(), after_available, after_frozen, 0);
        let _ = infer_balance_ledger_command(&before_balance, &after_balance)?;

        Ok(Self {
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            before_available,
            before_frozen,
            after_available,
            after_frozen,
            status: BalanceLedgerEntryStatus::Applied,
            reason,
        })
    }

    /// 从余额快照和命令创建 Draft 流水。
    pub fn draft_from_balance(
        entry_id: String,
        balance: &Balance,
        command: BalanceLedgerCommand,
        reason: BalanceLedgerReason,
    ) -> Result<Self, BalanceLedgerEntryError> {
        if !same_balance_business_snapshot(balance, balance_ledger_command_balance(&command)) {
            return Err(BalanceLedgerEntryError::SnapshotMismatch);
        }
        let after_balance = apply_balance_ledger_command(&command)?;
        Ok(Self {
            entry_id,
            account_id: balance.account_id.clone(),
            asset_id: balance.asset_id.clone(),
            balance_entity_id: balance.entity_id(),
            before_available: balance.available,
            before_frozen: balance.frozen,
            after_available: after_balance.available,
            after_frozen: after_balance.frozen,
            status: BalanceLedgerEntryStatus::Draft,
            reason,
        })
    }

    /// 基于余额前后快照创建“立即单冻结余额”流水。
    pub fn reserve_for_immediate_order(
        entry_id: String,
        updated_balance: &UpdatedEntityPair<Balance>,
        order_id: String,
    ) -> Result<Self, BalanceLedgerEntryError> {
        Self::from_transition(
            entry_id,
            updated_balance.after.account_id.clone(),
            updated_balance.after.asset_id.clone(),
            updated_balance.after.entity_id(),
            updated_balance.before.available,
            updated_balance.before.frozen,
            updated_balance.after.available,
            updated_balance.after.frozen,
            BalanceLedgerReason::ReserveForImmediateOrder { order_id },
        )
    }

    /// 返回是否处于 Draft 状态。
    pub fn is_draft(&self) -> bool {
        matches!(self.status, BalanceLedgerEntryStatus::Draft)
    }

    /// 返回是否处于 Applied 状态。
    pub fn is_applied(&self) -> bool {
        matches!(self.status, BalanceLedgerEntryStatus::Applied)
    }

    /// 将 Draft 流水应用为 Applied。
    pub fn apply(&mut self, command: &BalanceLedgerCommand) -> Result<(), BalanceLedgerEntryError> {
        if self.is_applied() {
            return Err(BalanceLedgerEntryError::AlreadyApplied);
        }
        if !self.is_draft() {
            return Err(BalanceLedgerEntryError::InvalidStatusTransition);
        }

        let before_balance = Balance::new(
            self.account_id.clone(),
            self.asset_id.clone(),
            self.before_available,
            self.before_frozen,
            0,
        );
        if !same_balance_business_snapshot(balance_ledger_command_balance(command), &before_balance)
        {
            return Err(BalanceLedgerEntryError::CommandMismatch);
        }

        let expected = apply_balance_ledger_command(command)?;
        if expected.available != self.after_available || expected.frozen != self.after_frozen {
            return Err(BalanceLedgerEntryError::SnapshotMismatch);
        }

        self.status = BalanceLedgerEntryStatus::Applied;
        Ok(())
    }

    /// 返回这条流水是否对应指定余额实体。
    pub fn belongs_to_balance(&self, balance_entity_id: &str) -> bool {
        self.balance_entity_id == balance_entity_id
    }

    /// 返回流水记录的前后余额是否与给定 pair 完全一致。
    pub fn matches_balance_update(&self, updated_balance: &UpdatedEntityPair<Balance>) -> bool {
        self.account_id == updated_balance.after.account_id
            && self.asset_id == updated_balance.after.asset_id
            && self.balance_entity_id == updated_balance.after.entity_id()
            && self.before_available == updated_balance.before.available
            && self.before_frozen == updated_balance.before.frozen
            && self.after_available == updated_balance.after.available
            && self.after_frozen == updated_balance.after.frozen
            && self.is_applied()
    }
}

impl MiStateMachine for BalanceLedgerEntry {
    type Command = BalanceLedgerCommand;
    type State = BalanceLedgerEntryStatus;
    type Error = BalanceLedgerEntryError;
    type Changes = BalanceLedgerEntryMiChanges;

    fn state(&self) -> &Self::State {
        &self.status
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if balance_ledger_command_amount(cmd) == 0 {
            return Err(BalanceLedgerEntryError::InvalidAmount);
        }
        Ok(())
    }

    fn validate_state_transition(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        self.pre_check_command(cmd)?;
        if self.is_applied() {
            Err(BalanceLedgerEntryError::AlreadyApplied)
        } else if !self.is_draft() {
            Err(BalanceLedgerEntryError::InvalidStatusTransition)
        } else {
            let before_balance = Balance::new(
                self.account_id.clone(),
                self.asset_id.clone(),
                self.before_available,
                self.before_frozen,
                0,
            );
            if !same_balance_business_snapshot(balance_ledger_command_balance(cmd), &before_balance)
            {
                return Err(BalanceLedgerEntryError::CommandMismatch);
            }
            let expected = apply_balance_ledger_command(cmd)?;
            if expected.available == self.after_available && expected.frozen == self.after_frozen {
                Ok(())
            } else {
                Err(BalanceLedgerEntryError::SnapshotMismatch)
            }
        }
    }

    fn compute_changes(&self, cmd: &Self::Command) -> Result<Self::Changes, Self::Error> {
        self.validate_state_transition(cmd)?;

        let before = self.clone();
        let mut after = self.clone();
        after.apply(cmd)?;
        let updated_balance = UpdatedEntityPair {
            before: balance_ledger_command_balance(cmd).clone(),
            after: apply_balance_ledger_command(cmd)?,
        };

        Ok(BalanceLedgerEntryMiChanges {
            updated_entry: UpdatedEntityPair { before, after },
            updated_balance,
        })
    }
}

impl Entity for BalanceLedgerEntry {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.entry_id.clone()
    }

    fn entity_type() -> u8 {
        BALANCE_LEDGER_ENTRY_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let inferred_command = inferred_balance_ledger_command(self);
        vec![
            EntityFieldChange::new("entry_id", "", self.entry_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("balance_entity_id", "", self.balance_entity_id.clone()),
            EntityFieldChange::new(
                "command",
                "",
                inferred_command.as_ref().map(balance_ledger_command_label).unwrap_or_default(),
            ),
            EntityFieldChange::new(
                "command_amount",
                "",
                inferred_command
                    .as_ref()
                    .map(balance_ledger_command_amount)
                    .unwrap_or(0)
                    .to_string(),
            ),
            EntityFieldChange::new("status", "", self.status.as_str()),
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
            | "status"
            | "reason"
            | "reason_order_id"
            | "reason_trade_ids"
            | "reason_settlement_ids"
            | "reason_funding_batch_id"
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
    use common_entity::{Entity, MiStateMachine, ReplayableChanges};

    use super::*;

    fn updated_balance() -> UpdatedEntityPair<Balance> {
        UpdatedEntityPair {
            before: Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3),
            after: Balance::new("trader-1".to_string(), "USDT".to_string(), 800, 200, 4),
        }
    }

    fn freeze_command(balance: &Balance, amount: u64) -> BalanceLedgerCommand {
        BalanceLedgerCommand::Freeze { balance: balance.clone(), amount }
    }

    fn debit_command(balance: &Balance, amount: u64) -> BalanceLedgerCommand {
        BalanceLedgerCommand::DebitAvailable { balance: balance.clone(), amount }
    }

    #[test]
    fn constructor_keeps_balance_snapshot_and_reason() {
        let updated_balance = updated_balance();
        let entry = BalanceLedgerEntry::reserve_for_immediate_order(
            "ledger-trader-1-BTCUSDT-7-USDT".to_string(),
            &updated_balance,
            "trader-1-BTCUSDT-7".to_string(),
        )
        .unwrap();

        assert_eq!(entry.account_id, "trader-1");
        assert_eq!(entry.asset_id, "USDT");
        assert_eq!(entry.balance_entity_id, "trader-1:USDT");
        assert_eq!(entry.before_available, 1_000);
        assert_eq!(entry.before_frozen, 0);
        assert_eq!(entry.after_available, 800);
        assert_eq!(entry.after_frozen, 200);
        assert_eq!(entry.status, BalanceLedgerEntryStatus::Applied);
        assert_eq!(
            entry.reason,
            BalanceLedgerReason::ReserveForImmediateOrder {
                order_id: "trader-1-BTCUSDT-7".to_string(),
            }
        );
        assert!(entry.belongs_to_balance("trader-1:USDT"));
        assert!(entry.matches_balance_update(&updated_balance));
    }

    #[test]
    fn create_event_contains_balance_transition_and_reason() {
        let entry = BalanceLedgerEntry::from_transition(
            "ledger-trader-1-BTCUSDT-7-USDT".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            1_000,
            0,
            800,
            200,
            BalanceLedgerReason::ReserveForImmediateOrder {
                order_id: "trader-1-BTCUSDT-7".to_string(),
            },
        )
        .unwrap();

        let event = entry.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("balance_entity_id")
                && change.new_value_bytes() == b"trader-1:USDT"
        }));
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
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_order_id")
                && change.new_value_bytes() == b"trader-1-BTCUSDT-7"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_trade_ids")
                && change.new_value_bytes().is_empty()
        }));
    }

    #[test]
    fn create_event_contains_settlement_reason_references() {
        let entry = BalanceLedgerEntry::from_transition(
            "ledger-settle-1-1-buyer-BTC".to_string(),
            "buyer".to_string(),
            "BTC".to_string(),
            "buyer:BTC".to_string(),
            0,
            0,
            3,
            0,
            BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                trade_ids: vec!["trade-1".to_string(), "trade-2".to_string()],
                settlement_ids: vec!["settle-1-1".to_string(), "settle-1-2".to_string()],
            },
        )
        .unwrap();

        let event = entry.track_create_event().unwrap();

        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason")
                && change.new_value_bytes() == b"settle_spot_trade_buyer_receive_base"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_trade_ids")
                && change.new_value_bytes() == b"trade-1,trade-2"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_settlement_ids")
                && change.new_value_bytes() == b"settle-1-1,settle-1-2"
        }));
    }

    #[test]
    fn create_event_contains_funding_reason_references() {
        let entry = BalanceLedgerEntry::from_transition(
            "balance-ledger:funding:funding-1:trader-1:USDC".to_string(),
            "trader-1".to_string(),
            "USDC".to_string(),
            "trader-1:USDC".to_string(),
            1_000,
            0,
            990,
            0,
            BalanceLedgerReason::SettlePerpFunding {
                funding_batch_id: "funding-1".to_string(),
                settlement_ids: vec![
                    "funding-1-position-1".to_string(),
                    "funding-1-position-2".to_string(),
                ],
                position_ids: vec!["position-1".to_string(), "position-2".to_string()],
            },
        )
        .unwrap();

        let event = entry.track_create_event().unwrap();

        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason")
                && change.new_value_bytes() == b"settle_perp_funding"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_settlement_ids")
                && change.new_value_bytes() == b"funding-1-position-1,funding-1-position-2"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_funding_batch_id")
                && change.new_value_bytes() == b"funding-1"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_position_ids")
                && change.new_value_bytes() == b"position-1,position-2"
        }));
    }

    #[test]
    fn mi_state_machine_applies_draft_entry() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);
        let command = freeze_command(&balance, 200);
        let entry = BalanceLedgerEntry::draft_from_balance(
            "ledger-draft-1".to_string(),
            &balance,
            command,
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-1".to_string() },
        )
        .unwrap();

        let changes = entry.compute_changes(&freeze_command(&balance, 200)).unwrap();

        assert_eq!(changes.updated_entry.before.status, BalanceLedgerEntryStatus::Draft);
        assert_eq!(changes.updated_entry.after.status, BalanceLedgerEntryStatus::Applied);
        assert_eq!(changes.updated_balance.before, balance);
        assert_eq!(changes.updated_balance.after.available, 800);
        assert_eq!(changes.updated_balance.after.frozen, 200);
        assert_eq!(changes.updated_entry.after.after_available, 800);
        assert_eq!(changes.updated_entry.after.after_frozen, 200);

        let events = changes.to_replayable_events().unwrap();
        assert_eq!(events.len(), 2);
        assert!(events[0].is_updated());
        assert!(events[1].is_created());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("available")
                && change.new_value_bytes() == b"800"
        }));
        assert!(events[1].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("command")
                && change.new_value_bytes() == b"freeze"
        }));
    }

    #[test]
    fn mi_state_machine_rejects_command_mismatch() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);
        let command = freeze_command(&balance, 200);
        let entry = BalanceLedgerEntry::draft_from_balance(
            "ledger-draft-1".to_string(),
            &balance,
            command,
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-1".to_string() },
        )
        .unwrap();

        let result = entry.compute_changes(&debit_command(&balance, 200));

        assert_eq!(result, Err(BalanceLedgerEntryError::CommandMismatch));
    }

    #[test]
    fn mi_state_machine_rejects_already_applied_entry() {
        let updated_balance = updated_balance();
        let entry = BalanceLedgerEntry::reserve_for_immediate_order(
            "ledger-trader-1-BTCUSDT-7-USDT".to_string(),
            &updated_balance,
            "trader-1-BTCUSDT-7".to_string(),
        )
        .unwrap();

        let result = entry.compute_changes(&freeze_command(&updated_balance.before, 200));

        assert_eq!(result, Err(BalanceLedgerEntryError::AlreadyApplied));
    }

    #[test]
    fn mi_state_machine_rejects_snapshot_mismatch_on_apply() {
        let mut entry = BalanceLedgerEntry::draft_from_balance(
            "ledger-draft-1".to_string(),
            &Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3),
            freeze_command(
                &Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3),
                200,
            ),
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-1".to_string() },
        )
        .unwrap();
        entry.after_available = 700;

        let result = entry.compute_changes(&freeze_command(
            &Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3),
            200,
        ));

        assert_eq!(result, Err(BalanceLedgerEntryError::SnapshotMismatch));
    }
}
