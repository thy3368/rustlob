use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, LogEntry, LogEntrySequencer, ReplayableEntityChanges,
};
use example_core_entity::PlaceSpotOrderPipelineV2Cmd;
use example_core_use_case::pipe::place_spot_order_pipeline_v2::{
    PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error,
};
use example_core_use_case::pipe_line::cancel_spot_order::{
    CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3Error,
};
use example_core_use_case::pipe_line::{SpotBlockCmdV2, SpotBlockCommandV2, SpotBlockErrorV2};
use example_outbound_adapter::database::{
    CancelSpotOrderBuildBlockSinkError, CancelSpotOrderBuildBlockSourceError,
    PlaceSpotOrderBuildBlockSinkError, PlaceSpotOrderBuildBlockSourceError, State4BuildBlock,
};

use super::cancel_spot_order_executor_v3::execute_cancel_spot_order_v3_in_build_block;
use super::place_spot_order_executor_v3::execute_place_spot_order_v3;

pub type PlaceSpotBlockExecutionResult = Result<
    ExecutionResultV3<PlaceSpotOrderPipelineV2Changes>,
    ExecutionErrorV3<
        PlaceSpotOrderPipelineV2Error,
        PlaceSpotOrderBuildBlockSourceError,
        PlaceSpotOrderBuildBlockSinkError,
    >,
>;

pub type CancelSpotBlockExecutionResult = Result<
    ExecutionResultV3<CancelSpotOrderFsmV3Changes>,
    ExecutionErrorV3<
        CancelSpotOrderFsmV3Error,
        CancelSpotOrderBuildBlockSourceError,
        CancelSpotOrderBuildBlockSinkError,
    >,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockCommandExecutionV2 {
    Place {
        command: LogEntry<PlaceSpotOrderPipelineV2Cmd>,
        result: PlaceSpotBlockExecutionResult,
    },
    Cancel {
        command: LogEntry<CancelSpotOrderFsmV3Cmd>,
        result: CancelSpotBlockExecutionResult,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBlockExecutionV2 {
    pub command_results: Vec<SpotBlockCommandExecutionV2>,
}

impl ReplayableEntityChanges for SpotBlockExecutionV2 {
    fn replayable_events(&self) -> Vec<&common_entity::ReplayableDomainEvent> {
        self.command_results
            .iter()
            .flat_map(|item| match item {
                SpotBlockCommandExecutionV2::Place {
                    result: Ok(result), ..
                } => result.state_changed.replayable_events(),
                SpotBlockCommandExecutionV2::Cancel {
                    result: Ok(result), ..
                } => result.state_changed.replayable_events(),
                _ => Vec::new(),
            })
            .collect()
    }
}

pub fn execute_spot_block_v2(
    command: &SpotBlockCmdV2,
    state: &mut State4BuildBlock,
) -> Result<SpotBlockExecutionV2, SpotBlockErrorV2> {
    command.validate()?;
    let mut command_results = Vec::with_capacity(command.items.len());
    for item in &command.items {
        match item.payload().clone() {
            SpotBlockCommandV2::Place(payload) => {
                let command = LogEntrySequencer::derive(item, payload);
                let mut next = state.clone();
                let result = execute_place_spot_order_v3(&command, &mut next);
                if result.is_ok() {
                    *state = next;
                }
                command_results.push(SpotBlockCommandExecutionV2::Place { command, result });
            }
            SpotBlockCommandV2::Cancel(payload) => {
                let command = LogEntrySequencer::derive(item, payload);
                let mut next = state.clone();
                let result = execute_cancel_spot_order_v3_in_build_block(&command, &mut next);
                if result.is_ok() {
                    *state = next;
                }
                command_results.push(SpotBlockCommandExecutionV2::Cancel { command, result });
            }
        }
    }
    Ok(SpotBlockExecutionV2 { command_results })
}

pub fn execute_spot_block_v3(
    cmds: Vec<LogEntry<SpotBlockCommandV2>>,

    state: &mut State4BuildBlock,
) -> Result<SpotBlockExecutionV2, SpotBlockErrorV2> {

    todo!()
}
