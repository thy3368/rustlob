mod client;
mod models;

use std::env;

use models::RpcServiceConfig;

/// 创建 MatchingService 实例

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量或命令行参数选择服务类型
    let service_type = env::args().nth(1).unwrap_or_else(|| "websocket_axum".to_string());

    match service_type.as_str() {
        "jsonrpc" | "rpc" => {
            // 启动 JSON-RPC 服务
            println!("启动 JSON-RPC 服务...");
            let config = RpcServiceConfig {
                listen_addr: "127.0.0.1:3030".to_string(),
                threads: 4,
                order_capacity: 100000,
                price_range: 1000000
            };

            // let handler = create_matching_service(config.order_capacity,
            // config.price_range); let service =
            // LobRpcService::new(config); let server =
            // service.start(handler); server.wait();
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
                    price_range: 1000000
                };
                // let handler = create_matching_service(config.order_capacity,
                // config.price_range); let service =
                // LobRpcService::new(config); let server =
                // service.start(handler); server.wait();
            });
        }

        _ => {
            eprintln!("未知的服务类型: {}", service_type);
            eprintln!("用法: sapp [websocket_axum|axum|jsonrpc|all]");
            eprintln!("  websocket_axum - 启动 WebSocket 服务 (默认, 端口 9090, 推荐)");
            eprintln!("  axum      - 启动 HTTP REST API (端口 8080)");
            eprintln!("  jsonrpc   - 启动 JSON-RPC (端口 3030)");
            eprintln!("  all       - 同时启动所有服务");
            std::process::exit(1);
        }
    }

    Ok(())
}
