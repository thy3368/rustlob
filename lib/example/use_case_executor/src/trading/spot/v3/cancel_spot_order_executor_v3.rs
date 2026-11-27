use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, LogEntry, StateSinkV3, StateSourceV3, UseCaseExecutorV3,
};
use example_core_use_case::pipe_line::cancel_spot_order::{
    CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3Error,
    CancelSpotOrderFsmV3UseCase,
};
use example_outbound_adapter::database::build_block_cancel_spot_order::CancelSpotOrderBuildBlockSinkError;
use example_outbound_adapter::database::{CancelSpotOrderBuildBlockSourceError, State4BuildBlock};

/// 使用共享 V3 runtime 执行 Cancel Spot Order，并复用 outbound 完成状态读写与事件副作用。
pub fn execute_cancel_spot_order_v3_with_outbound<OB>(
    command: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    outbound: &mut OB,
) -> Result<
    ExecutionResultV3<CancelSpotOrderFsmV3Changes>,
    ExecutionErrorV3<
        CancelSpotOrderFsmV3Error,
        <OB as StateSourceV3<CancelSpotOrderFsmV3UseCase>>::Error,
        <OB as StateSinkV3<CancelSpotOrderFsmV3UseCase>>::Error,
    >,
>
where
    OB: StateSourceV3<CancelSpotOrderFsmV3UseCase> + StateSinkV3<CancelSpotOrderFsmV3UseCase>,
{
    UseCaseExecutorV3.execute_with_outbound::<CancelSpotOrderFsmV3UseCase, OB>(
        &CancelSpotOrderFsmV3UseCase,
        command,
        outbound,
    )
}

/// 在调用方独占的 build-block 状态上顺序执行取消，不持久化或发布 replay 事件。
pub fn execute_cancel_spot_order_v3_in_build_block(
    command: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    state: &mut State4BuildBlock,
) -> Result<
    ExecutionResultV3<CancelSpotOrderFsmV3Changes>,
    ExecutionErrorV3<
        CancelSpotOrderFsmV3Error,
        CancelSpotOrderBuildBlockSourceError,
        CancelSpotOrderBuildBlockSinkError,
    >,
> {
    execute_cancel_spot_order_v3_with_outbound(command, state)
}
