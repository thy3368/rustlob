mod models;
mod server;
mod client;

use models::RpcServiceConfig;
use server::json_rpc_service::LobRpcService;
use std::env;
use server::{restful_service, websocket_service};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量或命令行参数选择服务类型
    let service_type = env::args()
        .nth(1)
        .unwrap_or_else(|| "websocket".to_string());

    match service_type.as_str() {
        "websocket" | "ws" => {
            // 启动 WebSocket 服务（默认，最高性能）
            println!("启动 WebSocket 服务...");
            let port = env::var("WS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9090);

            websocket_service::start(port).await?;
        }

        "axum" | "http" => {
            // 启动 Axum HTTP REST API 服务
            println!("启动 Axum HTTP 服务...");
            let port = env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080);

            restful_service::start(port).await?;
        }

        "jsonrpc" | "rpc" => {
            // 启动 JSON-RPC 服务
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
                let service = LobRpcService::new(config);
                let server = service.start();
                server.wait();
            });

            // 启动 HTTP 服务（后台任务）
            tokio::spawn(async move {
                if let Err(e) = restful_service::start(8080).await {
                    eprintln!("HTTP服务错误: {}", e);
                }
            });

            // 启动 WebSocket 服务（主线程）
            websocket_service::start(9090).await?;
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
