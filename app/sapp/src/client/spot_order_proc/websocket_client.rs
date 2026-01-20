use lob::lob::{IdempotentSpotCmd, IdempotentSpotResult, SpotOrderExgProc};

// 实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderExgProc for WebSocketClient {
    fn handle(&mut self, _command: IdempotentSpotCmd) -> IdempotentSpotResult { todo!() }
}
