//! LOB JSON-RPC 服务实现

use crate::models::*;
use jsonrpc_core::{IoHandler, Params};
use jsonrpc_http_server::{Server, ServerBuilder, DomainsValidation};
use lob::lob::domain::service::handler::{Command, CommandResult, OrderCommandHandler};
use lob::lob::domain::service::matching_service::MatchingService;
use lob::lob::domain::::in_memory::InMemoryOrderRepository;
use lob::lob::domain::::OrderRepository;  // 导入 trait 以使用其方法
use lob::lob::domain::entity::lob_types::{Side, TraderId};
use serde_json::json;
use std::sync::{Arc, Mutex};

/// 解析 Side 字符串
fn parse_side(s: &str) -> Option<Side> {
    match s.to_uppercase().as_str() {
        "BUY" | "B" => Some(Side::Buy),
        "SELL" | "S" => Some(Side::Sell),
        _ => None,
    }
}

/// LOB JSON-RPC 服务
pub struct LobRpcService {
    config: RpcServiceConfig,
    matching_service: Arc<Mutex<MatchingService<InMemoryOrderRepository>>>,
}

impl LobRpcService {
    /// 创建新的 RPC 服务
    pub fn new(config: RpcServiceConfig) -> Self {
        let repository = InMemoryOrderRepository::new(config.order_capacity, config.price_range);
        let matching_service = MatchingService::new(repository);

        Self {
            config,
            matching_service: Arc::new(Mutex::new(matching_service)),
        }
    }

    /// 构建 JSON-RPC handler
    fn build_handler(&self) -> IoHandler {
        let mut io = IoHandler::new();

        self.register_limit_order(&mut io);
        self.register_market_order(&mut io);
        self.register_iceberg_order(&mut io);
        self.register_cancel_order(&mut io);
        self.register_book_status(&mut io);
        self.register_health_check(&mut io);

        io
    }

    /// 注册限价单 RPC 方法
    fn register_limit_order(&self, io: &mut IoHandler) {
        let service = self.matching_service.clone();

        io.add_method("place_limit_order", move |params: Params| {
            let service = service.clone();

            async move {
                let req: LimitOrderRequest = params.parse()?;

                let side = parse_side(&req.side)
                    .ok_or_else(|| jsonrpc_core::Error::invalid_params("Invalid side"))?;
                let trader = TraderId::from_str(&req.trader_id);

                let command = Command::LimitOrder {
                    trader,
                    side,
                    price: req.price,
                    quantity: req.quantity,
                };

                let mut svc = service.lock().unwrap();
                let result = svc.handle(command);

                match result {
                    CommandResult::LimitOrder { order_id, trades } => {
                        let trade_infos: Vec<TradeInfo> = trades
                            .iter()
                            .map(|t| TradeInfo {
                                buyer: t.buyer.to_string(),
                                seller: t.seller.to_string(),
                                price: t.price,
                                quantity: t.quantity,
                            })
                            .collect();

                        Ok(json!({
                            "order_id": order_id,
                            "trades": trade_infos,
                            "status": "success"
                        }))
                    }
                    _ => Err(jsonrpc_core::Error::internal_error()),
                }
            }
        });
    }

    /// 注册市价单 RPC 方法
    fn register_market_order(&self, io: &mut IoHandler) {
        let service = self.matching_service.clone();

        io.add_method("place_market_order", move |params: Params| {
            let service = service.clone();

            async move {
                let req: MarketOrderRequest = params.parse()?;

                let side = parse_side(&req.side)
                    .ok_or_else(|| jsonrpc_core::Error::invalid_params("Invalid side"))?;
                let trader = TraderId::from_str(&req.trader_id);

                let command = Command::MarketOrder {
                    trader,
                    side,
                    quantity: req.quantity,
                };

                let mut svc = service.lock().unwrap();
                let result = svc.handle(command);

                match result {
                    CommandResult::MarketOrder { trades } => {
                        let trade_infos: Vec<TradeInfo> = trades
                            .iter()
                            .map(|t| TradeInfo {
                                buyer: t.buyer.to_string(),
                                seller: t.seller.to_string(),
                                price: t.price,
                                quantity: t.quantity,
                            })
                            .collect();

                        Ok(json!({
                            "trades": trade_infos,
                            "status": "success"
                        }))
                    }
                    _ => Err(jsonrpc_core::Error::internal_error()),
                }
            }
        });
    }

    /// 注册冰山单 RPC 方法
    fn register_iceberg_order(&self, io: &mut IoHandler) {
        let service = self.matching_service.clone();

        io.add_method("place_iceberg_order", move |params: Params| {
            let service = service.clone();

            async move {
                let req: IcebergOrderRequest = params.parse()?;

            let side = parse_side(&req.side)
                .ok_or_else(|| jsonrpc_core::Error::invalid_params("Invalid side"))?;
            let trader = TraderId::from_str(&req.trader_id);

            let command = Command::IcebergOrder {
                trader,
                side,
                price: req.price,
                total_quantity: req.total_quantity,
                display_quantity: req.display_quantity,
            };

            let mut svc = service.lock().unwrap();
            let result = svc.handle(command);

            match result {
                CommandResult::IcebergOrder {
                    order_id,
                    trades,
                    remaining_total,
                    current_display,
                } => {
                    let trade_infos: Vec<TradeInfo> = trades
                        .iter()
                        .map(|t| TradeInfo {
                            buyer: t.buyer.to_string(),
                            seller: t.seller.to_string(),
                            price: t.price,
                            quantity: t.quantity,
                        })
                        .collect();

                    Ok(json!({
                        "order_id": order_id,
                        "trades": trade_infos,
                        "remaining_total": remaining_total,
                        "current_display": current_display,
                        "status": "success"
                    }))
                }
                _ => Err(jsonrpc_core::Error::internal_error()),
            }
            }
        });
    }

    /// 注册取消订单 RPC 方法
    fn register_cancel_order(&self, io: &mut IoHandler) {
        let service = self.matching_service.clone();

        io.add_method("cancel_order", move |params: Params| {
            let service = service.clone();

            async move {
                let req: CancelOrderRequest = params.parse()?;

            let command = Command::CancelOrder {
                order_id: req.order_id,
            };

            let mut svc = service.lock().unwrap();
            let result = svc.handle(command);

            match result {
                CommandResult::CancelOrder { success } => {
                    Ok(json!({
                        "success": success,
                        "order_id": req.order_id
                    }))
                }
                _ => Err(jsonrpc_core::Error::internal_error()),
                }
            }
        });
    }

    /// 注册订单簿状态 RPC 方法
    fn register_book_status(&self, io: &mut IoHandler) {
        let service = self.matching_service.clone();

        io.add_method("get_book_status", move |_params: Params| {
            let service = service.clone();

            async move {
            let svc = service.lock().unwrap();
            let repo = svc.repository();

            let best_bid = repo.best_bid();
            let best_ask = repo.best_ask();
            let spread = match (best_bid, best_ask) {
                (Some(bid), Some(ask)) if ask > bid => Some(ask - bid),
                _ => None,
            };
            let active_orders = repo.active_order_count();

            Ok(json!({
                "best_bid": best_bid,
                "best_ask": best_ask,
                "spread": spread,
                "active_orders": active_orders
            }))
            }
        });
    }

    /// 注册健康检查 RPC 方法
    fn register_health_check(&self, io: &mut IoHandler) {
        io.add_method("health", |_params: Params| async move {
            Ok(json!({
                "status": "ok",
                "service": "lob-matching-service",
                "version": "0.1.0"
            }))
        });
    }

    /// 启动服务
    pub fn start(self) -> Server {
        let io = self.build_handler();

        let server = ServerBuilder::new(io)
            .threads(self.config.threads)
            .cors(DomainsValidation::Disabled)
            .start_http(&self.config.listen_addr.parse().unwrap())
            .expect("无法启动 JSON-RPC 服务器");

        self.print_banner();

        server
    }

    /// 打印启动信息
    fn print_banner(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║           LOB Matching Service - JSON-RPC Server            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
        println!("✓ 服务器已启动");
        println!("  监听地址: http://{}", self.config.listen_addr);
        println!("  工作线程: {}", self.config.threads);
        println!("  订单容量: {}", self.config.order_capacity);
        println!("  价格范围: 0 - {}", self.config.price_range);
        println!();
        println!("可用的 RPC 方法:");
        println!("  📝 place_limit_order    - 提交限价单");
        println!("  🚀 place_market_order   - 提交市价单");
        println!("  🧊 place_iceberg_order  - 提交冰山单");
        println!("  ❌ cancel_order         - 取消订单");
        println!("  📊 get_book_status      - 获取订单簿状态");
        println!("  💚 health               - 健康检查");
        println!();
        println!("示例请求:");
        println!(r#"  curl -X POST http://{} \"#, self.config.listen_addr);
        println!(r#"    -H "Content-Type: application/json" \"#);
        println!(r#"    -d '{{"jsonrpc":"2.0","method":"place_limit_order","params":{{"trader_id":"TRADER001","side":"BUY","price":10000,"quantity":100}},"id":1}}'"#);
        println!();
        println!("按 Ctrl+C 停止服务器...");
        println!();
    }
}
