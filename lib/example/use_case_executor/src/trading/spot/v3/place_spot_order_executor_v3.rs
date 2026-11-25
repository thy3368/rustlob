use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, LogEntry, StateSinkV3, StateSourceV3, UseCaseExecutorV3,
};
use example_core_use_case::pipe_line::place_spot_order::{
    PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error, PlaceSpotOrderPipelineV2UseCase,
};
use example_core_entity::PlaceSpotOrderPipelineV2Cmd;
use example_outbound_adapter::database::State4BuildBlock;

/// 使用共享运行时执行 Place Spot Order V3，并将同一 outbound 用于状态读写和事件副作用。
pub fn execute_place_spot_order_v3_with_outbound<OB>(
    command: &LogEntry<PlaceSpotOrderPipelineV2Cmd>,
    outbound: &mut OB,
) -> Result<
    ExecutionResultV3<PlaceSpotOrderPipelineV2Changes>,
    ExecutionErrorV3<
        PlaceSpotOrderPipelineV2Error,
        <OB as StateSourceV3<PlaceSpotOrderPipelineV2UseCase>>::Error,
        <OB as StateSinkV3<PlaceSpotOrderPipelineV2UseCase>>::Error,
    >,
>
where
    OB: StateSourceV3<PlaceSpotOrderPipelineV2UseCase>
        + StateSinkV3<PlaceSpotOrderPipelineV2UseCase>,
{
    UseCaseExecutorV3.execute_with_outbound::<PlaceSpotOrderPipelineV2UseCase, OB>(
        &PlaceSpotOrderPipelineV2UseCase,
        command,
        outbound,
    )
}

/// 在 build-block 投影上直接执行完整现货下单 pipeline。
pub fn execute_place_spot_order_v3(
    command: &LogEntry<PlaceSpotOrderPipelineV2Cmd>,
    state: &mut State4BuildBlock,
) -> Result<
    ExecutionResultV3<PlaceSpotOrderPipelineV2Changes>,
    ExecutionErrorV3<
        PlaceSpotOrderPipelineV2Error,
        <State4BuildBlock as StateSourceV3<PlaceSpotOrderPipelineV2UseCase>>::Error,
        <State4BuildBlock as StateSinkV3<PlaceSpotOrderPipelineV2UseCase>>::Error,
    >,
> {
    execute_place_spot_order_v3_with_outbound(command, state)
}
