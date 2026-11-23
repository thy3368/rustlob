use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, StateSinkV3, StateSourceV3, UseCaseExecutorV3,
};
use example_core_use_case::{SpotBlockChanges, SpotBlockCmd, SpotBlockError, SpotBlockUseCase};

pub fn execute_spot_block_with_outbound<OB>(
    command: &SpotBlockCmd,
    outbound: &OB,
) -> Result<
    ExecutionResultV3<SpotBlockChanges>,
    ExecutionErrorV3<
        SpotBlockError,
        <OB as StateSourceV3<SpotBlockUseCase>>::Error,
        <OB as StateSinkV3>::Error,
    >,
>
where
    OB: StateSourceV3<SpotBlockUseCase> + StateSinkV3,
{
    UseCaseExecutorV3.execute_presequenced::<SpotBlockUseCase, OB, OB>(
        &SpotBlockUseCase,
        command,
        outbound,
        outbound,
    )
}
