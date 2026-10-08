use common_entity::{EntityReplayableEvent, ExecutionError, ExecutionResult};
use example_core_use_case::{
    MatchSpotOrderV3Changes, MatchSpotOrderV3Cmd, MatchSpotOrderV3Error, ORDER_ENTITY_TYPE,
};
use example_outbound_adapter::DefaultSpotOrderV2PlaceOutboundError;
use thiserror::Error;

use super::EntityReplayableEventHandler;
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

        execute_match_spot_order_v3(&command)
            .map(Some)
            .map_err(MatchSpotOrderCreatedEventError::Executor)
    }
}

#[cfg(test)]
fn handle_with_executor<Output, ExecutorError>(
    event: &EntityReplayableEvent,
    mut execute: impl FnMut(&MatchSpotOrderV3Cmd) -> Result<Output, ExecutorError>,
) -> Result<Option<Output>, MatchSpotOrderCreatedEventError<ExecutorError>> {
    let Some(command) = match_spot_order_command_from_event(event)
        .map_err(MatchSpotOrderCreatedEventError::Parse)?
    else {
        return Ok(None);
    };

    execute(&command).map(Some).map_err(MatchSpotOrderCreatedEventError::Executor)
}

pub fn match_spot_order_command_from_event(
    event: &EntityReplayableEvent,
) -> Result<Option<MatchSpotOrderV3Cmd>, MatchSpotOrderCreatedEventParseError> {
    if event.entity_type != ORDER_ENTITY_TYPE || !event.is_created() {
        return Ok(None);
    }

    Ok(Some(MatchSpotOrderV3Cmd {
        party_id: required_string_field(event, "account_id")?,
        asset: required_u32_field(event, "asset")?,
        order_id: required_u64_field(event, "order_id")?,
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
    InvalidNumber { field: &'static str, expected: &'static str, value: String },
}

fn required_string_field(
    event: &EntityReplayableEvent,
    field: &'static str,
) -> Result<String, MatchSpotOrderCreatedEventParseError> {
    let value = field_value(event, field)
        .ok_or(MatchSpotOrderCreatedEventParseError::MissingField(field))?;
    let value = std::str::from_utf8(value)
        .map_err(|_| MatchSpotOrderCreatedEventParseError::InvalidUtf8 { field })?;
    Ok(value.to_owned())
}

fn required_u32_field(
    event: &EntityReplayableEvent,
    field: &'static str,
) -> Result<u32, MatchSpotOrderCreatedEventParseError> {
    let value = field_value(event, field)
        .ok_or(MatchSpotOrderCreatedEventParseError::MissingField(field))?;
    let value = std::str::from_utf8(value)
        .map_err(|_| MatchSpotOrderCreatedEventParseError::InvalidUtf8 { field })?;
    value.parse::<u32>().map_err(|_| MatchSpotOrderCreatedEventParseError::InvalidNumber {
        field,
        expected: "u32",
        value: value.to_owned(),
    })
}

fn required_u64_field(
    event: &EntityReplayableEvent,
    field: &'static str,
) -> Result<u64, MatchSpotOrderCreatedEventParseError> {
    let value = field_value(event, field)
        .ok_or(MatchSpotOrderCreatedEventParseError::MissingField(field))?;
    let value = std::str::from_utf8(value)
        .map_err(|_| MatchSpotOrderCreatedEventParseError::InvalidUtf8 { field })?;
    value.parse::<u64>().map_err(|_| MatchSpotOrderCreatedEventParseError::InvalidNumber {
        field,
        expected: "u64",
        value: value.to_owned(),
    })
}

fn field_value<'a>(event: &'a EntityReplayableEvent, field: &str) -> Option<&'a [u8]> {
    event.field_changes.iter().find_map(|change| {
        (change.field_name_as_str().ok() == Some(field)).then(|| change.new_value_bytes())
    })
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use common_entity::{EntityChangeType, ReplayFieldChange};

    use super::*;

    #[test]
    fn non_order_events_are_ignored_without_calling_executor() {
        let calls = RefCell::new(0);
        let result = handle_with_executor(
            &event_with_fields(
                &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"1")],
                5,
                EntityChangeType::Created.as_tag(),
            ),
            |_: &MatchSpotOrderV3Cmd| -> Result<(), &'static str> {
                *calls.borrow_mut() += 1;
                Ok(())
            },
        );

        assert_eq!(result, Ok(None));
        assert_eq!(*calls.borrow(), 0);
    }

    #[test]
    fn non_created_order_events_are_ignored_without_calling_executor() {
        let calls = RefCell::new(0);
        let result = handle_with_executor(
            &event_with_fields(
                &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"1")],
                ORDER_ENTITY_TYPE,
                EntityChangeType::Updated.as_tag(),
            ),
            |_: &MatchSpotOrderV3Cmd| -> Result<(), &'static str> {
                *calls.borrow_mut() += 1;
                Ok(())
            },
        );

        assert_eq!(result, Ok(None));
        assert_eq!(*calls.borrow(), 0);
    }

    #[test]
    fn order_created_event_is_mapped_to_command_and_executed() {
        let seen = RefCell::new(Vec::new());
        let result = handle_with_executor(
            &order_created_event(),
            |command: &MatchSpotOrderV3Cmd| -> Result<&'static str, &'static str> {
                seen.borrow_mut().push(command.clone());
                Ok("matched")
            },
        );

        assert_eq!(result, Ok(Some("matched")));
        assert_eq!(
            seen.into_inner(),
            vec![MatchSpotOrderV3Cmd { party_id: "buyer".to_string(), asset: 10001, order_id: 1 }]
        );
    }

    #[test]
    fn missing_order_field_returns_parse_error_without_calling_executor() {
        let calls = RefCell::new(0);
        let result = handle_with_executor(
            &event_with_fields(
                &[("account_id", b"buyer"), ("asset", b"10001")],
                ORDER_ENTITY_TYPE,
                EntityChangeType::Created.as_tag(),
            ),
            |_: &MatchSpotOrderV3Cmd| -> Result<(), &'static str> {
                *calls.borrow_mut() += 1;
                Ok(())
            },
        );

        assert!(matches!(
            result,
            Err(MatchSpotOrderCreatedEventError::Parse(
                MatchSpotOrderCreatedEventParseError::MissingField("order_id")
            ))
        ));
        assert_eq!(*calls.borrow(), 0);
    }

    #[test]
    fn invalid_order_field_returns_parse_error_without_calling_executor() {
        let calls = RefCell::new(0);
        let invalid_asset = event_with_fields(
            &[("account_id", b"buyer"), ("asset", b"not-a-number"), ("order_id", b"1")],
            ORDER_ENTITY_TYPE,
            EntityChangeType::Created.as_tag(),
        );
        assert!(matches!(
            handle_with_executor(
                &invalid_asset,
                |_: &MatchSpotOrderV3Cmd| -> Result<(), &'static str> {
                    *calls.borrow_mut() += 1;
                    Ok(())
                }
            ),
            Err(MatchSpotOrderCreatedEventError::Parse(
                MatchSpotOrderCreatedEventParseError::InvalidNumber { field: "asset", .. }
            ))
        ));

        let invalid_utf8 = event_with_fields(
            &[("account_id", &[0xff]), ("asset", b"10001"), ("order_id", b"1")],
            ORDER_ENTITY_TYPE,
            EntityChangeType::Created.as_tag(),
        );
        assert!(matches!(
            handle_with_executor(
                &invalid_utf8,
                |_: &MatchSpotOrderV3Cmd| -> Result<(), &'static str> {
                    *calls.borrow_mut() += 1;
                    Ok(())
                }
            ),
            Err(MatchSpotOrderCreatedEventError::Parse(
                MatchSpotOrderCreatedEventParseError::InvalidUtf8 { field: "account_id" }
            ))
        ));
        assert_eq!(*calls.borrow(), 0);
    }

    #[test]
    fn executor_failure_is_wrapped_and_returned() {
        let result = handle_with_executor(
            &order_created_event(),
            |_: &MatchSpotOrderV3Cmd| -> Result<(), &'static str> { Err("executor failed") },
        );

        assert_eq!(result, Err(MatchSpotOrderCreatedEventError::Executor("executor failed")));
    }

    fn order_created_event() -> EntityReplayableEvent {
        event_with_fields(
            &[("account_id", b"buyer"), ("asset", b"10001"), ("order_id", b"1")],
            ORDER_ENTITY_TYPE,
            EntityChangeType::Created.as_tag(),
        )
    }

    fn event_with_fields(
        fields: &[(&str, &[u8])],
        entity_type: u8,
        change_type: u8,
    ) -> EntityReplayableEvent {
        let mut event = EntityReplayableEvent::new(1, 1, 0, 1, 1, entity_type, change_type);
        for (field_name, value) in fields {
            event.add_field_change(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(field_name),
                &[],
                value,
                0,
            ));
        }
        event
    }
}
