use std::collections::HashMap;

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    CommandWithGivenState, Entity, EntityReplayableEvent, MiStateMachineOwnedUnchecked,
    ReplayableChanges,
};
use thiserror::Error;

use super::spot_order::{SpotOrder, SpotOrderStatus, SpotReservationAssetKind};
use super::spot_trade::SpotTrade;
use crate::entity::Balance;
use crate::entity::account::balance_ledger_entry::BalanceLedgerReason;
use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error,
};

/// `SpotOrder` 的 owned 状态机命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderMiCommand<'a> {
    /// 订单进入系统并冻结对应余额。
    Place(PlaceSpotOrderCmd<'a>),
    /// 基于本次撮合输入推进订单成交状态。
    Match(MatchSpotOrderCmd),
    /// 对当前开放订单执行业务撤销。
    Cancel(CancelSpotOrderCmd),
}

/// `SpotOrder` MI 各命令分支所需的外部已加载上下文。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderMiGivenState<'a> {
    /// `Place` 不需要额外上下文，只要求显式选择对应分支。
    Place,
    /// `Match` 需要外部已按撮合优先级加载好的 maker 订单切片。
    Match { makers: &'a [SpotOrder] },
    /// `Cancel` 不需要额外上下文，只要求显式选择对应分支。
    Cancel,
}

/// `Place` 命令只携带冻结余额所需的 taker 快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderCmd<'a> {
    /// taker base 余额快照。
    pub taker_base_balance: &'a Balance,
    /// taker quote 余额快照。
    pub taker_quote_balance: &'a Balance,
}

/// `Match` 命令携带撮合所需的全部上下文。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderCmd {
    /// 一次撮合批次 ID，用于稳定生成多条 trade id。
    pub match_id: String,
}

/// `Place` 命令的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderChanges {
    /// taker 订单在本 case 内的起止状态。
    pub updated_order: UpdatedEntityPair<SpotOrder>,
    /// 本 case 涉及的余额起止状态；同一个 balance 实例最多出现一次。
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    /// 本次创建的余额流水；按业务发生顺序排列。
    pub created_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

/// `Match` 命令的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderChanges {
    /// 本次撮合新生成的成交事实。
    pub created_trades: Vec<SpotTrade>,
    /// 本次撮合中实际发生变化的 maker 订单 before/after，顺序与撮合顺序一致。
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrder>>,
    /// 本次撮合后的 taker 订单 before/after。
    pub updated_taker_order: UpdatedEntityPair<SpotOrder>,
}

/// `Cancel` 命令不携带额外参数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderCmd;

/// `Cancel` 命令的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderChanges {
    /// 订单 before/after。
    pub updated_order: UpdatedEntityPair<SpotOrder>,
}

/// `SpotOrder` 状态机的一次业务变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderMiChanges {
    /// `Place` 的 changes。
    Place(PlaceSpotOrderChanges),
    /// `Match` 的 changes。
    Match(MatchSpotOrderChanges),
    /// `Cancel` 的 changes。
    Cancel(CancelSpotOrderChanges),
}

impl ReplayableChanges for PlaceSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events =
            Vec::with_capacity(1 + self.updated_balances.len() + self.created_ledger_entries.len());
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
        events.extend(balance_replay_events_from_ledger_entries(
            &self.updated_balances,
            &self.created_ledger_entries,
        )?);
        for entry in &self.created_ledger_entries {
            events.push(entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl ReplayableChanges for MatchSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events =
            Vec::with_capacity(self.created_trades.len() + self.updated_maker_orders.len() + 1);
        for (trade, maker) in self.created_trades.iter().zip(&self.updated_maker_orders) {
            events.push(trade.track_create_event()?);
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        events.push(
            self.updated_taker_order
                .after
                .track_update_event_from(&self.updated_taker_order.before)?,
        );
        Ok(events)
    }
}

impl ReplayableChanges for CancelSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        Ok(vec![self.updated_order.after.track_update_event_from(&self.updated_order.before)?])
    }
}

impl ReplayableChanges for SpotOrderMiChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Place(changes) => changes.to_replayable_events(),
            Self::Match(changes) => changes.to_replayable_events(),
            Self::Cancel(changes) => changes.to_replayable_events(),
        }
    }
}

fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerEntryV2],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut current_balances = HashMap::<String, Balance>::with_capacity(updated_balances.len());
    let mut expected_after_balances =
        HashMap::<String, Balance>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.entity_id();
        current_balances.insert(balance_id.clone(), balance.before.clone());
        expected_after_balances.insert(balance_id, balance.after.clone());
    }

    let mut events = Vec::with_capacity(ledger_entries.len());
    for entry in ledger_entries {
        let Some(before) = current_balances.get(&entry.balance_entity_id).cloned() else {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry does not belong to updated balances".to_string(),
            ));
        };
        if before.available != entry.before_available || before.frozen != entry.before_frozen {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let next_version = before
            .version
            .checked_add(1)
            .ok_or(common_entity::EntityError::VersionOverflow { version: before.version })?;
        let after = Balance::new(
            before.account_id.clone(),
            before.asset_id.clone(),
            entry.after_available,
            entry.after_frozen,
            next_version,
        );
        events.push(after.track_update_event_from(&before)?);
        current_balances.insert(entry.balance_entity_id.clone(), after);
    }

    for (balance_id, expected_after) in expected_after_balances {
        if current_balances.get(&balance_id) != Some(&expected_after) {
            return Err(common_entity::EntityError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    Ok(events)
}

/// `SpotOrder` owned 状态机的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotOrderMiStateMachineError {
    #[error("spot order execution state is inconsistent")]
    InconsistentExecutionState,
    #[error("command `{command}` is not allowed when order status is `{status}`")]
    CommandNotAllowedInCurrentState { command: &'static str, status: &'static str },
    #[error("invalid spot order state machine command: {reason}")]
    InvalidCommandFields { reason: &'static str },
    #[error("filled quantity overflow or exceeds order qty")]
    FilledQtyOverflowOrExceedsQty,
    #[error("spot order time-in-force semantics mismatch: {reason}")]
    TimeInForceSemanticsMismatch { reason: &'static str },
    #[error("spot order version overflow")]
    VersionOverflow,
    #[error("freeze balance account does not match order account")]
    FreezeBalanceAccountMismatch,
    #[error("spot order reservation must be greater than zero")]
    MissingReservation,
    #[error("freeze balance is insufficient")]
    InsufficientFreezeBalance,
    #[error("failed to create balance ledger entry for spot order freeze")]
    BalanceLedgerCreationFailed,
    #[error("match id must not be empty")]
    InvalidMatchId,
    #[error("order is not matchable")]
    OrderNotMatchable,
    #[error("maker order must not be the taker order")]
    MakerIsTaker,
    #[error("maker order has the same side as taker")]
    SameSideMaker,
    #[error("maker order must be a limit order")]
    MakerMustBeLimit,
    #[error("maker order trades a different asset")]
    AssetMismatch,
    #[error("maker order trades a different symbol")]
    SymbolMismatch,
    #[error("no spot trades were matched")]
    NoTradesMatched,
    #[error("arithmetic overflow while deriving match result")]
    ArithmeticOverflow,
}

impl From<crate::entity::SpotOrderMatchError> for SpotOrderMiStateMachineError {
    fn from(value: crate::entity::SpotOrderMatchError) -> Self {
        match value {
            crate::entity::SpotOrderMatchError::InconsistentExecutionState => {
                Self::InconsistentExecutionState
            }
            crate::entity::SpotOrderMatchError::OrderNotMatchable => Self::OrderNotMatchable,
            crate::entity::SpotOrderMatchError::MakerIsTaker => Self::MakerIsTaker,
            crate::entity::SpotOrderMatchError::SameSideMaker => Self::SameSideMaker,
            crate::entity::SpotOrderMatchError::MakerMustBeLimit => Self::MakerMustBeLimit,
            crate::entity::SpotOrderMatchError::AssetMismatch => Self::AssetMismatch,
            crate::entity::SpotOrderMatchError::SymbolMismatch => Self::SymbolMismatch,
            crate::entity::SpotOrderMatchError::NoTradesMatched => Self::NoTradesMatched,
            crate::entity::SpotOrderMatchError::ArithmeticOverflow => Self::ArithmeticOverflow,
        }
    }
}

impl<'a> CommandWithGivenState for SpotOrderMiCommand<'a> {
    type GivenState = SpotOrderMiGivenState<'a>;
}

impl MiStateMachineOwnedUnchecked for SpotOrder {
    type Command<'a> = SpotOrderMiCommand<'a>;
    type State = SpotOrderStatus;
    type Error = SpotOrderMiStateMachineError;
    type AfterChanges = SpotOrderMiChanges;

    fn state(&self) -> &Self::State {
        &self.status
    }

    fn pre_check_command<'a>(&self, cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
        match cmd {
            SpotOrderMiCommand::Place(place) => self.pre_check_place_command(place),
            SpotOrderMiCommand::Match(match_cmd) => self.pre_check_match_command(match_cmd),
            SpotOrderMiCommand::Cancel(_) => Ok(()),
        }
    }

    fn validate_state_transition<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        if !self.has_consistent_execution_state() {
            return Err(SpotOrderMiStateMachineError::InconsistentExecutionState);
        }

        match (cmd, given_state) {
            (SpotOrderMiCommand::Place(_), SpotOrderMiGivenState::Place) => {
                if matches!(self.state(), SpotOrderStatus::Open) {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "place",
                        status: self.state().as_str(),
                    })
                }
            }
            (SpotOrderMiCommand::Place(_), _) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "place command requires place given state",
                })
            }
            (SpotOrderMiCommand::Match(_), SpotOrderMiGivenState::Match { makers }) => {
                for maker in *makers {
                    if !maker.has_consistent_execution_state() {
                        return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                            reason: "maker execution state is inconsistent",
                        });
                    }
                }
                if matches!(self.state(), SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
                {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "match",
                        status: self.state().as_str(),
                    })
                }
            }
            (SpotOrderMiCommand::Match(_), _) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "match command requires match given state",
                })
            }
            (SpotOrderMiCommand::Cancel(_), SpotOrderMiGivenState::Cancel) => {
                if self.can_be_cancelled() {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "cancel",
                        status: self.state().as_str(),
                    })
                }
            }
            (SpotOrderMiCommand::Cancel(_), _) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "cancel command requires cancel given state",
                })
            }
        }
    }

    fn compute_after_changes_unchecked<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        match (cmd, given_state) {
            (SpotOrderMiCommand::Place(place), SpotOrderMiGivenState::Place) => {
                self.compute_place_changes(place)
            }
            (SpotOrderMiCommand::Match(match_cmd), SpotOrderMiGivenState::Match { makers }) => {
                self.compute_match_changes(match_cmd, makers)
            }
            (SpotOrderMiCommand::Cancel(cancel), SpotOrderMiGivenState::Cancel) => {
                self.compute_cancel_changes(cancel)
            }
            (SpotOrderMiCommand::Place(_), _)
            | (SpotOrderMiCommand::Match(_), _)
            | (SpotOrderMiCommand::Cancel(_), _) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "command and given state branch mismatch",
                })
            }
        }
    }
}

impl SpotOrder {
    fn checked_next_version(version: u64) -> Result<u64, SpotOrderMiStateMachineError> {
        version.checked_add(1).ok_or(SpotOrderMiStateMachineError::VersionOverflow)
    }

    /// 推进一次 `place` 命令后的单订单生命周期。
    ///
    /// `place` 只表示订单被系统接受并进入后续业务流程，因此只推进版本；
    /// 订单状态与拒绝原因仍保持订单自身当前语义。
    pub(crate) fn evolve_after_place_acceptance(
        &self,
    ) -> Result<SpotOrder, SpotOrderMiStateMachineError> {
        let mut after = self.clone();
        after.version = Self::checked_next_version(after.version)?;
        Ok(after)
    }

    /// 将 `place + match + release` 管线内的中间订单状态压缩成 case 级 authoritative after。
    ///
    /// UC6 对外只暴露一次主订单更新事件，因此版本必须相对 `place` 前快照只前进一步；
    /// `filled_qty / status / status_reason` 保持订单状态机已经推导出的最终语义。
    pub(crate) fn project_authoritative_after_place_pipeline(
        &self,
        before_place: &SpotOrder,
    ) -> Result<SpotOrder, SpotOrderMiStateMachineError> {
        let mut after = self.clone();
        after.version = Self::checked_next_version(before_place.version)?;
        Ok(after)
    }

    /// 推进一次用户撤单后的单订单生命周期。
    pub(crate) fn evolve_after_user_cancel(
        &self,
    ) -> Result<SpotOrder, SpotOrderMiStateMachineError> {
        let mut after = self.clone();
        after.status = SpotOrderStatus::Canceled;
        after.status_reason = Some(crate::entity::SpotOrderStatusReason::CanceledByUser);
        after.version = Self::checked_next_version(after.version)?;
        Ok(after)
    }

    pub(crate) fn pre_check_place_command(
        &self,
        _cmd: &PlaceSpotOrderCmd<'_>,
    ) -> Result<(), SpotOrderMiStateMachineError> {
        Ok(())
    }

    pub(crate) fn pre_check_match_command(
        &self,
        cmd: &MatchSpotOrderCmd,
    ) -> Result<(), SpotOrderMiStateMachineError> {
        if cmd.match_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidMatchId);
        }
        Ok(())
    }

    pub(crate) fn compute_place_changes(
        &self,
        cmd: &PlaceSpotOrderCmd<'_>,
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let before = self.clone();
        let after = self.evolve_after_place_acceptance()?;

        let (frozen_balance, created_freeze_ledger_entry) =
            derive_place_freeze_changes(&after, taker_freeze_balance(&after, cmd))?;

        Ok(SpotOrderMiChanges::Place(PlaceSpotOrderChanges {
            updated_order: UpdatedEntityPair { before, after },
            updated_balances: vec![frozen_balance],
            created_ledger_entries: vec![created_freeze_ledger_entry],
        }))
    }

    pub(crate) fn compute_match_changes(
        &self,
        cmd: &MatchSpotOrderCmd,
        makers: &[SpotOrder],
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let mut taker_order = self.clone();
        let taker_order_before = taker_order.clone();
        let mut taker_remaining = taker_order.remaining_qty()?;
        let mut total_taker_fill = 0_u64;
        let mut created_trades = Vec::new();
        let mut updated_maker_orders = Vec::new();
        let best_maker = makers.first();

        if taker_order.would_be_rejected_as_alo(best_maker)? {
            taker_order.reject_as_bad_alo()?;
            return Ok(SpotOrderMiChanges::Match(MatchSpotOrderChanges {
                created_trades,
                updated_maker_orders,
                updated_taker_order: UpdatedEntityPair {
                    before: taker_order_before,
                    after: taker_order,
                },
            }));
        }

        for (trade_index, maker_order) in makers.iter().cloned().enumerate() {
            if taker_remaining == 0 {
                break;
            }

            maker_order.ensure_matchable()?;
            maker_order.ensure_compatible_maker_for(&taker_order)?;

            if !taker_order.crosses_order(&maker_order)? {
                break;
            }

            let maker_price =
                maker_order.limit_price().ok_or(SpotOrderMiStateMachineError::MakerMustBeLimit)?;
            let maker_remaining = maker_order.remaining_qty()?;
            let trade_qty = taker_remaining.min(maker_remaining);
            if trade_qty == 0 {
                continue;
            }

            let trade = SpotTrade::new(
                format!("{}-{}", cmd.match_id, trade_index + 1),
                cmd.match_id.clone(),
                taker_order.asset,
                taker_order.symbol.clone(),
                taker_order.order_id.clone(),
                maker_order.order_id.clone(),
                taker_order.account_id.clone(),
                maker_order.account_id.clone(),
                taker_order.side,
                maker_price,
                trade_qty,
            );
            created_trades.push(trade);

            let mut next_maker_order = maker_order;
            let previous_maker_order = next_maker_order.clone();
            next_maker_order.apply_fill(trade_qty)?;
            updated_maker_orders
                .push(UpdatedEntityPair { before: previous_maker_order, after: next_maker_order });

            taker_remaining = taker_remaining
                .checked_sub(trade_qty)
                .ok_or(SpotOrderMiStateMachineError::ArithmeticOverflow)?;
            total_taker_fill = total_taker_fill
                .checked_add(trade_qty)
                .ok_or(SpotOrderMiStateMachineError::ArithmeticOverflow)?;
        }

        let finalization = taker_order.finalize_after_match(total_taker_fill)?;
        taker_order.apply_finalization(finalization)?;

        Ok(SpotOrderMiChanges::Match(MatchSpotOrderChanges {
            created_trades,
            updated_maker_orders,
            updated_taker_order: UpdatedEntityPair {
                before: taker_order_before,
                after: taker_order,
            },
        }))
    }

    pub(crate) fn compute_cancel_changes(
        &self,
        _cmd: &CancelSpotOrderCmd,
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let before = self.clone();
        let after = self.evolve_after_user_cancel()?;
        Ok(SpotOrderMiChanges::Cancel(CancelSpotOrderChanges {
            updated_order: UpdatedEntityPair { before, after },
        }))
    }
}

fn derive_place_freeze_changes(
    order: &SpotOrder,
    freeze_balance: &Balance,
) -> Result<(UpdatedEntityPair<Balance>, BalanceLedgerEntryV2), SpotOrderMiStateMachineError> {
    if !freeze_balance.belongs_to_account(&order.account_id) {
        return Err(SpotOrderMiStateMachineError::FreezeBalanceAccountMismatch);
    }

    let amount = order.reservation_amount();
    if amount == 0 {
        return Err(SpotOrderMiStateMachineError::MissingReservation);
    }

    if !freeze_balance.can_reserve(amount) {
        return Err(SpotOrderMiStateMachineError::InsufficientFreezeBalance);
    }

    let mut after_balance = freeze_balance.clone();
    let created_ledger_entry = BalanceLedgerEntryV2::freeze(
        format!("balance-ledger:{}:{}", order.order_id, freeze_balance.entity_id()),
        &mut after_balance,
        amount,
        BalanceLedgerReason::FreezeForOrder { order_id: order.order_id.clone() },
    )
    .map_err(map_balance_ledger_error)?;

    Ok((
        UpdatedEntityPair { before: freeze_balance.clone(), after: after_balance },
        created_ledger_entry,
    ))
}

fn taker_freeze_balance<'a>(order: &SpotOrder, cmd: &'a PlaceSpotOrderCmd<'a>) -> &'a Balance {
    match order.reservation_asset_kind() {
        SpotReservationAssetKind::Quote => &cmd.taker_quote_balance,
        SpotReservationAssetKind::Base => &cmd.taker_base_balance,
    }
}

fn map_balance_ledger_error(error: BalanceLedgerEntryV2Error) -> SpotOrderMiStateMachineError {
    match error {
        BalanceLedgerEntryV2Error::InvalidAmount => {
            SpotOrderMiStateMachineError::MissingReservation
        }
        BalanceLedgerEntryV2Error::InsufficientAvailableBalance => {
            SpotOrderMiStateMachineError::InsufficientFreezeBalance
        }
        BalanceLedgerEntryV2Error::ArithmeticOverflow => {
            SpotOrderMiStateMachineError::VersionOverflow
        }
        BalanceLedgerEntryV2Error::InsufficientFrozenBalance => {
            SpotOrderMiStateMachineError::BalanceLedgerCreationFailed
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{MiStateMachineOwned, MiStateMachineOwnedUnchecked, ReplayableChanges};

    use super::*;
    use crate::entity::{SpotOrderExecution, SpotOrderSide, SpotOrderTimeInForce};

    fn sample_order(time_in_force: SpotOrderTimeInForce) -> SpotOrder {
        SpotOrder::new(
            "order-1".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            time_in_force,
            10,
            0,
            1_000,
            Some("cloid-1".to_string()),
        )
    }

    fn sell_maker(order_id: &str, price: u64, qty: u64) -> SpotOrder {
        SpotOrder::new(
            order_id.to_string(),
            10_001,
            Some(99),
            "maker-account".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            0,
            0,
            None,
        )
    }

    fn buy_maker(order_id: &str, price: u64, qty: u64) -> SpotOrder {
        SpotOrder::new(
            order_id.to_string(),
            10_001,
            Some(99),
            "maker-account".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            0,
            price.saturating_mul(qty),
            None,
        )
    }

    fn base_balance(available: u64) -> Balance {
        Balance::new("trader-1".to_string(), "BTC".to_string(), available, 0, 7)
    }

    fn quote_balance(available: u64) -> Balance {
        Balance::new("trader-1".to_string(), "USDT".to_string(), available, 0, 7)
    }

    #[test]
    fn place_happy_path_only_freezes_balance_without_trade() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let taker_base_balance = base_balance(0);
        let taker_quote_balance = quote_balance(2_000);
        let cmd = SpotOrderMiCommand::Place(PlaceSpotOrderCmd {
            taker_base_balance: &taker_base_balance,
            taker_quote_balance: &taker_quote_balance,
        });

        let changes =
            MiStateMachineOwned::compute_after_changes(&order, &cmd, &SpotOrderMiGivenState::Place)
                .unwrap();
        let events = changes.to_replayable_events().unwrap();

        let SpotOrderMiChanges::Place(place) = changes else {
            panic!("expected place changes");
        };
        assert_eq!(place.updated_order.after.status, SpotOrderStatus::Open);
        assert_eq!(place.updated_order.after.version, 2);
        assert_eq!(place.updated_balances.len(), 1);
        assert_eq!(place.created_ledger_entries.len(), 1);
        assert_eq!(place.updated_balances[0].after.available, 1_000);
        assert_eq!(place.updated_balances[0].after.frozen, 1_000);
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn authoritative_place_pipeline_projection_keeps_order_semantics_but_collapses_version_step() {
        let before = sample_order(SpotOrderTimeInForce::Ioc);
        let after_match =
            before.clone().with_execution_state(SpotOrderStatus::Filled, 10).with_version(4);

        let projected = after_match.project_authoritative_after_place_pipeline(&before).unwrap();

        assert_eq!(projected.filled_qty, 10);
        assert_eq!(projected.status, SpotOrderStatus::Filled);
        assert_eq!(projected.status_reason, None);
        assert_eq!(projected.version, 2);
    }

    #[test]
    fn match_happy_path_partially_fills_taker() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![sell_maker("maker-1", 100, 4)];
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: "match-1".to_string() });

        let changes = MiStateMachineOwned::compute_after_changes(
            &order,
            &cmd,
            &SpotOrderMiGivenState::Match { makers: &makers },
        )
        .unwrap();

        let SpotOrderMiChanges::Match(matching) = changes else {
            panic!("expected match changes");
        };
        assert_eq!(matching.created_trades.len(), 1);
        assert_eq!(matching.updated_maker_orders.len(), 1);
        assert_eq!(matching.updated_taker_order.after.filled_qty, 4);
        assert_eq!(matching.updated_taker_order.after.status, SpotOrderStatus::PartiallyFilled);
    }

    #[test]
    fn match_happy_path_fills_remaining_qty_from_partially_filled_taker() {
        let order = sample_order(SpotOrderTimeInForce::Gtc)
            .with_execution_state(SpotOrderStatus::PartiallyFilled, 4);
        let makers = vec![sell_maker("maker-1", 100, 6)];
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: "match-2".to_string() });

        let changes = MiStateMachineOwned::compute_after_changes(
            &order,
            &cmd,
            &SpotOrderMiGivenState::Match { makers: &makers },
        )
        .unwrap();

        let SpotOrderMiChanges::Match(matching) = changes else {
            panic!("expected match changes");
        };
        assert_eq!(matching.created_trades.len(), 1);
        assert_eq!(matching.updated_taker_order.after.filled_qty, 10);
        assert_eq!(matching.updated_taker_order.after.status, SpotOrderStatus::Filled);
    }

    #[test]
    fn cancel_happy_path_updates_order_status_only() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let cmd = SpotOrderMiCommand::Cancel(CancelSpotOrderCmd);

        let changes = MiStateMachineOwned::compute_after_changes(
            &order,
            &cmd,
            &SpotOrderMiGivenState::Cancel,
        )
        .unwrap();

        let SpotOrderMiChanges::Cancel(cancel) = changes else {
            panic!("expected cancel changes");
        };
        assert_eq!(cancel.updated_order.after.status, SpotOrderStatus::Canceled);
    }

    #[test]
    fn evolve_after_user_cancel_advances_single_order_lifecycle() {
        let order = sample_order(SpotOrderTimeInForce::Gtc)
            .with_execution_state(SpotOrderStatus::PartiallyFilled, 4)
            .with_version(7);

        let canceled = order.evolve_after_user_cancel().unwrap();

        assert_eq!(canceled.status, SpotOrderStatus::Canceled);
        assert_eq!(
            canceled.status_reason,
            Some(crate::entity::SpotOrderStatusReason::CanceledByUser)
        );
        assert_eq!(canceled.filled_qty, 4);
        assert_eq!(canceled.version, 8);
    }

    #[test]
    fn pre_check_rejects_empty_match_id_before_transition() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![sell_maker("maker-1", 100, 4)];
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: String::new() });

        assert_eq!(
            MiStateMachineOwned::compute_after_changes(
                &order,
                &cmd,
                &SpotOrderMiGivenState::Match { makers: &makers },
            ),
            Err(SpotOrderMiStateMachineError::InvalidMatchId)
        );
    }

    #[test]
    fn validate_transition_rejects_match_when_order_is_not_matchable() {
        let order = sample_order(SpotOrderTimeInForce::Gtc)
            .with_execution_state(SpotOrderStatus::Filled, 10);
        let makers = vec![sell_maker("maker-1", 100, 4)];
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: "match-3".to_string() });

        assert_eq!(
            MiStateMachineOwnedUnchecked::validate_state_transition(
                &order,
                &cmd,
                &SpotOrderMiGivenState::Match { makers: &makers },
            ),
            Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                command: "match",
                status: "filled",
            })
        );
    }

    #[test]
    fn match_with_same_side_maker_fails_without_falling_back_to_place() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![buy_maker("maker-1", 100, 4)];
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: "match-4".to_string() });

        assert_eq!(
            MiStateMachineOwned::compute_after_changes(
                &order,
                &cmd,
                &SpotOrderMiGivenState::Match { makers: &makers },
            ),
            Err(SpotOrderMiStateMachineError::SameSideMaker)
        );
    }

    #[test]
    fn validate_transition_rejects_match_when_given_state_branch_mismatches() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: "match-5".to_string() });

        assert_eq!(
            MiStateMachineOwnedUnchecked::validate_state_transition(
                &order,
                &cmd,
                &SpotOrderMiGivenState::Place,
            ),
            Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "match command requires match given state",
            })
        );
    }

    #[test]
    fn compute_after_changes_unchecked_stays_on_match_path() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let makers = vec![sell_maker("maker-1", 100, 4)];
        let cmd = SpotOrderMiCommand::Match(MatchSpotOrderCmd { match_id: "match-6".to_string() });

        let changes = MiStateMachineOwnedUnchecked::compute_after_changes_unchecked(
            &order,
            &cmd,
            &SpotOrderMiGivenState::Match { makers: &makers },
        )
        .unwrap();

        match changes {
            SpotOrderMiChanges::Match(matching) => {
                assert_eq!(matching.created_trades.len(), 1);
            }
            SpotOrderMiChanges::Place(_) | SpotOrderMiChanges::Cancel(_) => {
                panic!("expected match path")
            }
        }
    }
}
