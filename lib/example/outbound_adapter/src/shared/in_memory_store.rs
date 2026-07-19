use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use cmd_handler::EntityReplayableEvent;
use example_core::{Balance, MarketRules, Reservation, SpotOrderV2, SpotTrade};

use super::StoreSnapshot;
use crate::shared::StoreError;

#[derive(Debug, Clone)]
pub(crate) struct StoreState {
    pub(crate) balances: HashMap<String, Balance>,
    pub(crate) market_rules_by_symbol: HashMap<String, MarketRules>,
    pub(crate) orders: HashMap<String, SpotOrderV2>,
    pub(crate) trades: HashMap<String, SpotTrade>,
    pub(crate) reservations: HashMap<String, Reservation>,
    pub(crate) persisted_events: Vec<EntityReplayableEvent>,
    pub(crate) published_events: Vec<EntityReplayableEvent>,
    pub(crate) next_order_sequence: u64,
}

impl Default for StoreState {
    fn default() -> Self {
        Self {
            balances: HashMap::new(),
            market_rules_by_symbol: HashMap::new(),
            orders: HashMap::new(),
            trades: HashMap::new(),
            reservations: HashMap::new(),
            persisted_events: Vec::new(),
            published_events: Vec::new(),
            next_order_sequence: 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryStore {
    state: Arc<Mutex<StoreState>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seed_balances(
        base_balance: Balance,
        quote_balance: Balance,
        market_rules: MarketRules,
    ) -> Result<Self, StoreError> {
        let store = Self::new();
        store.seed_balance(base_balance)?;
        store.seed_balance(quote_balance)?;
        store.seed_market_rules(market_rules)?;
        Ok(store)
    }

    pub fn seed_balance(&self, balance: Balance) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.balances.insert(balance_key(&balance.account_id, &balance.asset_id), balance);
        Ok(())
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.market_rules_by_symbol.insert(market_rules.symbol.clone(), market_rules);
        Ok(())
    }

    pub fn seed_order(&self, order: SpotOrderV2) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.orders.insert(order.order_id.clone(), order);
        Ok(())
    }

    pub fn seed_reservation(&self, reservation: Reservation) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.reservations.insert(reservation.reservation_id.clone(), reservation);
        Ok(())
    }

    pub fn snapshot_with_broker_depth(
        &self,
        broker_message_count: usize,
    ) -> Result<StoreSnapshot, StoreError> {
        let state = self.lock_state()?;
        Ok(StoreSnapshot {
            balances: state.balances.clone(),
            orders: state.orders.clone(),
            trades: state.trades.clone(),
            reservations: state.reservations.clone(),
            persisted_event_count: state.persisted_events.len(),
            published_event_count: state.published_events.len(),
            broker_message_count,
            next_order_sequence: state.next_order_sequence,
        })
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        self.snapshot_with_broker_depth(0)
    }

    pub(crate) fn lock_state(&self) -> Result<MutexGuard<'_, StoreState>, StoreError> {
        self.state.lock().map_err(|_| StoreError::StoreUnavailable)
    }
}

pub(crate) fn balance_key(account_id: &str, asset_id: &str) -> String {
    format!("{account_id}:{asset_id}")
}
