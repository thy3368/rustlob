use lob::lob::{ConditionalOrderProc, IdempotentConditionalCmd, IdempotentConditionalResult};
use crate::proc::spot_exchange::SpotOrderExchangeProcImpl;

impl ConditionalOrderProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: IdempotentConditionalCmd) -> IdempotentConditionalResult {
        todo!()
    }
}