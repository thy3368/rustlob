//! LOB JSON-RPC 服务实现
//!
//! 通过单一 handle 接口转发所有命令

use std::sync::{Arc, Mutex};

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{DomainsValidation, Server, ServerBuilder};
use lob::lob::{Cmd, Side, SpotCmdAny, SpotCmdResult, SpotOrderExgProc, TraderId};

use crate::models::*;

/// 解析 Side 字符串
fn parse_side(s: &str) -> std::result::Result<Side, jsonrpc_core::Error> {
    match s.to_uppercase().as_str() {
        "BUY" | "B" => Ok(Side::Buy),
        "SELL" | "S" => Ok(Side::Sell),
        _ => Err(jsonrpc_core::Error::invalid_params("Invalid side"))
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
pub struct LobRpcImpl<H: SpotOrderExgProc + Send + 'static> {
    handler: Arc<Mutex<H>>
}

impl<H: SpotOrderExgProc + Send + 'static> LobRpcImpl<H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler))
        }
    }

    /// 将请求转换为 SpotCommand
    /// 注意: IcebergOrder 已移至 ConditionalCommand，需通过
    /// ConditionalOrderHandler 处理
    fn parse_command(cmd: &CommandRequest) -> std::result::Result<SpotCmdAny, jsonrpc_core::Error> {
        use lob::lob::{Symbol, TimeInForce};

        match cmd.command.as_str() {
            "LimitOrder" => Ok(SpotCmdAny::LimitOrder {
                trader: TraderId::from_str(cmd.trader_id.as_deref().unwrap_or("")),
                trading_pair: Symbol::from_str(cmd.symbol.as_deref().unwrap_or("BTCUSDT")),
                side: parse_side(cmd.side.as_deref().unwrap_or(""))?,
                price: cmd.price.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing price"))?,
                quantity: cmd.quantity.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing quantity"))?,
                time_in_force: TimeInForce::GoodTillCancel,
                client_order_id: cmd.client_order_id.clone()
            }),
            "MarketOrder" => Ok(SpotCmdAny::MarketOrder {
                trader: TraderId::from_str(cmd.trader_id.as_deref().unwrap_or("")),
                symbol: Symbol::from_str(cmd.symbol.as_deref().unwrap_or("BTCUSDT")),
                side: parse_side(cmd.side.as_deref().unwrap_or(""))?,
                quantity: cmd.quantity.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing quantity"))?,
                price_limit: cmd.price_limit,
                time_in_force: None,
                client_order_id: cmd.client_order_id.clone()
            }),
            "CancelOrder" => Ok(SpotCmdAny::CancelOrder {
                order_id: cmd.order_id.ok_or_else(|| jsonrpc_core::Error::invalid_params("missing order_id"))?
            }),
            _ => Err(jsonrpc_core::Error::invalid_params(format!("unknown command: {}", cmd.command)))
        }
    }

    /// 将 SpotCommandResult 转换为响应
    fn to_response(result: SpotCmdResult) -> CommandResponse {
        use lob::lob::OrderStatus;

        match result {
            SpotCmdResult::LimitOrder {
                order_id,
                status,
                filled_quantity,
                remaining_quantity,
                trades
            } => CommandResponse {
                success: matches!(status, OrderStatus::Filled | OrderStatus::PartiallyFilled | OrderStatus::Pending),
                order_id: Some(order_id),
                filled_quantity: Some(filled_quantity),
                remaining_quantity: Some(remaining_quantity),
                trades: Some(
                    trades
                        .iter()
                        .map(|t| TradeInfo {
                            buyer: t.buyer().to_string(),
                            seller: t.seller().to_string(),
                            price: t.price,
                            quantity: t.quantity
                        })
                        .collect()
                ),
                ..Default::default()
            },
            SpotCmdResult::MarketOrder {
                status,
                filled_quantity,
                trades
            } => CommandResponse {
                success: matches!(status, OrderStatus::Filled | OrderStatus::PartiallyFilled),
                filled_quantity: Some(filled_quantity),
                trades: Some(
                    trades
                        .iter()
                        .map(|t| TradeInfo {
                            buyer: t.buyer().to_string(),
                            seller: t.seller().to_string(),
                            price: t.price,
                            quantity: t.quantity
                        })
                        .collect()
                ),
                ..Default::default()
            },
            SpotCmdResult::CancelOrder {
                order_id,
                status
            } => CommandResponse {
                success: matches!(status, OrderStatus::Cancelled),
                order_id: Some(order_id),
                ..Default::default()
            },
            _ => CommandResponse {
                success: false,
                error: Some("unsupported command result".to_string()),
                ..Default::default()
            }
        }
    }
}

impl<H: SpotOrderExgProc + Send + Sync + 'static> LobRpc for LobRpcImpl<H> {
    fn execute(&self, cmd: CommandRequest) -> Result<CommandResponse> {
        let spot_command = Self::parse_command(&cmd)?;
        // 使用请求中的 nonce，如果没有则生成一个
        let nonce = cmd.nonce.unwrap_or_else(|| {
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
        });
        let idempotent_cmd = Cmd::new(nonce, spot_command);

        let mut h = self.handler.lock().unwrap();
        let idempotent_result = h.handle(idempotent_cmd);

        match idempotent_result {
            Ok(cmd_response) => {
                let mut response = Self::to_response(cmd_response.result);
                response.nonce = Some(cmd_response.metadata.nonce);
                response.is_duplicate = Some(cmd_response.metadata.is_duplicate);
                Ok(response)
            }
            Err(e) => Err(jsonrpc_core::Error::invalid_params(format!("Command failed: {}", e)))
        }
    }

    fn health(&self) -> Result<HealthResponse> {
        Ok(HealthResponse {
            status: "ok".to_string(),
            service: "lob_repo-matching-service".to_string(),
            version: "0.1.0".to_string()
        })
    }
}

/// LOB JSON-RPC 服务
pub struct LobRpcService {
    config: RpcServiceConfig
}

impl LobRpcService {
    pub fn new(config: RpcServiceConfig) -> Self {
        Self {
            config
        }
    }

    pub fn start<H: SpotOrderExgProc + Send + Sync + 'static>(self, handler: H) -> Server {
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
