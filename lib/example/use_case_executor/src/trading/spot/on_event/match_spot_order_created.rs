use common_entity::{
    EntityReplayableEvent, EntityReplayableEventHandler, ExecutionError, ExecutionResult,
    ReplayEventFieldParseError,
};
use example_core_use_case::{
    MatchSpotOrderV3Changes, MatchSpotOrderV3Cmd, MatchSpotOrderV3Error, ORDER_ENTITY_TYPE,
};
use example_outbound_adapter::DefaultSpotOrderV2PlaceOutboundError;
use thiserror::Error;

use crate::trading::spot::match_spot_order_v3_executor::execute_match_spot_order_v3;

pub type MatchSpotOrderExecutorError =
    ExecutionError<MatchSpotOrderV3Error, DefaultSpotOrderV2PlaceOutboundError>;
pub type DefaultMatchSpotOrderCreatedEventOutput = ExecutionResult<MatchSpotOrderV3Changes>;
pub type DefaultMatchSpotOrderCreatedEventError =
    MatchSpotOrderCreatedEventError<MatchSpotOrderExecutorError>;

/// 订单 created event 到撮合用例执行的业务 handler。
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderCreatedEventHandler;

impl MatchSpotOrderCreatedEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl EntityReplayableEventHandler for MatchSpotOrderCreatedEventHandler {
    type Output = DefaultMatchSpotOrderCreatedEventOutput;
    type Error = DefaultMatchSpotOrderCreatedEventError;

    fn handle(
        &mut self,
        event: &EntityReplayableEvent,
    ) -> Result<Option<Self::Output>, Self::Error> {
        let Some(command) = match_spot_order_command_from_event(event)
            .map_err(MatchSpotOrderCreatedEventError::Parse)?
        else {
            return Ok(None);
        };

        execute_match_spot_order_v3(&common_entity::LogEntrySequencer::from_metadata(
            event.lsn,
            event.logical_ts,
            0,
            event.lsn,
            command,
        ))
        .map(Some)
        .map_err(MatchSpotOrderCreatedEventError::Executor)
    }
}

pub fn match_spot_order_command_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<MatchSpotOrderV3Cmd>, MatchSpotOrderCreatedEventParseError> {
    if event.entity_type != ORDER_ENTITY_TYPE || !event.is_created() {
        return Ok(None);
    }

    Ok(Some(MatchSpotOrderV3Cmd {
        party_id: event.required_string_field("account_id")?,
        asset: event.required_u32_field("asset")?,
        order_id: event.required_u64_field("order_id")?,
    }))
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MatchSpotOrderCreatedEventError<ExecutorError> {
    #[error(transparent)]
    Parse(#[from] MatchSpotOrderCreatedEventParseError),
    #[error("spot order executor failed: {0:?}")]
    Executor(ExecutorError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MatchSpotOrderCreatedEventParseError {
    #[error("spot order event is missing field `{0}`")]
    MissingField(&'static str),
    #[error("spot order event field `{field}` is not valid UTF-8")]
    InvalidUtf8 { field: &'static str },
    #[error("spot order event field `{field}` is not a valid {expected}: {value}")]
    InvalidNumber {
        field: &'static str,
        expected: &'static str,
        value: String,
    },
}

impl From<ReplayEventFieldParseError> for MatchSpotOrderCreatedEventParseError {
    fn from(error: ReplayEventFieldParseError) -> Self {
        match error {
            ReplayEventFieldParseError::MissingField(field) => Self::MissingField(field),
            ReplayEventFieldParseError::InvalidUtf8 { field } => Self::InvalidUtf8 { field },
            ReplayEventFieldParseError::InvalidNumber {
                field,
                expected,
                value,
            } => Self::InvalidNumber {
                field,
                expected,
                value,
            },
        }
    }
}
