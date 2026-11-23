use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, LogEntry, StateSinkV3, StateSourceV3, UseCaseExecutorV3,
};
use example_core_use_case::pipeline::place_spot_order::{
    PlaceSpotOrderV3Changes, PlaceSpotOrderV3Cmd, PlaceSpotOrderV3Error, PlaceSpotOrderV3UseCase,
};

/// 使用共享运行时执行 Place Spot Order V3，并将同一 outbound 用于状态读写和事件副作用。
pub fn execute_place_spot_order_v3_with_outbound<OB>(
    command: &LogEntry<PlaceSpotOrderV3Cmd>,
    outbound: &OB,
) -> Result<
    ExecutionResultV3<PlaceSpotOrderV3Changes>,
    ExecutionErrorV3<
        PlaceSpotOrderV3Error,
        <OB as StateSourceV3<PlaceSpotOrderV3UseCase>>::Error,
        <OB as StateSinkV3>::Error,
    >,
>
where
    OB: StateSourceV3<PlaceSpotOrderV3UseCase> + StateSinkV3,
{
    UseCaseExecutorV3.execute::<PlaceSpotOrderV3UseCase, OB, OB>(
        &PlaceSpotOrderV3UseCase,
        command,
        outbound,
        outbound,
    )
}
