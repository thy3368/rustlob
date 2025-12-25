use lob::lob::{AlgoOrderProc, IdempotentAlgoCommand, IdempotentAlgoResult};

use crate::proc::spot_exchange::SpotOrderExchangeProcImpl;

impl AlgoOrderProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: IdempotentAlgoCommand) -> IdempotentAlgoResult { todo!() }
}
