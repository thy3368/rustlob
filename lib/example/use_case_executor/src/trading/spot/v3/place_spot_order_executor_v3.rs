use common_entity::{
    ExecutionError, ExecutionResult, LogEntry, StateSink, StateSource, UseCaseExecutor,
};
use example_core_use_case::pipeline::place_spot_order::{
    PlaceSpotOrderV3Changes, PlaceSpotOrderV3Cmd, PlaceSpotOrderV3Error, PlaceSpotOrderV3UseCase,
};

/// 使用共享运行时执行 Place Spot Order V3，并将同一 outbound 用于状态读写和事件副作用。
pub fn execute_place_spot_order_v3_with_outbound<OB>(
    command: &LogEntry<PlaceSpotOrderV3Cmd>,
    outbound: &OB,
) -> Result<
    ExecutionResult<PlaceSpotOrderV3Changes>,
    ExecutionError<PlaceSpotOrderV3Error, <OB as StateSink<PlaceSpotOrderV3UseCase>>::Error>,
>
where
    OB: StateSource<
            PlaceSpotOrderV3UseCase,
            Error = <OB as StateSink<PlaceSpotOrderV3UseCase>>::Error,
        > + StateSink<PlaceSpotOrderV3UseCase>,
{
    UseCaseExecutor.execute::<PlaceSpotOrderV3UseCase, OB, OB>(
        &PlaceSpotOrderV3UseCase,
        command,
        outbound,
        outbound,
    )
}
