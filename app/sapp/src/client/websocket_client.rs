use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderHandler};

//实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderHandler for WebSocketClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult {
        todo!()
    }
}
