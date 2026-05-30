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
use example_core::{WithdrawQuoteCmd, WithdrawQuoteError, WithdrawQuoteState};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::common::{execute_withdraw_quote_with_mapper, find_string_field, find_u64_field};

pub trait WithdrawQuoteOutboundAccess {
    type Outbound: CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound;
}

impl<T> WithdrawQuoteOutboundAccess for T
where
    T: CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>,
{
    type Outbound = Self;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawQuoteHttpRequest {
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
    pub trader_id: String,
    pub amount: u64,
}

impl WithdrawQuoteHttpRequest {
    fn into_envelope(self) -> CommandEnvelope<WithdrawQuoteCmd> {
        CommandEnvelope {
            meta: CommandMeta { trace_id: self.trace_id, command_id: self.command_id },
            command: WithdrawQuoteCmd { party_id: self.trader_id, amount: self.amount },
        }
    }
}

#[derive(Debug, Deserialize)]
struct WithdrawQuoteHttpPayload {
    trace_id: Option<String>,
    command_id: Option<String>,
    trader_id: String,
    amount: u64,
}

impl From<WithdrawQuoteHttpPayload> for WithdrawQuoteHttpRequest {
    fn from(value: WithdrawQuoteHttpPayload) -> Self {
        Self {
            trace_id: value.trace_id,
            command_id: value.command_id,
            trader_id: value.trader_id,
            amount: value.amount,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawQuoteHttpResponse {
    pub account_id: String,
    pub available_quote: u64,
    pub frozen_quote: u64,
    pub domain_event_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct WithdrawQuoteHttpReplyMapper;

impl UseCaseReplyMapper for WithdrawQuoteHttpReplyMapper {
    type Reply = WithdrawQuoteHttpResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        WithdrawQuoteHttpResponse {
            account_id: find_string_field(&events, "account_id")
                .unwrap_or_else(|| "missing-account-id".to_string()),
            available_quote: find_u64_field(&events, "available_quote").unwrap_or(0),
            frozen_quote: find_u64_field(&events, "frozen_quote").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_withdraw_quote_http<OB>(
    request: WithdrawQuoteHttpRequest,
    outbound: &OB,
) -> Result<WithdrawQuoteHttpResponse, WithdrawQuoteError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>,
{
    execute_withdraw_quote_with_mapper(
        request.into_envelope(),
        outbound,
        &WithdrawQuoteHttpReplyMapper,
    )
}

pub fn build_withdraw_http_router<S>() -> Router<Arc<S>>
where
    S: Send + Sync + 'static + WithdrawQuoteOutboundAccess,
{
    Router::new().route("/withdrawals/quote", post(create_withdraw::<S>))
}

pub fn build_withdraw_actix_scope<S>() -> Scope
where
    S: Send + Sync + 'static + WithdrawQuoteOutboundAccess,
{
    web::scope("").route("/withdrawals/quote", web::post().to(create_withdraw_actix::<S>))
}

async fn create_withdraw<S>(
    State(state): State<Arc<S>>,
    Json(payload): Json<WithdrawQuoteHttpPayload>,
) -> Result<Json<Value>, WithdrawHttpApiError>
where
    S: Send + Sync + 'static + WithdrawQuoteOutboundAccess,
{
    let response = handle_withdraw_quote_http(payload.into(), state.withdraw_quote_outbound())?;

    Ok(Json(json!({
        "account_id": response.account_id,
        "available_quote": response.available_quote,
        "frozen_quote": response.frozen_quote,
        "domain_event_count": response.domain_event_count
    })))
}

async fn create_withdraw_actix<S>(
    state: web::Data<Arc<S>>,
    payload: web::Json<WithdrawQuoteHttpPayload>,
) -> Result<HttpResponse, WithdrawHttpApiError>
where
    S: Send + Sync + 'static + WithdrawQuoteOutboundAccess,
{
    let response = handle_withdraw_quote_http(
        payload.into_inner().into(),
        state.get_ref().withdraw_quote_outbound(),
    )?;

    Ok(HttpResponse::Ok().json(json!({
        "account_id": response.account_id,
        "available_quote": response.available_quote,
        "frozen_quote": response.frozen_quote,
        "domain_event_count": response.domain_event_count
    })))
}

#[derive(Debug)]
struct WithdrawHttpApiError {
    status_code: u16,
    code: &'static str,
    message: String,
}

impl std::fmt::Display for WithdrawHttpApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl From<WithdrawQuoteError> for WithdrawHttpApiError {
    fn from(error: WithdrawQuoteError) -> Self {
        let (status_code, code) = match error {
            WithdrawQuoteError::InvalidAmount => (400, "invalid_amount"),
            WithdrawQuoteError::InsufficientQuoteBalance => (400, "insufficient_quote_balance"),
            WithdrawQuoteError::AccountNotFound => (404, "account_not_found"),
            WithdrawQuoteError::ArithmeticOverflow => (500, "arithmetic_overflow"),
            WithdrawQuoteError::EventDecodeFailed => (500, "event_decode_failed"),
            WithdrawQuoteError::StoreUnavailable => (500, "store_unavailable"),
        };

        Self { status_code, code, message: error.to_string() }
    }
}

impl IntoResponse for WithdrawHttpApiError {
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

impl ResponseError for WithdrawHttpApiError {
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
    use example_core::WithdrawQuoteError;
    use serde_json::{Value, json};

    use super::*;
    use crate::common::tests::TestOutbound;

    #[actix_web::test]
    async fn http_adapter_translates_withdraw_request_and_maps_response()
    -> Result<(), WithdrawQuoteError> {
        let outbound = TestOutbound::default();
        let request = WithdrawQuoteHttpRequest {
            trace_id: Some("trace-withdraw".to_string()),
            command_id: Some("cmd-withdraw".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        };

        let response = handle_withdraw_quote_http(request, &outbound)?;
        let counts =
            outbound.snapshot_event_counts().map_err(|_| WithdrawQuoteError::StoreUnavailable)?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 750);
        assert_eq!(response.frozen_quote, 0);
        assert_eq!(response.domain_event_count, 1);
        assert_eq!(counts, (1, 1));

        Ok(())
    }

    #[actix_web::test]
    async fn actix_http_route_translates_withdraw_request_and_maps_response() {
        let outbound = Arc::new(TestOutbound::default());
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(outbound))
                .service(build_withdraw_actix_scope::<TestOutbound>()),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/withdrawals/quote")
            .set_json(json!({
                "trace_id": "trace-withdraw",
                "command_id": "cmd-withdraw",
                "trader_id": "trader-1",
                "amount": 250
            }))
            .to_request();

        let response = test::call_service(&app, request).await;
        let body: Value = test::read_body_json(response).await;

        assert_eq!(body["account_id"], "trader-1");
        assert_eq!(body["available_quote"], 750);
        assert_eq!(body["frozen_quote"], 0);
        assert_eq!(body["domain_event_count"], 1);
    }
}
