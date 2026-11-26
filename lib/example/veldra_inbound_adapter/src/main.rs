use std::io;

use actix_web::{App, HttpServer};
use example_veldra_inbound_adapter::bootstrap::on_command::exchange_v2::build_exchange_v2_scope;
use example_veldra_inbound_adapter::bootstrap::on_command::info_v2::build_info_v2_scope;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // HTTP 入口监听地址来自 EXAMPLE_VELDRA_INBOUND_ADDR，默认使用 127.0.0.1:8080。
    let bind_address = std::env::var("EXAMPLE_VELDRA_INBOUND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // 启动 HTTP API 服务。
    HttpServer::new(|| {
        App::new()
            .service(build_exchange_v2_scope())
            .service(build_info_v2_scope())
    })
    .bind(bind_address)?
    .run()
    .await
}
