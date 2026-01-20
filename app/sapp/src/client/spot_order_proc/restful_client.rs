use spot_proc::proc::behavior::trading_spot_order_behavior::{IdemSpotResult, SpotCmdAny, SpotOrderExchBehavior};

// 实现restful的的client
pub struct RestfulClient {}

impl SpotOrderExchBehavior for RestfulClient {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        todo!()
    }
}
