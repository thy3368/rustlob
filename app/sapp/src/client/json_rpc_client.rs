use lob::lob::{SpotCommand, SpotCommandResult, SpotOrderHandler};

pub struct JsonRpcClient {}

impl SpotOrderHandler for JsonRpcClient {
    fn handle(&mut self, _command: SpotCommand) -> SpotCommandResult {
        todo!()
    }
}
