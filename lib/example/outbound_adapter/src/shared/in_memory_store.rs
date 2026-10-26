use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use common_entity::EntityReplayableEvent;
use example_core_use_case::{Balance, MarketRules, Reservation, SpotOrderV2, SpotTrade};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;

use super::StoreSnapshot;
use crate::shared::StoreError;

#[derive(Debug, Clone)]
pub(crate) struct StoreState {
    pub(crate) balances: HashMap<String, Balance>,
    pub(crate) market_rules_by_symbol: HashMap<String, MarketRules>,
    pub(crate) orders: HashMap<u64, SpotOrderV2>,
    pub(crate) trades: HashMap<String, SpotTrade>,
    pub(crate) reservations: HashMap<String, Reservation>,
    pub(crate) persisted_events: Vec<EntityReplayableEvent>,
    pub(crate) published_events: Vec<EntityReplayableEvent>,
    pub(crate) next_order_sequence: u64,
    pub(crate) lob: LocalLobBtree<SpotOrderV2>,
}

impl StoreState {
    fn new(asset: u32) -> Self {
        Self {
            balances: HashMap::new(),
            market_rules_by_symbol: HashMap::new(),
            orders: HashMap::new(),
            trades: HashMap::new(),
            reservations: HashMap::new(),
            persisted_events: Vec::new(),
            published_events: Vec::new(),
            next_order_sequence: 1,
            lob: LocalLobBtree::new(asset),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryStore {
    state: Arc<Mutex<StoreState>>,
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new(1)
    }
}

impl InMemoryStore {
    pub fn new(asset: u32) -> Self {
        Self { state: Arc::new(Mutex::new(StoreState::new(asset))) }
    }

    pub fn seed_balances(
        base_balance: Balance,
        quote_balance: Balance,
        market_rules: MarketRules,
    ) -> Result<Self, StoreError> {
        let store = Self::new(1);
        store.seed_balance(base_balance)?;
        store.seed_balance(quote_balance)?;
        store.seed_market_rules(market_rules)?;
        Ok(store)
    }

    pub fn seed_balance(&self, balance: Balance) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.balances.insert(balance_key(balance.account_id, balance.asset_id), balance);
        Ok(())
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.market_rules_by_symbol.insert(market_rules.symbol.clone(), market_rules);
        Ok(())
    }

    pub fn seed_order(&self, order: SpotOrderV2) -> Result<(), StoreError> {
        let order_id = order.order_id;
        let mut state = self.lock_state()?;
        state.lob.insert_order(order.clone()).map_err(|_| StoreError::StoreUnavailable)?;
        state.orders.insert(order_id, order);
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

pub(crate) fn balance_key(
    account_id: impl BalanceKeyPart,
    asset_id: impl BalanceKeyPart,
) -> String {
    format!("{}:{}", account_id.as_id(), asset_id.as_id())
}

pub(crate) trait BalanceKeyPart {
    fn as_id(&self) -> u64;
}

impl BalanceKeyPart for u64 {
    fn as_id(&self) -> u64 {
        *self
    }
}
impl BalanceKeyPart for &str {
    fn as_id(&self) -> u64 {
        common_entity::stable_string_entity_id(self)
    }
}
impl BalanceKeyPart for String {
    fn as_id(&self) -> u64 {
        common_entity::stable_string_entity_id(self)
    }
}
impl BalanceKeyPart for &String {
    fn as_id(&self) -> u64 {
        common_entity::stable_string_entity_id(self)
    }
}
