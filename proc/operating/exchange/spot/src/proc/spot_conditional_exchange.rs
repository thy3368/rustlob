use lob::lob::{ConditionalOrderProc, IdempotentConditionalCommand, IdempotentConditionalResult};
use crate::proc::spot_exchange::SpotOrderExchangeProcImpl;

impl ConditionalOrderProc for SpotOrderExchangeProcImpl {
    fn handle(&mut self, cmd: IdempotentConditionalCommand) -> IdempotentConditionalResult {
        todo!()
    }
}