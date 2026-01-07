use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderExchangeProc};

// 实现restful的的client
pub struct RestfulClient {}

impl SpotOrderExchangeProc for RestfulClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult { todo!() }
}
