use lob::lob::{ConditionalOrderProc, IdempotentConditionalCmd, IdempotentConditionalResult};
use crate::proc::spot_exg::SpotOrderExgProcImpl;

impl ConditionalOrderProc for SpotOrderExgProcImpl {
    fn handle(&mut self, cmd: IdempotentConditionalCmd) -> IdempotentConditionalResult {
        todo!()
    }
}