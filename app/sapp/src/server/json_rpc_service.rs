//! LOB JSON-RPC 服务实现
//!
//! 通过单一 handle 接口转发所有命令

use crate::models::*;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{DomainsValidation, Server, ServerBuilder};
use lob::lob::{SpotCommand, SpotCommandResult, SpotOrderProc, Side, TraderId, Command};
use std::sync::{Arc, Mutex};

/// 解析 Side 字符串
fn parse_side(s: &str) -> std::result::Result<Side, jsonrpc_core::Error> {
    match s.to_uppercase().as_str() {
        "BUY" | "B" => Ok(Side::Buy),
        "SELL" | "S" => Ok(Side::Sell),
        _ => Err(jsonrpc_core::Error::invalid_params("Invalid side")),
    }
}

/// LOB RPC 接口 - 统一命令入口
#[rpc(server)]
pub trait LobRpc {
    /// 统一命令处理接口
    #[rpc(name = "execute")]
    fn execute(&self, cmd: CommandRequest) -> Result<CommandResponse>;

    /// 健康检查
    #[rpc(name = "health")]
    fn health(&self) -> Result<HealthResponse>;
}

/// LOB RPC 服务实现
pub struct LobRpcImpl<H: SpotOrderProc + Send + 'static> {
    handler: Arc<Mutex<H>>,
}

impl<H: SpotOrderProc + Send + 'static> LobRpcImpl<H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }

    /// 将请求转换为 SpotCommand
    /// 注意: IcebergOrder 已移至 ConditionalCommand，需通过 ConditionalOrderHandler 处理
    fn parse_command(cmd: &CommandRequest) -> std::result::Result<SpotCommand, jsonrpc_core::Error> {
        match cmd.command.as_str() {
            "LimitOrder" => Ok(SpotCommand::LimitOrder {
                trader: TraderId::from_str(cmd.trader_id.as_deref().unwrap_or("")),
                side: parse_side(cmd.side.as_deref().unwrap_or(""))?,
                price: cmd.price.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing price"))?,
                quantity: cmd.quantity.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing quantity"))?,
            }),
            "MarketOrder" => Ok(SpotCommand::MarketOrder {
                trader: TraderId::from_str(cmd.trader_id.as_deref().unwrap_or("")),
                side: parse_side(cmd.side.as_deref().unwrap_or(""))?,
                quantity: cmd.quantity.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing quantity"))?,
            }),
            "CancelOrder" => Ok(SpotCommand::CancelOrder {
                order_id: cmd.order_id.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing order_id"))?,
            }),
            _ => Err(jsonrpc_core::Error::invalid_params(format!("unknown command: {}", cmd.command))),
        }
    }

    /// 将 SpotCommandResult 转换为响应
    fn to_response(result: SpotCommandResult) -> CommandResponse {
        match result {
            SpotCommandResult::LimitOrder { order_id, trades } => CommandResponse {
                success: true,
                order_id: Some(order_id),
                trades: Some(trades.iter().map(|t| TradeInfo {
                    buyer: t.buyer().to_string(),
                    seller: t.seller().to_string(),
                    price: t.price,
                    quantity: t.quantity,
                }).collect()),
                ..Default::default()
            },
            SpotCommandResult::MarketOrder { trades } => CommandResponse {
                success: true,
                trades: Some(trades.iter().map(|t| TradeInfo {
                    buyer: t.buyer().to_string(),
                    seller: t.seller().to_string(),
                    price: t.price,
                    quantity: t.quantity,
                }).collect()),
                ..Default::default()
            },
            SpotCommandResult::CancelOrder { success } => CommandResponse {
                success,
                ..Default::default()
            },
            _ => CommandResponse {
                success: false,
                error: Some("unsupported command result".to_string()),
                ..Default::default()
            },
        }
    }
}

impl<H: SpotOrderProc + Send + Sync + 'static> LobRpc for LobRpcImpl<H> {
    fn execute(&self, cmd: CommandRequest) -> Result<CommandResponse> {
        let spot_command = Self::parse_command(&cmd)?;
        // 使用请求中的 nonce，如果没有则生成一个
        let nonce = cmd.nonce.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64
        });
        let idempotent_cmd = Command::new(nonce, spot_command);

        let mut h = self.handler.lock().unwrap();
        let idempotent_result = h.handle(idempotent_cmd);

        let mut response = Self::to_response(idempotent_result.result);
        response.nonce = Some(idempotent_result.nonce);
        response.is_duplicate = Some(idempotent_result.is_duplicate);
        Ok(response)
    }

    fn health(&self) -> Result<HealthResponse> {
        Ok(HealthResponse {
            status: "ok".to_string(),
            service: "lob-matching-service".to_string(),
            version: "0.1.0".to_string(),
        })
    }
}

/// LOB JSON-RPC 服务
pub struct LobRpcService {
    config: RpcServiceConfig,
}

impl LobRpcService {
    pub fn new(config: RpcServiceConfig) -> Self {
        Self { config }
    }

    pub fn start<H: SpotOrderProc + Send + Sync + 'static>(self, handler: H) -> Server {
        let rpc_impl = LobRpcImpl::new(handler);
        let mut io = jsonrpc_core::IoHandler::new();
        io.extend_with(rpc_impl.to_delegate());

        let server = ServerBuilder::new(io)
            .threads(self.config.threads)
            .cors(DomainsValidation::Disabled)
            .start_http(&self.config.listen_addr.parse().unwrap())
            .expect("无法启动 JSON-RPC 服务器");

        self.print_banner();
        server
    }

    fn print_banner(&self) {
        println!("LOB JSON-RPC Server @ http://{}", self.config.listen_addr);
        println!("Methods: execute, health");
    }
}
