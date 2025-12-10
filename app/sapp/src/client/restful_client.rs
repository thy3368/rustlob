use lob::lob::{IdempotentSpotCommand, IdempotentSpotResult, SpotOrderProc};

//实现restful的的client
pub struct RestfulClient {}

impl SpotOrderProc for RestfulClient {
    fn handle(&mut self, _command: IdempotentSpotCommand) -> IdempotentSpotResult {
        todo!()
    }
}
