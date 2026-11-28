use std::collections::{BTreeMap, HashSet};

use common_entity::{
    Entity, LogEntry, ReplayableDomainEvent, StateSinkV3, StateSourceV3, UpdatedEntityEvents,
};
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::{BalanceV2, PartyId, PlaceSpotOrderPipelineV2Cmd, SpotOrderV7};
use example_core_use_case::pipe::line::place_spot_order_pipeline_v2::PlaceSpotOrderPipelineV2Changes;
use example_core_use_case::pipe::match_spot_order::MatchSpotOrderPipeChanges;
use example_core_use_case::pipe::settle_matched_spot_trades::SettleTradesPipeChanges;
use example_core_use_case::pipe_line::place_spot_order_pipeline_v2::{
    PlaceSpotOrderPipelineV2State, PlaceSpotOrderPipelineV2UseCase,
};
use lob_repo_v2::lob::AssetLobV2;
use thiserror::Error;

use super::State4BuildBlock;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceSpotOrderBuildBlockSourceError {
    #[error("order id already exists")]
    DuplicateOrderId,
    #[error("client order id already exists for the user")]
    DuplicateClientOrderId,
    #[error("order book for the requested symbol was not found")]
    LobNotFound,
    #[error("order book and user order projection disagree")]
    StaleLobProjection,
    #[error("maker order was not found in the user projection")]
    OrderNotFound,
    #[error("maker order matches more than one user projection")]
    AmbiguousOrder,
    #[error("required balance was not found")]
    BalanceNotFound,
    #[error("required balance matches more than one projection")]
    AmbiguousBalance,
}

impl StateSourceV3<PlaceSpotOrderPipelineV2UseCase> for State4BuildBlock {
    type Error = PlaceSpotOrderBuildBlockSourceError;

    fn load_given_state(
        &self,
        command: &LogEntry<PlaceSpotOrderPipelineV2Cmd>,
    ) -> Result<PlaceSpotOrderPipelineV2State, Self::Error> {
        let payload = command.payload();
        if self
            .user_symbol_orders
            .values()
            .flatten()
            .any(|order| order.order_base().order_id == payload.order_id)
        {
            return Err(PlaceSpotOrderBuildBlockSourceError::DuplicateOrderId);
        }
        if payload.cloid.as_ref().is_some_and(|cloid| {
            self.user_symbol_orders
                .iter()
                .filter(|((party_id, _), _)| party_id.as_ref() == command.party_id())
                .flat_map(|(_, orders)| orders)
                .any(|order| order.order_base().cloid.as_ref() == Some(cloid))
        }) {
            return Err(PlaceSpotOrderBuildBlockSourceError::DuplicateClientOrderId);
        }

        let lob = self
            .spot_symbol_lob
            .get(&payload.symbol)
            .ok_or(PlaceSpotOrderBuildBlockSourceError::LobNotFound)?;
        let (makers, _) = lob.maker_orders(payload.price, payload.size, payload.side);
        let maker_orders: Vec<_> = makers.unwrap_or_default().into_iter().cloned().collect();
        for maker in &maker_orders {
            let projected = unique_order(self, maker.order_base().order_id)?;
            if projected != maker {
                return Err(PlaceSpotOrderBuildBlockSourceError::StaleLobProjection);
            }
        }

        let mut base_balances = BTreeMap::new();
        let mut quote_balances = BTreeMap::new();
        insert_balance_pair(
            self,
            &payload.party_id,
            payload.symbol,
            &mut base_balances,
            &mut quote_balances,
        )?;
        for maker in &maker_orders {
            let (key, _) = unique_order_with_key(self, maker.order_base().order_id)?;
            insert_balance_pair(self, &key.0, key.1, &mut base_balances, &mut quote_balances)?;
        }

        Ok(PlaceSpotOrderPipelineV2State {
            maker_orders,
            base_balances,
            quote_balances,
            trade_id_start: self.next_trade_id,
            maker_fee_bps: self.maker_fee_bps,
            taker_fee_bps: self.taker_fee_bps,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceSpotOrderBuildBlockSinkError {
    #[error("place-order changes have an unexpected branch or shape")]
    UnexpectedChangeShape,
    #[error("order projection was not found")]
    OrderNotFound,
    #[error("order projection is ambiguous")]
    AmbiguousOrder,
    #[error("order predecessor is stale")]
    StaleOrderProjection,
    #[error("balance projection was not found")]
    BalanceNotFound,
    #[error("balance projection is ambiguous")]
    AmbiguousBalance,
    #[error("balance predecessor is stale")]
    StaleBalanceProjection,
    #[error("order book was not found")]
    LobNotFound,
    #[error("order book contains a stale projection")]
    StaleLobProjection,
    #[error("failed to update the order book: {0}")]
    Lob(String),
    #[error("trade id already exists")]
    TradeIdConflict,
    #[error("trade id sequence overflow")]
    TradeIdOverflow,
}

impl StateSinkV3<PlaceSpotOrderPipelineV2UseCase> for State4BuildBlock {
    type Error = PlaceSpotOrderBuildBlockSinkError;

    fn persist(&mut self, _events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&mut self, changes: &PlaceSpotOrderPipelineV2Changes) -> Result<(), Self::Error> {
        let mut next = self.clone();
        apply_pipeline_changes(&mut next, changes)?;
        *self = next;
        Ok(())
    }

    fn publish(&mut self, _events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn apply_pipeline_changes(
    state: &mut State4BuildBlock,
    changes: &PlaceSpotOrderPipelineV2Changes,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    let (created, frozen, matched, settled, unfrozen) = match changes {
        PlaceSpotOrderPipelineV2Changes::Resting {
            created_order,
            frozen,
        } => (created_order, frozen, None, None, None),
        PlaceSpotOrderPipelineV2Changes::Rejected {
            created_order,
            frozen,
            matched,
            unfrozen,
        } => (created_order, frozen, Some(matched), None, Some(unfrozen)),
        PlaceSpotOrderPipelineV2Changes::PartiallyFilled {
            created_order,
            frozen,
            matched,
            settled,
            unfrozen,
        }
        | PlaceSpotOrderPipelineV2Changes::Filled {
            created_order,
            frozen,
            matched,
            settled,
            unfrozen,
        }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill {
            created_order,
            frozen,
            matched,
            settled,
            unfrozen,
        } => (
            created_order,
            frozen,
            Some(matched),
            Some(settled),
            Some(unfrozen),
        ),
    };

    let taker = created.after();
    if created.order.event().previous_lsn != 0
        || unique_order(state, taker.order_base().order_id).is_ok()
    {
        return Err(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
    }
    let taker_key = find_key_for_party(state, taker)?;
    state
        .user_symbol_orders
        .entry(taker_key.clone())
        .or_default()
        .push(taker.clone());
    sync_lob_order(state, None, taker)?;

    apply_order_update(state, &frozen.order)?;
    apply_balance_update(state, &frozen.balance)?;

    if let Some(matched) = matched {
        validate_branch_shape(changes, matched, settled)?;
        apply_match_changes(state, matched)?;
    }
    if let Some(settled) = settled {
        apply_settlement_changes(state, settled)?;
    }
    if let Some(unfrozen) = unfrozen {
        for balance in &unfrozen.updated_balances {
            apply_balance_update(state, balance)?;
        }
    }

    let trade_count = matched.map_or(0, match_trade_count);
    let increment = u64::try_from(trade_count)
        .map_err(|_| PlaceSpotOrderBuildBlockSinkError::TradeIdOverflow)?;
    state.next_trade_id = state
        .next_trade_id
        .checked_add(increment)
        .ok_or(PlaceSpotOrderBuildBlockSinkError::TradeIdOverflow)?;
    Ok(())
}

fn apply_match_changes(
    state: &mut State4BuildBlock,
    changes: &MatchSpotOrderPipeChanges,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    match changes {
        MatchSpotOrderPipeChanges::Resting => Ok(()),
        MatchSpotOrderPipeChanges::Rejected { taker } => apply_order_update(state, taker),
        MatchSpotOrderPipeChanges::PartiallyFilled {
            taker,
            makers,
            trades,
        }
        | MatchSpotOrderPipeChanges::Filled {
            taker,
            makers,
            trades,
        }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill {
            taker,
            makers,
            trades,
        } => {
            if makers.len() != trades.len() || trades.is_empty() {
                return Err(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
            }
            apply_order_update(state, taker)?;
            for maker in makers {
                apply_order_update(state, maker)?;
            }
            let mut ids = HashSet::with_capacity(trades.len());
            for (offset, trade) in trades.iter().enumerate() {
                let after = trade.after();
                let offset = u64::try_from(offset)
                    .map_err(|_| PlaceSpotOrderBuildBlockSinkError::TradeIdOverflow)?;
                let expected_id = state
                    .next_trade_id
                    .checked_add(offset)
                    .ok_or(PlaceSpotOrderBuildBlockSinkError::TradeIdOverflow)?;
                if trade.event().previous_lsn != 0
                    || after.comon_trade.trade_id != expected_id
                    || !ids.insert(after.comon_trade.trade_id)
                    || trade_exists(state, after.comon_trade.trade_id)
                {
                    return Err(PlaceSpotOrderBuildBlockSinkError::TradeIdConflict);
                }
            }
            Ok(())
        }
    }
}

fn validate_branch_shape(
    outer: &PlaceSpotOrderPipelineV2Changes,
    matched: &MatchSpotOrderPipeChanges,
    settled: Option<&SettleTradesPipeChanges>,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    let valid = matches!(
        (outer, matched, settled),
        (
            PlaceSpotOrderPipelineV2Changes::Rejected { .. },
            MatchSpotOrderPipeChanges::Rejected { .. },
            None
        ) | (
            PlaceSpotOrderPipelineV2Changes::PartiallyFilled { .. },
            MatchSpotOrderPipeChanges::PartiallyFilled { .. },
            Some(_)
        ) | (
            PlaceSpotOrderPipelineV2Changes::Filled { .. },
            MatchSpotOrderPipeChanges::Filled { .. },
            Some(_)
        ) | (
            PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { .. },
            MatchSpotOrderPipeChanges::CanceledAfterPartialFill { .. },
            Some(_)
        )
    );
    if valid {
        Ok(())
    } else {
        Err(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape)
    }
}

fn apply_settlement_changes(
    state: &mut State4BuildBlock,
    changes: &SettleTradesPipeChanges,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    for balance in &changes.updated_balances {
        apply_balance_update(state, balance)?;
    }
    for trade in &changes.settled_trades {
        let before_id = trade.after().comon_trade.trade_id;
        let events = trade.events();
        if events.len() != 1 || events[0].entity_id != before_id {
            return Err(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
        }
        append_trade_to_parties(state, trade.after())?;
    }
    Ok(())
}

fn apply_order_update(
    state: &mut State4BuildBlock,
    changes: &UpdatedEntityEvents<SpotOrderV7>,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    let events = changes.events();
    let first = events
        .first()
        .ok_or(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape)?;
    let id = changes.after().order_base().order_id;
    if events.iter().any(|event| event.entity_id != id) {
        return Err(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
    }
    let (key, index, current) = unique_order_position(state, id)?;
    if (current.lsn(), current.command_index())
        != (first.previous_lsn, first.previous_command_index)
    {
        return Err(PlaceSpotOrderBuildBlockSinkError::StaleOrderProjection);
    }
    let before = current.clone();
    state
        .user_symbol_orders
        .get_mut(&key)
        .ok_or(PlaceSpotOrderBuildBlockSinkError::OrderNotFound)?[index] = changes.after().clone();
    sync_lob_order(state, Some(&before), changes.after())
}

fn apply_balance_update(
    state: &mut State4BuildBlock,
    changes: &UpdatedEntityEvents<BalanceV2>,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    let first = changes
        .events()
        .first()
        .ok_or(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape)?;
    let id = changes.after().entity_id();
    if changes.events().iter().any(|event| event.entity_id != id) {
        return Err(PlaceSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
    }
    let (is_base, key, current) = unique_balance_position(state, id)?;
    if current.position() != (first.previous_lsn, first.previous_command_index) {
        return Err(PlaceSpotOrderBuildBlockSinkError::StaleBalanceProjection);
    }
    let balances = if is_base {
        &mut state.user_symbol_base_balance
    } else {
        &mut state.user_symbol_quote_balance
    };
    balances.insert(key, changes.after().clone());
    Ok(())
}

fn sync_lob_order(
    state: &mut State4BuildBlock,
    before: Option<&SpotOrderV7>,
    after: &SpotOrderV7,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    let lob = state
        .spot_symbol_lob
        .get_mut(&after.order_base().symbol)
        .ok_or(PlaceSpotOrderBuildBlockSinkError::LobNotFound)?;
    if let Some(before) = before {
        if lob.order(before.order_base().order_id) != Some(before) {
            return Err(PlaceSpotOrderBuildBlockSinkError::StaleLobProjection);
        }
    }
    if before.is_none() && lob.order(after.order_base().order_id).is_some() {
        return Err(PlaceSpotOrderBuildBlockSinkError::StaleLobProjection);
    }
    lob.insert_order(after.clone())
        .map_err(|error| PlaceSpotOrderBuildBlockSinkError::Lob(error.to_string()))
}

fn unique_order(
    state: &State4BuildBlock,
    order_id: u64,
) -> Result<&SpotOrderV7, PlaceSpotOrderBuildBlockSourceError> {
    unique_order_with_key(state, order_id).map(|(_, order)| order)
}

fn unique_order_with_key(
    state: &State4BuildBlock,
    order_id: u64,
) -> Result<(&(PartyId, base_types_v2::Symbol), &SpotOrderV7), PlaceSpotOrderBuildBlockSourceError>
{
    let mut found = state
        .user_symbol_orders
        .iter()
        .flat_map(|(key, orders)| orders.iter().map(move |order| (key, order)))
        .filter(|(_, order)| order.order_base().order_id == order_id);
    let value = found
        .next()
        .ok_or(PlaceSpotOrderBuildBlockSourceError::OrderNotFound)?;
    if found.next().is_some() {
        return Err(PlaceSpotOrderBuildBlockSourceError::AmbiguousOrder);
    }
    Ok(value)
}

fn insert_balance_pair(
    state: &State4BuildBlock,
    party_id: &PartyId,
    symbol: base_types_v2::Symbol,
    base_balances: &mut BTreeMap<(PartyId, base_types_v2::Symbol), BalanceV2>,
    quote_balances: &mut BTreeMap<(PartyId, base_types_v2::Symbol), BalanceV2>,
) -> Result<(), PlaceSpotOrderBuildBlockSourceError> {
    let key = (party_id.clone(), symbol);
    base_balances.insert(
        key.clone(),
        state
            .user_symbol_base_balance
            .get(&key)
            .cloned()
            .ok_or(PlaceSpotOrderBuildBlockSourceError::BalanceNotFound)?,
    );
    quote_balances.insert(
        key.clone(),
        state
            .user_symbol_quote_balance
            .get(&key)
            .cloned()
            .ok_or(PlaceSpotOrderBuildBlockSourceError::BalanceNotFound)?,
    );
    Ok(())
}

fn unique_order_position(
    state: &State4BuildBlock,
    order_id: u64,
) -> Result<
    ((PartyId, base_types_v2::Symbol), usize, &SpotOrderV7),
    PlaceSpotOrderBuildBlockSinkError,
> {
    let mut found = state
        .user_symbol_orders
        .iter()
        .flat_map(|(key, orders)| {
            orders
                .iter()
                .enumerate()
                .map(move |(index, order)| (key.clone(), index, order))
        })
        .filter(|(_, _, order)| order.order_base().order_id == order_id);
    let value = found
        .next()
        .ok_or(PlaceSpotOrderBuildBlockSinkError::OrderNotFound)?;
    if found.next().is_some() {
        return Err(PlaceSpotOrderBuildBlockSinkError::AmbiguousOrder);
    }
    Ok(value)
}

fn unique_balance_position(
    state: &State4BuildBlock,
    balance_id: u64,
) -> Result<(bool, (PartyId, base_types_v2::Symbol), &BalanceV2), PlaceSpotOrderBuildBlockSinkError>
{
    let mut found = state
        .user_symbol_base_balance
        .iter()
        .map(|(key, balance)| (true, key.clone(), balance))
        .chain(
            state
                .user_symbol_quote_balance
                .iter()
                .map(|(key, balance)| (false, key.clone(), balance)),
        )
        .filter(|(_, _, balance)| balance.entity_id() == balance_id);
    let value = found
        .next()
        .ok_or(PlaceSpotOrderBuildBlockSinkError::BalanceNotFound)?;
    if found.next().is_some() {
        return Err(PlaceSpotOrderBuildBlockSinkError::AmbiguousBalance);
    }
    Ok(value)
}

fn find_key_for_party(
    state: &State4BuildBlock,
    order: &SpotOrderV7,
) -> Result<(PartyId, base_types_v2::Symbol), PlaceSpotOrderBuildBlockSinkError> {
    let symbol = order.order_base().symbol;
    let party = &order.order_base().owner;
    let mut keys = state
        .user_symbol_base_balance
        .iter()
        .filter(|((_, candidate), balance)| *candidate == symbol && balance.party_id() == party)
        .map(|(key, _)| key.clone());
    let key = keys
        .next()
        .ok_or(PlaceSpotOrderBuildBlockSinkError::BalanceNotFound)?;
    if keys.next().is_some() {
        return Err(PlaceSpotOrderBuildBlockSinkError::AmbiguousBalance);
    }
    Ok(key)
}

fn append_trade_to_parties(
    state: &mut State4BuildBlock,
    trade: &SpotTradeV3,
) -> Result<(), PlaceSpotOrderBuildBlockSinkError> {
    let taker_key = unique_order_position(state, trade.comon_trade.taker_order_id)?.0;
    let maker_key = unique_order_position(state, trade.comon_trade.maker_order_id)?.0;
    state
        .user_symbol_trades
        .entry(taker_key.clone())
        .or_default()
        .push(trade.clone());
    if maker_key != taker_key {
        state
            .user_symbol_trades
            .entry(maker_key)
            .or_default()
            .push(trade.clone());
    }
    Ok(())
}

fn trade_exists(state: &State4BuildBlock, trade_id: u64) -> bool {
    state
        .user_symbol_trades
        .values()
        .flatten()
        .any(|trade| trade.comon_trade.trade_id == trade_id)
}

fn match_trade_count(changes: &MatchSpotOrderPipeChanges) -> usize {
    match changes {
        MatchSpotOrderPipeChanges::PartiallyFilled { trades, .. }
        | MatchSpotOrderPipeChanges::Filled { trades, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { trades, .. } => trades.len(),
        MatchSpotOrderPipeChanges::Resting | MatchSpotOrderPipeChanges::Rejected { .. } => 0,
    }
}
