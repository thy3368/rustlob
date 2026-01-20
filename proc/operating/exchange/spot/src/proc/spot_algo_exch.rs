use crate::proc::behavior::trading_spot_order_proc::{AlgoOrderProc, IdemAlgoResult, IdemAlgoCmd};
use crate::proc::spot_exch::SpotOrderExchProcImpl;

impl AlgoOrderProc for SpotOrderExchProcImpl {
    fn handle(&mut self, cmd: IdemAlgoCmd) -> IdemAlgoResult { todo!() }
}
