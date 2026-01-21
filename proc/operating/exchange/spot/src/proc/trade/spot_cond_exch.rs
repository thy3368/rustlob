use crate::proc::behavior::spot_trade_behavior::{ConditionalOrderProc, IdemCondResult, IdemCondCmd};
use crate::proc::trade::spot_exch::SpotOrderExchBehaviorImpl;

impl ConditionalOrderProc for SpotOrderExchBehaviorImpl {
    fn handle(&mut self, cmd: IdemCondCmd) -> IdemCondResult {
        todo!()
    }
}