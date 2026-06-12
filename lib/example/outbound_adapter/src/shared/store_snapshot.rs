use std::collections::HashMap;

use example_core::{Balance, SpotOrder, SpotSettlement, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSnapshot {
    pub balances: HashMap<String, Balance>,
    pub orders: HashMap<String, SpotOrder>,
    pub trades: HashMap<String, SpotTrade>,
    pub settlements: HashMap<String, SpotSettlement>,
    pub persisted_event_count: usize,
    pub published_event_count: usize,
    pub broker_message_count: usize,
    pub next_order_sequence: u64,
}
