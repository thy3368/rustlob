use lob::lob::{AlgoOrderProc, IdempotentAlgoCmd, IdempotentAlgoResult};

use crate::proc::spot_exchange::SpotOrderExchangeProcImpl;

impl AlgoOrderProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: IdempotentAlgoCmd) -> IdempotentAlgoResult { todo!() }
}
