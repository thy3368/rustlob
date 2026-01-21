use spot_behavior::proc::behavior::spot_trade_behavior::{IdemSpotResult, SpotCmdAny, SpotTradeBehavior};

// 实现websocket的client
pub struct WebSocketClient {}

impl SpotTradeBehavior for WebSocketClient {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult {
        todo!()
    }
}
