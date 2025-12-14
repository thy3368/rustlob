use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderExchangeProc};

// 实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderExchangeProc for WebSocketClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult { todo!() }
}
