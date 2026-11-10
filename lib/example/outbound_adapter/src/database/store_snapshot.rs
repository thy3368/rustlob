use std::collections::HashMap;

use example_core_use_case::{Balance, SpotOrderV7, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSnapshot {
    pub balances: HashMap<String, Balance>,
    pub orders: HashMap<u64, SpotOrderV7>,
    pub trades: HashMap<String, SpotTrade>,
    pub persisted_event_count: usize,
    pub published_event_count: usize,
    pub broker_message_count: usize,
    pub next_order_sequence: u64,
}
