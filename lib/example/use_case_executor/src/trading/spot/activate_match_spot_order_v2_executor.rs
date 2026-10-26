use common_entity::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    ActivateMatchSpotOrderV2Changes, ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2Error,
    ActivateMatchSpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_activate_match_spot_order_v2(
    command: &LogEntry<ActivateMatchSpotOrderV2Cmd>,
) -> Result<
    ExecutionResult<ActivateMatchSpotOrderV2Changes>,
    ExecutionError<ActivateMatchSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_activate_match_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_activate_match_spot_order_v2_with_outbound<OB>(
    command: &LogEntry<ActivateMatchSpotOrderV2Cmd>,
    outbound: &OB,
) -> Result<
    ExecutionResult<ActivateMatchSpotOrderV2Changes>,
    ExecutionError<
        ActivateMatchSpotOrderV2Error,
        <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            ActivateMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<ActivateMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<ActivateMatchSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<ActivateMatchSpotOrderV2UseCase, OB, OB>(
        &ActivateMatchSpotOrderV2UseCase,
        command,
        outbound,
        outbound,
    )
}
use common_entity::LogEntry;
