use crate::cqrs::cqrs_types::CmdResp;
// =============================================================================
// CORE TRAIT: Handler 基础接口
// =============================================================================


pub trait Handler<C, R, E>: Send + Sync {
    async fn handle(&self, cmd: C) -> Result<CmdResp<R>, E>;
}



// =============================================================================
// CMDHANDLER: 同步命令处理器
// =============================================================================

pub trait CmdHandler<C, R, E>: Send + Sync {
    fn cmd_handle(&self, cmd: C) -> Result<R, E>;
}
