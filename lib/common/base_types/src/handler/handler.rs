use crate::cqrs::cqrs_types::CmdResp;

pub trait Handler<C, R, E>: Send + Sync {
    async fn handle(&self, cmd: C) -> Result<CmdResp<R>, E>;
}
