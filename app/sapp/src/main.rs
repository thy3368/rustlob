mod client;
mod models;
mod server;

use account::{
    AccountServiceImpl, InMemoryAccountRepository, InMemoryBalanceRepository, TradingPair,
};
use lob::lob::{MemoryOrderRepository, MatchingService};
use models::RpcServiceConfig;
use server::json_rpc_service::LobRpcService;
use std::env;

/// 创建 MatchingService 实例
fn create_matching_service(
    order_capacity: usize,
    price_range: usize,
) -> MatchingService<MemoryOrderRepository, AccountServiceImpl<InMemoryAccountRepository, InMemoryBalanceRepository>>
{
    let lob_repo = MemoryOrderRepository::new(order_capacity, price_range);
    let account_repo = InMemoryAccountRepository::new();
    let balance_repo = InMemoryBalanceRepository::with_default_timestamp();
    let account_service = AccountServiceImpl::new(
        account_repo,
        balance_repo,
        || std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    );
    let trading_pair = TradingPair::BTC_USDT;

    MatchingService::new(lob_repo, account_service, trading_pair)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量或命令行参数选择服务类型
    let service_type = env::args()
        .nth(1)
        .unwrap_or_else(|| "websocket".to_string());

    match service_type.as_str() {
        "jsonrpc" | "rpc" => {
            // 启动 JSON-RPC 服务
            println!("启动 JSON-RPC 服务...");
            let config = RpcServiceConfig {
                listen_addr: "127.0.0.1:3030".to_string(),
                threads: 4,
                order_capacity: 100000,
                price_range: 1000000,
            };

            let handler = create_matching_service(config.order_capacity, config.price_range);
            let service = LobRpcService::new(config);
            let server = service.start(handler);
            server.wait();
        }

        "all" => {
            // 同时启动所有服务
            println!("同时启动 WebSocket, HTTP 和 JSON-RPC 服务...");

            // 启动 JSON-RPC 服务（后台线程）
            let _rpc_handle = std::thread::spawn(|| {
                let config = RpcServiceConfig {
                    listen_addr: "127.0.0.1:3030".to_string(),
                    threads: 4,
                    order_capacity: 100000,
                    price_range: 1000000,
                };
                let handler = create_matching_service(config.order_capacity, config.price_range);
                let service = LobRpcService::new(config);
                let server = service.start(handler);
                server.wait();
            });
        }

        _ => {
            eprintln!("未知的服务类型: {}", service_type);
            eprintln!("用法: sapp [websocket|axum|jsonrpc|all]");
            eprintln!("  websocket - 启动 WebSocket 服务 (默认, 端口 9090, 推荐)");
            eprintln!("  axum      - 启动 HTTP REST API (端口 8080)");
            eprintln!("  jsonrpc   - 启动 JSON-RPC (端口 3030)");
            eprintln!("  all       - 同时启动所有服务");
            std::process::exit(1);
        }
    }

    Ok(())
}
