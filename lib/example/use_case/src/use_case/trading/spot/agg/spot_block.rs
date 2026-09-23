use std::collections::HashMap;

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, ExecutionContext, IssuedByParty, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, SpotOrderSide, SpotOrderStatus, SpotOrderType, SpotOrderV2};
use crate::{
    CancelSpotOrderV2Changes, CancelSpotOrderV2Cmd, CancelSpotOrderV2Error,
    CancelSpotOrderV2Lookup, CancelSpotOrderV2State, CancelSpotOrderV2UseCase,
    MatchSpotOrderV2Changes, MatchSpotOrderV2Cmd, MatchSpotOrderV2Error, MatchSpotOrderV2State,
    ModifySpotOrderV2Changes, ModifySpotOrderV2Cmd, ModifySpotOrderV2Error, ModifySpotOrderV2State,
    ModifySpotOrderV2UseCase, OpenMatchSpotOrderV2UseCase, OrderId, PlaceMatchSpotOrderV2Changes,
    PlaceMatchSpotOrderV2Error, PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBlockCmd {
    pub commands: Vec<SpotBlockCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBlockCommand {
    Cancel(CancelSpotOrderV2Cmd),
    Modify(ModifySpotOrderV2Cmd),
    PlaceMatch(PlaceOnlySpotOrderV2Cmd),
    Match(MatchSpotOrderV2Cmd),
}

impl IssuedByParty for SpotBlockCmd {
    fn party_id(&self) -> Option<&str> {
        shared_party_id(&self.commands)
    }
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
    Cancel(CancelSpotOrderV2Changes),
    Modify(ModifySpotOrderV2Changes),
    PlaceMatch(PlaceMatchSpotOrderV2Changes),
    Match(MatchSpotOrderV2Changes),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockError {
    #[error("spot block must contain at least one command")]
    EmptyCommands,
    #[error("spot block command party id must not be empty")]
    EmptyPartyId,
    #[error("spot block commands must share one party id")]
    MixedPartyIds,
    #[error("spot block state has invalid order or balance indexes")]
    InvalidStateIndexing,
    #[error("spot block state contains duplicate order entity identity")]
    DuplicateOrderIdentity,
    #[error("spot block state contains duplicate exchange oid")]
    DuplicateExchangeOid,
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
    Cancel(#[from] CancelSpotOrderV2Error),
    #[error(transparent)]
    Modify(#[from] ModifySpotOrderV2Error),
    #[error(transparent)]
    PlaceMatch(#[from] PlaceMatchSpotOrderV2Error),
    #[error(transparent)]
    Match(#[from] MatchSpotOrderV2Error),
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
                SpotBlockAppliedChanges::Cancel(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::Modify(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::PlaceMatch(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
                SpotBlockAppliedChanges::Match(changes) => {
                    events.extend(changes.to_replayable_events()?);
                }
            }
        }
        Ok(events)
    }
}

impl StateMachineV2Unchecked for SpotBlockUseCase {
    type Command = SpotBlockCmd;
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

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
        _context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut working = WorkingSpotBlockState::try_from_state(given_state)?;
        let mut item_results = Vec::with_capacity(cmd.commands.len());

        for (command_index, command) in cmd.commands.iter().cloned().enumerate() {
            let item_result = match &command {
                SpotBlockCommand::Cancel(cancel_cmd) => {
                    match apply_cancel(cancel_cmd, given_state, &mut working) {
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
                    match apply_modify(modify_cmd, given_state, &mut working) {
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
                    match apply_place_match(place_cmd, given_state, &mut working) {
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
                SpotBlockCommand::Match(match_cmd) => {
                    match apply_match(match_cmd, given_state, &mut working) {
                        Ok(changes) => SpotBlockItemResult::Applied {
                            command_index,
                            command,
                            changes: SpotBlockAppliedChanges::Match(changes),
                        },
                        Err(error) => {
                            SpotBlockItemResult::Rejected { command_index, command, error }
                        }
                    }
                }
            };
            item_results.push(item_result);
        }

        Ok(SpotBlockChanges { item_results })
    }
}

impl StateMachineOwnedV2Diff for SpotBlockUseCase {
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
    order_by_entity_id: HashMap<String, usize>,
    order_by_oid: HashMap<u64, usize>,
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
            order_by_oid: HashMap::new(),
            order_by_cloid: HashMap::new(),
            balance_by_entity_id: HashMap::new(),
        };
        working.rebuild_indexes()?;
        Ok(working)
    }

    fn rebuild_indexes(&mut self) -> Result<(), SpotBlockError> {
        self.order_by_entity_id.clear();
        self.order_by_oid.clear();
        self.order_by_cloid.clear();
        self.balance_by_entity_id.clear();

        for (index, order) in self.orders.iter().enumerate() {
            if self.order_by_entity_id.insert(order.entity_id(), index).is_some() {
                return Err(SpotBlockError::DuplicateOrderIdentity);
            }
            if let Some(exchange_oid) = order.exchange_oid {
                if self.order_by_oid.insert(exchange_oid, index).is_some() {
                    return Err(SpotBlockError::DuplicateExchangeOid);
                }
            }
            if let Some(cloid) = &order.client_order_id {
                if self.order_by_cloid.insert(cloid.clone(), index).is_some() {
                    return Err(SpotBlockError::DuplicateClientOrderId);
                }
            }
        }

        for (index, balance) in self.balances.iter().enumerate() {
            if self.balance_by_entity_id.insert(balance.entity_id(), index).is_some() {
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
            CancelSpotOrderV2Lookup::Oid(oid) => self.order_by_oid.get(oid),
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
            OrderId::Oid(oid) => self.order_by_oid.get(oid),
            OrderId::Cloid(cloid) => self.order_by_cloid.get(cloid),
        };
        index
            .and_then(|index| self.orders.get(*index))
            .cloned()
            .ok_or(SpotBlockItemError::OrderNotFound)
    }

    fn order_by_order_id(&self, order_id: &str) -> Result<SpotOrderV2, SpotBlockItemError> {
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
            let balance_id = balance.entity_id();
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
    cmd: &CancelSpotOrderV2Cmd,
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

fn apply_modify(
    cmd: &ModifySpotOrderV2Cmd,
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
    cmd: &PlaceOnlySpotOrderV2Cmd,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<PlaceMatchSpotOrderV2Changes, SpotBlockItemError> {
    PlaceMatchSpotOrderV2UseCase.check_command(cmd).map_err(SpotBlockItemError::PlaceMatch)?;
    let place_order = place_parent_order_cmd(cmd);
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

fn apply_match(
    cmd: &MatchSpotOrderV2Cmd,
    block_state: &SpotBlockState,
    working: &mut WorkingSpotBlockState,
) -> Result<MatchSpotOrderV2Changes, SpotBlockItemError> {
    OpenMatchSpotOrderV2UseCase.check_command(cmd).map_err(SpotBlockItemError::Match)?;
    let taker_order = working.order_by_order_id(&cmd.order_id)?;
    let state = MatchSpotOrderV2State {
        maker_orders: maker_candidates_for_open_match(&taker_order, &working.orders),
        taker_order,
        settlement_balances: working.balances.clone(),
        base_asset_id: block_state.base_asset_id.clone(),
        quote_asset_id: block_state.quote_asset_id.clone(),
        fee_account_id: block_state.fee_account_id.clone(),
        maker_fee_bps: block_state.maker_fee_bps,
        taker_fee_bps: block_state.taker_fee_bps,
    };
    OpenMatchSpotOrderV2UseCase
        .validate_state_given(cmd, &state)
        .map_err(SpotBlockItemError::Match)?;
    let changes = OpenMatchSpotOrderV2UseCase
        .compute_state_diff(cmd, state)
        .map_err(SpotBlockItemError::Match)?;
    apply_match_changes(&changes, working)?;
    Ok(changes)
}

fn apply_match_changes(
    changes: &MatchSpotOrderV2Changes,
    working: &mut WorkingSpotBlockState,
) -> Result<(), SpotBlockItemError> {
    if let Some(taker) = &changes.updated_taker_order {
        working
            .replace_order(taker.after.clone())
            .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    }
    for maker in &changes.updated_maker_orders {
        working
            .replace_order(maker.after.clone())
            .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
    }
    let balances_after =
        changes.updated_balances.iter().map(|pair| pair.after.clone()).collect::<Vec<_>>();
    working.replace_balances(&balances_after).map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)
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
            created_taker_order,
            match_changes,
        } => {
            let taker_after = match_changes
                .updated_taker_order
                .as_ref()
                .map(|pair| pair.after.clone())
                .unwrap_or_else(|| created_taker_order.clone());
            working
                .upsert_order(taker_after)
                .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            for maker in &match_changes.updated_maker_orders {
                working
                    .replace_order(maker.after.clone())
                    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            }
            let balances_after = match_changes
                .updated_balances
                .iter()
                .map(|pair| pair.after.clone())
                .collect::<Vec<_>>();
            working.replace_balances(&balances_after)
        }
        PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order,
            created_child_orders,
            match_changes,
        } => {
            let parent_after = match_changes
                .updated_taker_order
                .as_ref()
                .map(|pair| pair.after.clone())
                .unwrap_or_else(|| created_parent_order.clone());
            working
                .upsert_order(parent_after)
                .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            for child in created_child_orders {
                working
                    .upsert_order(child.clone())
                    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            }
            for maker in &match_changes.updated_maker_orders {
                working
                    .replace_order(maker.after.clone())
                    .map_err(|_| SpotBlockItemError::OrderLookupAmbiguous)?;
            }
            let balances_after = match_changes
                .updated_balances
                .iter()
                .map(|pair| pair.after.clone())
                .collect::<Vec<_>>();
            working.replace_balances(&balances_after)
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
        .filter(|order| matches!(order.order_type, SpotOrderType::Limit { .. }))
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
        .filter(|order| matches!(order.order_type, SpotOrderType::Limit { .. }))
        .cloned()
        .collect()
}

fn side_from_place_order(order: &PlaceOnlySpotOrderV2OrderCmd) -> SpotOrderSide {
    if order.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell }
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
        SpotBlockCommand::Cancel(cmd) => Some(cmd.party_id.as_str()),
        SpotBlockCommand::Modify(cmd) => Some(cmd.party_id.as_str()),
        SpotBlockCommand::PlaceMatch(cmd) => cmd.party_id(),
        SpotBlockCommand::Match(cmd) => Some(cmd.party_id.as_str()),
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{ReplayableChanges, StateMachineOwnedV2Diff, StateMachineV2Unchecked};

    use super::*;
    use crate::entity::{BalanceLedgerOperation, Reservation, SpotOrderStatusReason, SpotOrderTif};
    use crate::{
        PlaceOnlySpotOrderV2OrderType, PlaceSpotOrderV2Input, SpotOrderStatus, SpotOrderType,
    };

    fn block_state(order: SpotOrderV2, balances: Vec<Balance>) -> SpotBlockState {
        SpotBlockState {
            orders: vec![order],
            balances,
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            fee_account_id: "fee".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        }
    }

    fn empty_state() -> SpotBlockState {
        SpotBlockState {
            orders: vec![],
            balances: vec![],
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            fee_account_id: "fee".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        }
    }

    fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
        Balance::new(account_id.to_owned(), asset_id.to_owned(), available, frozen, 1)
    }

    fn buy_order(price: u64, qty: u64) -> Result<SpotOrderV2, Box<dyn std::error::Error>> {
        Ok(SpotOrderV2::new_active(
            "order-1".to_owned(),
            10_001,
            Some(777),
            "buyer".to_owned(),
            "BTCUSDT".to_owned(),
            SpotOrderSide::Buy,
            price,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            qty,
            "BTC",
            "USDT",
            5,
            10,
            Some("original-cloid".to_owned()),
        )?)
    }

    fn open_taker_buy_order(
        order_id: &str,
        price: u64,
        qty: u64,
    ) -> Result<SpotOrderV2, Box<dyn std::error::Error>> {
        Ok(SpotOrderV2::place(PlaceSpotOrderV2Input {
            order_id: order_id.to_owned(),
            asset: 10_001,
            account_id: "buyer".to_owned(),
            symbol: "BTCUSDT".to_owned(),
            side: SpotOrderSide::Buy,
            limit_price: price,
            order_type: SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            qty,
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            base_balance_entity_id: "buyer:BTC".to_owned(),
            quote_balance_entity_id: "buyer:USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
            client_order_id: None,
        })?
        .order)
    }

    fn test_principal_reservation(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        qty: u64,
        order_price: u64,
    ) -> Result<Reservation, Box<dyn std::error::Error>> {
        Ok(SpotOrderV2::principal_reservation(
            order_id,
            account_id,
            side,
            qty,
            order_price,
            "BTC",
            "USDT",
        )?)
    }

    fn sell_order(
        order_id: &str,
        account_id: &str,
        price: u64,
        qty: u64,
    ) -> Result<SpotOrderV2, Box<dyn std::error::Error>> {
        Ok(SpotOrderV2::new(
            order_id.to_owned(),
            10_001,
            Some(900),
            account_id.to_owned(),
            "BTCUSDT".to_owned(),
            SpotOrderSide::Sell,
            price,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            qty,
            0,
            SpotOrderStatus::Open,
            None,
            test_principal_reservation(order_id, account_id, SpotOrderSide::Sell, qty, price)?,
            None,
            1,
        ))
    }

    fn cancel_cmd() -> SpotBlockCommand {
        SpotBlockCommand::Cancel(CancelSpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            lookup: CancelSpotOrderV2Lookup::Oid(777),
        })
    }

    fn modify_cmd() -> SpotBlockCommand {
        SpotBlockCommand::Modify(ModifySpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            order_id: OrderId::Oid(777),
            is_buy: true,
            price: "12000".to_owned(),
            size: "3".to_owned(),
            order_type: crate::ModifySpotOrderV2OrderType::Limit { tif: "gtc".to_owned() },
            cloid: None,
        })
    }

    fn place_trigger_cmd(order_id: &str, cloid: Option<&str>) -> SpotBlockCommand {
        SpotBlockCommand::PlaceMatch(PlaceOnlySpotOrderV2Cmd::Single(
            PlaceOnlySpotOrderV2OrderCmd {
                party_id: "buyer".to_owned(),
                asset: 10_001,
                order_id: order_id.to_owned(),
                symbol: "BTCUSDT".to_owned(),
                is_buy: true,
                price: "100".to_owned(),
                size: "1".to_owned(),
                order_type: PlaceOnlySpotOrderV2OrderType::Trigger {
                    is_market: false,
                    trigger_price: "90".to_owned(),
                    trigger_role: "sl".to_owned(),
                },
                reduce_only: false,
                cloid: cloid.map(str::to_owned),
                base_asset_id: "BTC".to_owned(),
                quote_asset_id: "USDT".to_owned(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            },
        ))
    }

    fn match_cmd(order_id: &str) -> SpotBlockCommand {
        SpotBlockCommand::Match(MatchSpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            order_id: order_id.to_owned(),
        })
    }

    #[test]
    fn check_command_rejects_empty_block() {
        let cmd = SpotBlockCmd { commands: vec![] };

        assert_eq!(SpotBlockUseCase.check_command(&cmd), Err(SpotBlockError::EmptyCommands));
    }

    #[test]
    fn check_command_rejects_mixed_parties() {
        let mut modify = modify_cmd();
        let SpotBlockCommand::Modify(inner) = &mut modify else {
            unreachable!();
        };
        inner.party_id = "seller".to_owned();
        let cmd = SpotBlockCmd { commands: vec![cancel_cmd(), modify] };

        assert_eq!(SpotBlockUseCase.check_command(&cmd), Err(SpotBlockError::MixedPartyIds));
    }

    #[test]
    fn check_command_rejects_mixed_parties_with_match() {
        let mut match_command = match_cmd("order-1");
        let SpotBlockCommand::Match(inner) = &mut match_command else {
            unreachable!();
        };
        inner.party_id = "seller".to_owned();
        let cmd = SpotBlockCmd { commands: vec![cancel_cmd(), match_command] };

        assert_eq!(SpotBlockUseCase.check_command(&cmd), Err(SpotBlockError::MixedPartyIds));
    }

    #[test]
    fn modify_after_cancel_sees_cancelled_order_and_is_rejected()
    -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 2)?;
        let frozen = order.reservation.remaining_amount + order.fee_reservation.remaining_amount;
        let state = block_state(order, vec![balance("buyer", "USDT", 100_000, frozen)]);
        let cmd = SpotBlockCmd { commands: vec![cancel_cmd(), modify_cmd()] };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        assert_eq!(changes.item_results.len(), 2);
        assert!(matches!(
            &changes.item_results[0],
            SpotBlockItemResult::Applied { changes: SpotBlockAppliedChanges::Cancel(_), .. }
        ));
        assert!(matches!(
            &changes.item_results[1],
            SpotBlockItemResult::Rejected {
                error: SpotBlockItemError::Modify(ModifySpotOrderV2Error::OrderNotModifiable),
                ..
            }
        ));
        let cancel_events = match &changes.item_results[0] {
            SpotBlockItemResult::Applied {
                changes: SpotBlockAppliedChanges::Cancel(changes),
                ..
            } => changes.to_replayable_events()?,
            _ => unreachable!(),
        };
        assert_eq!(changes.to_replayable_events()?.len(), cancel_events.len());
        Ok(())
    }

    #[test]
    fn item_failure_does_not_stop_later_command() -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 2)?;
        let frozen = order.reservation.remaining_amount + order.fee_reservation.remaining_amount;
        let state = block_state(order, vec![balance("buyer", "USDT", 100_000, frozen)]);
        let missing_cancel = SpotBlockCommand::Cancel(CancelSpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            lookup: CancelSpotOrderV2Lookup::Oid(404),
        });
        let cmd = SpotBlockCmd { commands: vec![missing_cancel, cancel_cmd()] };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        assert!(matches!(
            &changes.item_results[0],
            SpotBlockItemResult::Rejected { error: SpotBlockItemError::OrderNotFound, .. }
        ));
        assert!(matches!(
            &changes.item_results[1],
            SpotBlockItemResult::Applied { changes: SpotBlockAppliedChanges::Cancel(_), .. }
        ));
        Ok(())
    }

    #[test]
    fn match_missing_taker_is_rejected_and_later_command_runs()
    -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 2)?;
        let frozen = order.reservation.remaining_amount + order.fee_reservation.remaining_amount;
        let state = block_state(order, vec![balance("buyer", "USDT", 100_000, frozen)]);
        let cmd = SpotBlockCmd { commands: vec![match_cmd("missing-order"), cancel_cmd()] };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        assert!(matches!(
            &changes.item_results[0],
            SpotBlockItemResult::Rejected { error: SpotBlockItemError::OrderNotFound, .. }
        ));
        assert!(matches!(
            &changes.item_results[1],
            SpotBlockItemResult::Applied { changes: SpotBlockAppliedChanges::Cancel(_), .. }
        ));
        Ok(())
    }

    #[test]
    fn match_success_updates_working_state_for_later_command()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut taker = open_taker_buy_order("taker-buy", 100, 2)?;
        taker.exchange_oid = Some(100);
        let maker = sell_order("maker-1", "seller", 100, 1)?;
        let mut state = empty_state();
        state.orders = vec![taker, maker];
        state.balances = vec![
            balance("buyer", "USDT", 1_201, 0),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let cmd = SpotBlockCmd {
            commands: vec![
                match_cmd("taker-buy"),
                SpotBlockCommand::Cancel(CancelSpotOrderV2Cmd {
                    party_id: "buyer".to_owned(),
                    asset: 10_001,
                    lookup: CancelSpotOrderV2Lookup::Oid(100),
                }),
            ],
        };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        let SpotBlockItemResult::Applied {
            changes: SpotBlockAppliedChanges::Match(matched), ..
        } = &changes.item_results[0]
        else {
            panic!("match should apply: {:?}", changes.item_results[0]);
        };
        let Some(taker_after_match) = matched.updated_taker_order.as_ref() else {
            panic!("match should update taker");
        };
        assert_eq!(taker_after_match.after.filled_qty(), 1);
        let SpotBlockItemResult::Applied {
            changes: SpotBlockAppliedChanges::Cancel(cancel), ..
        } = &changes.item_results[1]
        else {
            panic!("cancel should apply after partial match: {:?}", changes.item_results[1]);
        };
        assert_eq!(cancel.updated_order.before.filled_qty(), 1);
        assert_eq!(cancel.updated_order.after.status, SpotOrderStatus::Canceled);
        Ok(())
    }

    #[test]
    fn place_match_success_updates_working_book_for_later_cancel_or_modify()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut state = empty_state();
        state.balances = vec![balance("buyer", "USDT", 1_000, 0)];
        let cmd = SpotBlockCmd {
            commands: vec![
                place_trigger_cmd("new-order", Some("new-cloid")),
                SpotBlockCommand::Cancel(CancelSpotOrderV2Cmd {
                    party_id: "buyer".to_owned(),
                    asset: 10_001,
                    lookup: CancelSpotOrderV2Lookup::Cloid("new-cloid".to_owned()),
                }),
            ],
        };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        assert!(matches!(
            &changes.item_results[0],
            SpotBlockItemResult::Applied {
                changes: SpotBlockAppliedChanges::PlaceMatch(
                    PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { .. }
                ),
                ..
            }
        ));
        assert!(matches!(&changes.item_results[1], SpotBlockItemResult::Rejected { .. }));
        let place_events = match &changes.item_results[0] {
            SpotBlockItemResult::Applied {
                changes: SpotBlockAppliedChanges::PlaceMatch(changes),
                ..
            } => changes.to_replayable_events()?,
            _ => unreachable!(),
        };
        let block_events = changes.to_replayable_events()?;
        assert_eq!(block_events.len(), place_events.len());
        assert!(block_events.iter().all(EntityReplayableEvent::is_created));
        Ok(())
    }

    #[test]
    fn to_replayable_events_includes_match_events_in_block_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let taker = open_taker_buy_order("taker-buy", 100, 2)?;
        let maker = sell_order("maker-1", "seller", 100, 1)?;
        let mut state = empty_state();
        state.orders = vec![taker, maker];
        state.balances = vec![
            balance("buyer", "USDT", 1_201, 0),
            balance("buyer", "BTC", 0, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 1),
            balance("fee", "USDT", 0, 0),
        ];
        let cmd = SpotBlockCmd {
            commands: vec![
                place_trigger_cmd("new-order", None),
                match_cmd("missing-order"),
                match_cmd("taker-buy"),
            ],
        };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;
        let events = changes.to_replayable_events()?;
        let place_events = match &changes.item_results[0] {
            SpotBlockItemResult::Applied {
                changes: SpotBlockAppliedChanges::PlaceMatch(changes),
                ..
            } => changes.to_replayable_events()?,
            _ => unreachable!(),
        };
        let match_events = match &changes.item_results[2] {
            SpotBlockItemResult::Applied {
                changes: SpotBlockAppliedChanges::Match(changes),
                ..
            } => changes.to_replayable_events()?,
            other => panic!("match should apply: {other:?}"),
        };

        assert!(matches!(&changes.item_results[1], SpotBlockItemResult::Rejected { .. }));
        assert_eq!(events.len(), place_events.len() + match_events.len());
        assert!(events[0].is_created());
        assert_eq!(events[place_events.len()].is_updated(), match_events[0].is_updated());
        Ok(())
    }

    #[test]
    fn to_replayable_events_skips_rejected_items_and_preserves_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 2)?;
        let frozen = order.reservation.remaining_amount + order.fee_reservation.remaining_amount;
        let state = block_state(order, vec![balance("buyer", "USDT", 100_000, frozen)]);
        let cmd = SpotBlockCmd {
            commands: vec![
                place_trigger_cmd("new-order", None),
                SpotBlockCommand::Cancel(CancelSpotOrderV2Cmd {
                    party_id: "buyer".to_owned(),
                    asset: 10_001,
                    lookup: CancelSpotOrderV2Lookup::Oid(404),
                }),
                cancel_cmd(),
            ],
        };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;
        let events = changes.to_replayable_events()?;

        assert_eq!(changes.item_results.len(), 3);
        assert!(events[0].is_created());
        assert!(events[1].is_updated());
        assert!(events.iter().skip(1).any(|event| {
            event.is_created()
                && matches!(
                    &changes.item_results[2],
                    SpotBlockItemResult::Applied {
                        changes: SpotBlockAppliedChanges::Cancel(_),
                        ..
                    }
                )
        }));
        assert!(events.iter().any(EntityReplayableEvent::is_updated));
        Ok(())
    }

    #[test]
    fn place_match_success_updates_working_order_count() -> Result<(), Box<dyn std::error::Error>> {
        let mut state = empty_state();
        state.orders.push(sell_order("maker-1", "seller", 100, 1)?);
        state.balances = vec![
            balance("buyer", "USDT", 1_000, 0),
            balance("seller", "BTC", 0, 1),
            balance("seller", "USDT", 0, 0),
            balance("buyer", "BTC", 0, 0),
            balance("fee", "USDT", 0, 0),
        ];
        let cmd = SpotBlockCmd { commands: vec![place_trigger_cmd("resting-trigger", None)] };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        assert!(matches!(
            &changes.item_results[0],
            SpotBlockItemResult::Applied {
                changes: SpotBlockAppliedChanges::PlaceMatch(
                    PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { .. }
                ),
                ..
            }
        ));
        Ok(())
    }

    #[test]
    fn cancel_changes_order_to_canceled_in_block() -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 2)?;
        let frozen = order.reservation.remaining_amount + order.fee_reservation.remaining_amount;
        let state = block_state(order, vec![balance("buyer", "USDT", 100_000, frozen)]);
        let cmd = SpotBlockCmd { commands: vec![cancel_cmd()] };

        let changes = SpotBlockUseCase.compute_state_diff(&cmd, state)?;

        let SpotBlockItemResult::Applied {
            changes: SpotBlockAppliedChanges::Cancel(cancel), ..
        } = &changes.item_results[0]
        else {
            panic!("cancel should apply");
        };
        assert_eq!(cancel.updated_order.after.status, SpotOrderStatus::Canceled);
        assert_eq!(
            cancel.updated_order.after.status_reason,
            Some(SpotOrderStatusReason::CanceledByUser)
        );
        assert_eq!(
            cancel.created_balance_ledger_entries.first().map(|entry| entry.operation),
            Some(BalanceLedgerOperation::Unfreeze)
        );
        Ok(())
    }
}
