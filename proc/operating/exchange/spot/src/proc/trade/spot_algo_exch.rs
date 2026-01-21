use crate::proc::behavior::spot_trade_behavior::{AlgoOrderProc, IdemAlgoResult, IdemAlgoCmd};
use crate::proc::trade::spot_exch::SpotOrderExchBehaviorImpl;

impl AlgoOrderProc for SpotOrderExchBehaviorImpl {
    fn handle(&mut self, cmd: IdemAlgoCmd) -> IdemAlgoResult { todo!() }
}
