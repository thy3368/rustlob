use crate::proc::behavior::trading_spot_order_behavior::{ConditionalOrderProc, IdemCondResult, IdemCondCmd};
use crate::proc::spot_exch::SpotOrderExchBehaviorImpl;

impl ConditionalOrderProc for SpotOrderExchBehaviorImpl {
    fn handle(&mut self, cmd: IdemCondCmd) -> IdemCondResult {
        todo!()
    }
}