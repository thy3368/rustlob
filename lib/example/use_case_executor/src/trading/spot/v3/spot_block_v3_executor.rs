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
use example_outbound_adapter::database::{
    CancelSpotOrderBuildBlockSinkError, CancelSpotOrderBuildBlockSourceError,
    PlaceSpotOrderBuildBlockSinkError, PlaceSpotOrderBuildBlockSourceError, State4BuildBlock,
};

use super::cancel_spot_order_executor_v3::execute_cancel_spot_order_v3_in_build_block;
use super::place_spot_order_executor_v3::execute_place_spot_order_v3;
use crate::trading::spot::v3::spot_block_v2::{
    SpotBlockCmdV2, SpotBlockCommandV2, SpotBlockErrorV2,
};

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
pub enum SpotBlockCommandExecutionV3 {
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
pub struct SpotBlockExecutionV3 {
    pub command_results: Vec<SpotBlockCommandExecutionV3>,
}

impl ReplayableEntityChanges for SpotBlockExecutionV3 {
    fn replayable_events(&self) -> Vec<&common_entity::ReplayableDomainEvent> {
        self.command_results
            .iter()
            .flat_map(|item| match item {
                SpotBlockCommandExecutionV3::Place {
                    result: Ok(result), ..
                } => result.state_changed.replayable_events(),
                SpotBlockCommandExecutionV3::Cancel {
                    result: Ok(result), ..
                } => result.state_changed.replayable_events(),
                _ => Vec::new(),
            })
            .collect()
    }
}

pub fn execute_spot_block_v3(
    cmds: &[LogEntry<SpotBlockCommandV2>],
    state: &mut State4BuildBlock,
) -> Result<SpotBlockExecutionV3, SpotBlockErrorV2> {
    SpotBlockCmdV2::validate_entries(cmds)?;
    let mut command_results = Vec::with_capacity(cmds.len());
    for item in cmds {
        match item.payload().clone() {
            SpotBlockCommandV2::Place(payload) => {
                let command = LogEntrySequencer::derive(item, payload);
                let mut candidate = state.clone();
                let result = execute_place_spot_order_v3(&command, &mut candidate);
                if result.is_ok() {
                    *state = candidate;
                }
                command_results.push(SpotBlockCommandExecutionV3::Place { command, result });
            }
            SpotBlockCommandV2::Cancel(payload) => {
                let command = LogEntrySequencer::derive(item, payload);
                let mut candidate = state.clone();
                let result = execute_cancel_spot_order_v3_in_build_block(&command, &mut candidate);
                if result.is_ok() {
                    *state = candidate;
                }
                command_results.push(SpotBlockCommandExecutionV3::Cancel { command, result });
            }
        }
    }
    Ok(SpotBlockExecutionV3 { command_results })
}

///todo 增加函数 比如 State4BuildBlock 前后 生成diff, 找出create/update的entity, 写入rocks_db
