use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use example_composition_root::InMemoryExampleApplication;
use example_core::PlaceOrderError;
use example_inbound_adapter::{
    build_deposit_http_router, build_orders_http_router, build_withdraw_http_router,
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("http_demo failed: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(InMemoryExampleApplication::new_in_memory()?);
    let bind_addr =
        std::env::var("HTTP_DEMO_ADDR").unwrap_or_else(|_| "127.0.0.1:3001".to_string());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str()).await?;

    let router = build_orders_http_router::<InMemoryExampleApplication>()
        .merge(build_deposit_http_router::<InMemoryExampleApplication>())
        .merge(build_withdraw_http_router::<InMemoryExampleApplication>())
        .route("/snapshot", get(get_snapshot))
        .with_state(app);

    println!("http_demo listening on http://{bind_addr}");
    println!("POST /orders");
    println!("POST /deposits/quote");
    println!("POST /withdrawals/quote");
    println!("GET  /snapshot");

    axum::serve(listener, router).await?;
    Ok(())
}

async fn get_snapshot(
    State(app): State<Arc<InMemoryExampleApplication>>,
) -> Result<Json<Value>, SnapshotApiError> {
    let snapshot = app.snapshot().map_err(SnapshotApiError::from)?;

    let accounts = snapshot
        .accounts
        .into_iter()
        .map(|(account_id, account)| {
            (
                account_id,
                json!({
                    "account_id": account.account_id,
                    "available_quote": account.available_quote,
                    "frozen_quote": account.frozen_quote,
                    "version": account.version
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    let orders = snapshot
        .orders
        .into_iter()
        .map(|(order_id, order)| {
            (
                order_id,
                json!({
                    "order_id": order.order_id,
                    "account_id": order.account_id,
                    "symbol": order.symbol,
                    "qty": order.qty,
                    "price": order.price,
                    "reserved_quote": order.reserved_quote
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    Ok(Json(json!({
        "accounts": accounts,
        "orders": orders,
        "persisted_event_count": snapshot.persisted_event_count,
        "published_event_count": snapshot.published_event_count,
        "next_order_sequence": snapshot.next_order_sequence
    })))
}

#[derive(Debug)]
struct SnapshotApiError {
    message: String,
}

impl From<PlaceOrderError> for SnapshotApiError {
    fn from(error: PlaceOrderError) -> Self {
        Self { message: error.to_string() }
    }
}

impl IntoResponse for SnapshotApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": {
                    "code": "snapshot_unavailable",
                    "message": self.message
                }
            })),
        )
            .into_response()
    }
}
