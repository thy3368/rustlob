use lob::lob::{IdempotentSpotCmd, IdempotentSpotResult, SpotOrderExchangeProc};

// 实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderExchangeProc for WebSocketClient {
    fn handle(&mut self, _command: IdempotentSpotCmd) -> IdempotentSpotResult { todo!() }
}
