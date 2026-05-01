use axum::{Router, routing::{get, post}};

use crate::service::AppState;

pub mod dto;
pub mod handlers;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/l1/health", get(handlers::health))
        .route("/api/l1/transactions", post(handlers::submit_transactions))
        .route("/api/l1/blocks/execute", post(handlers::execute_block))
        .with_state(state)
}
