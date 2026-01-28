use crate::proc::behavior::spot_trade_behavior::{ConditionalTradeProc, IdemCondResult, IdemCondCmd};
use crate::proc::v1::spot_trade::SpotTradeBehaviorImpl;

impl ConditionalTradeProc for SpotTradeBehaviorImpl {
    fn handle(&mut self, cmd: IdemCondCmd) -> IdemCondResult {
        todo!()
    }
}