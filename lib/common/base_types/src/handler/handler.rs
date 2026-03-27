use crate::cqrs::cqrs_types::CmdResp;

pub trait Handler<C, R, E>: Send + Sync {
    async fn handle(&self, cmd: C) -> Result<CmdResp<R>, E>;
}

// 业务功能 里面包含 取数，计算changelog,持久化，发消息等能力
pub trait CmdHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;
}

pub trait QueryHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;
}

// 外部调进来
pub trait InboundHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;
}
