use spot_proc::proc::behavior::trading_spot_order_proc::{IdemSpotResult, SpotCmdAny, SpotOrderExchProc};

// 实现restful的的client
pub struct RestfulClient {}

impl SpotOrderExchProc for RestfulClient {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        todo!()
    }
}
