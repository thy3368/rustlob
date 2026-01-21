use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{SpotTradeBehaviorV2, SpotTradeCmdAny, SpotTradeRes};
use crate::proc::trade::spot_trade::SpotTradeBehaviorImpl;

pub struct SpotTradeBehaviorV2Impl {
    
}
impl SpotTradeBehaviorV2 for SpotTradeBehaviorImpl {
    fn handle(&mut self, cmd: SpotTradeCmdAny) -> Result<CmdResp<SpotTradeRes>, SpotCmdError> {
        match cmd {
            SpotTradeCmdAny::NewOrder(_) => {}
            SpotTradeCmdAny::TestNewOrder(_) => {}
            SpotTradeCmdAny::CancelOrder(_) => {}
            SpotTradeCmdAny::CancelAllOpenOrders(_) => {}
            SpotTradeCmdAny::CancelReplaceOrder(_) => {}
            SpotTradeCmdAny::QueryOrder(_) => {}
            SpotTradeCmdAny::CurrentOpenOrders(_) => {}
            SpotTradeCmdAny::AllOrders(_) => {}
            SpotTradeCmdAny::NewOcoOrder(_) => {}
            SpotTradeCmdAny::NewOtoOrder(_) => {}
            SpotTradeCmdAny::NewOtocoOrder(_) => {}
            SpotTradeCmdAny::CancelOcoOrder(_) => {}
            SpotTradeCmdAny::QueryOcoOrder(_) => {}
            SpotTradeCmdAny::AllOcoOrders(_) => {}
            SpotTradeCmdAny::OpenOcoOrders(_) => {}
            SpotTradeCmdAny::Account(_) => {}
            SpotTradeCmdAny::MyTrades(_) => {}
            SpotTradeCmdAny::QueryUnfilledOrderCount(_) => {}
            SpotTradeCmdAny::QueryPreventedMatches(_) => {}
            SpotTradeCmdAny::QueryAllocations(_) => {}
            SpotTradeCmdAny::QueryCommissionRates(_) => {}
        }
        todo!()
    }
}