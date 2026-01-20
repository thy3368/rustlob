use lob::lob::{IdempotentSpotCmd, IdempotentSpotResult, SpotOrderExgProc};

pub struct JsonRpcClient {}

impl SpotOrderExgProc for JsonRpcClient {
    fn handle(&mut self, _command: IdempotentSpotCmd) -> IdempotentSpotResult { todo!() }
}
