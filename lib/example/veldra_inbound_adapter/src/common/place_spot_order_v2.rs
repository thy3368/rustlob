use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, MiFamilyExecutionResult, MiFamilyOutbound, MiFamilyStateSource,
    MiStateMachineFamilyExecutor,
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
    MiFamilyExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<
        example_core_use_case::SpotOrderV2UseCaseFamilyV3Error,
        DefaultSpotOrderV2PlaceOutboundError,
    >,
> {
    execute_place_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub(crate) fn execute_place_spot_order_v2_with_outbound<OB>(
    command: &SpotOrderV2CommandV3,
    outbound: &OB,
) -> Result<
    MiFamilyExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<
        example_core_use_case::SpotOrderV2UseCaseFamilyV3Error,
        <OB as MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>>::Error,
    >,
>
where
    OB: MiFamilyStateSource<
            SpotOrderV2UseCaseFamilyV3,
            Error = <OB as MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>>::Error,
        > + MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>,
{
    MiStateMachineFamilyExecutor.execute::<SpotOrderV2UseCaseFamilyV3, OB, OB>(
        &SpotOrderV2UseCaseFamilyV3,
        command,
        outbound,
        outbound,
    )
}
