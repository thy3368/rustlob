use common_entity::{EntityError, EntityReplayableEvent, ReplayableChanges, UseCase};
use example_core_use_case::{
    ScheduleHyperliquidCancelChanges, ScheduleHyperliquidCancelCmd, ScheduleHyperliquidCancelError,
    ScheduleHyperliquidCancelState, ScheduleHyperliquidCancelUseCase, UserId,
};
use example_outbound_adapter::{
    HyperliquidScheduleCancelInMemoryOutbound, HyperliquidScheduleCancelOutboundError,
};
use serde::{Deserialize, Serialize};

use crate::bootstrap::on_command::exchange::actions::DEFAULT_EXCHANGE_PARTY_ID;
use crate::bootstrap::on_command::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler,
};
use crate::bootstrap::on_command::exchange::common::validate::validate_envelope_common;
use crate::bootstrap::on_command::exchange::common::wire::{
    ExchangeRequestEnvelopeWire, ok_default_response,
};
use crate::bootstrap::on_command::exchange::error::ExchangeHttpError;
#[cfg(test)]
use crate::common::parse::parse_json_request;

#[derive(Debug, thiserror::Error)]
pub enum ScheduleCancelContractError {
    #[error("Unexpected `action.type` for scheduleCancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.time`. Expected it to be at least 5 seconds after `nonce`.")]
    InvalidTime,
}

#[derive(Debug, thiserror::Error)]
pub enum ScheduleCancelExecutionError<OE>
where
    OE: std::fmt::Display,
{
    #[error("{0}")]
    Business(ScheduleHyperliquidCancelError),
    #[error("project replayable events failed: {0}")]
    ProjectEvents(EntityError),
    #[error("load_state failed: {0}")]
    LoadState(OE),
    #[error("persist failed: {0}")]
    Persist(OE),
    #[error("replay failed: {0}")]
    Replay(OE),
    #[error("publish failed: {0}")]
    Publish(OE),
}

pub mod reply {
    pub use crate::bootstrap::on_command::exchange::common::wire::ExchangeEmptyResponseWire as ScheduleCancelResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    time: Option<u64>,
}

/// 已通过 wire 校验的计划撤单请求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleCancelRequest {
    pub party_id: String,
    pub cancel_at_ms: Option<u64>,
    pub now_ms: u64,
}

impl ScheduleCancelRequest {
    fn from_wire(request: RequestWire) -> Self {
        Self {
            party_id: request
                .common
                .vault_address
                .unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string()),
            cancel_at_ms: request.action.time,
            now_ms: request.common.nonce,
        }
    }
}

pub trait ScheduleCancelOutbound {
    type Error: std::fmt::Display;

    fn load_given_state(
        &self,
        cmd: &ScheduleHyperliquidCancelCmd,
    ) -> Result<ScheduleHyperliquidCancelState, Self::Error>;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

impl ScheduleCancelOutbound for HyperliquidScheduleCancelInMemoryOutbound {
    type Error = HyperliquidScheduleCancelOutboundError;

    fn load_given_state(
        &self,
        cmd: &ScheduleHyperliquidCancelCmd,
    ) -> Result<ScheduleHyperliquidCancelState, Self::Error> {
        common_entity::StateSource::<ScheduleHyperliquidCancelUseCase>::load_given_state(
            self,
            &common_entity::LogEntrySequencer::from_metadata(0, cmd.now_ms, 0, 0, cmd.clone()),
        )
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        common_entity::StateSink::<ScheduleHyperliquidCancelUseCase>::persist(self, events)
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        common_entity::StateSink::<ScheduleHyperliquidCancelUseCase>::replay(self, events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        common_entity::StateSink::<ScheduleHyperliquidCancelUseCase>::publish(self, events)
    }
}

pub(crate) struct ScheduleCancelAction;

impl ExchangeActionHandler for ScheduleCancelAction {
    type Request = RequestWire;
    type Reply = reply::ScheduleCancelResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "scheduleCancel" {
        return Err(ExchangeHttpError::contract(
            ScheduleCancelContractError::UnexpectedActionType(request.action.type_.clone()),
        ));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if let Some(time) = request.action.time {
        let Some(earliest) = request.common.nonce.checked_add(5_000) else {
            return Err(ExchangeHttpError::contract(ScheduleCancelContractError::InvalidTime));
        };
        if time < earliest {
            return Err(ExchangeHttpError::contract(ScheduleCancelContractError::InvalidTime));
        }
    }
    Ok(())
}

pub fn execute_schedule_cancel<OB>(
    request: &ScheduleCancelRequest,
    lsn: u64,
    logical_ts: u64,
    client_id: u64,
    request_id: u64,
    outbound: &OB,
) -> Result<ScheduleHyperliquidCancelChanges, ScheduleCancelExecutionError<OB::Error>>
where
    OB: ScheduleCancelOutbound,
{
    let use_case = ScheduleHyperliquidCancelUseCase;
    let payload = ScheduleHyperliquidCancelCmd {
        user_id: UserId(common_entity::stable_string_entity_id(&request.party_id)),
        cancel_at_ms: request.cancel_at_ms,
        now_ms: request.now_ms,
    };
    let cmd = common_entity::LogEntrySequencer::from_metadata(
        lsn, logical_ts, client_id, request_id, payload,
    );

    use_case.check_command(&cmd).map_err(ScheduleCancelExecutionError::Business)?;
    let state = outbound
        .load_given_state(cmd.payload())
        .map_err(ScheduleCancelExecutionError::LoadState)?;
    use_case.validate_state_given(&cmd, &state).map_err(ScheduleCancelExecutionError::Business)?;
    let changes = use_case
        .compute_state_changed_only(&cmd, &state)
        .map_err(ScheduleCancelExecutionError::Business)?;
    let mut events =
        changes.to_replayable_events().map_err(ScheduleCancelExecutionError::ProjectEvents)?;
    for event in &mut events {
        event.lsn = cmd.lsn();
        event.logical_ts = cmd.logical_ts();
    }

    outbound.persist(&events).map_err(ScheduleCancelExecutionError::Persist)?;
    outbound.replay(&events).map_err(ScheduleCancelExecutionError::Replay)?;
    outbound.publish(&events).map_err(ScheduleCancelExecutionError::Publish)?;
    Ok(changes)
}

async fn execute(
    request: RequestWire,
) -> Result<reply::ScheduleCancelResponseWire, ExchangeHttpError> {
    let nonce = request.common.nonce;
    let request = ScheduleCancelRequest::from_wire(request);
    execute_schedule_cancel(
        &request,
        nonce,
        nonce,
        0,
        nonce,
        crate::bootstrap::runtime::schedule_cancel_outbound(),
    )
    .map_err(ExchangeHttpError::contract)?;
    // 官方文档未展示 scheduleCancel 成功响应。
    // 官方 Python SDK 的 basic_schedule_cancel.py 仅原样打印结果，没有约束 response shape。
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::fmt;

    use example_core_use_case::ScheduledCancel;

    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.time, Some(1710000006000));
    }

    #[test]
    fn rejects_too_soon_time() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": { "type": "scheduleCancel", "time": 1710000004000 },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `action.time`. Expected it to be at least 5 seconds after `nonce`."
        );
    }

    #[actix_web::test]
    async fn reply_snapshot_is_stable() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        let response = execute(request).await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    #[test]
    fn helper_maps_wire_semantics_and_runs_the_core_use_case() {
        let request = ScheduleCancelRequest {
            party_id: "trader-1".to_string(),
            cancel_at_ms: Some(1_710_000_006_000),
            now_ms: 1_710_000_000_000,
        };
        let outbound = ObservingOutbound::default();

        let changes =
            execute_schedule_cancel(&request, 9, 11, 2, 3, &outbound).expect("计划撤单应执行成功");

        let seen = outbound.seen_cmd.borrow().clone().expect("应观察到 core command");
        assert_eq!(seen.user_id, UserId(common_entity::stable_string_entity_id("trader-1")));
        assert_eq!(seen.cancel_at_ms, request.cancel_at_ms);
        assert_eq!(seen.now_ms, request.now_ms);
        assert_eq!(changes.changed_schedule.after.cancel_at_ms(), request.cancel_at_ms);
        assert_eq!(outbound.steps.borrow().as_slice(), ["load", "persist", "replay", "publish"]);
        let batches = outbound.batches.borrow();
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0], batches[1]);
        assert_eq!(batches[1], batches[2]);
        assert_eq!(batches[0][0].lsn, 9);
        assert_eq!(batches[0][0].logical_ts, 11);
    }

    #[test]
    fn helper_propagates_daily_trigger_limit_without_writing() {
        let request = ScheduleCancelRequest {
            party_id: "trader-1".to_string(),
            cancel_at_ms: Some(1_710_000_006_000),
            now_ms: 1_710_000_000_000,
        };
        let outbound = ObservingOutbound { at_daily_limit: true, ..Default::default() };

        let error = execute_schedule_cancel(&request, 1, 1, 1, 1, &outbound)
            .expect_err("日触发上限应拒绝新计划");

        assert!(matches!(
            error,
            ScheduleCancelExecutionError::Business(
                ScheduleHyperliquidCancelError::DailyTriggerLimitReached
            )
        ));
        assert_eq!(outbound.steps.borrow().as_slice(), ["load"]);
        assert!(outbound.batches.borrow().is_empty());
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": { "type": "scheduleCancel", "time": 1710000006000 },
            "nonce": 1710000000000,
            "vaultAddress": "0x3333333333333333333333333333333333333333",
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakeOutboundError;

    impl fmt::Display for FakeOutboundError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("fake outbound error")
        }
    }

    #[derive(Default)]
    struct ObservingOutbound {
        at_daily_limit: bool,
        steps: RefCell<Vec<&'static str>>,
        seen_cmd: RefCell<Option<ScheduleHyperliquidCancelCmd>>,
        batches: RefCell<Vec<Vec<EntityReplayableEvent>>>,
    }

    impl ScheduleCancelOutbound for ObservingOutbound {
        type Error = FakeOutboundError;

        fn load_given_state(
            &self,
            cmd: &ScheduleHyperliquidCancelCmd,
        ) -> Result<ScheduleHyperliquidCancelState, Self::Error> {
            self.steps.borrow_mut().push("load");
            self.seen_cmd.borrow_mut().replace(cmd.clone());
            let mut scheduled_cancel = ScheduledCancel::new(cmd.user_id.0);
            if self.at_daily_limit {
                for _ in 0..10 {
                    scheduled_cancel
                        .replace(Some(cmd.now_ms), cmd.now_ms)
                        .expect("fixture should arm");
                    scheduled_cancel.trigger(cmd.now_ms).expect("fixture should trigger");
                }
            }
            Ok(ScheduleHyperliquidCancelState { scheduled_cancel })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.steps.borrow_mut().push("persist");
            self.batches.borrow_mut().push(events.to_vec());
            Ok(())
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.steps.borrow_mut().push("replay");
            self.batches.borrow_mut().push(events.to_vec());
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.steps.borrow_mut().push("publish");
            self.batches.borrow_mut().push(events.to_vec());
            Ok(())
        }
    }
}
