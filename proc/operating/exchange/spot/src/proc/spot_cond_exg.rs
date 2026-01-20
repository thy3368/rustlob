use lob::lob::domain::service::trading_spot_order_proc::{ConditionalOrderProc, IdemConditionalResult, IdempotentConditionalCmd};
use crate::proc::spot_exg::SpotOrderExgProcImpl;

impl ConditionalOrderProc for SpotOrderExgProcImpl {
    fn handle(&mut self, cmd: IdempotentConditionalCmd) -> IdemConditionalResult {
        todo!()
    }
}