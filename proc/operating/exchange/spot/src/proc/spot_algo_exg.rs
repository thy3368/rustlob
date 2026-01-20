use lob::lob::domain::service::trading_spot_order_proc::{AlgoOrderProc, IdemAlgoResult, IdempotentAlgoCmd};
use crate::proc::spot_exg::SpotOrderExgProcImpl;

impl AlgoOrderProc for SpotOrderExgProcImpl {
    fn handle(&mut self, cmd: IdempotentAlgoCmd) -> IdemAlgoResult { todo!() }
}
