use std::io;

use actix_web::{App, HttpServer};
use example_veldra_inbound_adapter::command::exchange::http::build_exchange_scope;
use example_veldra_inbound_adapter::command::info::http::build_info_scope;
use example_veldra_inbound_adapter::on_event::spot::on_aeron_match_spot_order::{
    AeronListenerConfig, run_aeron_listener, run_match_spot_order_aeron,
};
use example_veldra_inbound_adapter::on_time::spot::timer::run_timer;
use use_case_executor::trading::spot::on_event::SpotTradeCreatedEventHandler;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind_address = std::env::var("EXAMPLE_VELDRA_INBOUND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    tokio::spawn(async {
        if let Err(error) = run_timer().await {
            eprintln!("timer task stopped: {error}");
        }
    });

    tokio::task::spawn_blocking(|| {
        if let Err(error) = run_match_spot_order_aeron() {
            eprintln!("Aeron spot order receiver stopped: {error}");
        }
    });

    tokio::task::spawn_blocking(|| match AeronListenerConfig::spot_trade_from_env() {
        Ok(config) => {
            if let Err(error) = run_aeron_listener(config, SpotTradeCreatedEventHandler::default())
            {
                eprintln!("Aeron spot trade receiver stopped: {error}");
            }
        }
        Err(error) => {
            eprintln!("Aeron spot trade receiver stopped: invalid Aeron configuration: {error}");
        }
    });

    HttpServer::new(|| App::new().service(build_exchange_scope()).service(build_info_scope()))
        .bind(bind_address)?
        .run()
        .await
}
