use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, LogAttr, LogEntry, LogEntrySequencer,
    ReplayableEntityChanges, Shard,
};
use example_core_entity::PlaceSpotOrderPipelineV2Cmd;
use example_core_use_case::pipe::line::place_spot_order_pipeline_v2::{
    PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error,
};
use example_core_use_case::pipe_line::cancel_spot_order::{
    CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3Error,
};
use example_outbound_adapter::database::build_block_cancel_spot_order::CancelSpotOrderBuildBlockSinkError;
use example_outbound_adapter::database::{
    CancelSpotOrderBuildBlockSourceError, PlaceSpotOrderBuildBlockSinkError,
    PlaceSpotOrderBuildBlockSourceError, State4BuildBlock,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::cancel_spot_order_executor_v3::execute_cancel_spot_order_v3_in_build_block;
use super::place_spot_order_executor_v3::execute_place_spot_order_v3;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBlockCmds {
    pub items: Vec<LogEntry<SpotBlockCommandV2>>,
}

impl SpotBlockCmds {
    pub fn validate(&self) -> Result<(), SpotBlockErrorV2> {
        Self::validate_entries(&self.items)
    }

    pub fn validate_entries(
        items: &[LogEntry<SpotBlockCommandV2>],
    ) -> Result<(), SpotBlockErrorV2> {
        let first = items.first().ok_or(SpotBlockErrorV2::EmptyCommands)?;
        for pair in items.windows(2) {
            if pair[0].lsn() >= pair[1].lsn() {
                return Err(SpotBlockErrorV2::NonIncreasingLsn);
            }
            if pair[0].logical_ts() >= pair[1].logical_ts() {
                return Err(SpotBlockErrorV2::NonIncreasingLogicalTimestamp);
            }
        }
        if items
            .iter()
            .any(|item| item.topic() != first.topic() || item.partition() != first.partition())
        {
            return Err(SpotBlockErrorV2::MixedShard);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBlockCommandV2 {
    Place(PlaceSpotOrderPipelineV2Cmd),
    Cancel(CancelSpotOrderFsmV3Cmd),
}

impl Shard for SpotBlockCmds {
    fn topic(&self) -> u8 {
        self.items.last().map_or(0, LogEntry::topic)
    }

    fn partition(&self) -> u8 {
        self.items.last().map_or(0, LogEntry::partition)
    }
}

impl LogAttr for SpotBlockCmds {
    fn lsn(&self) -> u64 {
        self.items.last().map_or(0, LogEntry::lsn)
    }

    fn logical_ts(&self) -> u64 {
        self.items.last().map_or(0, LogEntry::logical_ts)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockErrorV2 {
    #[error("spot block must contain at least one command")]
    EmptyCommands,
    #[error("command LSNs must be strictly increasing")]
    NonIncreasingLsn,
    #[error("command logical timestamps must be strictly increasing")]
    NonIncreasingLogicalTimestamp,
    #[error("all commands must use the same topic and partition")]
    MixedShard,
}

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

pub fn build_block_spot_v3(
    cmds: &[LogEntry<SpotBlockCommandV2>],
    state: &mut State4BuildBlock,
) -> Result<SpotBlockExecutionV3, SpotBlockErrorV2> {
    SpotBlockCmds::validate_entries(cmds)?;
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

// RocksDB 持久化由调用方在执行完成后通过 outbound adapter 显式完成第二阶段。
