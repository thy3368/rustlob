use lob::lob::{SpotCommand, SpotCommandResult, SpotOrderHandler};

//实现websocket的client
pub struct WebSocketClient {}

impl SpotOrderHandler for WebSocketClient {
    fn handle(&mut self, _command: SpotCommand) -> SpotCommandResult {
        todo!()
    }
}
