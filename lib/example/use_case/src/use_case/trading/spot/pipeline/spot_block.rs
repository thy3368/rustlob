use std::collections::HashMap;

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, OrderSide, OrderType, SpotOrderStatus, SpotOrderV2};
use crate::{
    ActivateMatchSpotOrderV2Changes, ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2Error,
    ActivateMatchSpotOrderV2State, ActivateMatchSpotOrderV2UseCase, ActivateSpotOrderV2Changes,
    ActivateSpotOrderV2Cmd, ActivateSpotOrderV2Error, ActivateSpotOrderV2State,
    ActivateSpotOrderV2UseCase, CancelSpotOrderV2Changes, CancelSpotOrderV2Cmd,
    CancelSpotOrderV2Error, CancelSpotOrderV2Lookup, CancelSpotOrderV2State,
    CancelSpotOrderV2UseCase, ModifySpotOrderV2Changes, ModifySpotOrderV2Cmd,
    ModifySpotOrderV2Error, ModifySpotOrderV2State, ModifySpotOrderV2UseCase, OrderId,
    PlaceMatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error, PlaceMatchSpotOrderV2State,
    PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBlockCmd {
    pub commands: Vec<SpotBlockCommand>,
}

// LogEntry<SpotBlockCommand>

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBlockCommand {
    Activate(ActivateSpotOrderV2Cmd),
    Cancel(CancelSpotOrderV2Cmd),
    Modify(ModifySpotOrderV2Cmd),
    PlaceMatch(PlaceOnlySpotOrderV2Cmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBlockState {
    pub orders: Vec<SpotOrderV2>,
    pub balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBlockChanges {
    pub item_results: Vec<SpotBlockItemResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockItemResult {
    Applied { command_index: usize, command: SpotBlockCommand, changes: SpotBlockAppliedChanges },
    Rejected { command_index: usize, command: SpotBlockCommand, error: SpotBlockItemError },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockAppliedChanges {
    Activate(ActivateSpotOrderV2Changes),
    Cancel(CancelSpotOrderV2Changes),
    Modify(ModifySpotOrderV2Changes),
    PlaceMatch(PlaceMatchSpotOrderV2Changes),
    ActivateMatch(ActivateMatchSpotOrderV2Changes),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockError {
    #[error("spot block must contain at least one on_command")]
    EmptyCommands,
    #[error("spot block on_command party id must not be empty")]
    EmptyPartyId,
    #[error("spot block commands must share one party id")]
    MixedPartyIds,
    #[error("spot block state has invalid order or balance indexes")]
    InvalidStateIndexing,
    #[error("spot block state contains duplicate order entity identity")]
    DuplicateOrderIdentity,
    #[error("spot block state contains duplicate client order id")]
    DuplicateClientOrderId,
    #[error("spot block state contains duplicate balance entity id")]
    DuplicateBalanceEntityId,
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockItemError {
    #[error(transparent)]
    Activate(#[from] ActivateSpotOrderV2Error),
    #[error(transparent)]
    Cancel(#[from] CancelSpotOrderV2Error),
    #[error(transparent)]
    Modify(#[from] ModifySpotOrderV2Error),
    #[error(transparent)]
    PlaceMatch(#[from] PlaceMatchSpotOrderV2Error),
    #[error(transparent)]
    ActivateMatch(#[from] ActivateMatchSpotOrderV2Error),
    #[error("order not found")]
    OrderNotFound,
    #[error("order lookup is ambiguous")]
    OrderLookupAmbiguous,
    #[error("place-match branch cannot be mapped into working state")]
    UnsupportedPlaceBranch,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpotBlockUseCase;

impl ReplayableChanges for SpotBlockChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::new();
        for item in &self.item_results {
            let SpotBlockItemResult::Applied { changes, .. } = item else {
                continue;
            };
            match changes {
                SpotBlockAppliedChanges::Activate(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::Cancel(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::Modify(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::PlaceMatch(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::ActivateMatch(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
            }
        }
        Ok(events)
    }
}

impl StateMachineV2 for SpotBlockUseCase {
    type Command = LogEntry<SpotBlockCmd>;
    type StateGiven = SpotBlockState;
    type Error = SpotBlockError;
    type StateChanged = SpotBlockChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.commands.is_empty() {
            return Err(SpotBlockError::EmptyCommands);
        }
        if cmd.commands.iter().any(|command| command_party_id(command).is_none_or(str::is_empty)) {
            return Err(SpotBlockError::EmptyPartyId);
        }
        shared_party_id(&cmd.commands).ok_or(SpotBlockError::MixedPartyIds)?;
        Ok(())
    }

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if given_state.fee_account_id.is_empty() {
            return Err(SpotBlockError::InvalidFeeAccountId);
        }
        WorkingSpotBlockState::try_from_state(given_state).map(|_| ())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut working = WorkingSpotBlockState::try_from_state(given_state)?;
        let mut item_results = Vec::with_capacity(cmd.commands.len());

        self.compute_action_results(cmd, given_state, &mut working, &mut item_results);

        //todo 从trade中取成交价，看哪些条件单需要激活
        self.compute_conditional_order_results(cmd, given_state, &mut working, &mut item_results);
        item_results.sort_by_key(|item| match item {
            SpotBlockItemResult::Applied { command_index, .. }
            | SpotBlockItemResult::Rejected { command_index, .. } => *command_index,
        });

        Ok(SpotBlockChanges { item_results })
    }
}

impl SpotBlockUseCase {
    fn compute_action_results(
        &self,
        cmd: &LogEntry<SpotBlockCmd>,
        given_state: &SpotBlockState,
        working: &mut WorkingSpotBlockState,
        item_results: &mut Vec<SpotBlockItemResult>,
    ) {
        for (command_index, command) in cmd.commands.iter().cloned().enumerate() {
            if matches!(&command, SpotBlockCommand::Activate(_)) {
                continue;
            }
            let item_result = match &command {
                SpotBlockCommand::Cancel(cancel_cmd) => {
                    match apply_cancel(
                        &common_entity::LogEntrySequencer::derive(cmd, cancel_cmd.clone()),
                        given_state,
                        working,
                    ) {
                        Ok(changes) => SpotBlockItemResult::Applied {
                            command_index,
                            command,
                            changes: SpotBlockAppliedChanges::Cancel(changes),
                        },
                        Err(error) => {
                            SpotBlockItemResult::Rejected { command_index, command, error }
                        }
                    }
                }
                SpotBlockCommand::Modify(modify_cmd) => {
                    match apply_modify(
                        &common_entity::LogEntrySequencer::derive(cmd, modify_cmd.clone()),
                        given_state,
                        working,
                    ) {
                        Ok(changes) => SpotBlockItemResult::Applied {
                            command_index,
                            command,
                            changes: SpotBlockAppliedChanges::Modify(changes),
                        },
                        Err(error) => {
                            SpotBlockItemResult::Rejected { command_index, command, error }
                        }
                    }
                }
                SpotBlockCommand::PlaceMatch(place_cmd) => {
                    match apply_place_match(
                        &common_entity::LogEntrySequencer::derive(cmd, place_cmd.clone()),
                        given_state,
                        working,
                    ) {
                        Ok(changes) => SpotBlockItemResult::Applied {
                            command_index,
                            command,
                            changes: SpotBlockAppliedChanges::PlaceMatch(changes),
                        },
                        Err(error) => {
                            SpotBlockItemResult::Rejected { command_index, command, error }
                        }
                    }
                }

                SpotBlockCommand::Activate(_) => {
                    unreachable!("activate commands are handled later")
                }
            };
            item_results.push(item_result);
        }
    }

    fn compute_conditional_order_results(
        &self,
        cmd: &LogEntry<SpotBlockCmd>,
        given_state: &SpotBlockState,
        working: &mut WorkingSpotBlockState,
        item_results: &mut Vec<SpotBlockItemResult>,
    ) {
        for (command_index, command) in cmd.commands.iter().cloned().enumerate() {
            let SpotBlockCommand::Activate(activate_cmd) = command else {
                continue;
            };
            let item_result = match apply_activate(
                &common_entity::LogEntrySequencer::derive(cmd, activate_cmd.clone()),
                given_state,
                working,
            ) {
                Ok(changes) => SpotBlockItemResult::Applied {
                    command_index,
                    command: SpotBlockCommand::Activate(activate_cmd),
                    changes: SpotBlockAppliedChanges::Activate(changes),
                },
                Err(error) => SpotBlockItemResult::Rejected {
                    command_index,
                    command: SpotBlockCommand::Activate(activate_cmd),
                    error,
                },
            };
            item_results.push(item_result);
        }
        let mut triggered = std::collections::HashSet::new();
        for result in item_results.iter() {
            if let SpotBlockItemResult::Applied {
                command: SpotBlockCommand::Activate(command),
                ..
            } = result
            {
                triggered.insert(command.order_id);
            }
        }
        let sources = item_results
            .iter()
            .filter_map(|result| match result {
                SpotBlockItemResult::Applied {
                    command_index,
                    command: SpotBlockCommand::PlaceMatch(command),
                    changes: SpotBlockAppliedChanges::PlaceMatch(changes),
                } => Some((*command_index, command.clone(), changes.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();
        for (source_index, source_command, changes) in sources {
            let trades = match &changes {
                PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { .. } => &[][..],
                PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched { match_changes, .. }
                | PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
                    match_changes, ..
                } => match_changes.created_trades(),
            };
            for trade in trades {
                for pending in working.orders.clone() {
                    if pending.status != SpotOrderStatus::Pending
                        || !pending.trades_asset(trade.asset)
                        || !pending.is_triggered_by_market_price(trade.price)
                        || triggered.contains(&pending.order_id())
                    {
                        continue;
                    }
                    let activate_cmd = ActivateMatchSpotOrderV2Cmd {
                        party_id: pending.account_id().to_string(),
                        asset: trade.asset,
                        order_id: pending.order_id(),
                    };
                    let entry = common_entity::LogEntrySequencer::derive(cmd, activate_cmd);
                    match apply_activate_match(&entry, given_state, working) {
                        Ok(changes) => item_results.push(SpotBlockItemResult::Applied {
                            command_index: source_index,
                            command: SpotBlockCommand::PlaceMatch(source_command.clone()),
                            changes: SpotBlockAppliedChanges::ActivateMatch(changes),
                        }),
                        Err(error) => item_results.push(SpotBlockItemResult::Rejected {
                            command_index: source_index,
                            command: SpotBlockCommand::PlaceMatch(source_command.clone()),
                            error,
                        }),
                    }
                    triggered.insert(pending.order_id());
                }
            }
        }
    }
}

impl StateMachineV2Diff for SpotBlockUseCase {
    type StateDiff = SpotBlockChanges;

    fn do_compute_state_diff(
        _given_state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        //todo 统计本block中 新增/个性的对象
        Ok(after)
    }
}

#[derive(Debug, Clone)]
struct WorkingSpotBlockState {
    orders: Vec<SpotOrderV2>,
    balances: Vec<Balance>,
    order_by_entity_id: HashMap<u64, usize>,
    order_by_cloid: HashMap<String, usize>,
    balance_by_entity_id: HashMap<String, usize>,
}

impl WorkingSpotBlockState {
    fn try_from_state(state: &SpotBlockState) -> Result<Self, SpotBlockError> {
        Self::try_from_parts(state.orders.clone(), state.balances.clone())
    }

    fn try_from_parts(
        orders: Vec<SpotOrderV2>,
        balances: Vec<Balance>,
    ) -> Result<Self, SpotBlockError> {
        let mut working = Self {
            orders,
            balances,
            order_by_entity_id: HashMap::new(),
            order_by_cloid: HashMap::new(),
            balance_by_entity_id: HashMap::new(),
        };
        working.rebuild_indexes()?;
        Ok(working)
    }

    fn rebuild_indexes(&mut self) -> Result<(), SpotBlockError> {
        self.order_by_entity_id.clear();
        self.order_by_cloid.clear();
        self.balance_by_entity_id.clear();

        for (index, order) in self.orders.iter().enumerate() {
            if self.order_by_entity_id.insert(order.entity_id(), index).is_some() {
                return Err(SpotBlockError::DuplicateOrderIdentity);
            }
            if let Some(cloid) = &order.cloid {
                if self.order_by_cloid.insert(cloid.clone(), index).is_some() {
                    return Err(SpotBlockError::DuplicateClientOrderId);
                }
            }
        }

        for (index, balance) in self.balances.iter().enumerate() {
            if self.balance_by_entity_id.insert(balance.business_entity_id(), index).is_some() {
                return Err(SpotBlockError::DuplicateBalanceEntityId);
            }
        }
        Ok(())
    }

    fn order_for_cancel(
        &self,
        lookup: &CancelSpotOrderV2Lookup,
    ) -> Result<SpotOrderV2, SpotBlockItemError> {
        let index = match lookup {
            CancelSpotOrderV2Lookup::Oid(order_id) => self.order_by_entity_id.get(order_id),
            CancelSpotOrderV2Lookup::Cloid(cloid) => self.order_by_cloid.get(cloid),
            CancelSpotOrderV2Lookup::Missing => None,
        };
        index
            .and_then(|index| self.orders.get(*index))
            .cloned()
            .ok_or(SpotBlockItemError::OrderNotFound)
    }

    fn order_for_modify(&self, lookup: &OrderId) -> Result<SpotOrderV2, SpotBlockItemError> {
        let index = match lookup {
            OrderId::Oid(order_id) => self.order_by_entity_id.get(order_id),
            OrderId::Cloid(cloid) => self.order_by_cloid.get(cloid),
        };
        index
            .and_then(|index| self.orders.get(*index))
            .cloned()
            .ok_or(SpotBlockItemError::OrderNotFound)
    }

    fn order_by_order_id(&self, order_id: u64) -> Result<SpotOrderV2, SpotBlockItemError> {
        let mut matches = self.orders.iter().filter(|order| order.order_id() == order_id);
        let order = matches.next().ok_or(SpotBlockItemError::OrderNotFound)?;
        if matches.next().is_some() {
            return Err(SpotBlockItemError::OrderLookupAmbiguous);
        }
        Ok(order.clone())
    }

    fn replace_order(&mut self, order: SpotOrderV2) -> Result<(), SpotBlockError> {
        let order_id = order.entity_id();
        let index =
            *self.order_by_entity_id.get(&order_id).ok_or(SpotBlockError::InvalidStateIndexing)?;
        self.orders[index] = order;
        self.rebuild_indexes()
    }

    fn upsert_order(&mut self, order: SpotOrderV2) -> Result<(), SpotBlockError> {
        if let Some(index) = self.order_by_entity_id.get(&order.entity_id()).copied() {
            self.orders[index] = order;
        } else {
            self.orders.push(order);
        }
        self.rebuild_indexes()
    }

    fn replace_balances(&mut self, balances: &[Balance]) -> Result<(), SpotBlockError> {
        for balance in balances {
            let balance_id = balance.business_entity_id();
            let index = *self
                .balance_by_entity_id
                .get(&balance_id)
                .ok_or(SpotBlockError::InvalidStateIndexing)?;
            self.balances[index] = balance.clone();
        }
        self.rebuild_indexes()
    }
}

fn apply_cancel(
    cmd: &LogEntry<CancelSpotOrderV2Cmd>,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<CancelSpotOrderV2Changes, SpotBlockItemError> {
    CancelSpotOrderV2UseCase.check_command(cmd).map_err(SpotBlockItemError::Cancel)?;
    let order = working.order_for_cancel(&cmd.lookup)?;
    let state = CancelSpotOrderV2State {
        order,
        balances: working.balances.clone(),
        base_asset_id: block_state.base_asset_id.clone(),
        quote_asset_id: block_state.quote_asset_id.clone(),
        maker_fee_bps: block_state.maker_fee_bps,
        taker_fee_bps: block_state.taker_fee_bps,
    };
    CancelSpotOrderV2UseCase
        .validate_state_given(cmd, &state)
        .map_err(SpotBlockItemError::Cancel)?;
    let changes = CancelSpotOrderV2UseCase
        .compute_state_diff(cmd, state)
        .map_err(SpotBlockItemError::Cancel)?;
    working
        .replace_order(changes.updated_order.after.clone())
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    let balances_after =
        changes.updated_balances.iter().map(|pair| pair.after.clone()).collect::<Vec<_>>();
    working
        .replace_balances(&balances_after)
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    Ok(changes)
}

fn apply_activate(
    cmd: &LogEntry<ActivateSpotOrderV2Cmd>,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<ActivateSpotOrderV2Changes, SpotBlockItemError> {
    ActivateSpotOrderV2UseCase.check_command(cmd).map_err(SpotBlockItemError::Activate)?;
    let pending_order = working.order_by_order_id(cmd.order_id)?;
    let state = ActivateSpotOrderV2State {
        pending_order,
        balances: working.balances.clone(),
        base_asset_id: block_state.base_asset_id.clone(),
        quote_asset_id: block_state.quote_asset_id.clone(),
        maker_fee_bps: block_state.maker_fee_bps,
        taker_fee_bps: block_state.taker_fee_bps,
    };
    ActivateSpotOrderV2UseCase
        .validate_state_given(cmd, &state)
        .map_err(SpotBlockItemError::Activate)?;
    let changes = ActivateSpotOrderV2UseCase
        .compute_state_diff(cmd, state)
        .map_err(SpotBlockItemError::Activate)?;
    working
        .replace_order(changes.updated_order.after.clone())
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    let balances_after =
        changes.updated_balances.iter().map(|pair| pair.after.clone()).collect::<Vec<_>>();
    working
        .replace_balances(&balances_after)
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    Ok(changes)
}

fn apply_activate_match(
    cmd: &LogEntry<ActivateMatchSpotOrderV2Cmd>,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<ActivateMatchSpotOrderV2Changes, SpotBlockItemError> {
    ActivateMatchSpotOrderV2UseCase
        .check_command(cmd)
        .map_err(SpotBlockItemError::ActivateMatch)?;
    let pending_order = working.order_by_order_id(cmd.payload().order_id)?;
    let state = ActivateMatchSpotOrderV2State {
        pending_order: pending_order.clone(),
        maker_orders: maker_candidates_for_open_match(&pending_order, &working.orders),
        settlement_balances: working.balances.clone(),
        base_asset_id: block_state.base_asset_id.clone(),
        quote_asset_id: block_state.quote_asset_id.clone(),
        fee_account_id: block_state.fee_account_id.clone(),
        maker_fee_bps: block_state.maker_fee_bps,
        taker_fee_bps: block_state.taker_fee_bps,
    };
    ActivateMatchSpotOrderV2UseCase
        .validate_state_given(cmd, &state)
        .map_err(SpotBlockItemError::ActivateMatch)?;
    let changes = ActivateMatchSpotOrderV2UseCase
        .compute_state_diff(cmd, state)
        .map_err(SpotBlockItemError::ActivateMatch)?;
    working
        .replace_order(changes.activation_changes.updated_order.after.clone())
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    for maker in changes.match_changes.updated_maker_orders() {
        working
            .replace_order(maker.after.clone())
            .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    }
    let balances = changes
        .match_changes
        .updated_balances()
        .iter()
        .map(|pair| pair.after.clone())
        .collect::<Vec<_>>();
    if !balances.is_empty() {
        working
            .replace_balances(&balances)
            .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    } else {
        let balances = changes
            .activation_changes
            .updated_balances
            .iter()
            .map(|pair| pair.after.clone())
            .collect::<Vec<_>>();
        working
            .replace_balances(&balances)
            .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    }
    if let Some(taker) = changes.match_changes.taker_order_after() {
        working
            .replace_order(taker.clone())
            .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    }
    Ok(changes)
}

fn apply_modify(
    cmd: &LogEntry<ModifySpotOrderV2Cmd>,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<ModifySpotOrderV2Changes, SpotBlockItemError> {
    ModifySpotOrderV2UseCase.check_command(cmd).map_err(SpotBlockItemError::Modify)?;
    let order = working.order_for_modify(&cmd.order_id)?;
    let state = ModifySpotOrderV2State {
        order,
        balances: working.balances.clone(),
        base_asset_id: block_state.base_asset_id.clone(),
        quote_asset_id: block_state.quote_asset_id.clone(),
        maker_fee_bps: block_state.maker_fee_bps,
        taker_fee_bps: block_state.taker_fee_bps,
    };
    ModifySpotOrderV2UseCase
        .validate_state_given(cmd, &state)
        .map_err(SpotBlockItemError::Modify)?;
    let changes = ModifySpotOrderV2UseCase
        .compute_state_diff(cmd, state)
        .map_err(SpotBlockItemError::Modify)?;
    working
        .replace_order(changes.updated_order.after.clone())
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    let balances_after =
        changes.updated_balances.iter().map(|pair| pair.after.clone()).collect::<Vec<_>>();
    working
        .replace_balances(&balances_after)
        .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    Ok(changes)
}

fn apply_place_match(
    cmd: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<PlaceMatchSpotOrderV2Changes, SpotBlockItemError> {
    PlaceMatchSpotOrderV2UseCase.check_command(cmd).map_err(SpotBlockItemError::PlaceMatch)?;
    let place_order = place_parent_order_cmd(cmd.payload());
    let state = PlaceMatchSpotOrderV2State {
        maker_orders: maker_candidates_for(place_order, &working.orders),
        settlement_balances: working.balances.clone(),
        fee_account_id: block_state.fee_account_id.clone(),
    };
    PlaceMatchSpotOrderV2UseCase
        .validate_state_given(cmd, &state)
        .map_err(SpotBlockItemError::PlaceMatch)?;
    let changes = PlaceMatchSpotOrderV2UseCase
        .compute_state_diff(cmd, state)
        .map_err(SpotBlockItemError::PlaceMatch)?;
    apply_place_match_changes(&changes, working)?;
    Ok(changes)
}

fn apply_place_match_changes(
    changes: &PlaceMatchSpotOrderV2Changes,
    working: &mut WorkingSpotBlockState,
) -> Result<(), SpotBlockItemError> {
    match changes {
        PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order } => {
            working.upsert_order(created_order.clone())
        }
        PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
            created_taker_order: _,
            activation_changes,
            match_changes,
        } => {
            let taker_after = match_changes
                .taker_order_after()
                .cloned()
                .unwrap_or_else(|| activation_changes.updated_order.after.clone());
            working
                .upsert_order(taker_after)
                .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            for maker in match_changes.updated_maker_orders() {
                working
                    .replace_order(maker.after.clone())
                    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            }
            let balances_after = match_changes
                .updated_balances()
                .iter()
                .map(|pair| pair.after.clone())
                .collect::<Vec<_>>();
            if balances_after.is_empty() {
                let activation_balances_after = activation_changes
                    .updated_balances
                    .iter()
                    .map(|pair| pair.after.clone())
                    .collect::<Vec<_>>();
                working.replace_balances(&activation_balances_after)
            } else {
                working.replace_balances(&balances_after)
            }
        }
        PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order: _,
            created_child_orders,
            activation_changes,
            match_changes,
        } => {
            let parent_after = match_changes
                .taker_order_after()
                .cloned()
                .unwrap_or_else(|| activation_changes.updated_order.after.clone());
            working
                .upsert_order(parent_after)
                .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            for child in created_child_orders {
                working
                    .upsert_order(child.clone())
                    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            }
            for maker in match_changes.updated_maker_orders() {
                working
                    .replace_order(maker.after.clone())
                    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            }
            let balances_after = match_changes
                .updated_balances()
                .iter()
                .map(|pair| pair.after.clone())
                .collect::<Vec<_>>();
            if balances_after.is_empty() {
                let activation_balances_after = activation_changes
                    .updated_balances
                    .iter()
                    .map(|pair| pair.after.clone())
                    .collect::<Vec<_>>();
                working.replace_balances(&activation_balances_after)
            } else {
                working.replace_balances(&balances_after)
            }
        }
    }
    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)
}

fn maker_candidates_for(
    place_order: &PlaceOnlySpotOrderV2OrderCmd,
    orders: &[SpotOrderV2],
) -> Vec<SpotOrderV2> {
    let taker_side = side_from_place_order(place_order);
    orders
        .iter()
        .filter(|order| order.asset == place_order.asset)
        .filter(|order| order.side != taker_side)
        .filter(|order| {
            matches!(order.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
        })
        .filter(|order| matches!(order.order_type, OrderType::Limit { .. }))
        .cloned()
        .collect()
}

fn maker_candidates_for_open_match(
    taker_order: &SpotOrderV2,
    orders: &[SpotOrderV2],
) -> Vec<SpotOrderV2> {
    orders
        .iter()
        .filter(|order| order.entity_id() != taker_order.entity_id())
        .filter(|order| order.order_id() != taker_order.order_id())
        .filter(|order| order.asset == taker_order.asset)
        .filter(|order| order.side != taker_order.side)
        .filter(|order| {
            matches!(order.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
        })
        .filter(|order| matches!(order.order_type, OrderType::Limit { .. }))
        .cloned()
        .collect()
}

fn side_from_place_order(order: &PlaceOnlySpotOrderV2OrderCmd) -> OrderSide {
    if order.is_buy { OrderSide::Buy } else { OrderSide::Sell }
}

fn place_parent_order_cmd(cmd: &PlaceOnlySpotOrderV2Cmd) -> &PlaceOnlySpotOrderV2OrderCmd {
    match cmd {
        PlaceOnlySpotOrderV2Cmd::Single(order) => order,
        PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, .. } => parent,
    }
}

fn shared_party_id(commands: &[SpotBlockCommand]) -> Option<&str> {
    let mut iter = commands.iter().map(command_party_id);
    let first = iter.next()??;
    if first.is_empty() {
        return None;
    }
    iter.all(|party_id| party_id == Some(first)).then_some(first)
}

fn command_party_id(command: &SpotBlockCommand) -> Option<&str> {
    match command {
        SpotBlockCommand::Activate(cmd) => Some(cmd.party_id.as_str()),
        SpotBlockCommand::Cancel(cmd) => Some(cmd.party_id.as_str()),
        SpotBlockCommand::Modify(cmd) => Some(cmd.party_id.as_str()),
        SpotBlockCommand::PlaceMatch(cmd) => match cmd {
            PlaceOnlySpotOrderV2Cmd::Single(order) => Some(order.party_id.as_str()),
            PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, .. } => Some(parent.party_id.as_str()),
        },
    }
}
