use std::sync::Arc;

use actix_web::{HttpResponse, Scope, web};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use common_entity::{
    EntityReplayableEvent, ExecutionError, LogEntry, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    PlaceMatchSpotOrderV2Error, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
};
use serde::{Deserialize, Serialize};

use crate::common::{HttpInboundError, InboundReplyMapper, find_last_u64_field, find_u64_field};

pub trait PlaceOrderOutboundAccess {
    type OutboundError: std::error::Error + Send + Sync + 'static;
    type Outbound: StateSource<PlaceMatchSpotOrderV2UseCase, Error = Self::OutboundError>
        + StateSink<PlaceMatchSpotOrderV2UseCase, Error = Self::OutboundError>;

    fn place_order_outbound(&self) -> &Self::Outbound;
    fn command_metadata(&self) -> (u64, u64, u64, u64);
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
    fn into_command(
        self,
        lsn: u64,
        logical_ts: u64,
        client_id: u64,
        request_id: u64,
    ) -> LogEntry<PlaceOnlySpotOrderV2Cmd> {
        let _trace_id = self.trace_id;
        let _command_id = self.command_id;
        let payload = PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
            order_id: 11,
            party_id: self.trader_id,
            asset: 10_001,
            symbol: self.symbol,
            is_buy: true,
            price: self.price.to_string(),
            size: self.qty.to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "Gtc".to_string() },
            reduce_only: false,
            cloid: None,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        });
        common_entity::LogEntrySequencer::from_metadata(
            lsn, logical_ts, client_id, request_id, payload,
        )
    }
}

impl crate::common::ExampleBusinessErrorMapping for PlaceMatchSpotOrderV2Error {
    fn inbound_error_code(&self) -> &'static str {
        "spot_order_v2_rejected"
    }

    fn http_status_code(&self) -> u16 {
        400
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceOrderHttpResponse {
    pub order_id: u64,
    pub principal_reservation_amount: u64,
    pub remaining_quote: u64,
    pub domain_event_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct PlaceOrderHttpReplyMapper;

impl crate::common::InboundReplyMapper for PlaceOrderHttpReplyMapper {
    type Reply = PlaceOrderHttpResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        PlaceOrderHttpResponse {
            order_id: find_u64_field(&events, "order_id").unwrap_or(0),
            principal_reservation_amount: find_last_u64_field(
                &events,
                "reservation_original_amount",
            )
            .unwrap_or(0),
            remaining_quote: find_u64_field(&events, "available").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_place_order_http<OB>(
    request: PlaceOrderHttpRequest,
    lsn: u64,
    logical_ts: u64,
    client_id: u64,
    request_id: u64,
    outbound: &OB,
) -> Result<
    PlaceOrderHttpResponse,
    ExecutionError<
        PlaceMatchSpotOrderV2Error,
        <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            PlaceMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<PlaceMatchSpotOrderV2UseCase>,
{
    let command = request.into_command(lsn, logical_ts, client_id, request_id);
    let result = StateMachineExecutor.execute::<PlaceMatchSpotOrderV2UseCase, OB, OB>(
        &PlaceMatchSpotOrderV2UseCase,
        &command,
        outbound,
        outbound,
    )?;
    let _changes = result.changes;
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

async fn create_order<S>(
    State(state): State<Arc<S>>,
    Json(payload): Json<PlaceOrderHttpRequest>,
) -> Result<Json<PlaceOrderHttpResponse>, HttpApiError>
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    let (lsn, logical_ts, client_id, request_id) = state.command_metadata();
    let response = handle_place_order_http(
        payload,
        lsn,
        logical_ts,
        client_id,
        request_id,
        state.place_order_outbound(),
    )
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
    let (lsn, logical_ts, client_id, request_id) = state.command_metadata();
    let response = handle_place_order_http(
        payload.into_inner(),
        lsn,
        logical_ts,
        client_id,
        request_id,
        state.get_ref().place_order_outbound(),
    )
    .map_err(HttpInboundError::from_mi_execution_error)?;
    Ok(HttpResponse::Ok().json(response))
}

type HttpApiError = HttpInboundError;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test as actix_test};
    use serde_json::{Value, json};

    use super::*;
    use crate::common::tests::PlaceOrderTestOutbound;

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

        let response = handle_place_order_http(request, 1, 1_000_000_000, 1, 1, &outbound)
            .expect("place-match order should execute");
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.order_id, 11);
        assert_eq!(response.principal_reservation_amount, 300);
        assert_eq!(response.remaining_quote, 700);
        assert_eq!(response.domain_event_count, 6);
        assert_eq!(counts, (6, 6));

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

        assert_eq!(body["order_id"], 11);
        assert_eq!(body["principal_reservation_amount"], 300);
        assert_eq!(body["remaining_quote"], 700);
        assert_eq!(body["domain_event_count"], 6);
    }
}
