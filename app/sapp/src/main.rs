mod models;
mod json_rpc_service;
mod rest_service;
mod websocket_service;

use models::RpcServiceConfig;
use json_rpc_service::LobRpcService;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量或命令行参数选择服务类型
    let service_type = env::args()
        .nth(1)
        .unwrap_or_else(|| "axum".to_string());

    match service_type.as_str() {
        "axum" | "http" => {
            // 启动 Axum HTTP REST API 服务（默认）
            println!("启动 Axum HTTP 服务...");
            let port = env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080);

            rest_service::start(port).await?;
        }

        "jsonrpc" | "rpc" => {
            // 启动 JSON-RPC 服务（原有）
            println!("启动 JSON-RPC 服务...");
            let config = RpcServiceConfig {
                listen_addr: "127.0.0.1:3030".to_string(),
                threads: 4,
                order_capacity: 100000,
                price_range: 1000000,
            };

            let service = LobRpcService::new(config);
            let server = service.start();
            server.wait();
        }

        "both" => {
            // 同时启动两个服务
            println!("同时启动 HTTP 和 JSON-RPC 服务...");

            // 启动 JSON-RPC 服务（后台线程）
            let _rpc_handle = std::thread::spawn(|| {
                let config = RpcServiceConfig {
                    listen_addr: "127.0.0.1:3030".to_string(),
                    threads: 4,
                    order_capacity: 100000,
                    price_range: 1000000,
                };
                let service = LobRpcService::new(config);
                let server = service.start();
                server.wait();
            });

            // 启动 Axum 服务（主线程）
            rest_service::start(8080).await?;
        }

        _ => {
            eprintln!("未知的服务类型: {}", service_type);
            eprintln!("用法: sapp [axum|jsonrpc|both]");
            eprintln!("  axum     - 启动 HTTP REST API (默认, 端口 8080)");
            eprintln!("  jsonrpc  - 启动 JSON-RPC (端口 3030)");
            eprintln!("  both     - 同时启动两个服务");
            std::process::exit(1);
        }
    }

    Ok(())
}
