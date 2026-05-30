use std::sync::Arc;

use actix_web::{HttpResponse, ResponseError, Scope, web};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseOutbound, UseCaseReplyMapper,
};
use example_core::{PlaceOrderCmd, PlaceOrderError, PlaceOrderState};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::common::{execute_place_order_with_mapper, find_string_field, find_u64_field};

pub trait PlaceOrderOutboundAccess {
    type Outbound: CommandUseCaseOutbound<PlaceOrderCmd, PlaceOrderState, PlaceOrderError>;

    fn place_order_outbound(&self) -> &Self::Outbound;
}

impl<T> PlaceOrderOutboundAccess for T
where
    T: CommandUseCaseOutbound<PlaceOrderCmd, PlaceOrderState, PlaceOrderError>,
{
    type Outbound = Self;

    fn place_order_outbound(&self) -> &Self::Outbound {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Deserialize)]
struct CreateOrderHttpPayload {
    trace_id: Option<String>,
    command_id: Option<String>,
    trader_id: String,
    symbol: String,
    qty: u64,
    price: u64,
}

impl From<CreateOrderHttpPayload> for PlaceOrderHttpRequest {
    fn from(value: CreateOrderHttpPayload) -> Self {
        Self {
            trace_id: value.trace_id,
            command_id: value.command_id,
            trader_id: value.trader_id,
            symbol: value.symbol,
            qty: value.qty,
            price: value.price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
) -> Result<PlaceOrderHttpResponse, PlaceOrderError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<PlaceOrderCmd, PlaceOrderState, PlaceOrderError>,
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
    Json(payload): Json<CreateOrderHttpPayload>,
) -> Result<Json<Value>, HttpApiError>
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    let response = handle_place_order_http(payload.into(), state.place_order_outbound())?;

    Ok(Json(json!({
        "order_id": response.order_id,
        "reserved_quote": response.reserved_quote,
        "remaining_quote": response.remaining_quote,
        "domain_event_count": response.domain_event_count
    })))
}

async fn create_order_actix<S>(
    state: web::Data<Arc<S>>,
    payload: web::Json<CreateOrderHttpPayload>,
) -> Result<HttpResponse, HttpApiError>
where
    S: Send + Sync + 'static + PlaceOrderOutboundAccess,
{
    let response = handle_place_order_http(
        payload.into_inner().into(),
        state.get_ref().place_order_outbound(),
    )?;

    Ok(HttpResponse::Ok().json(json!({
        "order_id": response.order_id,
        "reserved_quote": response.reserved_quote,
        "remaining_quote": response.remaining_quote,
        "domain_event_count": response.domain_event_count
    })))
}

#[derive(Debug)]
struct HttpApiError {
    status_code: u16,
    code: &'static str,
    message: String,
}

impl std::fmt::Display for HttpApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl From<PlaceOrderError> for HttpApiError {
    fn from(error: PlaceOrderError) -> Self {
        let (status_code, code) = match error {
            PlaceOrderError::InvalidQty => (400, "invalid_qty"),
            PlaceOrderError::InvalidPrice => (400, "invalid_price"),
            PlaceOrderError::QtyBelowMin => (400, "qty_below_min"),
            PlaceOrderError::TradingDisabled => (400, "trading_disabled"),
            PlaceOrderError::SymbolNotTradable => (400, "symbol_not_tradable"),
            PlaceOrderError::InsufficientQuoteBalance => (400, "insufficient_quote_balance"),
            PlaceOrderError::AccountNotFound => (404, "account_not_found"),
            PlaceOrderError::MarketRulesNotFound => (404, "market_rules_not_found"),
            PlaceOrderError::ArithmeticOverflow => (500, "arithmetic_overflow"),
            PlaceOrderError::EventDecodeFailed => (500, "event_decode_failed"),
            PlaceOrderError::StoreUnavailable => (500, "store_unavailable"),
        };

        Self { status_code, code, message: error.to_string() }
    }
}

impl IntoResponse for HttpApiError {
    fn into_response(self) -> Response {
        let status = axum::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        (
            status,
            Json(json!({
                "error": {
                    "code": self.code,
                    "message": self.message
                }
            })),
        )
            .into_response()
    }
}

impl ResponseError for HttpApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.status_code)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "error": {
                "code": self.code,
                "message": self.message
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test};
    use example_core::PlaceOrderError;
    use serde_json::{Value, json};

    use super::*;
    use crate::common::tests::TestOutbound;

    #[actix_web::test]
    async fn http_adapter_translates_request_and_maps_response() -> Result<(), PlaceOrderError> {
        let outbound = TestOutbound::default();
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
        let outbound = Arc::new(TestOutbound::default());
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(outbound))
                .service(build_orders_actix_scope::<TestOutbound>()),
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
