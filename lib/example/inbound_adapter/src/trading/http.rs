use std::sync::Arc;

use actix_web::{HttpResponse, Scope, web};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, MiFamilyOutbound, MiStateMachineFamilyExecutor, UseCaseReplyMapper,
};
use example_core::{
    PlaceSpotOrderV2CmdV3, SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3,
    SpotOrderV2UseCaseFamilyV3, SpotOrderV2UseCaseFamilyV3Error,
};
use serde::{Deserialize, Serialize};

use crate::common::{HttpInboundError, find_string_field, find_u64_field};

pub trait PlaceOrderOutboundAccess {
    type OutboundError: std::error::Error + Send + Sync + 'static;
    type Outbound: MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3, Error = Self::OutboundError>;

    fn place_order_outbound(&self) -> &Self::Outbound;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceOrderHttpRequest {
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
    pub trader_id: String,
    pub symbol: String,
    pub qty: u64,
    pub price: u64,
}

impl PlaceOrderHttpRequest {
    fn into_command(self) -> SpotOrderV2CommandV3 {
        let _adapter_symbol = self.symbol;
        let _trace_id = self.trace_id;
        let _command_id = self.command_id;
        SpotOrderV2CommandV3::Place(PlaceSpotOrderV2CmdV3 {
            party_id: self.trader_id,
            asset: 10_001,
            is_buy: true,
            price: self.price.to_string(),
            size: self.qty.to_string(),
            tif: "Gtc".to_string(),
            cloid: None,
        })
    }
}

struct PlaceOrderHttpExecutionSpec;

impl cmd_handler::command_use_case_def2::MiFamilyExecutionSpec<SpotOrderV2UseCaseFamilyV3>
    for PlaceOrderHttpExecutionSpec
{
    type Request = SpotOrderV2CommandV3;

    fn command(request: &Self::Request) -> SpotOrderV2CommandV3 {
        request.clone()
    }
}

impl crate::common::ExampleBusinessErrorMapping for SpotOrderV2UseCaseFamilyV3Error {
    fn inbound_error_code(&self) -> &'static str {
        match self {
            SpotOrderV2UseCaseFamilyV3Error::InvalidPrice => "invalid_price",
            SpotOrderV2UseCaseFamilyV3Error::InvalidSize => "invalid_qty",
            SpotOrderV2UseCaseFamilyV3Error::InvalidTimeInForce => "invalid_time_in_force",
            SpotOrderV2UseCaseFamilyV3Error::InsufficientAvailableBalance => {
                "insufficient_available_balance"
            }
            SpotOrderV2UseCaseFamilyV3Error::InsufficientFrozenBalance => {
                "insufficient_frozen_balance"
            }
            SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow => "arithmetic_overflow",
            _ => "spot_order_v2_rejected",
        }
    }

    fn http_status_code(&self) -> u16 {
        match self {
            SpotOrderV2UseCaseFamilyV3Error::ArithmeticOverflow => 500,
            _ => 400,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceOrderHttpResponse {
    pub order_id: String,
    pub reserved_quote: u64,
    pub remaining_quote: u64,
    pub domain_event_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct PlaceOrderHttpReplyMapper;

impl UseCaseReplyMapper for PlaceOrderHttpReplyMapper {
    type Reply = PlaceOrderHttpResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        PlaceOrderHttpResponse {
            order_id: find_string_field(&events, "order_id")
                .unwrap_or_else(|| "missing-order-id".to_string()),
            reserved_quote: find_u64_field(&events, "reserved_quote").unwrap_or(0),
            remaining_quote: find_u64_field(&events, "available").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_place_order_http<OB>(
    request: PlaceOrderHttpRequest,
    outbound: &OB,
) -> Result<
    PlaceOrderHttpResponse,
    MiFamilyExecutionError<SpotOrderV2UseCaseFamilyV3Error, OB::Error>,
>
where
    OB: MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>,
{
    let command = request.into_command();
    let result = MiStateMachineFamilyExecutor
        .execute::<SpotOrderV2UseCaseFamilyV3, PlaceOrderHttpExecutionSpec, OB>(
            &SpotOrderV2UseCaseFamilyV3,
            &command,
            outbound,
        )?;
    let _changes: SpotOrderV2CaseChangesV3 = result.changes;
    Ok(PlaceOrderHttpReplyMapper.map(result.events))
}

pub fn build_orders_http_router<S>() -> Router<Arc<S>>
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    Router::new().route("/orders", post(create_order::<S>))
}

pub fn build_orders_actix_scope<S>() -> Scope
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    web::scope("").route("/orders", web::post().to(create_order_actix::<S>))
}

#[inbound_adapter_support::collect_http_endpoint(
    name = "place_order_http",
    method = "POST",
    path = "/orders",
    summary = "Submit a spot order request.",
    error_response_schema = "ExampleHttpErrorResponse",
    error_codes = [
        (400, "invalid_qty"),
        (400, "invalid_price"),
        (400, "qty_below_min"),
        (400, "trading_disabled"),
        (400, "symbol_not_tradable"),
        (400, "insufficient_quote_balance"),
        (500, "arithmetic_overflow"),
        (500, "outbound_load_state_failed"),
        (500, "outbound_persist_failed"),
        (500, "outbound_replay_failed"),
        (500, "outbound_publish_failed")
    ]
)]
async fn create_order<S>(
    State(state): State<Arc<S>>,
    Json(payload): Json<PlaceOrderHttpRequest>,
) -> Result<Json<PlaceOrderHttpResponse>, HttpApiError>
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    let response = handle_place_order_http(payload, state.place_order_outbound())
        .map_err(HttpInboundError::from_mi_execution_error)?;
    Ok(Json(response))
}

async fn create_order_actix<S>(
    state: web::Data<Arc<S>>,
    payload: web::Json<PlaceOrderHttpRequest>,
) -> Result<HttpResponse, HttpApiError>
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    let response =
        handle_place_order_http(payload.into_inner(), state.get_ref().place_order_outbound())
            .map_err(HttpInboundError::from_mi_execution_error)?;
    Ok(HttpResponse::Ok().json(response))
}

type HttpApiError = HttpInboundError;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test as actix_test};
    use inbound_adapter_support::{HttpApiDescriptor, http_error};
    use serde_json::{Value, json};

    use super::*;
    use crate::common::tests::PlaceOrderTestOutbound;

    #[test]
    fn collected_http_endpoint_descriptor_matches_macro_and_signature() {
        assert_eq!(
            create_order_http_api_descriptor(),
            HttpApiDescriptor {
                name: "place_order_http",
                method: "POST",
                path: "/orders",
                summary: "Submit a spot order request.",
                request_schema_name: stringify!(PlaceOrderHttpRequest),
                success_response_schema_name: stringify!(PlaceOrderHttpResponse),
                error_response_schema_name: "ExampleHttpErrorResponse",
                error_codes: vec![
                    http_error(400, "invalid_qty"),
                    http_error(400, "invalid_price"),
                    http_error(400, "qty_below_min"),
                    http_error(400, "trading_disabled"),
                    http_error(400, "symbol_not_tradable"),
                    http_error(400, "insufficient_quote_balance"),
                    http_error(500, "arithmetic_overflow"),
                    http_error(500, "outbound_load_state_failed"),
                    http_error(500, "outbound_persist_failed"),
                    http_error(500, "outbound_replay_failed"),
                    http_error(500, "outbound_publish_failed"),
                ],
            }
        );
    }

    #[actix_web::test]
    async fn http_adapter_translates_request_and_maps_response()
    -> Result<(), Box<dyn std::error::Error>> {
        let outbound = PlaceOrderTestOutbound::default();
        let request = PlaceOrderHttpRequest {
            trace_id: Some("trace-http".to_string()),
            command_id: Some("cmd-http".to_string()),
            trader_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 3,
            price: 100,
        };

        let response =
            handle_place_order_http(request, &outbound).expect("v3 place order should execute");
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.order_id, "trader-1-BTCUSDT-11");
        assert_eq!(response.reserved_quote, 300);
        assert_eq!(response.remaining_quote, 700);
        assert_eq!(response.domain_event_count, 3);
        assert_eq!(counts, (3, 3));

        Ok(())
    }

    #[actix_web::test]
    async fn actix_http_route_translates_request_and_maps_response() {
        let outbound = Arc::new(PlaceOrderTestOutbound::default());
        let app = actix_test::init_service(
            App::new()
                .app_data(web::Data::new(outbound))
                .service(build_orders_actix_scope::<PlaceOrderTestOutbound>()),
        )
        .await;

        let request = actix_test::TestRequest::post()
            .uri("/orders")
            .set_json(json!({
                "trace_id": "trace-http",
                "command_id": "cmd-http",
                "trader_id": "trader-1",
                "symbol": "BTCUSDT",
                "qty": 3,
                "price": 100
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        let body: Value = actix_test::read_body_json(response).await;

        assert_eq!(body["order_id"], "trader-1-BTCUSDT-11");
        assert_eq!(body["reserved_quote"], 300);
        assert_eq!(body["remaining_quote"], 700);
        assert_eq!(body["domain_event_count"], 3);
    }
}
