use std::collections::HashMap;

use example_core::{Balance, Reservation, SpotOrderV2, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSnapshot {
    pub balances: HashMap<String, Balance>,
    pub orders: HashMap<String, SpotOrderV2>,
    pub trades: HashMap<String, SpotTrade>,
    pub reservations: HashMap<String, Reservation>,
    pub persisted_event_count: usize,
    pub published_event_count: usize,
    pub broker_message_count: usize,
    pub next_order_sequence: u64,
}
