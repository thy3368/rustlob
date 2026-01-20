use spot_proc::proc::behavior::trading_spot_order_behavior::{IdemSpotResult, SpotCmdAny, SpotOrderExchBehavior};

pub struct JsonRpcClient {}

impl SpotOrderExchBehavior for JsonRpcClient {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        todo!()
    }
}
