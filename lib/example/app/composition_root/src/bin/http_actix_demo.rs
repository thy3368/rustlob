use std::sync::Arc;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use example_composition_root::InMemoryExampleApplication;
use example_inbound_adapter::{
    build_deposit_actix_scope, build_orders_actix_scope, build_withdraw_actix_scope,
};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}

async fn run() -> std::io::Result<()> {
    let app = Arc::new(
        InMemoryExampleApplication::new_in_memory()
            .map_err(|error| std::io::Error::other(error.to_string()))?,
    );

    let bind_addr =
        std::env::var("HTTP_ACTIX_DEMO_ADDR").unwrap_or_else(|_| "127.0.0.1:3002".to_string());

    println!("http_actix_demo listening on http://{bind_addr}");
    println!("POST /orders");
    println!("POST /deposits/quote");
    println!("POST /withdrawals/quote");
    println!("GET  /snapshot");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app.clone()))
            .service(build_orders_actix_scope::<InMemoryExampleApplication>())
            .service(build_deposit_actix_scope::<InMemoryExampleApplication>())
            .service(build_withdraw_actix_scope::<InMemoryExampleApplication>())
            .route("/snapshot", web::get().to(get_snapshot))
    })
    .bind(bind_addr)?
    .run()
    .await
}

async fn get_snapshot(app: web::Data<Arc<InMemoryExampleApplication>>) -> impl Responder {
    match app.snapshot() {
        Ok(snapshot) => HttpResponse::Ok().json(json!({
            "accounts": snapshot.accounts.into_iter().map(|(account_id, account)| (
                account_id,
                json!({
                    "account_id": account.account_id,
                    "available_quote": account.available_quote,
                    "frozen_quote": account.frozen_quote,
                    "version": account.version
                })
            )).collect::<serde_json::Map<String, serde_json::Value>>(),
            "orders": snapshot.orders.into_iter().map(|(order_id, order)| (
                order_id,
                json!({
                    "order_id": order.order_id,
                    "account_id": order.account_id,
                    "symbol": order.symbol,
                    "qty": order.qty,
                    "price": order.price,
                    "reserved_quote": order.reserved_quote
                })
            )).collect::<serde_json::Map<String, serde_json::Value>>(),
            "persisted_event_count": snapshot.persisted_event_count,
            "published_event_count": snapshot.published_event_count,
            "next_order_sequence": snapshot.next_order_sequence
        })),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": {
                "code": "snapshot_unavailable",
                "message": error.to_string()
            }
        })),
    }
}
