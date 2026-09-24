use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    ActivateSpotOrderV2Changes, ActivateSpotOrderV2Cmd, ActivateSpotOrderV2Error,
    ActivateSpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_activate_spot_order_v2(
    command: &ActivateSpotOrderV2Cmd,
) -> Result<
    ExecutionResult<ActivateSpotOrderV2Changes>,
    ExecutionError<ActivateSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_activate_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_activate_spot_order_v2_with_outbound<OB>(
    command: &ActivateSpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<ActivateSpotOrderV2Changes>,
    ExecutionError<ActivateSpotOrderV2Error, <OB as StateSink<ActivateSpotOrderV2UseCase>>::Error>,
>
where
    OB: StateSource<
            ActivateSpotOrderV2UseCase,
            Error = <OB as StateSink<ActivateSpotOrderV2UseCase>>::Error,
        > + StateSink<ActivateSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<ActivateSpotOrderV2UseCase, OB, OB>(
        &ActivateSpotOrderV2UseCase,
        command,
        outbound,
        outbound,
    )
}
