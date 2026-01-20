use spot_proc::proc::behavior::trading_spot_order_behavior::{IdemSpotResult, SpotCmdAny, SpotOrderExchProc};

pub struct JsonRpcClient {}

impl SpotOrderExchProc for JsonRpcClient {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        todo!()
    }
}
