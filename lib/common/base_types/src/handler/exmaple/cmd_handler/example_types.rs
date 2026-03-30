//! CommandHandler 示例共享类型。

pub struct AccountBalance {
    pub user_id: String,
    pub asset: String,
    pub free: i64,
    pub locked: i64,
}

pub struct OrderBookSnapshot {
    pub symbol: String,
    pub best_bid: Option<i64>,
    pub best_ask: Option<i64>,
    pub last_price: Option<i64>,
}

pub struct Order {
    pub order_id: String,
    pub status: OrderStatus,
}

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Open,
    Partial,
    Filled,
    Cancelled,
    Rejected,
}

pub struct BalanceChange {
    pub user_id: String,
    pub asset: String,
    pub change: i64,
}

pub struct Trade {
    pub trade_id: String,
    pub taker_order_id: String,
    pub maker_order_id: String,
    pub price: i64,
    pub quantity: i64,
}

#[derive(Debug)]
pub struct HandlerError(pub String);

impl Default for AccountBalance {
    fn default() -> Self {
        Self { user_id: String::new(), asset: String::new(), free: 0, locked: 0 }
    }
}

impl Default for OrderBookSnapshot {
    fn default() -> Self {
        Self { symbol: String::new(), best_bid: None, best_ask: None, last_price: None }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self { order_id: String::new(), status: OrderStatus::Pending }
    }
}
