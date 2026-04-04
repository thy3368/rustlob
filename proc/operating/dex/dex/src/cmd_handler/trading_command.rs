#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCmd {
    pub trader_id: u64,
    pub market: String,
    pub side: OrderSide,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradingCommand {
    PlaceOrder(PlaceOrderCmd),
    CancelOrder(CancelOrderCmd),
    AmendOrder(AmendOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradingCommandEnvelope {
    pub command_id: u64,
    pub trader_id: u64,
    pub nonce: u64,
    pub timestamp_ns: u64,
    pub command: TradingCommand,
}
