use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, MiFamilyExecutionResult, MiFamilyExecutionSpec, MiFamilyOutbound,
    MiStateMachineFamilyExecutor,
};
use example_core::{
    Balance, Reservation, SpotOrderV2, SpotOrderV2CaseChanges, SpotOrderV2Command,
    SpotOrderV2GivenState, SpotOrderV2UseCaseFamily,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum CancelContractError {
    #[error("Unexpected `action.type` for cancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`action.cancels` must contain at least one cancel request.")]
    EmptyCancels,
    #[error("Invalid `action.cancels[].o`. Expected a positive order id.")]
    InvalidOid,
    #[error("Invalid `action.f`. Omit `f` unless fast cancel is enabled.")]
    InvalidFastFlag,
}

pub mod reply {
    use serde::Serialize;

    use crate::exchange::common::wire::{
        ExchangeResponseEnvelopeWire, ExchangeResponseWire, ExchangeStatusesDataWire,
    };

    pub type CancelResponseWire = ExchangeResponseWire<CancelResponseDataWire>;
    pub type CancelResponseEnvelopeWire = ExchangeResponseEnvelopeWire<CancelResponseDataWire>;
    pub type CancelResponseDataWire = ExchangeStatusesDataWire<CancelStatusWire>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum CancelStatusWire {
        Success(&'static str),
        Error { error: String },
    }
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    cancels: Vec<CancelItemWire>,
    f: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CancelItemWire {
    a: u32,
    o: u64,
}

pub(crate) const DEFAULT_EXCHANGE_PARTY_ID: &str = "default-exchange-party";

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CancelSpotOrderV2Lookup {
    Oid(u64),
    Cloid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2Request {
    pub party_id: String,
    pub asset: u32,
    pub lookup: CancelSpotOrderV2Lookup,
}

impl CancelSpotOrderV2Request {
    fn from_wire_cancel(party_id: String, cancel: &CancelItemWire) -> Self {
        Self { party_id, asset: cancel.a, lookup: CancelSpotOrderV2Lookup::Oid(cancel.o) }
    }

    #[allow(dead_code)]
    pub fn from_cloid(party_id: String, asset: u32, cloid: String) -> Self {
        Self { party_id, asset, lookup: CancelSpotOrderV2Lookup::Cloid(cloid) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV2CancelLoadedState {
    pub order: SpotOrderV2,
    pub principal_reservation: Reservation,
    pub fee_reservation: Reservation,
    pub balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

pub struct SpotOrderV2CancelExecutionSpec;

impl MiFamilyExecutionSpec<SpotOrderV2UseCaseFamily> for SpotOrderV2CancelExecutionSpec {
    type Request = CancelSpotOrderV2Request;
    type LoadedState = SpotOrderV2CancelLoadedState;

    fn command(_request: &Self::Request) -> SpotOrderV2Command {
        SpotOrderV2Command::Cancel(Default::default())
    }

    fn given_state(loaded: &Self::LoadedState) -> SpotOrderV2GivenState<'_> {
        SpotOrderV2GivenState::Cancel {
            order: &loaded.order,
            principal_reservation: &loaded.principal_reservation,
            fee_reservation: &loaded.fee_reservation,
            balances: &loaded.balances,
            base_asset_id: &loaded.base_asset_id,
            quote_asset_id: &loaded.quote_asset_id,
            maker_fee_bps: loaded.maker_fee_bps,
            taker_fee_bps: loaded.taker_fee_bps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2CancelOutboundError {
    #[error("spot order v2 cancel state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2CancelOutbound;

impl MiFamilyOutbound<CancelSpotOrderV2Request, SpotOrderV2CancelLoadedState>
    for DefaultSpotOrderV2CancelOutbound
{
    type Error = DefaultSpotOrderV2CancelOutboundError;

    fn load_state(
        &self,
        _request: &CancelSpotOrderV2Request,
    ) -> Result<SpotOrderV2CancelLoadedState, Self::Error> {
        Err(DefaultSpotOrderV2CancelOutboundError::StateUnavailable)
    }

    fn persist(&self, _events: &[cmd_handler::EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[cmd_handler::EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[cmd_handler::EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub(crate) struct CancelAction;

impl ExchangeActionHandler for CancelAction {
    type Request = RequestWire;
    type Reply = reply::CancelResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "cancel" {
        return Err(ExchangeHttpError::contract(CancelContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.cancels.is_empty() {
        return Err(ExchangeHttpError::contract(CancelContractError::EmptyCancels));
    }
    if matches!(request.action.f, Some(false)) {
        return Err(ExchangeHttpError::contract(CancelContractError::InvalidFastFlag));
    }
    if request.action.cancels.iter().any(|cancel| cancel.o == 0) {
        return Err(ExchangeHttpError::contract(CancelContractError::InvalidOid));
    }
    Ok(())
}

async fn execute(request: RequestWire) -> Result<reply::CancelResponseWire, ExchangeHttpError> {
    let outbound = DefaultSpotOrderV2CancelOutbound;
    let statuses = execute_with_outbound(request, &outbound);
    Ok(ok_statuses_response("cancel", statuses))
}

pub fn execute_cancel_spot_order_v2<OB>(
    request: &CancelSpotOrderV2Request,
    outbound: &OB,
) -> Result<
    MiFamilyExecutionResult<SpotOrderV2CaseChanges>,
    MiFamilyExecutionError<example_core::SpotOrderV2UseCaseFamilyError, OB::Error>,
>
where
    OB: MiFamilyOutbound<CancelSpotOrderV2Request, SpotOrderV2CancelLoadedState>,
{
    MiStateMachineFamilyExecutor
        .execute::<SpotOrderV2UseCaseFamily, SpotOrderV2CancelExecutionSpec, OB>(
            &SpotOrderV2UseCaseFamily,
            request,
            outbound,
        )
}

fn execute_with_outbound<OB>(request: RequestWire, outbound: &OB) -> Vec<reply::CancelStatusWire>
where
    OB: MiFamilyOutbound<CancelSpotOrderV2Request, SpotOrderV2CancelLoadedState>,
    OB::Error: std::fmt::Display,
{
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    request
        .action
        .cancels
        .iter()
        .map(|cancel| {
            let cancel_request =
                CancelSpotOrderV2Request::from_wire_cancel(party_id.clone(), cancel);
            match execute_cancel_spot_order_v2(&cancel_request, outbound) {
                Ok(_) => reply::CancelStatusWire::Success("success"),
                Err(error) => {
                    reply::CancelStatusWire::Error { error: cancel_execution_error_message(error) }
                }
            }
        })
        .collect()
}

pub(crate) fn cancel_execution_error_message<BE, OE>(
    error: MiFamilyExecutionError<BE, OE>,
) -> String
where
    BE: std::fmt::Display,
    OE: std::fmt::Display,
{
    match error {
        MiFamilyExecutionError::Business(error) => error.to_string(),
        MiFamilyExecutionError::ProjectEvents(error) => {
            format!("project replayable events failed: {error}")
        }
        MiFamilyExecutionError::LoadState(error) => format!("load_state failed: {error}"),
        MiFamilyExecutionError::Persist(error) => format!("persist failed: {error}"),
        MiFamilyExecutionError::Replay(error) => format!("replay failed: {error}"),
        MiFamilyExecutionError::Publish(error) => format!("publish failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use cmd_handler::command_use_case_def2::MiFamilyOutbound;
    use example_core::{
        ReservationKind, ReservationMarketKind, SpotOrderExecution, SpotOrderSide, SpotOrderStatus,
        SpotOrderTimeInForce,
    };

    use super::*;

    #[test]
    fn parses_cancel_request() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_cancel_request_json())
                .expect("cancel request should parse");

        assert_eq!(request.action.type_, "cancel");
        assert_eq!(request.action.cancels.len(), 1);
        assert_eq!(request.action.cancels[0].o, 77738308);
    }

    #[test]
    fn rejects_false_fast_flag() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "cancel",
                    "cancels": [{ "a": 10000, "o": 77738308 }],
                    "f": false
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("cancel request parses");

        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `action.f`. Omit `f` unless fast cancel is enabled."
        );
    }

    #[actix_web::test]
    async fn cancel_default_path_returns_item_error_when_state_is_unavailable() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_cancel_request_json())
                .expect("cancel request parses");
        let response = execute(request).await.expect("cancel response builds");

        let actual = serde_json::to_string_pretty(&response).expect("cancel response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "cancel",
    "data": {
      "statuses": [
        {
          "error": "load_state failed: spot order v2 cancel state is not wired for default HTTP path"
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("fake outbound error")]
    struct FakeOutboundError;

    #[derive(Debug, Default)]
    struct FakeSpotOrderV2CancelOutbound {
        observed_lookup: Arc<Mutex<Option<CancelSpotOrderV2Lookup>>>,
    }

    impl MiFamilyOutbound<CancelSpotOrderV2Request, SpotOrderV2CancelLoadedState>
        for FakeSpotOrderV2CancelOutbound
    {
        type Error = FakeOutboundError;

        fn load_state(
            &self,
            request: &CancelSpotOrderV2Request,
        ) -> Result<SpotOrderV2CancelLoadedState, Self::Error> {
            *self.observed_lookup.lock().expect("lookup observation lock should be available") =
                Some(request.lookup.clone());

            let order = SpotOrderV2::new(
                "order-1".to_string(),
                request.asset,
                Some(77738308),
                request.party_id.clone(),
                "BTCUSDT".to_string(),
                SpotOrderSide::Buy,
                SpotOrderExecution::Limit { price: 100 },
                SpotOrderTimeInForce::Gtc,
                2,
                0,
                SpotOrderStatus::Open,
                None,
                0,
                200,
                None,
                1,
            );

            Ok(SpotOrderV2CancelLoadedState {
                principal_reservation: reservation(
                    &order.order_id,
                    &order.account_id,
                    ReservationKind::SpotBuyQuote,
                    "USDT",
                    200,
                ),
                fee_reservation: reservation(
                    &order.order_id,
                    &order.account_id,
                    ReservationKind::SpotBuyFeeQuote,
                    "USDT",
                    1,
                ),
                balances: vec![Balance::new(
                    request.party_id.clone(),
                    "USDT".to_string(),
                    1000,
                    201,
                    1,
                )],
                order,
                base_asset_id: "BTC".to_string(),
                quote_asset_id: "USDT".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            })
        }

        fn persist(
            &self,
            _events: &[cmd_handler::EntityReplayableEvent],
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn replay(
            &self,
            _events: &[cmd_handler::EntityReplayableEvent],
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(
            &self,
            _events: &[cmd_handler::EntityReplayableEvent],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn reservation(
        order_id: &str,
        account_id: &str,
        kind: ReservationKind,
        asset_id: &str,
        amount: u64,
    ) -> Reservation {
        Reservation::new(
            format!("reservation:{order_id}:{asset_id}:{kind:?}"),
            account_id.to_string(),
            order_id.to_string(),
            ReservationMarketKind::Spot,
            kind,
            asset_id.to_string(),
            amount,
        )
        .expect("test reservation should be valid")
    }

    #[test]
    fn spot_order_v2_cancel_request_maps_wire_and_executes_with_fake_outbound() {
        let wire =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_cancel_request_json())
                .expect("request parses");
        let request = CancelSpotOrderV2Request::from_wire_cancel(
            "buyer".to_string(),
            &wire.action.cancels[0],
        );
        let outbound = FakeSpotOrderV2CancelOutbound::default();

        let result = execute_cancel_spot_order_v2(&request, &outbound)
            .expect("spot order v2 cancel family should execute");

        assert_eq!(
            *outbound.observed_lookup.lock().expect("lookup observation lock should be available"),
            Some(CancelSpotOrderV2Lookup::Oid(77738308))
        );
        let SpotOrderV2CaseChanges::Cancel(changes) = result.changes else {
            panic!("expected cancel changes");
        };
        assert_eq!(changes.updated_order.after.status, SpotOrderStatus::Canceled);
        assert_eq!(changes.created_reservation_released.len(), 2);
        assert!(!result.events.is_empty());
    }

    #[test]
    fn cancel_error_shape_snapshot_is_stable() {
        let response = reply::CancelResponseWire {
            status: "ok",
            response: reply::CancelResponseEnvelopeWire {
                type_: "cancel",
                data: reply::CancelResponseDataWire {
                    statuses: vec![reply::CancelStatusWire::Error {
                        error: "Order was never placed, already canceled, or filled.".to_string(),
                    }],
                },
            },
        };

        let actual = serde_json::to_string_pretty(&response).expect("cancel response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "cancel",
    "data": {
      "statuses": [
        {
          "error": "Order was never placed, already canceled, or filled."
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    fn valid_cancel_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "cancel",
                "cancels": [{ "a": 10000, "o": 77738308 }]
            },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
