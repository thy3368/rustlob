use crate::StateMachineV2Diff;

/// adapter-side request 到 family on_command 的映射约定。
///
/// executor 不直接依赖该 trait；adapter 可用它把 request 转成 on_command 后再执行。
pub trait ExecutionCommandSpec<F>
where
    F: StateMachineV2Diff,
{
    type Request;

    fn command(
        request: &Self::Request,
        lsn: u64,
        logical_ts: u64,
        client_id: u64,
        request_id: u64,
    ) -> F::Command;
}
