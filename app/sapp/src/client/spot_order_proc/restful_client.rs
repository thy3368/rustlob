use lob::lob::{IdempotentSpotCmd, IdempotentSpotResult, SpotOrderExchangeProc};

// 实现restful的的client
pub struct RestfulClient {}

impl SpotOrderExchangeProc for RestfulClient {
    fn handle(&mut self, _command: IdempotentSpotCmd) -> IdempotentSpotResult { todo!() }
}
