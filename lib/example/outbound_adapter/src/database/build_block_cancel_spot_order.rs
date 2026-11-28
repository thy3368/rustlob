use base_types_v2::Symbol;
use common_entity::{Entity, LogEntry, ReplayableDomainEvent, StateSinkV3, StateSourceV3};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{OrderSide, SpotOrderV7};
use example_core_use_case::CancelSpotOrderFsmV3Changes;
use example_core_use_case::pipe_line::cancel_spot_order::{
    CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3State, CancelSpotOrderFsmV3UseCase,
};
use thiserror::Error;

use super::State4BuildBlock;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderBuildBlockSourceError {
    #[error("cancel target order was not found")]
    OrderNotFound,
    #[error("cancel target matches more than one order")]
    AmbiguousOrder,
    #[error("balance for the asset held by the order was not found")]
    BalanceNotFound,
    #[error("order book for the order symbol was not found")]
    LobNotFound,
}

impl StateSourceV3<CancelSpotOrderFsmV3UseCase> for State4BuildBlock {
    type Error = CancelSpotOrderBuildBlockSourceError;

    fn load_given_state(
        &self,
        command: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    ) -> Result<CancelSpotOrderFsmV3State, Self::Error> {
        let mut matches = self
            .user_symbol_orders
            .iter()
            .filter(|((party_id, _), _)| party_id == command.party_id())
            .flat_map(|(_, orders)| orders)
            .filter(|order| matches_order(order, &command.payload().order_id));
        let order = matches
            .next()
            .ok_or(CancelSpotOrderBuildBlockSourceError::OrderNotFound)?;
        if matches.next().is_some() {
            return Err(CancelSpotOrderBuildBlockSourceError::AmbiguousOrder);
        }
        let symbol = order.order_base().symbol;
        if !self.spot_symbol_lob.contains_key(&symbol) {
            return Err(CancelSpotOrderBuildBlockSourceError::LobNotFound);
        }
        let key = (command.party_id().to_owned(), symbol);
        let balances = match order.order_base().side {
            OrderSide::Buy => &self.user_symbol_quote_balance,
            OrderSide::Sell => &self.user_symbol_base_balance,
        };
        let balance = balances
            .get(&key)
            .ok_or(CancelSpotOrderBuildBlockSourceError::BalanceNotFound)?;
        Ok(CancelSpotOrderFsmV3State {
            order: order.clone(),
            balance: balance.clone(),
        })
    }
}

fn matches_order(order: &SpotOrderV7, order_id: &OrderId) -> bool {
    match order_id {
        OrderId::Oid(oid) => order.order_base().order_id == *oid,
        OrderId::Cloid(cloid) => order.order_base().cloid.as_deref() == Some(cloid.as_str()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderBuildBlockSinkError {
    #[error("cancel target order was not found while replaying changes")]
    OrderNotFound,
    #[error("cancel target matches more than one order while replaying changes")]
    AmbiguousOrder,
    #[error("loaded order position is stale")]
    StaleOrderProjection,
    #[error("order book is missing the loaded order or contains a stale projection")]
    StaleLobProjection,
    #[error("order book for the changed order was not found")]
    LobNotFound,
    #[error("balance required by changes was not found")]
    BalanceNotFound,
    #[error("loaded balance position is stale")]
    StaleBalanceProjection,
    #[error("cancel changes expect one order event and at most one balance event")]
    UnexpectedChangeShape,
    #[error("failed to update the order book: {0}")]
    Lob(String),
}

impl StateSinkV3<CancelSpotOrderFsmV3UseCase> for State4BuildBlock {
    type Error = CancelSpotOrderBuildBlockSinkError;

    fn persist(&mut self, _events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&mut self, changes: &CancelSpotOrderFsmV3Changes) -> Result<(), Self::Error> {
        let order_events = changes.order.events();
        let balance_events = changes.balance.as_ref().map(|change| change.events());
        if order_events.len() != 1 || balance_events.is_some_and(|events| events.len() != 1) {
            return Err(CancelSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
        }
        let order_event = &order_events[0];
        if order_event.entity_type != <SpotOrderV7 as Entity>::entity_type()
            || order_event.domain_event_type != 7
            || order_event.command_index != 0
            || balance_events.is_some_and(|events| events[0].command_index != 1)
        {
            return Err(CancelSpotOrderBuildBlockSinkError::UnexpectedChangeShape);
        }

        let mut found = self
            .user_symbol_orders
            .iter()
            .flat_map(|(key, orders)| {
                orders
                    .iter()
                    .enumerate()
                    .map(move |(index, order)| (key.clone(), index, order))
            })
            .filter(|(_, _, order)| Entity::entity_id(*order) == order_event.entity_id);
        let (key, index, order) = found
            .next()
            .ok_or(CancelSpotOrderBuildBlockSinkError::OrderNotFound)?;
        if found.next().is_some() {
            return Err(CancelSpotOrderBuildBlockSinkError::AmbiguousOrder);
        }
        if (Entity::lsn(order), Entity::command_index(order))
            != (order_event.previous_lsn, order_event.previous_command_index)
        {
            return Err(CancelSpotOrderBuildBlockSinkError::StaleOrderProjection);
        }

        self.apply_changes(key, index, changes)
    }

    fn publish(&mut self, _events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl State4BuildBlock {
    fn apply_changes(
        &mut self,
        key: (example_core_entity::PartyId, Symbol),
        index: usize,
        changes: &CancelSpotOrderFsmV3Changes,
    ) -> Result<(), CancelSpotOrderBuildBlockSinkError> {
        let orders = self
            .user_symbol_orders
            .get(&key)
            .ok_or(CancelSpotOrderBuildBlockSinkError::OrderNotFound)?;
        let current = orders
            .get(index)
            .ok_or(CancelSpotOrderBuildBlockSinkError::OrderNotFound)?;
        let order_event = &changes.order.events()[0];
        if (Entity::lsn(current), Entity::command_index(current))
            != (order_event.previous_lsn, order_event.previous_command_index)
        {
            return Err(CancelSpotOrderBuildBlockSinkError::StaleOrderProjection);
        }

        let after = changes.order.after();
        let symbol = after.order_base().symbol;
        let mut next_orders = orders.clone();
        next_orders[index] = after.clone();
        let mut next_lob = self
            .spot_symbol_lob
            .get(&symbol)
            .cloned()
            .ok_or(CancelSpotOrderBuildBlockSinkError::LobNotFound)?;
        if next_lob.order(current.order_base().order_id) != Some(current) {
            return Err(CancelSpotOrderBuildBlockSinkError::StaleLobProjection);
        }
        next_lob
            .insert_order(after.clone())
            .map_err(|error| CancelSpotOrderBuildBlockSinkError::Lob(error.to_string()))?;

        let next_balance = if let Some(balance_changes) = &changes.balance {
            let balances = match after.order_base().side {
                OrderSide::Buy => &self.user_symbol_quote_balance,
                OrderSide::Sell => &self.user_symbol_base_balance,
            };
            let current_balance = balances
                .get(&key)
                .ok_or(CancelSpotOrderBuildBlockSinkError::BalanceNotFound)?;
            let event = &balance_changes.events()[0];
            if current_balance.position() != (event.previous_lsn, event.previous_command_index) {
                return Err(CancelSpotOrderBuildBlockSinkError::StaleBalanceProjection);
            }
            Some(balance_changes.after().clone())
        } else {
            None
        };

        self.user_symbol_orders.insert(key.clone(), next_orders);
        self.spot_symbol_lob.insert(symbol, next_lob);
        if let Some(balance) = next_balance {
            match after.order_base().side {
                OrderSide::Buy => self.user_symbol_quote_balance.insert(key, balance),
                OrderSide::Sell => self.user_symbol_base_balance.insert(key, balance),
            };
        }
        Ok(())
    }
}
