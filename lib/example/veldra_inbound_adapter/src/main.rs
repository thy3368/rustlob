use std::io;

use actix_web::{App, HttpServer};
use example_outbound_adapter::InMemoryPlaceOrderOutbound;
use example_veldra_inbound_adapter::bootstrap::on_command::exchange::http::build_exchange_scope;
use example_veldra_inbound_adapter::bootstrap::on_command::info::http::build_info_scope;
use example_veldra_inbound_adapter::bootstrap::on_event::spot::on_aeron_match_spot_order::run_match_spot_order_aeron;
use example_veldra_inbound_adapter::bootstrap::on_event::spot::on_aeron_spot_trade::{
    run_spot_trade_aeron, run_trigger_pending_spot_orders_aeron,
};
use example_veldra_inbound_adapter::bootstrap::on_time::spot::timer::run_timer;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // HTTP 入口监听地址来自 EXAMPLE_VELDRA_INBOUND_ADDR，默认使用 127.0.0.1:8080。
    let bind_address = std::env::var("EXAMPLE_VELDRA_INBOUND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // 启动定时任务入口。
    tokio::spawn(async {
        if let Err(error) = run_timer().await {
            eprintln!("timer task stopped: {error}");
        }
    });

    // 监听撮合订单事件。
    tokio::task::spawn_blocking(|| {
        if let Err(error) = run_match_spot_order_aeron() {
            eprintln!("Aeron spot order receiver stopped: {error}");
        }
    });

    // 监听成交事件并生成、更新 K 线。
    tokio::task::spawn_blocking(|| {
        if let Err(error) = run_spot_trade_aeron() {
            eprintln!("Aeron spot trade receiver stopped: {error}");
        }
    });

    // 监听成交事件并触发 pending trigger orders。
    tokio::task::spawn_blocking(|| {
        let outbound = InMemoryPlaceOrderOutbound::new();
        if let Err(error) = run_trigger_pending_spot_orders_aeron(outbound) {
            eprintln!("Aeron spot trade pending order trigger receiver stopped: {error}");
        }
    });

    // 启动 HTTP API 服务。
    HttpServer::new(|| App::new().service(build_exchange_scope()).service(build_info_scope()))
        .bind(bind_address)?
        .run()
        .await
}
