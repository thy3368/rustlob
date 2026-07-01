use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityError, EntityFieldChange, EntityReplayableEvent, MiStateMachineOwnedUnchecked,
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
    /// 扣减冻结余额，`available` 不变，`frozen` 减少。
    DebitFrozen { balance: Balance, amount: u64 },
}

impl BalanceLedgerCommand {
    fn parts(&self) -> (&'static str, &Balance, u64) {
        match self {
            Self::Freeze { balance, amount } => ("freeze", balance, *amount),
            Self::Unfreeze { balance, amount } => ("unfreeze", balance, *amount),
            Self::CreditAvailable { balance, amount } => ("credit_available", balance, *amount),
            Self::DebitAvailable { balance, amount } => ("debit_available", balance, *amount),
            Self::DebitFrozen { balance, amount } => ("debit_frozen", balance, *amount),
        }
    }

    pub(crate) fn label(&self) -> &'static str {
        self.parts().0
    }

    pub(crate) fn balance(&self) -> &Balance {
        self.parts().1
    }

    pub(crate) fn amount(&self) -> u64 {
        self.parts().2
    }

    pub(crate) fn apply_to_balance(&self) -> Result<Balance, BalanceLedgerEntryError> {
        let (_, balance, amount) = self.parts();
        if amount == 0 {
            return Err(BalanceLedgerEntryError::InvalidAmount);
        }

        let (next_available, next_frozen) = match self {
            Self::Freeze { .. } => balance
                .reserve_after(amount)
                .ok_or(BalanceLedgerEntryError::InsufficientAvailableBalance)?,
            Self::Unfreeze { .. } => balance
                .release_after(amount)
                .ok_or(BalanceLedgerEntryError::InsufficientFrozenBalance)?,
            Self::CreditAvailable { .. } => (
                balance
                    .credit_available_after(amount)
                    .ok_or(BalanceLedgerEntryError::ArithmeticOverflow)?,
                balance.frozen,
            ),
            Self::DebitAvailable { .. } => (
                balance
                    .debit_available_after(amount)
                    .ok_or(BalanceLedgerEntryError::InsufficientAvailableBalance)?,
                balance.frozen,
            ),
            Self::DebitFrozen { .. } => (
                balance.available,
                balance
                    .debit_frozen_after(amount)
                    .ok_or(BalanceLedgerEntryError::InsufficientFrozenBalance)?,
            ),
        };

        let mut next_balance = balance.clone();
        let next_version = next_balance
            .version
            .checked_add(1)
            .ok_or(BalanceLedgerEntryError::ArithmeticOverflow)?;
        next_balance.apply_after(next_available, next_frozen, next_version);
        Ok(next_balance)
    }
}

fn infer_balance_ledger_command(
    before: &Balance,
    after: &Balance,
) -> Result<BalanceLedgerCommand, BalanceLedgerEntryError> {
    if before.account_id != after.account_id || before.asset_id != after.asset_id {
        return Err(BalanceLedgerEntryError::SnapshotMismatch);
    }

    let available_delta = i128::from(after.available) - i128::from(before.available);
    let frozen_delta = i128::from(after.frozen) - i128::from(before.frozen);

    if available_delta == 0 && frozen_delta == 0 {
        return Err(BalanceLedgerEntryError::InvalidAmount);
    }

    let positive_abs =
        |delta: i128| -> Option<u64> { if delta > 0 { u64::try_from(delta).ok() } else { None } };
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
            Ok(BalanceLedgerCommand::Freeze { balance: before.clone(), amount })
        }
        (None, Some(amount), Some(frozen_amount), None) if amount == frozen_amount => {
            Ok(BalanceLedgerCommand::Unfreeze { balance: before.clone(), amount })
        }
        (None, Some(amount), None, None) => {
            Ok(BalanceLedgerCommand::CreditAvailable { balance: before.clone(), amount })
        }
        (Some(amount), None, None, None) => {
            Ok(BalanceLedgerCommand::DebitAvailable { balance: before.clone(), amount })
        }
        (None, None, Some(amount), None) => {
            Ok(BalanceLedgerCommand::DebitFrozen { balance: before.clone(), amount })
        }
        _ => Err(BalanceLedgerEntryError::SnapshotMismatch),
    }
}

fn same_balance_ledger_command_kind_and_amount(
    lhs: &BalanceLedgerCommand,
    rhs: &BalanceLedgerCommand,
) -> bool {
    lhs.label() == rhs.label() && lhs.amount() == rhs.amount()
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
        /// 本次流水表达的清结算资金腿。
        leg: SpotSettlementLeg,
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

/// 现货成交清结算的一条资金腿。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotSettlementLeg {
    /// 买方收到 base。
    BuyerReceiveBase,
    /// 买方扣减已冻结 quote。
    BuyerDebitFrozenQuote,
    /// 卖方收到 quote。
    SellerReceiveQuote,
    /// 卖方扣减已冻结 base。
    SellerDebitFrozenBase,
}

impl SpotSettlementLeg {
    /// 返回稳定资金腿编码。
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BuyerReceiveBase => "buyer_receive_base",
            Self::BuyerDebitFrozenQuote => "buyer_debit_frozen_quote",
            Self::SellerReceiveQuote => "seller_receive_quote",
            Self::SellerDebitFrozenBase => "seller_debit_frozen_base",
        }
    }
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

    /// 返回结算资金腿；非现货成交资金腿场景返回 `None`。
    pub fn settlement_leg(&self) -> Option<SpotSettlementLeg> {
        match self {
            Self::SettleSpotTrade { leg, .. } => Some(*leg),
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
    fn before_balance(&self) -> Balance {
        Balance::new(
            self.account_id.clone(),
            self.asset_id.clone(),
            self.before_available,
            self.before_frozen,
            0,
        )
    }

    fn after_balance(&self) -> Balance {
        Balance::new(
            self.account_id.clone(),
            self.asset_id.clone(),
            self.after_available,
            self.after_frozen,
            0,
        )
    }

    fn inferred_command(&self) -> Result<BalanceLedgerCommand, BalanceLedgerEntryError> {
        infer_balance_ledger_command(&self.before_balance(), &self.after_balance())
    }

    fn matches_command_snapshot(&self, command: &BalanceLedgerCommand) -> bool {
        self.before_balance().matches_business_snapshot(command.balance())
    }

    fn validate_apply_command(
        &self,
        command: &BalanceLedgerCommand,
    ) -> Result<Balance, BalanceLedgerEntryError> {
        if self.is_applied() {
            return Err(BalanceLedgerEntryError::AlreadyApplied);
        }
        if !self.is_draft() {
            return Err(BalanceLedgerEntryError::InvalidStatusTransition);
        }
        if !self.matches_command_snapshot(command) {
            return Err(BalanceLedgerEntryError::CommandMismatch);
        }
        if self
            .inferred_command()
            .as_ref()
            .is_ok_and(|expected| !same_balance_ledger_command_kind_and_amount(expected, command))
        {
            return Err(BalanceLedgerEntryError::CommandMismatch);
        }

        let expected_after_balance = command.apply_to_balance()?;
        if !self.after_balance().matches_business_snapshot(&expected_after_balance) {
            return Err(BalanceLedgerEntryError::SnapshotMismatch);
        }

        Ok(expected_after_balance)
    }

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
        if !balance.matches_business_snapshot(command.balance()) {
            return Err(BalanceLedgerEntryError::SnapshotMismatch);
        }
        let after_balance = command.apply_to_balance()?;
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
        self.validate_apply_command(command)?;
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

impl MiStateMachineOwnedUnchecked for BalanceLedgerEntry {
    type Command = BalanceLedgerCommand;
    type State = BalanceLedgerEntryStatus;
    type Error = BalanceLedgerEntryError;
    type AfterChanges = BalanceLedgerEntryMiChanges;

    fn state(&self) -> &Self::State {
        &self.status
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.amount() == 0 {
            return Err(BalanceLedgerEntryError::InvalidAmount);
        }
        Ok(())
    }

    fn validate_state_transition(
        &self,
        cmd: &Self::Command,
        _given_state: &<Self::Command as common_entity::CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        self.validate_apply_command(cmd).map(|_| ())
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        _given_state: &<Self::Command as common_entity::CommandWithGivenState>::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        let expected_after_balance = self.validate_apply_command(cmd)?;

        let before = self.clone();
        let mut after = self.clone();
        after.status = BalanceLedgerEntryStatus::Applied;
        let updated_balance =
            UpdatedEntityPair { before: cmd.balance().clone(), after: expected_after_balance };

        Ok(BalanceLedgerEntryMiChanges {
            updated_entry: UpdatedEntityPair { before, after },
            updated_balance,
        })
    }
}

impl common_entity::CommandWithGivenState for BalanceLedgerCommand {
    type GivenState = ();
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
        let inferred_command = self.inferred_command().ok();
        let command_label =
            inferred_command.as_ref().map(BalanceLedgerCommand::label).unwrap_or_default();
        let command_amount =
            inferred_command.as_ref().map(BalanceLedgerCommand::amount).unwrap_or(0).to_string();

        vec![
            EntityFieldChange::new("entry_id", "", self.entry_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("balance_entity_id", "", self.balance_entity_id.clone()),
            EntityFieldChange::new("command", "", command_label),
            EntityFieldChange::new("command_amount", "", command_amount),
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
            EntityFieldChange::new(
                "reason_settlement_batch_id",
                "",
                self.reason.settlement_batch_id().unwrap_or_default(),
            ),
            EntityFieldChange::new(
                "reason_settlement_leg",
                "",
                self.reason.settlement_leg().map(|leg| leg.as_str()).unwrap_or_default(),
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

    fn unfreeze_command(balance: &Balance, amount: u64) -> BalanceLedgerCommand {
        BalanceLedgerCommand::Unfreeze { balance: balance.clone(), amount }
    }

    fn credit_command(balance: &Balance, amount: u64) -> BalanceLedgerCommand {
        BalanceLedgerCommand::CreditAvailable { balance: balance.clone(), amount }
    }

    fn debit_frozen_command(balance: &Balance, amount: u64) -> BalanceLedgerCommand {
        BalanceLedgerCommand::DebitFrozen { balance: balance.clone(), amount }
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

        let changes = common_entity::MiStateMachineOwned::compute_after_changes(
            &entry,
            &freeze_command(&balance, 200),
            &(),
        )
        .unwrap();

        assert_eq!(changes.updated_entry.before.status, BalanceLedgerEntryStatus::Draft);
        assert_eq!(changes.updated_entry.after.status, BalanceLedgerEntryStatus::Applied);
        assert_eq!(changes.updated_balance.before, balance);
        assert_eq!(changes.updated_balance.after.available, 800);
        assert_eq!(changes.updated_balance.after.frozen, 200);
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

        let result = common_entity::MiStateMachineOwned::compute_after_changes(
            &entry,
            &debit_command(&balance, 200),
            &(),
        );

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

        let result = common_entity::MiStateMachineOwned::compute_after_changes(
            &entry,
            &freeze_command(&updated_balance.before, 200),
            &(),
        );

        assert_eq!(result, Err(BalanceLedgerEntryError::AlreadyApplied));
    }

    #[test]
    fn command_apply_to_balance_covers_all_paths() {
        let freeze_balance =
            Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 200, 3);
        let freeze_after = freeze_command(&freeze_balance, 150).apply_to_balance().unwrap();
        assert_eq!(freeze_after.available, 850);
        assert_eq!(freeze_after.frozen, 350);
        assert_eq!(freeze_after.version, 4);

        let unfreeze_after = unfreeze_command(&freeze_balance, 100).apply_to_balance().unwrap();
        assert_eq!(unfreeze_after.available, 1_100);
        assert_eq!(unfreeze_after.frozen, 100);

        let credit_after = credit_command(&freeze_balance, 250).apply_to_balance().unwrap();
        assert_eq!(credit_after.available, 1_250);
        assert_eq!(credit_after.frozen, 200);

        let debit_after = debit_command(&freeze_balance, 300).apply_to_balance().unwrap();
        assert_eq!(debit_after.available, 700);
        assert_eq!(debit_after.frozen, 200);

        let debit_frozen_after =
            debit_frozen_command(&freeze_balance, 80).apply_to_balance().unwrap();
        assert_eq!(debit_frozen_after.available, 1_000);
        assert_eq!(debit_frozen_after.frozen, 120);
    }

    #[test]
    fn command_apply_to_balance_rejects_zero_amount() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 200, 3);

        let result = freeze_command(&balance, 0).apply_to_balance();

        assert_eq!(result, Err(BalanceLedgerEntryError::InvalidAmount));
    }

    #[test]
    fn inferred_command_recognizes_all_supported_balance_patterns() {
        let freeze_entry = BalanceLedgerEntry::from_transition(
            "freeze-entry".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            1_000,
            0,
            800,
            200,
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-1".to_string() },
        )
        .unwrap();
        assert_eq!(
            freeze_entry.inferred_command().unwrap(),
            BalanceLedgerCommand::Freeze {
                balance: Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 0),
                amount: 200,
            }
        );

        let unfreeze_entry = BalanceLedgerEntry::from_transition(
            "unfreeze-entry".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            800,
            200,
            950,
            50,
            BalanceLedgerReason::UnfreezeForCancel { order_id: "order-1".to_string() },
        )
        .unwrap();
        assert_eq!(
            unfreeze_entry.inferred_command().unwrap(),
            BalanceLedgerCommand::Unfreeze {
                balance: Balance::new("trader-1".to_string(), "USDT".to_string(), 800, 200, 0),
                amount: 150,
            }
        );

        let credit_entry = BalanceLedgerEntry::from_transition(
            "credit-entry".to_string(),
            "trader-1".to_string(),
            "BTC".to_string(),
            "trader-1:BTC".to_string(),
            10,
            2,
            15,
            2,
            BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                trade_ids: vec!["trade-1".to_string()],
                settlement_ids: vec!["settlement-1".to_string()],
            },
        )
        .unwrap();
        assert_eq!(
            credit_entry.inferred_command().unwrap(),
            BalanceLedgerCommand::CreditAvailable {
                balance: Balance::new("trader-1".to_string(), "BTC".to_string(), 10, 2, 0),
                amount: 5,
            }
        );

        let debit_entry = BalanceLedgerEntry::from_transition(
            "debit-entry".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            1_000,
            0,
            700,
            0,
            BalanceLedgerReason::SettlePerpFunding {
                funding_batch_id: "funding-1".to_string(),
                settlement_ids: vec!["settlement-1".to_string()],
                position_ids: vec!["position-1".to_string()],
            },
        )
        .unwrap();
        assert_eq!(
            debit_entry.inferred_command().unwrap(),
            BalanceLedgerCommand::DebitAvailable {
                balance: Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 0),
                amount: 300,
            }
        );

        let debit_frozen_entry = BalanceLedgerEntry::from_transition(
            "debit-frozen-entry".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            1_000,
            300,
            1_000,
            180,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: "trade-2".to_string(),
                match_id: "match-1".to_string(),
                settlement_batch_id: "batch-1".to_string(),
                leg: SpotSettlementLeg::BuyerDebitFrozenQuote,
            },
        )
        .unwrap();
        assert_eq!(
            debit_frozen_entry.inferred_command().unwrap(),
            BalanceLedgerCommand::DebitFrozen {
                balance: Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 300, 0),
                amount: 120,
            }
        );
    }

    #[test]
    fn validate_apply_command_rejects_command_snapshot_mismatch() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);
        let entry = BalanceLedgerEntry::draft_from_balance(
            "ledger-draft-1".to_string(),
            &balance,
            freeze_command(&balance, 200),
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-1".to_string() },
        )
        .unwrap();
        let wrong_snapshot = Balance::new("trader-1".to_string(), "USDT".to_string(), 900, 0, 3);

        let result = entry.validate_apply_command(&freeze_command(&wrong_snapshot, 200));

        assert_eq!(result, Err(BalanceLedgerEntryError::CommandMismatch));
    }

    #[test]
    fn compute_changes_reuses_single_balance_derivation_result() {
        let balance = Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3);
        let command = freeze_command(&balance, 200);
        let entry = BalanceLedgerEntry::draft_from_balance(
            "ledger-draft-1".to_string(),
            &balance,
            command.clone(),
            BalanceLedgerReason::ReserveForImmediateOrder { order_id: "order-1".to_string() },
        )
        .unwrap();

        let expected_after = entry.validate_apply_command(&command).unwrap();
        let changes =
            common_entity::MiStateMachineOwned::compute_after_changes(&entry, &command, &())
                .unwrap();

        assert_eq!(changes.updated_balance.before, balance);
        assert_eq!(changes.updated_balance.after, expected_after);
        assert_eq!(changes.updated_entry.after.status, BalanceLedgerEntryStatus::Applied);
    }

    #[test]
    fn created_field_changes_keep_command_and_reason_fields_stable() {
        let entry = BalanceLedgerEntry::from_transition(
            "settlement-ledger-1".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            1_000,
            300,
            1_000,
            120,
            BalanceLedgerReason::SettleSpotTrade {
                trade_id: "trade-1".to_string(),
                match_id: "match-1".to_string(),
                settlement_batch_id: "batch-1".to_string(),
                leg: SpotSettlementLeg::BuyerDebitFrozenQuote,
            },
        )
        .unwrap();

        let field = |name: &str| {
            entry
                .created_field_changes()
                .into_iter()
                .find(|change| change.field_name.as_ref() == name)
                .map(|change| change.new_value)
                .unwrap()
        };

        assert_eq!(field("command"), "debit_frozen");
        assert_eq!(field("command_amount"), "180");
        assert_eq!(field("reason"), "settle_spot_trade");
        assert_eq!(field("reason_trade_ids"), "trade-1");
        assert_eq!(field("reason_settlement_batch_id"), "batch-1");
        assert_eq!(field("reason_settlement_leg"), "buyer_debit_frozen_quote");
    }
}
