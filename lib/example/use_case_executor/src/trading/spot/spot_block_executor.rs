use common_entity::{ExecutionError, ExecutionResult, StateSink, StateSource, UseCaseExecutor};
use example_core_use_case::{SpotBlockChanges, SpotBlockCmd, SpotBlockError, SpotBlockUseCase};

pub fn execute_spot_block_with_outbound<OB>(
    command: &SpotBlockCmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<SpotBlockChanges>,
    ExecutionError<SpotBlockError, <OB as StateSink<SpotBlockUseCase>>::Error>,
>
where
    OB: StateSource<SpotBlockUseCase, Error = <OB as StateSink<SpotBlockUseCase>>::Error>
        + StateSink<SpotBlockUseCase>,
{
    UseCaseExecutor.execute_presequenced::<SpotBlockUseCase, OB, OB>(
        &SpotBlockUseCase,
        command,
        outbound,
        outbound,
    )
}
