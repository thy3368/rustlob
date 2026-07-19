use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{EventProjectError, ReplayableChanges};
use common_entity::MiStateMachineV2Unchecked;
use example_core::entity::{HyperliquidPerpLeverageSetting, HyperliquidPerpMarginMode};
use example_core::{
    UpdateHyperliquidPerpLeverageChanges, UpdateHyperliquidPerpLeverageCmd,
    UpdateHyperliquidPerpLeverageError, UpdateHyperliquidPerpLeverageState,
    UpdateHyperliquidPerpLeverageUseCase,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::actions::cancel::DEFAULT_EXCHANGE_PARTY_ID;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateLeverageContractError {
    #[error("Unexpected `action.type` for updateLeverage handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.leverage`. Expected an integer greater than or equal to 1.")]
    InvalidLeverage,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateLeverageExecutionError<OE>
where
    OE: std::fmt::Display,
{
    #[error("{0}")]
    Business(UpdateHyperliquidPerpLeverageError),
    #[error("project replayable events failed: {0}")]
    ProjectEvents(EventProjectError),
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
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as UpdateLeverageResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    asset: u32,
    #[serde(rename = "isCross")]
    is_cross: bool,
    leverage: u64,
}

/// adapter-side 请求对象，只表达 wire 已确认的字段，不携带业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateLeverageRequest {
    pub party_id: String,
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u64,
}

impl UpdateLeverageRequest {
    fn from_wire(request: RequestWire) -> Self {
        Self {
            party_id: request
                .common
                .vault_address
                .unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string()),
            asset: request.action.asset,
            is_cross: request.action.is_cross,
            leverage: request.action.leverage,
        }
    }
}

pub trait UpdateLeverageOutbound {
    type Error: std::fmt::Display;

    fn load_given_state(
        &self,
        cmd: &UpdateHyperliquidPerpLeverageCmd,
    ) -> Result<UpdateHyperliquidPerpLeverageState, Self::Error>;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultUpdateLeverageOutboundError {}

#[derive(Debug, Default)]
pub struct DefaultUpdateLeverageOutbound;

impl UpdateLeverageOutbound for DefaultUpdateLeverageOutbound {
    type Error = DefaultUpdateLeverageOutboundError;

    fn load_given_state(
        &self,
        cmd: &UpdateHyperliquidPerpLeverageCmd,
    ) -> Result<UpdateHyperliquidPerpLeverageState, Self::Error> {
        let margin_mode = if cmd.is_cross {
            HyperliquidPerpMarginMode::Cross
        } else {
            HyperliquidPerpMarginMode::Isolated
        };
        let before_leverage = if cmd.leverage == 1 { 2 } else { 1 };
        Ok(UpdateHyperliquidPerpLeverageState {
            account_id: cmd.party_id.clone(),
            leverage_setting: HyperliquidPerpLeverageSetting::new(
                cmd.party_id.clone(),
                cmd.asset,
                margin_mode,
                before_leverage,
                1,
            ),
        })
    }

    fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub(crate) struct UpdateLeverageAction;

impl ExchangeActionHandler for UpdateLeverageAction {
    type Request = RequestWire;
    type Reply = reply::UpdateLeverageResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "updateLeverage" {
        return Err(ExchangeHttpError::contract(
            UpdateLeverageContractError::UnexpectedActionType(request.action.type_.clone()),
        ));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.leverage < 1 {
        return Err(ExchangeHttpError::contract(UpdateLeverageContractError::InvalidLeverage));
    }
    Ok(())
}

pub fn execute_update_perp_leverage<OB>(
    request: &UpdateLeverageRequest,
    outbound: &OB,
) -> Result<UpdateHyperliquidPerpLeverageChanges, UpdateLeverageExecutionError<OB::Error>>
where
    OB: UpdateLeverageOutbound,
{
    let use_case = UpdateHyperliquidPerpLeverageUseCase;
    let cmd = UpdateHyperliquidPerpLeverageCmd {
        party_id: request.party_id.clone(),
        asset: request.asset,
        is_cross: request.is_cross,
        leverage: request.leverage,
    };

    use_case.pre_check_command(&cmd).map_err(UpdateLeverageExecutionError::Business)?;
    let state = outbound.load_given_state(&cmd).map_err(UpdateLeverageExecutionError::LoadState)?;
    use_case
        .validate_against_given_state(&cmd, &state)
        .map_err(UpdateLeverageExecutionError::Business)?;
    let changes = use_case
        .compute_after_changes_unchecked(&cmd, &state)
        .map_err(UpdateLeverageExecutionError::Business)?;
    let events =
        changes.to_replayable_events().map_err(UpdateLeverageExecutionError::ProjectEvents)?;

    outbound.persist(&events).map_err(UpdateLeverageExecutionError::Persist)?;
    outbound.replay(&events).map_err(UpdateLeverageExecutionError::Replay)?;
    outbound.publish(&events).map_err(UpdateLeverageExecutionError::Publish)?;

    Ok(changes)
}

async fn execute(
    request: RequestWire,
) -> Result<reply::UpdateLeverageResponseWire, ExchangeHttpError> {
    let request = UpdateLeverageRequest::from_wire(request);
    let outbound = DefaultUpdateLeverageOutbound;
    execute_update_perp_leverage(&request, &outbound).map_err(ExchangeHttpError::contract)?;
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::fmt;

    use actix_web::ResponseError;

    use super::*;

    #[test]
    fn parses_update_leverage_request() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_update_leverage_json())
                .expect("request should parse");
        assert_eq!(request.action.type_, "updateLeverage");
        assert_eq!(request.action.leverage, 5);
    }

    #[test]
    fn rejects_zero_leverage() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "updateLeverage",
                    "asset": 7,
                    "isCross": true,
                    "leverage": 0
                },
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
            "Invalid `action.leverage`. Expected an integer greater than or equal to 1."
        );
    }

    #[actix_web::test]
    async fn cross_update_leverage_reply_snapshot_is_stable() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_update_leverage_json())
                .expect("request should parse");

        let response = execute(request).await.expect("response should build");

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    #[actix_web::test]
    async fn isolated_update_leverage_returns_core_margin_mode_error() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(isolated_update_leverage_json())
                .expect("request should parse");

        let error = execute(request).await.expect_err("isolated path should fail in core");

        assert_eq!(error.status_code(), actix_web::http::StatusCode::BAD_REQUEST);
        assert_eq!(error.to_string(), "margin mode is not supported yet");
    }

    #[test]
    fn helper_maps_wire_request_to_core_command_and_projects_leverage_event() {
        let request = UpdateLeverageRequest {
            party_id: "trader-1".to_string(),
            asset: 7,
            is_cross: true,
            leverage: 9,
        };
        let outbound = ObservingUpdateLeverageOutbound::new(4);

        let changes = execute_update_perp_leverage(&request, &outbound)
            .expect("cross leverage update should execute");

        let seen_cmd = outbound.seen_cmd.borrow().clone().expect("command should be observed");
        assert_eq!(
            seen_cmd,
            UpdateHyperliquidPerpLeverageCmd {
                party_id: "trader-1".to_string(),
                asset: 7,
                is_cross: true,
                leverage: 9,
            }
        );
        assert_eq!(changes.changed_leverage_setting.before.leverage, 4);
        assert_eq!(changes.changed_leverage_setting.after.leverage, 9);
        assert_eq!(changes.changed_leverage_setting.before.version, 1);
        assert_eq!(changes.changed_leverage_setting.after.version, 2);

        let events = changes.to_replayable_events().expect("events should project");
        assert_eq!(events.len(), 1);
        assert!(events[0].is_updated());
        assert_eq!(events[0].field_changes.len(), 1);
        assert_eq!(events[0].field_changes[0].field_name_as_str().ok(), Some("leverage"));
        assert_eq!(events[0].field_changes[0].old_value_bytes(), b"4");
        assert_eq!(events[0].field_changes[0].new_value_bytes(), b"9");
    }

    #[test]
    fn helper_runs_persist_replay_publish_in_order_with_same_events() {
        let request = UpdateLeverageRequest {
            party_id: "trader-1".to_string(),
            asset: 7,
            is_cross: true,
            leverage: 3,
        };
        let outbound = ObservingUpdateLeverageOutbound::new(8);

        execute_update_perp_leverage(&request, &outbound).expect("execution should succeed");

        assert_eq!(
            outbound.steps.borrow().as_slice(),
            ["load_given_state", "persist", "replay", "publish"]
        );
        let batches = outbound.event_batches.borrow();
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].0, "persist");
        assert_eq!(batches[1].0, "replay");
        assert_eq!(batches[2].0, "publish");
        assert_eq!(batches[0].1, batches[1].1);
        assert_eq!(batches[1].1, batches[2].1);
        assert_eq!(batches[0].1.len(), 1);
    }

    fn valid_update_leverage_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "updateLeverage",
                "asset": 7,
                "isCross": true,
                "leverage": 5
            },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }

    fn isolated_update_leverage_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "updateLeverage",
                "asset": 7,
                "isCross": false,
                "leverage": 5
            },
            "nonce": 1710000000000,
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

    impl std::error::Error for FakeOutboundError {}

    struct ObservingUpdateLeverageOutbound {
        before_leverage: u64,
        steps: RefCell<Vec<&'static str>>,
        seen_cmd: RefCell<Option<UpdateHyperliquidPerpLeverageCmd>>,
        event_batches: RefCell<Vec<(&'static str, Vec<EntityReplayableEvent>)>>,
    }

    impl ObservingUpdateLeverageOutbound {
        fn new(before_leverage: u64) -> Self {
            Self {
                before_leverage,
                steps: RefCell::new(Vec::new()),
                seen_cmd: RefCell::new(None),
                event_batches: RefCell::new(Vec::new()),
            }
        }
    }

    impl UpdateLeverageOutbound for ObservingUpdateLeverageOutbound {
        type Error = FakeOutboundError;

        fn load_given_state(
            &self,
            cmd: &UpdateHyperliquidPerpLeverageCmd,
        ) -> Result<UpdateHyperliquidPerpLeverageState, Self::Error> {
            self.steps.borrow_mut().push("load_given_state");
            self.seen_cmd.borrow_mut().replace(cmd.clone());
            let margin_mode = if cmd.is_cross {
                HyperliquidPerpMarginMode::Cross
            } else {
                HyperliquidPerpMarginMode::Isolated
            };
            Ok(UpdateHyperliquidPerpLeverageState {
                account_id: cmd.party_id.clone(),
                leverage_setting: HyperliquidPerpLeverageSetting::new(
                    cmd.party_id.clone(),
                    cmd.asset,
                    margin_mode,
                    self.before_leverage,
                    1,
                ),
            })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.steps.borrow_mut().push("persist");
            self.event_batches.borrow_mut().push(("persist", events.to_vec()));
            Ok(())
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.steps.borrow_mut().push("replay");
            self.event_batches.borrow_mut().push(("replay", events.to_vec()));
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.steps.borrow_mut().push("publish");
            self.event_batches.borrow_mut().push(("publish", events.to_vec()));
            Ok(())
        }
    }
}
