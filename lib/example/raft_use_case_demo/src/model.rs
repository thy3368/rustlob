use common_entity::{ExecutionResultV3, LogEntrySequencer, ReplayableEntityChanges};
use example_core_use_case::entity::common::identifiers::OrderId;
use example_core_use_case::{
    CancelSpotOrderFsmV3Cmd, OrderGroupRelation, OrderSide, OrderTif, OrderType,
    PlaceSpotOrderPipelineV2Cmd,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCmds, SpotBlockCommandExecutionV3, SpotBlockCommandV2, SpotBlockExecutionV3,
};

pub const USER_ID: &str = "buyer";
pub const CLOID: &str = "raft-order-1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotRaftRequest {
    pub block: SpotBlockCmds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandStatus {
    Applied,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotCommandResponse {
    pub status: CommandStatus,
    pub error: Option<String>,
    pub replayable_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotRaftResponse {
    pub applied_log_index: u64,
    pub commands: Vec<SpotCommandResponse>,
    pub block_error: Option<String>,
    pub replayable_event_count: usize,
}

impl SpotRaftResponse {
    pub(crate) fn blank(applied_log_index: u64) -> Self {
        Self {
            applied_log_index,
            commands: Vec::new(),
            block_error: None,
            replayable_event_count: 0,
        }
    }

    pub(crate) fn rejected_block(applied_log_index: u64, error: impl ToString) -> Self {
        Self {
            applied_log_index,
            commands: Vec::new(),
            block_error: Some(error.to_string()),
            replayable_event_count: 0,
        }
    }

    pub(crate) fn from_execution(applied_log_index: u64, execution: &SpotBlockExecutionV3) -> Self {
        let commands = execution
            .command_results
            .iter()
            .map(project_command)
            .collect();
        Self {
            applied_log_index,
            commands,
            block_error: None,
            replayable_event_count: execution.replayable_events().len(),
        }
    }
}

fn project_command(item: &SpotBlockCommandExecutionV3) -> SpotCommandResponse {
    match item {
        SpotBlockCommandExecutionV3::Place { result, .. } => project_result(result),
        SpotBlockCommandExecutionV3::Cancel { result, .. } => project_result(result),
    }
}

fn project_result<T, E>(result: &Result<ExecutionResultV3<T>, E>) -> SpotCommandResponse
where
    T: ReplayableEntityChanges,
    E: std::fmt::Debug,
{
    match result {
        Ok(changes) => SpotCommandResponse {
            status: CommandStatus::Applied,
            error: None,
            replayable_event_count: changes.state_changed.replayable_events().len(),
        },
        Err(error) => SpotCommandResponse {
            status: CommandStatus::Rejected,
            error: Some(format!("{error:?}")),
            replayable_event_count: 0,
        },
    }
}

pub fn place_request(lsn: u64) -> SpotRaftRequest {
    request(vec![entry(lsn, place_command())])
}

pub fn place_and_cancel_request() -> SpotRaftRequest {
    request(vec![
        entry(1, place_command()),
        entry(2, cancel_command(CLOID)),
    ])
}

pub fn cancel_request(lsn: u64) -> SpotRaftRequest {
    request(vec![entry(lsn, cancel_command(CLOID))])
}

pub fn invalid_request() -> SpotRaftRequest {
    request(Vec::new())
}

fn request(items: Vec<common_entity::LogEntry<SpotBlockCommandV2>>) -> SpotRaftRequest {
    SpotRaftRequest {
        block: SpotBlockCmds { items },
    }
}

fn entry(lsn: u64, command: SpotBlockCommandV2) -> common_entity::LogEntry<SpotBlockCommandV2> {
    LogEntrySequencer::from_metadata_with_topic(0, 0, lsn, lsn, 0, lsn, USER_ID.to_owned(), command)
}

fn place_command() -> SpotBlockCommandV2 {
    SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
        party_id: common_entity::stable_string_entity_id(USER_ID),
        order_id: 1,
        symbol: symbol(),
        side: OrderSide::Buy,
        price: Decimal::from(100),
        size: Decimal::from(2),
        order_type: OrderType::Limit { tif: OrderTif::Gtc },
        relation: OrderGroupRelation::Standalone,
        cloid: Some(CLOID.to_owned()),
        fee_bps: 10,
    })
}

fn cancel_command(cloid: &str) -> SpotBlockCommandV2 {
    SpotBlockCommandV2::Cancel(CancelSpotOrderFsmV3Cmd {
        order_id: OrderId::Cloid(cloid.to_owned()),
    })
}

pub(crate) fn symbol() -> base_types_v2::Symbol {
    base_types_v2::Symbol::new(1, base_types_v2::Coin::BTC, base_types_v2::Coin::USDT)
}
