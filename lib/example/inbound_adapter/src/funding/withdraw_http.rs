use std::sync::Arc;

use actix_web::{HttpResponse, Scope, web};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseExecutionError, CommandUseCaseOutbound,
    UseCaseReplyMapper,
};
use example_core::{WithdrawQuoteCmd, WithdrawQuoteError, WithdrawQuoteState};
use serde::{Deserialize, Serialize};

use crate::common::{
    HttpInboundError, execute_withdraw_quote_with_mapper, find_string_field, find_u64_field,
};

pub trait WithdrawQuoteOutboundAccess {
    type OutboundError: std::error::Error + Send + Sync + 'static;
    type Outbound: CommandUseCaseOutbound<
            Command = WithdrawQuoteCmd,
            State = WithdrawQuoteState,
            Error = Self::OutboundError,
        >;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            available_quote: find_u64_field(&events, "available").unwrap_or(0),
            frozen_quote: find_u64_field(&events, "frozen").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_withdraw_quote_http<OB>(
    request: WithdrawQuoteHttpRequest,
    outbound: &OB,
) -> Result<WithdrawQuoteHttpResponse, CommandUseCaseExecutionError<WithdrawQuoteError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = WithdrawQuoteCmd, State = WithdrawQuoteState>,
    OB::Error: 'static,
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

#[inbound_adapter_support::collect_http_endpoint(
    name = "withdraw_quote_http",
    method = "POST",
    path = "/withdrawals/quote",
    summary = "Quote a withdrawal against the trading account.",
    error_response_schema = "ExampleHttpErrorResponse",
    error_codes = [
        (400, "invalid_amount"),
        (400, "insufficient_quote_balance"),
        (500, "arithmetic_overflow"),
        (500, "outbound_load_state_failed"),
        (500, "outbound_persist_failed"),
        (500, "outbound_replay_failed"),
        (500, "outbound_publish_failed")
    ]
)]
async fn create_withdraw<S>(
    State(state): State<Arc<S>>,
    Json(payload): Json<WithdrawQuoteHttpRequest>,
) -> Result<Json<WithdrawQuoteHttpResponse>, WithdrawHttpApiError>
where
    S: Send + Sync + 'static + WithdrawQuoteOutboundAccess,
{
    let response = handle_withdraw_quote_http(payload, state.withdraw_quote_outbound())?;
    Ok(Json(response))
}

async fn create_withdraw_actix<S>(
    state: web::Data<Arc<S>>,
    payload: web::Json<WithdrawQuoteHttpRequest>,
) -> Result<HttpResponse, WithdrawHttpApiError>
where
    S: Send + Sync + 'static + WithdrawQuoteOutboundAccess,
{
    let response = handle_withdraw_quote_http(
        payload.into_inner(),
        state.get_ref().withdraw_quote_outbound(),
    )?;
    Ok(HttpResponse::Ok().json(response))
}

type WithdrawHttpApiError = HttpInboundError;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test};
    use serde_json::{Value, json};

    use super::*;
    use crate::common::tests::WithdrawQuoteTestOutbound;

    #[actix_web::test]
    async fn http_adapter_translates_withdraw_request_and_maps_response()
    -> Result<(), Box<dyn std::error::Error>> {
        let outbound = WithdrawQuoteTestOutbound::default();
        let request = WithdrawQuoteHttpRequest {
            trace_id: Some("trace-withdraw".to_string()),
            command_id: Some("cmd-withdraw".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        };

        let response = handle_withdraw_quote_http(request, &outbound)?;
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 750);
        assert_eq!(response.frozen_quote, 0);
        assert_eq!(response.domain_event_count, 1);
        assert_eq!(counts, (1, 1));

        Ok(())
    }

    #[actix_web::test]
    async fn actix_http_route_translates_withdraw_request_and_maps_response() {
        let outbound = Arc::new(WithdrawQuoteTestOutbound::default());
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(outbound))
                .service(build_withdraw_actix_scope::<WithdrawQuoteTestOutbound>()),
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
