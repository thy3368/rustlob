use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    PlaceMatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error, PlaceMatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2Cmd,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_place_match_spot_order_v2(
    command: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
) -> Result<
    ExecutionResult<PlaceMatchSpotOrderV2Changes>,
    ExecutionError<PlaceMatchSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_place_match_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_place_match_spot_order_v2_with_outbound<OB>(
    command: &LogEntry<PlaceOnlySpotOrderV2Cmd>,
    outbound: &OB,
) -> Result<
    ExecutionResult<PlaceMatchSpotOrderV2Changes>,
    ExecutionError<
        PlaceMatchSpotOrderV2Error,
        <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            PlaceMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<PlaceMatchSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<PlaceMatchSpotOrderV2UseCase, OB, OB>(
        &PlaceMatchSpotOrderV2UseCase,
        command,
        outbound,
        outbound,
    )
}
use common_entity::LogEntry;
