use lob::lob::{Command, CommandResult, OrderCommandHandler};

pub struct JsonRpcClient {}

impl OrderCommandHandler for JsonRpcClient {
    fn handle(&mut self, command: Command) -> CommandResult {
        todo!()
    }

    fn handler_name(&self) -> &'static str {
        todo!()
    }
}
