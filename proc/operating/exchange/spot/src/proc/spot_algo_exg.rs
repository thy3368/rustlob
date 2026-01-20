use lob::lob::{AlgoOrderProc, IdempotentAlgoCmd, IdempotentAlgoResult};

use crate::proc::spot_exg::SpotOrderExgProcImpl;

impl AlgoOrderProc for SpotOrderExgProcImpl {
    fn handle(&mut self, cmd: IdempotentAlgoCmd) -> IdempotentAlgoResult { todo!() }
}
