use lob::lob::{Command, CommandResult, OrderCommandHandler};

//实现restful的的client
pub struct RestClient {}

impl OrderCommandHandler for RestClient {
    fn handle(&mut self, command: Command) -> CommandResult {
        todo!()
    }

    fn handler_name(&self) -> &'static str {
        todo!()
    }
}
