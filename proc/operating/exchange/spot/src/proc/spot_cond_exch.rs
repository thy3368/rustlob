use crate::proc::behavior::trading_spot_order_proc::{ConditionalOrderProc, IdemCondResult, IdemCondCmd};
use crate::proc::spot_exch::SpotOrderExchProcImpl;

impl ConditionalOrderProc for SpotOrderExchProcImpl {
    fn handle(&mut self, cmd: IdemCondCmd) -> IdemCondResult {
        todo!()
    }
}