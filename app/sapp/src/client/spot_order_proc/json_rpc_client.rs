use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderExchangeProc};

pub struct JsonRpcClient {}

impl SpotOrderExchangeProc for JsonRpcClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult { todo!() }
}
