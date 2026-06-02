use std::collections::HashMap;

use example_core::{StoredOrder, TradingAccount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSnapshot {
    pub accounts: HashMap<String, TradingAccount>,
    pub orders: HashMap<String, StoredOrder>,
    pub persisted_event_count: usize,
    pub published_event_count: usize,
    pub next_order_sequence: u64,
}
