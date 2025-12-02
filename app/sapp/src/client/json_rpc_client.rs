use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderHandler};

pub struct JsonRpcClient {}

impl SpotOrderHandler for JsonRpcClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult {
        todo!()
    }
}
