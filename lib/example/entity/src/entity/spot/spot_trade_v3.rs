use crate::spot::common_trade::TradeBase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeV2 {
    pub comon_trade: TradeBase,
}
