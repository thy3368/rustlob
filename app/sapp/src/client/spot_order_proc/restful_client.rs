use lob::lob::{IdempotentSpotCmd, IdempotentSpotResult, SpotOrderExgProc};

// 实现restful的的client
pub struct RestfulClient {}

impl SpotOrderExgProc for RestfulClient {
    fn handle(&mut self, _command: IdempotentSpotCmd) -> IdempotentSpotResult { todo!() }
}
