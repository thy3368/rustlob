use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderProc};

pub struct JsonRpcClient {}

impl SpotOrderProc for JsonRpcClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult {
        todo!()
    }
}
