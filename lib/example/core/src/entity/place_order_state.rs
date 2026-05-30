use super::{MarketRules, TradingAccount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderState {
    pub trading_enabled: bool,
    pub next_order_sequence: u64,
    pub account: TradingAccount,
    pub market_rules: MarketRules,
}
