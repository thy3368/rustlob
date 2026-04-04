#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionKind {
    Call,
    Put,
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
pub struct SpotPlaceOrderCmd {
    pub trader_id: u64,
    pub market: String,
    pub side: SpotSide,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotCancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotAmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpPlaceOrderCmd {
    pub trader_id: u64,
    pub market: String,
    pub side: PerpSide,
    pub price: u64,
    pub quantity: u64,
    pub leverage: u32,
    pub reduce_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpCancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpAmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionPlaceOrderCmd {
    pub trader_id: u64,
    pub underlying: String,
    pub expiry_ts: u64,
    pub strike_price: u64,
    pub kind: OptionKind,
    pub side: OptionSide,
    pub premium: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionCancelOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionAmendOrderCmd {
    pub trader_id: u64,
    pub order_id: u64,
    pub new_price: Option<u64>,
    pub new_quantity: Option<u64>,
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
pub enum SpotCommand {
    PlaceOrder(SpotPlaceOrderCmd),
    CancelOrder(SpotCancelOrderCmd),
    AmendOrder(SpotAmendOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpCommand {
    PlaceOrder(PerpPlaceOrderCmd),
    CancelOrder(PerpCancelOrderCmd),
    AmendOrder(PerpAmendOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionCommand {
    PlaceOrder(OptionPlaceOrderCmd),
    CancelOrder(OptionCancelOrderCmd),
    AmendOrder(OptionAmendOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreasuryCommand {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeCommand {
    TradingCommand(TradingCommand),
    TreasuryCommand(TreasuryCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradingCommand {
    Spot(SpotCommand),
    Perp(PerpCommand),
    Option(OptionCommand),
    PlaceOrder(PlaceOrderCmd),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeCommandEnvelope {
    pub command_id: u64,
    pub trader_id: u64,
    pub nonce: u64,
    pub timestamp_ns: u64,
    pub command: ExchangeCommand,
}
