use lob::lob::domain::service::trading_spot_order_proc::{AlgoOrderProc, IdemAlgoResult, IdemAlgoCmd};
use crate::proc::spot_exch::SpotOrderExgProcImpl;

impl AlgoOrderProc for SpotOrderExgProcImpl {
    fn handle(&mut self, cmd: IdemAlgoCmd) -> IdemAlgoResult { todo!() }
}
