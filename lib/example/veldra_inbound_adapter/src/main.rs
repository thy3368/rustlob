use std::io;

use actix_web::{App, HttpServer};
use example_veldra_inbound_adapter::command::exchange::http::build_exchange_scope;
use example_veldra_inbound_adapter::command::info::http::build_info_scope;
use example_veldra_inbound_adapter::on_time::timer::run_timer;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind_address = std::env::var("EXAMPLE_VELDRA_INBOUND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    tokio::spawn(async {
        if let Err(error) = run_timer().await {
            eprintln!("timer task stopped: {error}");
        }
    });

    HttpServer::new(|| App::new().service(build_exchange_scope()).service(build_info_scope()))
        .bind(bind_address)?
        .run()
        .await
}
