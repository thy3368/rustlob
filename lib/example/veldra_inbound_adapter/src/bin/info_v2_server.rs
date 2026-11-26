use std::io;

use actix_web::{App, HttpServer};
use example_veldra_inbound_adapter::bootstrap::on_command::info_v2::build_info_v2_scope;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind_address = std::env::var("EXAMPLE_VELDRA_INFO_V2_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8081".to_owned());

    HttpServer::new(|| App::new().service(build_info_v2_scope()))
        .bind(bind_address)?
        .run()
        .await
}
