use crate::spot::common_trade::CommonTrade;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeV2 {
    pub comon_trade: CommonTrade,
}
