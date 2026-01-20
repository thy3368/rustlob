use lob::lob::domain::service::trading_spot_order_proc::{ConditionalOrderProc, IdemCondResult, IdemCondCmd};
use crate::proc::spot_exch::SpotOrderExgProcImpl;

impl ConditionalOrderProc for SpotOrderExgProcImpl {
    fn handle(&mut self, cmd: IdemCondCmd) -> IdemCondResult {
        todo!()
    }
}