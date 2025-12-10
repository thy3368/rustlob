use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderProc};

//实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderProc for WebSocketClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult {
        todo!()
    }
}
