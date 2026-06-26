use std::collections::HashMap;

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{Entity, EntityReplayableEvent, MiStateMachine, ReplayableChanges};
use thiserror::Error;

use super::spot_order::{SpotOrder, SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce};
use super::spot_trade::SpotTrade;
use crate::entity::Balance;
use crate::entity::account::balance_ledger_entry::{
    BalanceLedgerCommand, BalanceLedgerEntry, BalanceLedgerEntryError, BalanceLedgerReason,
    SpotSettlementLeg,
};

/// `SpotOrder` 的显式状态机命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderMiCommand {
    /// 下单并对 makers 进行即时撮合。
    Place(PlaceSpotOrderCmd),
    /// 对当前开放订单执行业务撤销。
    Cancel(CancelSpotOrderCmd),
}

/// `Place` 命令携带的 makers。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderCmd {
    /// 按撮合优先级排序的 makers。
    pub makers: Option<Vec<SpotOrder>>,
    /// taker base 余额快照。
    pub taker_base_balance: Balance,
    /// taker quote 余额快照。
    pub taker_quote_balance: Balance,
    /// 同步成交清结算所需的 maker 余额快照；本次下单没有成交时不会使用。
    pub settlement_inputs: Option<Vec<PlaceSpotOrderSettlementInput>>,
}

/// 某个 maker 参与同步清结算所需的余额快照。
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlaceSpotOrderSettlementInput {
    /// 该余额快照对应的 maker 订单 ID。
    pub maker_order_id: String,
    /// 本次流水所属清结算批次 ID。
    pub settlement_batch_id: String,
    /// maker base 余额快照。
    pub maker_base_balance: Balance,
    /// maker quote 余额快照。
    pub maker_quote_balance: Balance,
}

/// `Place` 命令的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderChanges {
    /// taker 订单在本 case 内的起止状态。
    pub updated_order: UpdatedEntityPair<SpotOrder>,
    /// 本次生成的成交事实。
    pub created_trades: Vec<SpotTrade>,
    /// 本 case 涉及的余额起止状态；同一个 balance 实例最多出现一次。
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    /// 本次创建的余额流水；按业务发生顺序排列，表达 freeze / settlement 中间过程。
    pub created_ledger_entries: Vec<BalanceLedgerEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlaceSpotOrderMatchingEffects {
    created_trades: Vec<SpotTrade>,
    updated_balances: Vec<UpdatedEntityPair<Balance>>,
    created_ledger_entries: Vec<BalanceLedgerEntry>,
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
    /// `Cancel` 的 changes。
    Cancel(CancelSpotOrderChanges),
}

impl ReplayableChanges for PlaceSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        let mut events = Vec::with_capacity(
            1 + self.created_trades.len()
                + self.updated_balances.len()
                + self.created_ledger_entries.len(),
        );
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
        for trade in &self.created_trades {
            events.push(trade.track_create_event()?);
        }
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

fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerEntry],
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
            Self::Cancel(changes) => changes.to_replayable_events(),
        }
    }
}

/// `SpotOrder` 显式状态机的业务错误。
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
    #[error("spot order settlement input is required when matching creates trades")]
    MissingSettlementInput,
    #[error("spot order settlement input does not match created trades")]
    SettlementInputMismatch,
    #[error("spot order settlement balance is insufficient")]
    InsufficientSettlementBalance,
}

impl MiStateMachine for SpotOrder {
    type Command = SpotOrderMiCommand;
    type State = SpotOrderStatus;
    type Error = SpotOrderMiStateMachineError;
    type Changes = SpotOrderMiChanges;

    fn state(&self) -> &Self::State {
        &self.status
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if !self.has_consistent_execution_state() {
            return Err(SpotOrderMiStateMachineError::InconsistentExecutionState);
        }

        match cmd {
            SpotOrderMiCommand::Place(place) => self.pre_check_place_command(place),
            SpotOrderMiCommand::Cancel(_) => Ok(()),
        }
    }

    fn validate_state_transition(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd {
            SpotOrderMiCommand::Place(_) => {
                if matches!(self.state(), SpotOrderStatus::Open) {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "place",
                        status: self.state().as_str(),
                    })
                }
            }
            SpotOrderMiCommand::Cancel(_) => {
                if self.can_be_cancelled() {
                    Ok(())
                } else {
                    Err(SpotOrderMiStateMachineError::CommandNotAllowedInCurrentState {
                        command: "cancel",
                        status: self.state().as_str(),
                    })
                }
            }
        }
    }

    fn compute_changes(&self, cmd: &Self::Command) -> Result<Self::Changes, Self::Error> {
        self.pre_check_command(cmd)?;
        self.validate_state_transition(cmd)?;

        match cmd {
            SpotOrderMiCommand::Place(place) => self.compute_place_changes(place),
            SpotOrderMiCommand::Cancel(cancel) => self.compute_cancel_changes(cancel),
        }
    }
}

impl SpotOrder {
    fn compute_place_changes(
        &self,
        cmd: &PlaceSpotOrderCmd,
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let before = self.clone();
        let mut after = self.clone();
        let mut created_trades = Vec::new();

        let Some(makers) = &cmd.makers else {
            after = self.finalize_place_without_makers()?;
            let (frozen_balance, created_freeze_ledger_entry) =
                derive_place_freeze_changes(&after, taker_freeze_balance(&after, cmd))?;
            return Ok(SpotOrderMiChanges::Place(PlaceSpotOrderChanges {
                updated_order: UpdatedEntityPair { before, after },
                created_trades,
                updated_balances: vec![frozen_balance],
                created_ledger_entries: vec![created_freeze_ledger_entry],
            }));
        };

        if makers.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "makers must not be empty when present",
            });
        }

        let mut remaining_qty = after
            .remaining_qty()
            .map_err(|_| SpotOrderMiStateMachineError::FilledQtyOverflowOrExceedsQty)?;

        for (index, maker) in makers.iter().enumerate() {
            if remaining_qty == 0 {
                break;
            }

            maker.ensure_matchable().map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker is not matchable",
                }
            })?;
            maker.ensure_compatible_maker_for(self).map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker is not compatible with taker",
                }
            })?;

            if !self.crosses_order(maker).map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "failed to evaluate crossing",
                }
            })? {
                break;
            }

            let maker_price =
                maker.limit_price().ok_or(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker must be a limit order",
                })?;
            let maker_remaining = maker.remaining_qty().map_err(|_| {
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "maker execution state is inconsistent",
                }
            })?;
            let trade_qty = remaining_qty.min(maker_remaining);
            if trade_qty == 0 {
                continue;
            }

            created_trades.push(SpotTrade::new(
                format!("{}-{}", self.order_id, index + 1),
                self.order_id.clone(),
                self.asset,
                self.symbol.clone(),
                self.order_id.clone(),
                maker.order_id.clone(),
                self.account_id.clone(),
                maker.account_id.clone(),
                self.side,
                maker_price,
                trade_qty,
            ));

            after.filled_qty = after
                .filled_qty
                .checked_add(trade_qty)
                .ok_or(SpotOrderMiStateMachineError::FilledQtyOverflowOrExceedsQty)?;
            remaining_qty = remaining_qty
                .checked_sub(trade_qty)
                .ok_or(SpotOrderMiStateMachineError::FilledQtyOverflowOrExceedsQty)?;
        }

        after.status = if after.filled_qty == after.qty {
            SpotOrderStatus::Filled
        } else if after.filled_qty > 0 && after.time_in_force == SpotOrderTimeInForce::Ioc {
            after.status_reason = Some(SpotOrderStatusReason::IocCancelRejected);
            SpotOrderStatus::Canceled
        } else if after.filled_qty > 0 {
            SpotOrderStatus::PartiallyFilled
        } else {
            SpotOrderStatus::Open
        };

        if after.filled_qty == 0 && after.time_in_force == SpotOrderTimeInForce::Ioc {
            after.status = SpotOrderStatus::Rejected;
            after.status_reason = Some(after.no_liquidity_status_reason());
        }

        after.version =
            after.version.checked_add(1).ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;

        let (frozen_balance, created_freeze_ledger_entry) =
            derive_place_freeze_changes(&after, taker_freeze_balance(&after, cmd))?;
        let mut balance_transitions = vec![frozen_balance.clone()];
        let mut created_ledger_entries = vec![created_freeze_ledger_entry];
        let created_trades = if created_trades.is_empty() {
            created_trades
        } else {
            let matching = derive_place_matching_changes(
                created_trades,
                &cmd.settlement_inputs,
                &frozen_balance.after,
                &cmd.taker_base_balance,
                &cmd.taker_quote_balance,
                self.side,
            )?;
            balance_transitions.extend(matching.updated_balances);
            created_ledger_entries.extend(matching.created_ledger_entries);
            matching.created_trades
        };
        let updated_balances = collapse_balance_transitions(balance_transitions)?;

        Ok(SpotOrderMiChanges::Place(PlaceSpotOrderChanges {
            updated_order: UpdatedEntityPair { before, after },
            created_trades,
            updated_balances,
            created_ledger_entries,
        }))
    }

    fn finalize_place_without_makers(&self) -> Result<SpotOrder, SpotOrderMiStateMachineError> {
        let mut after = self.clone();
        after.version =
            after.version.checked_add(1).ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;
        after.status = match after.time_in_force {
            SpotOrderTimeInForce::Ioc => {
                after.status_reason = Some(after.no_liquidity_status_reason());
                SpotOrderStatus::Rejected
            }
            _ => SpotOrderStatus::Open,
        };
        Ok(after)
    }

    fn pre_check_place_command(
        &self,
        cmd: &PlaceSpotOrderCmd,
    ) -> Result<(), SpotOrderMiStateMachineError> {
        if let Some(makers) = &cmd.makers {
            if makers.is_empty() {
                return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "makers must not be empty when present",
                });
            }
            for maker in makers {
                if !maker.has_consistent_execution_state() {
                    return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                        reason: "maker execution state is inconsistent",
                    });
                }
            }
        }

        Ok(())
    }

    fn compute_cancel_changes(
        &self,
        _cmd: &CancelSpotOrderCmd,
    ) -> Result<SpotOrderMiChanges, SpotOrderMiStateMachineError> {
        let before = self.clone();
        let mut after = self.clone();
        after.status = SpotOrderStatus::Canceled;
        after.status_reason = Some(SpotOrderStatusReason::CanceledByUser);
        after.version =
            after.version.checked_add(1).ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;
        Ok(SpotOrderMiChanges::Cancel(CancelSpotOrderChanges {
            updated_order: UpdatedEntityPair { before, after },
        }))
    }
}

fn derive_place_freeze_changes(
    order: &SpotOrder,
    freeze_balance: &Balance,
) -> Result<(UpdatedEntityPair<Balance>, BalanceLedgerEntry), SpotOrderMiStateMachineError> {
    if !freeze_balance.belongs_to_account(&order.account_id) {
        return Err(SpotOrderMiStateMachineError::FreezeBalanceAccountMismatch);
    }

    let amount = if order.reserved_quote > 0 {
        order.reserved_quote
    } else if order.reserved_base > 0 {
        order.reserved_base
    } else {
        return Err(SpotOrderMiStateMachineError::MissingReservation);
    };

    if !freeze_balance.can_reserve(amount) {
        return Err(SpotOrderMiStateMachineError::InsufficientFreezeBalance);
    }

    let entry_id = format!("balance-ledger:{}:{}", order.order_id, freeze_balance.entity_id());
    let balance_command = BalanceLedgerCommand::Freeze { balance: freeze_balance.clone(), amount };
    let draft_entry = BalanceLedgerEntry::draft_from_balance(
        entry_id,
        freeze_balance,
        balance_command.clone(),
        BalanceLedgerReason::FreezeForOrder { order_id: order.order_id.clone() },
    )
    .map_err(map_balance_ledger_error)?;
    let changes =
        draft_entry.compute_changes(&balance_command).map_err(map_balance_ledger_error)?;
    Ok((changes.updated_balance, changes.updated_entry.after))
}

fn taker_freeze_balance<'a>(order: &SpotOrder, cmd: &'a PlaceSpotOrderCmd) -> &'a Balance {
    if order.reserved_quote > 0 { &cmd.taker_quote_balance } else { &cmd.taker_base_balance }
}

fn derive_place_matching_changes(
    created_trades: Vec<SpotTrade>,
    settlement_inputs: &Option<Vec<PlaceSpotOrderSettlementInput>>,
    placing_frozen_balance: &Balance,
    taker_base_balance: &Balance,
    taker_quote_balance: &Balance,
    taker_side: crate::SpotOrderSide,
) -> Result<PlaceSpotOrderMatchingEffects, SpotOrderMiStateMachineError> {
    let inputs =
        settlement_inputs.as_ref().ok_or(SpotOrderMiStateMachineError::MissingSettlementInput)?;
    let inputs_by_maker_order_id = settlement_inputs_by_maker_order_id(inputs)?;

    let mut updated_balances = Vec::with_capacity(created_trades.len() * 4);
    let mut created_ledger_entries = Vec::with_capacity(created_trades.len() * 4);
    let mut current_balances = HashMap::new();
    for trade in &created_trades {
        let input = inputs_by_maker_order_id
            .get(trade.maker_order_id.as_str())
            .ok_or(SpotOrderMiStateMachineError::MissingSettlementInput)?;
        if input.settlement_batch_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::SettlementInputMismatch);
        }
        let quote_qty =
            trade.notional_quote().ok_or(SpotOrderMiStateMachineError::VersionOverflow)?;
        validate_settlement_input_accounts(trade, input, taker_base_balance, taker_quote_balance)?;

        let (buyer_base_balance, buyer_quote_balance, seller_quote_balance, seller_base_balance) =
            if taker_side == crate::SpotOrderSide::Buy {
                (
                    taker_base_balance,
                    placing_frozen_balance,
                    &input.maker_quote_balance,
                    &input.maker_base_balance,
                )
            } else {
                (
                    &input.maker_base_balance,
                    &input.maker_quote_balance,
                    taker_quote_balance,
                    placing_frozen_balance,
                )
            };
        let buyer_base_balance =
            current_balance_snapshot(&mut current_balances, buyer_base_balance);
        let buyer_quote_balance =
            current_balance_snapshot(&mut current_balances, buyer_quote_balance);
        let seller_quote_balance =
            current_balance_snapshot(&mut current_balances, seller_quote_balance);
        let seller_base_balance =
            current_balance_snapshot(&mut current_balances, seller_base_balance);

        let legs = [
            (
                SpotSettlementLeg::BuyerReceiveBase,
                BalanceLedgerCommand::CreditAvailable {
                    balance: buyer_base_balance,
                    amount: trade.qty,
                },
            ),
            (
                SpotSettlementLeg::BuyerDebitFrozenQuote,
                BalanceLedgerCommand::DebitFrozen {
                    balance: buyer_quote_balance,
                    amount: quote_qty,
                },
            ),
            (
                SpotSettlementLeg::SellerReceiveQuote,
                BalanceLedgerCommand::CreditAvailable {
                    balance: seller_quote_balance,
                    amount: quote_qty,
                },
            ),
            (
                SpotSettlementLeg::SellerDebitFrozenBase,
                BalanceLedgerCommand::DebitFrozen {
                    balance: seller_base_balance,
                    amount: trade.qty,
                },
            ),
        ];

        for (leg, balance_command) in legs {
            let entry_id = format!(
                "balance-ledger:{}:{}:{}",
                input.settlement_batch_id,
                trade.trade_id,
                leg.as_str()
            );
            let reason = BalanceLedgerReason::SettleSpotTrade {
                trade_id: trade.trade_id.clone(),
                match_id: trade.match_id.clone(),
                settlement_batch_id: input.settlement_batch_id.clone(),
                leg,
            };
            let draft_entry = BalanceLedgerEntry::draft_from_balance(
                entry_id,
                balance_ledger_command_balance_ref(&balance_command),
                balance_command.clone(),
                reason,
            )
            .map_err(map_settlement_balance_ledger_error)?;
            let changes = draft_entry
                .compute_changes(&balance_command)
                .map_err(map_settlement_balance_ledger_error)?;
            current_balances.insert(
                changes.updated_balance.after.entity_id(),
                changes.updated_balance.after.clone(),
            );
            updated_balances.push(changes.updated_balance);
            created_ledger_entries.push(changes.updated_entry.after);
        }
    }

    Ok(PlaceSpotOrderMatchingEffects { created_trades, updated_balances, created_ledger_entries })
}

fn collapse_balance_transitions(
    transitions: Vec<UpdatedEntityPair<Balance>>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, SpotOrderMiStateMachineError> {
    let mut index_by_balance_id = HashMap::<String, usize>::with_capacity(transitions.len());
    let mut collapsed: Vec<UpdatedEntityPair<Balance>> = Vec::new();
    for transition in transitions {
        let balance_id = transition.before.entity_id();
        if let Some(index) = index_by_balance_id.get(&balance_id).copied() {
            if collapsed[index].after != transition.before {
                return Err(SpotOrderMiStateMachineError::SettlementInputMismatch);
            }
            collapsed[index].after = transition.after;
        } else {
            index_by_balance_id.insert(balance_id, collapsed.len());
            collapsed.push(transition);
        }
    }
    Ok(collapsed)
}

fn settlement_inputs_by_maker_order_id(
    inputs: &[PlaceSpotOrderSettlementInput],
) -> Result<HashMap<&str, &PlaceSpotOrderSettlementInput>, SpotOrderMiStateMachineError> {
    let mut inputs_by_maker_order_id = HashMap::with_capacity(inputs.len());
    for input in inputs {
        if input.maker_order_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::SettlementInputMismatch);
        }
        if inputs_by_maker_order_id.insert(input.maker_order_id.as_str(), input).is_some() {
            return Err(SpotOrderMiStateMachineError::SettlementInputMismatch);
        }
    }
    Ok(inputs_by_maker_order_id)
}

fn current_balance_snapshot(
    current_balances: &mut HashMap<String, Balance>,
    initial: &Balance,
) -> Balance {
    current_balances.entry(initial.entity_id()).or_insert_with(|| initial.clone()).clone()
}

fn balance_ledger_command_balance_ref(command: &BalanceLedgerCommand) -> &Balance {
    match command {
        BalanceLedgerCommand::Freeze { balance, .. }
        | BalanceLedgerCommand::Unfreeze { balance, .. }
        | BalanceLedgerCommand::CreditAvailable { balance, .. }
        | BalanceLedgerCommand::DebitAvailable { balance, .. }
        | BalanceLedgerCommand::DebitFrozen { balance, .. } => balance,
    }
}

fn validate_settlement_input_accounts(
    trade: &SpotTrade,
    input: &PlaceSpotOrderSettlementInput,
    taker_base_balance: &Balance,
    taker_quote_balance: &Balance,
) -> Result<(), SpotOrderMiStateMachineError> {
    if input.maker_base_balance.account_id != trade.maker_account_id
        || input.maker_quote_balance.account_id != trade.maker_account_id
        || taker_base_balance.account_id != trade.taker_account_id
        || taker_quote_balance.account_id != trade.taker_account_id
    {
        Err(SpotOrderMiStateMachineError::SettlementInputMismatch)
    } else {
        Ok(())
    }
}

fn map_balance_ledger_error(error: BalanceLedgerEntryError) -> SpotOrderMiStateMachineError {
    match error {
        BalanceLedgerEntryError::InvalidAmount => SpotOrderMiStateMachineError::MissingReservation,
        BalanceLedgerEntryError::InsufficientAvailableBalance => {
            SpotOrderMiStateMachineError::InsufficientFreezeBalance
        }
        BalanceLedgerEntryError::ArithmeticOverflow => {
            SpotOrderMiStateMachineError::VersionOverflow
        }
        BalanceLedgerEntryError::SnapshotMismatch
        | BalanceLedgerEntryError::InsufficientFrozenBalance
        | BalanceLedgerEntryError::AlreadyApplied
        | BalanceLedgerEntryError::InvalidStatusTransition
        | BalanceLedgerEntryError::CommandMismatch => {
            SpotOrderMiStateMachineError::BalanceLedgerCreationFailed
        }
    }
}

fn map_settlement_balance_ledger_error(
    error: BalanceLedgerEntryError,
) -> SpotOrderMiStateMachineError {
    match error {
        BalanceLedgerEntryError::InvalidAmount => SpotOrderMiStateMachineError::MissingReservation,
        BalanceLedgerEntryError::InsufficientAvailableBalance
        | BalanceLedgerEntryError::InsufficientFrozenBalance => {
            SpotOrderMiStateMachineError::InsufficientSettlementBalance
        }
        BalanceLedgerEntryError::ArithmeticOverflow => {
            SpotOrderMiStateMachineError::VersionOverflow
        }
        BalanceLedgerEntryError::SnapshotMismatch
        | BalanceLedgerEntryError::AlreadyApplied
        | BalanceLedgerEntryError::InvalidStatusTransition
        | BalanceLedgerEntryError::CommandMismatch => {
            SpotOrderMiStateMachineError::SettlementInputMismatch
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{MiStateMachine, ReplayableChanges};

    use super::*;
    use crate::{SpotOrderExecution, SpotOrderSide};

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

    fn maker(price: u64, qty: u64) -> SpotOrder {
        SpotOrder::new(
            "maker-1".to_string(),
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

    fn quote_balance(available: u64) -> Balance {
        Balance::new("trader-1".to_string(), "USDT".to_string(), available, 0, 7)
    }

    fn base_balance(available: u64) -> Balance {
        Balance::new("trader-1".to_string(), "BTC".to_string(), available, 0, 7)
    }

    fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }
            std::str::from_utf8(change.new_value_bytes()).ok()
        })
    }

    fn balance_change<'a>(
        changes: &'a PlaceSpotOrderChanges,
        account_id: &str,
        asset_id: &str,
    ) -> &'a UpdatedEntityPair<Balance> {
        changes
            .updated_balances
            .iter()
            .find(|balance| {
                balance.before.account_id == account_id && balance.before.asset_id == asset_id
            })
            .expect("expected balance change")
    }

    fn place(makers: Option<Vec<SpotOrder>>) -> SpotOrderMiCommand {
        SpotOrderMiCommand::Place(PlaceSpotOrderCmd {
            makers,
            taker_base_balance: base_balance(0),
            taker_quote_balance: quote_balance(2_000),
            settlement_inputs: None,
        })
    }

    fn place_with_balance(
        makers: Option<Vec<SpotOrder>>,
        taker_base_balance: Balance,
        taker_quote_balance: Balance,
    ) -> SpotOrderMiCommand {
        SpotOrderMiCommand::Place(PlaceSpotOrderCmd {
            makers,
            taker_base_balance,
            taker_quote_balance,
            settlement_inputs: None,
        })
    }

    fn settlement_input() -> PlaceSpotOrderSettlementInput {
        PlaceSpotOrderSettlementInput {
            maker_order_id: "maker-1".to_string(),
            settlement_batch_id: "settle-1".to_string(),
            maker_base_balance: Balance::new(
                "maker-account".to_string(),
                "BTC".to_string(),
                0,
                4,
                3,
            ),
            maker_quote_balance: Balance::new(
                "maker-account".to_string(),
                "USDT".to_string(),
                0,
                0,
                3,
            ),
        }
    }

    fn place_with_settlement(makers: Vec<SpotOrder>) -> SpotOrderMiCommand {
        SpotOrderMiCommand::Place(PlaceSpotOrderCmd {
            makers: Some(makers),
            taker_base_balance: base_balance(0),
            taker_quote_balance: quote_balance(2_000),
            settlement_inputs: Some(vec![settlement_input()]),
        })
    }

    fn cancel() -> SpotOrderMiCommand {
        SpotOrderMiCommand::Cancel(CancelSpotOrderCmd)
    }

    #[test]
    fn open_place_zero_makers_gtc_stays_open() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);

        let result = <SpotOrder as MiStateMachine>::compute_changes(&order, &place(None)).unwrap();
        let events = result.to_replayable_events().unwrap();

        assert_eq!(events.len(), 3);
        let SpotOrderMiChanges::Place(changes) = result else {
            panic!("expected place changes");
        };
        assert!(changes.created_trades.is_empty());
        assert_eq!(changes.updated_balances.len(), 1);
        assert_eq!(changes.created_ledger_entries.len(), 1);
        assert_eq!(
            events[0]
                .field_changes
                .iter()
                .find(|c| c.field_name_as_str().ok() == Some("version"))
                .unwrap()
                .new_value_bytes(),
            b"2"
        );
        assert!(events[1].is_updated());
        assert!(events[2].is_created());
    }

    #[test]
    fn open_place_with_makers_emits_order_trade_balance_and_ledger() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_settlement(vec![maker(100, 4)]),
        )
        .unwrap();
        let events = result.to_replayable_events().unwrap();

        assert_eq!(events.len(), 12);
        assert!(events[0].is_updated());
        assert!(events[1].is_created());
        assert!(events[2].is_updated());
        assert!(events[7].is_created());
        assert_eq!(event_field(&events[0], "filled_qty"), Some("4"));
        assert_eq!(event_field(&events[0], "status"), Some("partially_filled"));
        assert_eq!(event_field(&events[1], "trade_id"), Some("order-1-1"));
        assert_eq!(
            event_field(&events[7], "entry_id"),
            Some("balance-ledger:order-1:trader-1:USDT")
        );
        assert_eq!(event_field(&events[8], "reason"), Some("settle_spot_trade"));
        assert_eq!(event_field(&events[8], "reason_settlement_leg"), Some("buyer_receive_base"));
    }

    #[test]
    fn buy_place_freezes_quote_balance_and_creates_applied_ledger() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_balance(None, base_balance(0), quote_balance(2_000)),
        )
        .unwrap();

        let SpotOrderMiChanges::Place(changes) = result else {
            panic!("expected place changes");
        };
        let quote_change = balance_change(&changes, "trader-1", "USDT");
        assert_eq!(quote_change.before.available, 2_000);
        assert_eq!(quote_change.after.available, 1_000);
        assert_eq!(quote_change.after.frozen, 1_000);
        assert!(changes.created_ledger_entries[0].matches_balance_update(quote_change));
        assert_eq!(
            changes.created_ledger_entries[0].entry_id,
            "balance-ledger:order-1:trader-1:USDT"
        );
    }

    #[test]
    fn sell_place_freezes_base_balance() {
        let mut order = sample_order(SpotOrderTimeInForce::Gtc);
        order.side = SpotOrderSide::Sell;
        order.reserved_quote = 0;
        order.reserved_base = 10;

        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_balance(None, base_balance(20), quote_balance(0)),
        )
        .unwrap();

        let SpotOrderMiChanges::Place(changes) = result else {
            panic!("expected place changes");
        };
        let base_change = balance_change(&changes, "trader-1", "BTC");
        assert_eq!(base_change.after.available, 10);
        assert_eq!(base_change.after.frozen, 10);
        assert_eq!(base_change.after.asset_id, "BTC");
    }

    #[test]
    fn place_with_trade_requires_settlement_inputs() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);

        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place(Some(vec![maker(100, 4)])),
        );

        assert_eq!(result, Err(SpotOrderMiStateMachineError::MissingSettlementInput));
    }

    #[test]
    fn place_with_trade_looks_up_settlement_input_by_maker_order_id() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let unused_input = PlaceSpotOrderSettlementInput {
            maker_order_id: "maker-unused".to_string(),
            settlement_batch_id: "settle-unused".to_string(),
            maker_base_balance: Balance::new(
                "maker-account".to_string(),
                "BTC".to_string(),
                0,
                4,
                3,
            ),
            maker_quote_balance: Balance::new(
                "maker-account".to_string(),
                "USDT".to_string(),
                0,
                0,
                3,
            ),
        };
        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &SpotOrderMiCommand::Place(PlaceSpotOrderCmd {
                makers: Some(vec![maker(100, 4)]),
                taker_base_balance: base_balance(0),
                taker_quote_balance: quote_balance(2_000),
                settlement_inputs: Some(vec![unused_input, settlement_input()]),
            }),
        )
        .unwrap();

        let SpotOrderMiChanges::Place(changes) = result else {
            panic!("expected place changes");
        };
        assert!(changes.created_ledger_entries.iter().skip(1).all(|entry| {
            matches!(
                &entry.reason,
                BalanceLedgerReason::SettleSpotTrade {
                    settlement_batch_id,
                    ..
                } if settlement_batch_id == "settle-1"
            )
        }));
    }

    #[test]
    fn place_with_trade_matching_contains_settlement_effects() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_settlement(vec![maker(100, 4)]),
        )
        .unwrap();

        let SpotOrderMiChanges::Place(changes) = result else {
            panic!("expected place changes");
        };
        assert_eq!(changes.created_trades.len(), 1);
        assert_eq!(changes.updated_balances.len(), 4);
        assert_eq!(changes.created_ledger_entries.len(), 5);
        assert!(
            changes.created_ledger_entries.iter().skip(1).all(|entry| {
                matches!(entry.reason, BalanceLedgerReason::SettleSpotTrade { .. })
            })
        );
        let taker_quote = balance_change(&changes, "trader-1", "USDT");
        assert_eq!(taker_quote.before.available, 2_000);
        assert_eq!(taker_quote.before.frozen, 0);
        assert_eq!(taker_quote.after.available, 1_000);
        assert_eq!(taker_quote.after.frozen, 600);
        assert!(changes.created_ledger_entries.iter().any(|entry| {
            matches!(
                entry.reason,
                BalanceLedgerReason::SettleSpotTrade {
                    leg: SpotSettlementLeg::BuyerDebitFrozenQuote,
                    ..
                }
            ) && entry.after_available == 1_000
                && entry.after_frozen == 600
        }));
    }

    #[test]
    fn place_rejects_insufficient_freeze_balance() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);

        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_balance(None, base_balance(0), quote_balance(999)),
        );

        assert_eq!(result, Err(SpotOrderMiStateMachineError::InsufficientFreezeBalance));
    }

    #[test]
    fn place_rejects_freeze_balance_from_other_account() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let balance = Balance::new("other".to_string(), "USDT".to_string(), 2_000, 0, 7);

        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_balance(None, base_balance(0), balance),
        );

        assert_eq!(result, Err(SpotOrderMiStateMachineError::FreezeBalanceAccountMismatch));
    }

    #[test]
    fn place_rejects_zero_reservation() {
        let mut order = sample_order(SpotOrderTimeInForce::Gtc);
        order.reserved_quote = 0;
        order.reserved_base = 0;

        let result = <SpotOrder as MiStateMachine>::compute_changes(
            &order,
            &place_with_balance(None, base_balance(0), quote_balance(2_000)),
        );

        assert_eq!(result, Err(SpotOrderMiStateMachineError::MissingReservation));
    }

    #[test]
    fn open_cancel_updates_order_only() {
        let order = sample_order(SpotOrderTimeInForce::Gtc);
        let result = <SpotOrder as MiStateMachine>::compute_changes(&order, &cancel()).unwrap();
        let events = result.to_replayable_events().unwrap();

        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(
            events[0]
                .field_changes
                .iter()
                .find(|c| c.field_name_as_str().ok() == Some("status_reason"))
                .unwrap()
                .new_value_bytes(),
            b"canceled"
        );
    }
}
