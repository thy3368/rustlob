#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredOrder {
    pub order_id: String,
    pub account_id: String,
    pub symbol: String,
    pub qty: u64,
    pub price: u64,
    pub reserved_quote: u64,
}
