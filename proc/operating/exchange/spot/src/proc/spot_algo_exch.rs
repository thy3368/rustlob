use crate::proc::behavior::trading_spot_order_behavior::{AlgoOrderProc, IdemAlgoResult, IdemAlgoCmd};
use crate::proc::spot_exch::SpotOrderExchBehaviorImpl;

impl AlgoOrderProc for SpotOrderExchBehaviorImpl {
    fn handle(&mut self, cmd: IdemAlgoCmd) -> IdemAlgoResult { todo!() }
}
