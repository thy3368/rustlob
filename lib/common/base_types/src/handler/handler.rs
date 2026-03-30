use crate::cqrs::cqrs_types::CmdResp;

pub trait Handler<C, R, E>: Send + Sync {
    async fn handle(&self, cmd: C) -> Result<CmdResp<R>, E>;
}

// 业务功能 里面包含 取数，计算changelog,持久化，发消息等能力
pub trait CmdHandler<C, R, E>: Send + Sync {
    fn handle(&self, cmd: C) -> Result<R, E>;

    //todo 1. queue state; 2. cal changelog for state; 3. publish changelog
    //todo 持久化分两类， 1，先存changelog, 通过异步回放实现state变更，发布changelog; 2,发布changelog, 异步回放changelog
}
