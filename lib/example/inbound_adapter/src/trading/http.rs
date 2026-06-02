use std::sync::Arc;

use actix_web::{HttpResponse, Scope, web};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseExecutionError, CommandUseCaseOutbound,
    UseCaseReplyMapper,
};
use example_core::{PlaceOrderCmd, PlaceOrderError, PlaceOrderState};
use serde::{Deserialize, Serialize};

use crate::common::{
    HttpInboundError, execute_place_order_with_mapper, find_string_field, find_u64_field,
};

pub trait PlaceOrderOutboundAccess {
    type OutboundError: std::error::Error + Send + Sync + 'static;
    type Outbound: CommandUseCaseOutbound<
            Command = PlaceOrderCmd,
            State = PlaceOrderState,
            Error = Self::OutboundError,
        >;

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
    fn into_envelope(self) -> CommandEnvelope<PlaceOrderCmd> {
        CommandEnvelope {
            meta: CommandMeta { trace_id: self.trace_id, command_id: self.command_id },
            command: PlaceOrderCmd {
                party_id: self.trader_id,
                symbol: self.symbol,
                qty: self.qty,
                price: self.price,
            },
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
            remaining_quote: find_u64_field(&events, "available_quote").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_place_order_http<OB>(
    request: PlaceOrderHttpRequest,
    outbound: &OB,
) -> Result<PlaceOrderHttpResponse, CommandUseCaseExecutionError<PlaceOrderError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = PlaceOrderCmd, State = PlaceOrderState>,
    OB::Error: 'static,
{
    execute_place_order_with_mapper(request.into_envelope(), outbound, &PlaceOrderHttpReplyMapper)
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
    let response = handle_place_order_http(payload, state.place_order_outbound())?;
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
        handle_place_order_http(payload.into_inner(), state.get_ref().place_order_outbound())?;
    Ok(HttpResponse::Ok().json(response))
}

type HttpApiError = HttpInboundError;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test};
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

        let response = handle_place_order_http(request, &outbound)?;
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.order_id, "trader-1-BTCUSDT-11");
        assert_eq!(response.reserved_quote, 300);
        assert_eq!(response.remaining_quote, 700);
        assert_eq!(response.domain_event_count, 2);
        assert_eq!(counts, (2, 2));

        Ok(())
    }

    #[actix_web::test]
    async fn actix_http_route_translates_request_and_maps_response() {
        let outbound = Arc::new(PlaceOrderTestOutbound::default());
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(outbound))
                .service(build_orders_actix_scope::<PlaceOrderTestOutbound>()),
        )
        .await;

        let request = test::TestRequest::post()
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

        let response = test::call_service(&app, request).await;
        let body: Value = test::read_body_json(response).await;

        assert_eq!(body["order_id"], "trader-1-BTCUSDT-11");
        assert_eq!(body["reserved_quote"], 300);
        assert_eq!(body["remaining_quote"], 700);
        assert_eq!(body["domain_event_count"], 2);
    }
}
