use std::sync::Arc;

use actix_web::{HttpResponse, Scope, web};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use common_entity::{
    EntityReplayableEvent, ExecutionError, LogEntry, LogEntrySequencer, StateSink, StateSource,
};
use example_core_use_case::{WithdrawQuoteCmd, WithdrawQuoteError, WithdrawQuoteUseCase};
use serde::{Deserialize, Serialize};

use crate::common::{
    HttpInboundError, InboundReplyMapper, execute_withdraw_quote_with_mapper, find_u64_field,
};

pub trait WithdrawQuoteOutboundAccess {
    type OutboundError: std::error::Error + Send + Sync + 'static;
    type Outbound: StateSource<WithdrawQuoteUseCase, Error = Self::OutboundError>
        + StateSink<WithdrawQuoteUseCase, Error = Self::OutboundError>;

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
    fn into_log_entry(self) -> LogEntry<WithdrawQuoteCmd> {
        LogEntrySequencer::from_metadata(
            1,
            1,
            1,
            1,
            WithdrawQuoteCmd { party_id: self.trader_id, amount: self.amount },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawQuoteHttpResponse {
    pub account_id: String,
    pub available_quote: u64,
    pub frozen_quote: u64,
    pub domain_event_count: usize,
}

#[derive(Debug, Clone)]
struct WithdrawQuoteHttpReplyMapper {
    account_id: String,
}

impl InboundReplyMapper for WithdrawQuoteHttpReplyMapper {
    type Reply = WithdrawQuoteHttpResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        WithdrawQuoteHttpResponse {
            account_id: self.account_id.clone(),
            available_quote: find_u64_field(&events, "available").unwrap_or(0),
            frozen_quote: find_u64_field(&events, "frozen").unwrap_or(0),
            domain_event_count: events.len(),
        }
    }
}

pub fn handle_withdraw_quote_http<OB>(
    request: WithdrawQuoteHttpRequest,
    outbound: &OB,
) -> Result<
    WithdrawQuoteHttpResponse,
    ExecutionError<WithdrawQuoteError, <OB as StateSource<WithdrawQuoteUseCase>>::Error>,
>
where
    OB: Send
        + Sync
        + StateSource<WithdrawQuoteUseCase>
        + StateSink<WithdrawQuoteUseCase, Error = <OB as StateSource<WithdrawQuoteUseCase>>::Error>,
    <OB as StateSource<WithdrawQuoteUseCase>>::Error: 'static,
{
    let mapper = WithdrawQuoteHttpReplyMapper { account_id: request.trader_id.clone() };
    execute_withdraw_quote_with_mapper(request.into_log_entry(), outbound, &mapper)
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
    use crate::common::HttpInboundError;
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

        let response = handle_withdraw_quote_http(request, &outbound)
            .map_err(HttpInboundError::from_execution_error)?;
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
