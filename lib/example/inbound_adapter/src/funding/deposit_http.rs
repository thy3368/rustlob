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
use example_core::{DepositQuoteCmd, DepositQuoteError, DepositQuoteState};
use serde::{Deserialize, Serialize};

use crate::common::{
    HttpInboundError, execute_deposit_quote_with_mapper, find_string_field, find_u64_field,
};

pub trait DepositQuoteOutboundAccess {
    type OutboundError: std::error::Error + Send + Sync + 'static;
    type Outbound: CommandUseCaseOutbound<
            Command = DepositQuoteCmd,
            State = DepositQuoteState,
            Error = Self::OutboundError,
        >;

    fn deposit_quote_outbound(&self) -> &Self::Outbound;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositQuoteHttpRequest {
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
    pub trader_id: String,
    pub amount: u64,
}

impl DepositQuoteHttpRequest {
    fn into_envelope(self) -> CommandEnvelope<DepositQuoteCmd> {
        CommandEnvelope {
            meta: CommandMeta { trace_id: self.trace_id, command_id: self.command_id },
            command: DepositQuoteCmd { party_id: self.trader_id, amount: self.amount },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositQuoteHttpResponse {
    pub account_id: String,
    pub available_quote: u64,
    pub frozen_quote: u64,
    pub domain_event_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct DepositQuoteHttpReplyMapper;

impl UseCaseReplyMapper for DepositQuoteHttpReplyMapper {
    type Reply = DepositQuoteHttpResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        DepositQuoteHttpResponse {
            account_id: find_string_field(&events, "account_id")
                .unwrap_or_else(|| "missing-account-id".to_string()),
            available_quote: find_u64_field(&events, "available_quote").unwrap_or(0),
            frozen_quote: find_u64_field(&events, "frozen_quote").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_deposit_quote_http<OB>(
    request: DepositQuoteHttpRequest,
    outbound: &OB,
) -> Result<DepositQuoteHttpResponse, CommandUseCaseExecutionError<DepositQuoteError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = DepositQuoteCmd, State = DepositQuoteState>,
    OB::Error: 'static,
{
    execute_deposit_quote_with_mapper(
        request.into_envelope(),
        outbound,
        &DepositQuoteHttpReplyMapper,
    )
}

pub fn build_deposit_http_router<S>() -> Router<Arc<S>>
where
    S: Send + Sync + 'static + DepositQuoteOutboundAccess,
{
    Router::new().route("/deposits/quote", post(create_deposit::<S>))
}

pub fn build_deposit_actix_scope<S>() -> Scope
where
    S: Send + Sync + 'static + DepositQuoteOutboundAccess,
{
    web::scope("").route("/deposits/quote", web::post().to(create_deposit_actix::<S>))
}

async fn create_deposit<S>(
    State(state): State<Arc<S>>,
    Json(payload): Json<DepositQuoteHttpRequest>,
) -> Result<Json<DepositQuoteHttpResponse>, DepositHttpApiError>
where
    S: Send + Sync + 'static + DepositQuoteOutboundAccess,
{
    let response = handle_deposit_quote_http(payload, state.deposit_quote_outbound())?;
    Ok(Json(response))
}

async fn create_deposit_actix<S>(
    state: web::Data<Arc<S>>,
    payload: web::Json<DepositQuoteHttpRequest>,
) -> Result<HttpResponse, DepositHttpApiError>
where
    S: Send + Sync + 'static + DepositQuoteOutboundAccess,
{
    let response =
        handle_deposit_quote_http(payload.into_inner(), state.get_ref().deposit_quote_outbound())?;
    Ok(HttpResponse::Ok().json(response))
}

type DepositHttpApiError = HttpInboundError;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{App, test};
    use serde_json::{Value, json};

    use super::*;
    use crate::common::tests::DepositQuoteTestOutbound;

    #[actix_web::test]
    async fn http_adapter_translates_deposit_request_and_maps_response()
    -> Result<(), Box<dyn std::error::Error>> {
        let outbound = DepositQuoteTestOutbound::default();
        let request = DepositQuoteHttpRequest {
            trace_id: Some("trace-deposit".to_string()),
            command_id: Some("cmd-deposit".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        };

        let response = handle_deposit_quote_http(request, &outbound)?;
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 1_250);
        assert_eq!(response.frozen_quote, 0);
        assert_eq!(response.domain_event_count, 1);
        assert_eq!(counts, (1, 1));

        Ok(())
    }

    #[actix_web::test]
    async fn actix_http_route_translates_deposit_request_and_maps_response() {
        let outbound = Arc::new(DepositQuoteTestOutbound::default());
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(outbound))
                .service(build_deposit_actix_scope::<DepositQuoteTestOutbound>()),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/deposits/quote")
            .set_json(json!({
                "trace_id": "trace-deposit",
                "command_id": "cmd-deposit",
                "trader_id": "trader-1",
                "amount": 250
            }))
            .to_request();

        let response = test::call_service(&app, request).await;
        let body: Value = test::read_body_json(response).await;

        assert_eq!(body["account_id"], "trader-1");
        assert_eq!(body["available_quote"], 1250);
        assert_eq!(body["frozen_quote"], 0);
        assert_eq!(body["domain_event_count"], 1);
    }
}
