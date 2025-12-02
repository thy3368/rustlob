use lob::lob::{SpotCommand, SpotCommandResult, SpotOrderHandler};

//实现restful的的client
pub struct RestfulClient {}

impl SpotOrderHandler for RestfulClient {
    fn handle(&mut self, _command: SpotCommand) -> SpotCommandResult {
        todo!()
    }
}
