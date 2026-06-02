use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use cmd_handler::EntityReplayableEvent;
use example_core::{MarketRules, StoredOrder, TradingAccount};

use super::StoreSnapshot;
use crate::shared::StoreError;

#[derive(Debug, Clone)]
pub(crate) struct StoreState {
    pub(crate) accounts: HashMap<String, TradingAccount>,
    pub(crate) market_rules_by_symbol: HashMap<String, MarketRules>,
    pub(crate) orders: HashMap<String, StoredOrder>,
    pub(crate) persisted_events: Vec<EntityReplayableEvent>,
    pub(crate) published_events: Vec<EntityReplayableEvent>,
    pub(crate) next_order_sequence: u64,
}

impl Default for StoreState {
    fn default() -> Self {
        Self {
            accounts: HashMap::new(),
            market_rules_by_symbol: HashMap::new(),
            orders: HashMap::new(),
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

    pub fn seeded(account: TradingAccount, market_rules: MarketRules) -> Result<Self, StoreError> {
        let store = Self::new();
        store.seed_account(account)?;
        store.seed_market_rules(market_rules)?;
        Ok(store)
    }

    pub fn seed_account(&self, account: TradingAccount) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.accounts.insert(account.account_id.clone(), account);
        Ok(())
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), StoreError> {
        let mut state = self.lock_state()?;
        state.market_rules_by_symbol.insert(market_rules.symbol.clone(), market_rules);
        Ok(())
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        let state = self.lock_state()?;
        Ok(StoreSnapshot {
            accounts: state.accounts.clone(),
            orders: state.orders.clone(),
            persisted_event_count: state.persisted_events.len(),
            published_event_count: state.published_events.len(),
            next_order_sequence: state.next_order_sequence,
        })
    }

    pub(crate) fn lock_state(&self) -> Result<MutexGuard<'_, StoreState>, StoreError> {
        self.state.lock().map_err(|_| StoreError::StoreUnavailable)
    }
}
