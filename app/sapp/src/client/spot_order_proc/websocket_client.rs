use spot_proc::proc::behavior::trading_spot_order_behavior::{IdemSpotResult, SpotCmdAny, SpotOrderExchProc};

// 实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderExchProc for WebSocketClient {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        todo!()
    }
}
