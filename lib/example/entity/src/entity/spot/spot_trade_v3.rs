use crate::BalanceV2;
use crate::entity::common::common_trade::TradeBase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeV3 {
    pub comon_trade: TradeBase,
}

impl SpotTradeV3 {
    pub fn settle(&self, p0: BalanceV2, p1: BalanceV2) {
        todo!()
    }
}
