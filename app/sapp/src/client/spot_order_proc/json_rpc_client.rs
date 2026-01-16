use lob::lob::{IdempotentSpotCmd, IdempotentSpotResult, SpotOrderExchangeProc};

pub struct JsonRpcClient {}

impl SpotOrderExchangeProc for JsonRpcClient {
    fn handle(&mut self, _command: IdempotentSpotCmd) -> IdempotentSpotResult { todo!() }
}
