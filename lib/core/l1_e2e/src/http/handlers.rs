use axum::{Json, extract::State, http::StatusCode};

use crate::http::dto::{ErrorResponse, ExecuteBlockRequest, ExecuteBlockResponse, SubmitTransactionsRequest, SubmitTransactionsResponse};
use crate::service::AppState;

pub async fn health(State(state): State<AppState>) -> Json<crate::service::HealthResponse> {
    Json(state.service.health())
}

pub async fn submit_transactions(
    State(state): State<AppState>,
    Json(request): Json<SubmitTransactionsRequest>,
) -> Result<(StatusCode, Json<SubmitTransactionsResponse>), (StatusCode, Json<ErrorResponse>)> {
    state
        .service
        .submit_transactions(request)
        .map(|reply| {
            (
                StatusCode::OK,
                Json(SubmitTransactionsResponse {
                    admitted_count: reply.admitted_count,
                    rejected_count: reply.rejected_count,
                }),
            )
        })
        .map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("{error:?}"),
                }),
            )
        })
}

pub async fn execute_block(
    State(state): State<AppState>,
    Json(request): Json<ExecuteBlockRequest>,
) -> Result<(StatusCode, Json<ExecuteBlockResponse>), (StatusCode, Json<ErrorResponse>)> {
    state
        .service
        .execute_block(request)
        .map(|reply| {
            (
                StatusCode::OK,
                Json(ExecuteBlockResponse {
                    block_height: reply.block_height,
                    block_event_count: reply.block_event_count,
                    node_state_update_count: reply.node_state_update_count,
                }),
            )
        })
        .map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("{error:?}"),
                }),
            )
        })
}
