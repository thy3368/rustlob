use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, MiStateMachine};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, SpotOrder, SpotOrderStatus, SpotOrderStatusReason};
use crate::{BalanceLedgerEntry, BalanceLedgerReason};

/// 撤销现货订单时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderState {
    /// 按 `asset + order_id` 查到的开放订单；不存在表示该订单不能撤销。
    pub open_order: Option<SpotOrder>,
    /// 订单所在账户 ID。
    pub account_id: String,
    /// base 资产余额快照。
    pub base_balance: Balance,
    /// quote 资产余额快照。
    pub quote_balance: Balance,
}

/// 撤销现货订单的命令。
///
/// 字段对齐 Hyperliquid exchange endpoint 的 cancel action：
/// `{"type": "cancel", "cancels": [{"a": asset, "o": oid}]}`。
/// `party_id` 是 core 层业务发起方，adapter 负责从签名地址或会话身份映射而来。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderCmd {
    /// 发起撤单的交易账户 ID。
    pub party_id: String,
    /// Hyperliquid 资产编号；现货使用 `10000 + spot index`。
    pub asset: u32,
    /// Hyperliquid `o`，交易所订单号 OID。
    pub order_id: u64,
}

impl IssuedByParty for CancelSpotOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 撤销现货订单可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 现货 asset 必须使用 Hyperliquid 的 `10000 + spot index` 编号。
    #[error("asset must be a Hyperliquid spot asset id")]
    InvalidSpotAsset,
    /// Hyperliquid OID 必须是正数。
    #[error("order_id must be greater than zero")]
    InvalidOrderId,
    /// 按 asset 和 OID 没有找到开放订单。
    #[error("open order was not found")]
    OrderNotFound,
    /// 命令账户、订单账户和账户快照不一致。
    #[error("order does not belong to command party")]
    OrderOwnerMismatch,
    /// 订单已经全部成交或已经撤销。
    #[error("order status is not cancelable")]
    OrderNotCancelable,
    /// 账户冻结余额不足以释放该订单。
    #[error("frozen balance is lower than order reservation")]
    FrozenBalanceMismatch,
    /// 生成账户释放事件时发生整数溢出。
    #[error("arithmetic overflow while deriving cancel result")]
    ArithmeticOverflow,
}

/// Use case that cancels one open spot order by Hyperliquid asset + OID.
///
/// 用例只表达业务规则：校验命令、校验已加载状态、生成订单删除事件和账户释放冻结余额事件。
/// 加载订单、持久化事件、发布事件和响应映射都属于 adapter / executor。
#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderUseCase;

/// 本次撤单的业务 changes。
///
/// `Changes` 是业务变化的唯一真相：
/// - create 场景直接保留新实体；
/// - update 场景优先保留 `UpdatedEntityPair<T>`；
/// - 不再并列维护可由 pair 的 `after` 直接投影出的重复 after 快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderChanges {
    /// 被撤销订单的 before/after 对。
    pub canceled_order: UpdatedEntityPair<SpotOrder>,
    /// 本次撤单释放的余额 before/after 对。
    pub released_balances: Vec<UpdatedEntityPair<Balance>>,
    /// 本次撤单生成的余额流水。
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntry>,
}

impl ReplayableChanges for CancelSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(
            1 + self.released_balances.len() + self.created_balance_ledger_entries.len(),
        );
        events
            .push(self.canceled_order.after.track_update_event_from(&self.canceled_order.before)?);
        for balance in &self.released_balances {
            events.push(balance.after.track_update_event_from(&balance.before)?);
        }
        for ledger_entry in &self.created_balance_ledger_entries {
            events.push(ledger_entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl CommandUseCase4 for CancelSpotOrderUseCase {
    type Command = CancelSpotOrderCmd;
    type GivenState = CancelSpotOrderState;
    type Error = CancelSpotOrderError;
    type Changes = CancelSpotOrderChanges;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(CancelSpotOrderError::InvalidPartyId);
        }

        if cmd.asset < 10_000 {
            return Err(CancelSpotOrderError::InvalidSpotAsset);
        }

        if cmd.order_id == 0 {
            return Err(CancelSpotOrderError::InvalidOrderId);
        }

        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let order = state.open_order.as_ref().ok_or(CancelSpotOrderError::OrderNotFound)?;

        if state.account_id != cmd.party_id || !order.belongs_to_account(&cmd.party_id) {
            return Err(CancelSpotOrderError::OrderOwnerMismatch);
        }

        if !order.can_be_cancelled() {
            return Err(CancelSpotOrderError::OrderNotCancelable);
        }

        if state.base_balance.frozen < order.base_to_release_on_cancel()
            || state.quote_balance.frozen < order.quote_to_release_on_cancel()
        {
            return Err(CancelSpotOrderError::FrozenBalanceMismatch);
        }

        Ok(())
    }

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        derive_cancel_changes(state)
    }
}

fn derive_cancel_changes(
    state: CancelSpotOrderState,
) -> Result<CancelSpotOrderChanges, CancelSpotOrderError> {
    let mut order_after = state.open_order.ok_or(CancelSpotOrderError::OrderNotFound)?;
    let order_before = order_after.clone();
    let release_base = order_after.base_to_release_on_cancel();
    let release_quote = order_after.quote_to_release_on_cancel();

    let next_order_version =
        order_after.version.checked_add(1).ok_or(CancelSpotOrderError::ArithmeticOverflow)?;
    order_after.status = SpotOrderStatus::Canceled;
    order_after.status_reason = Some(SpotOrderStatusReason::CanceledByUser);
    order_after.version = next_order_version;

    let (balance_before, balance_after) = if release_quote > 0 {
        release_balance(state.quote_balance, release_quote)?
    } else {
        release_balance(state.base_balance, release_base)?
    };
    let released_balance = UpdatedEntityPair { before: balance_before, after: balance_after };
    let reason = if release_quote > 0 {
        BalanceLedgerReason::CancelSpotOrderReleaseQuote { order_id: order_after.order_id.clone() }
    } else {
        BalanceLedgerReason::CancelSpotOrderReleaseBase { order_id: order_after.order_id.clone() }
    };
    let balance_command = if release_quote > 0 {
        crate::entity::account::balance_ledger_entry::BalanceLedgerCommand::Unfreeze {
            balance: released_balance.before.clone(),
            amount: release_quote,
        }
    } else {
        crate::entity::account::balance_ledger_entry::BalanceLedgerCommand::Unfreeze {
            balance: released_balance.before.clone(),
            amount: release_base,
        }
    };
    let draft_entry = BalanceLedgerEntry::draft_from_balance(
        format!("balance-ledger:cancel:{}", released_balance.after.entity_id()),
        &released_balance.before,
        balance_command.clone(),
        reason,
    )
    .map_err(|_| CancelSpotOrderError::ArithmeticOverflow)?;
    let balance_ledger_entry = draft_entry
        .compute_changes(&balance_command)
        .map_err(|_| CancelSpotOrderError::ArithmeticOverflow)?
        .updated_entry
        .after;
    Ok(CancelSpotOrderChanges {
        canceled_order: UpdatedEntityPair { before: order_before, after: order_after },
        released_balances: vec![released_balance],
        created_balance_ledger_entries: vec![balance_ledger_entry],
    })
}

fn release_balance(
    mut balance: Balance,
    release_amount: u64,
) -> Result<(Balance, Balance), CancelSpotOrderError> {
    let before = balance.clone();
    let (next_available, next_frozen) =
        balance.release_after(release_amount).ok_or(CancelSpotOrderError::ArithmeticOverflow)?;
    let next_version =
        balance.version.checked_add(1).ok_or(CancelSpotOrderError::ArithmeticOverflow)?;
    balance.apply_after(next_available, next_frozen, next_version);
    Ok((before, balance))
}

#[cfg(test)]
mod command_scenarios;

#[cfg(test)]
mod spot_order_scenarios;

#[cfg(test)]
mod given_state_scenarios;

#[cfg(test)]
mod compute_output_and_events_happy_path;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{
        Balance, SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderTimeInForce,
    };

    fn buy_order() -> SpotOrder {
        SpotOrder::new(
            "42".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 10 },
            SpotOrderTimeInForce::Gtc,
            2,
            0,
            20,
            None,
        )
    }

    fn base_balance() -> Balance {
        Balance {
            account_id: "trader-1".to_string(),
            asset_id: "BTC".to_string(),
            available: 5,
            frozen: 0,
            version: 3,
        }
    }

    fn quote_balance() -> Balance {
        Balance {
            account_id: "trader-1".to_string(),
            asset_id: "USDT".to_string(),
            available: 80,
            frozen: 20,
            version: 3,
        }
    }

    fn state(open_order: Option<SpotOrder>) -> CancelSpotOrderState {
        CancelSpotOrderState {
            open_order,
            account_id: "trader-1".to_string(),
            base_balance: base_balance(),
            quote_balance: quote_balance(),
        }
    }

    fn cmd() -> CancelSpotOrderCmd {
        CancelSpotOrderCmd { party_id: "trader-1".to_string(), asset: 10_001, order_id: 42 }
    }

    #[test]
    fn role_is_trader() {
        assert_eq!(CancelSpotOrderUseCase.role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_non_spot_asset() {
        let mut cmd = cmd();
        cmd.asset = 1;

        assert_eq!(
            CancelSpotOrderUseCase.pre_check_command(&cmd),
            Err(CancelSpotOrderError::InvalidSpotAsset)
        );
    }

    #[test]
    fn pre_check_rejects_zero_order_id() {
        let mut cmd = cmd();
        cmd.order_id = 0;

        assert_eq!(
            CancelSpotOrderUseCase.pre_check_command(&cmd),
            Err(CancelSpotOrderError::InvalidOrderId)
        );
    }

    #[test]
    fn validate_rejects_missing_open_order() {
        let state = state(None);

        assert_eq!(
            CancelSpotOrderUseCase.validate_against_state(&cmd(), &state),
            Err(CancelSpotOrderError::OrderNotFound)
        );
    }

    #[test]
    fn validate_rejects_order_owner_mismatch() {
        let state = state(Some(buy_order()));
        let mut cmd = cmd();
        cmd.party_id = "trader-2".to_string();

        assert_eq!(
            CancelSpotOrderUseCase.validate_against_state(&cmd, &state),
            Err(CancelSpotOrderError::OrderOwnerMismatch)
        );
    }
}
