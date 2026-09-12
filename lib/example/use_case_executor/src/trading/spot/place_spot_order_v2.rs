use cmd_handler::command_use_case_def2::{
    ExecutionResult, MiFamilyExecutionError, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3, SpotOrderV2UseCaseFamilyV3,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_place_spot_order_v2(
    command: &SpotOrderV2CommandV3,
) -> Result<
    ExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<
        example_core_use_case::SpotOrderV2UseCaseFamilyV3Error,
        DefaultSpotOrderV2PlaceOutboundError,
    >,
> {
    execute_place_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_place_spot_order_v2_with_outbound<OB>(
    command: &SpotOrderV2CommandV3,
    outbound: &OB,
) -> Result<
    ExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<
        example_core_use_case::SpotOrderV2UseCaseFamilyV3Error,
        <OB as StateSink<SpotOrderV2UseCaseFamilyV3>>::Error,
    >,
>
where
    OB: StateSource<
            SpotOrderV2UseCaseFamilyV3,
            Error = <OB as StateSink<SpotOrderV2UseCaseFamilyV3>>::Error,
        > + StateSink<SpotOrderV2UseCaseFamilyV3>,
{
    StateMachineExecutor.execute::<SpotOrderV2UseCaseFamilyV3, OB, OB>(
        &SpotOrderV2UseCaseFamilyV3,
        command,
        outbound,
        outbound,
    )
}
