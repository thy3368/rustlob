use lob::lob::{Command, CommandResult, OrderCommandHandler};

//实现restful的的client
pub struct WebSocketClient {}

impl OrderCommandHandler for WebSocketClient {
    fn handle(&mut self, command: Command) -> CommandResult {
        todo!()
    }

    fn handler_name(&self) -> &'static str {
        todo!()
    }
}
