mod exchange;

use std::io;

use actix_web::{App, HttpServer};

use crate::exchange::http::build_exchange_scope;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind_address =
        std::env::var("VELDRA_INBOUND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| App::new().service(build_exchange_scope())).bind(bind_address)?.run().await
}
