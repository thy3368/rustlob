use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use base_types::{OrderSide, TradingPair, Quantity, Price};
use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{OrderStatus, SpotOrder, SpotTrade, OrderType, TimeInForce, ExecutionMethod, ConditionalType, AlgorithmStrategy, SelfTradePrevention};
use diff::{ChangeLogEntry, ChangeType, FieldChange};
use rust_queue::queue::queue::Queue;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;
use crate::proc::v2::actor::spot_trade_acquiring_actor::{NewOrderCmdReceiver, SpotAcquiringActor};
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

pub struct SpotMatchActor {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    queue: Arc<MPMCQueue>,
}

/// 从 ChangeLogEntry 重构 SpotOrder
/// 
/// # Arguments
/// * `change_log` - 包含订单创建或更新信息的变更日志
/// 
/// # Returns
/// * `Ok(SpotOrder)` - 成功重构的订单
/// * `Err(String)` - 重构失败的原因
fn reconstruct_order_from_changelog(change_log: &ChangeLogEntry) -> Result<SpotOrder, String> {
    // 提取字段映射
    let fields_map: HashMap<&str, &str> = match change_log.change_type() {
        ChangeType::Created { fields } | ChangeType::Updated { changed_fields: fields } => {
            fields.iter()
                .map(|f| (f.field_name.as_ref(), f.new_value.as_str()))
                .collect()
        }
        ChangeType::Deleted => {
            return Err("Cannot reconstruct order from Deleted event".to_string());
        }
    };

    // 解析必需字段
    let order_id: u64 = fields_map.get("order_id")
        .and_then(|v| v.parse().ok())
        .ok_or("Missing or invalid order_id")?;

    let trading_pair = fields_map.get("trading_pair")
        .and_then(|v| parse_trading_pair(v))
        .ok_or("Missing or invalid trading_pair")?;

    let side = fields_map.get("side")
        .and_then(|v| parse_order_side(v))
        .ok_or("Missing or invalid side")?;

    let quantity = fields_map.get("total_qty")
        .and_then(|v| v.parse::<f64>().ok())
        .map(Quantity::from_f64)
        .ok_or("Missing or invalid total_qty")?;

    let price = fields_map.get("price")
        .and_then(|v| v.parse::<f64>().ok())
        .map(Price::from_f64);

    let time_in_force = fields_map.get("time_in_force")
        .and_then(|v| parse_time_in_force(v))
        .unwrap_or(TimeInForce::GTC);

    // 解析可选字段
    let client_order_id = fields_map.get("client_order_id").map(|v| v.to_string());
    let quote_order_qty = fields_map.get("quote_order_qty")
        .and_then(|v| v.parse::<f64>().ok())
        .map(Quantity::from_f64);

    let trader_id = fields_map.get("trader_id")
        .and_then(|v| parse_trader_id(v))
        .unwrap_or_default();

    // 解析订单状态（如果不是Pending，说明有问题）
    let status = fields_map.get("status")
        .and_then(|v| parse_order_status(v))
        .unwrap_or(OrderStatus::Pending);

    // 构建 SpotOrder
    let mut order = SpotOrder::create_order(
        order_id,
        trader_id,
        trading_pair,
        side,
        price.unwrap_or(Price::from_f64(0.0)),
        quantity,
        time_in_force,
        client_order_id,
        quote_order_qty,
    );

    // 设置其他字段
    order.status = status;
    order.order_type = fields_map.get("order_type")
        .and_then(|v| parse_order_type(v))
        .unwrap_or(OrderType::Limit);
    order.execution_method = fields_map.get("execution_method")
        .and_then(|v| parse_execution_method(v))
        .unwrap_or(ExecutionMethod::Limit);
    order.conditional_type = fields_map.get("conditional_type")
        .and_then(|v| parse_conditional_type(v))
        .unwrap_or(ConditionalType::None);

    // 设置计算字段
    if let Some(v) = fields_map.get("unfilled_qty").and_then(|v| v.parse::<f64>().ok()) {
        order.unfilled_qty = Quantity::from_f64(v);
    }
    if let Some(v) = fields_map.get("executed_qty").and_then(|v| v.parse::<f64>().ok()) {
        order.executed_qty = Quantity::from_f64(v);
    }
    if let Some(v) = fields_map.get("frozen_qty").and_then(|v| v.parse::<f64>().ok()) {
        order.frozen_qty = Quantity::from_f64(v);
    }

    Ok(order)
}

/// 解析交易对
fn parse_trading_pair(s: &str) -> Option<TradingPair> {
    match s {
        "BTCUSDT" | "BtcUsdt" => Some(TradingPair::BtcUsdt),
        "ETHUSDT" | "EthUsdt" => Some(TradingPair::EthUsdt),
        "BTCETH" | "BtcEth" => Some(TradingPair::BtcEth),
        _ => TradingPair::from_symbol_str(s),
    }
}

/// 解析订单方向
fn parse_order_side(s: &str) -> Option<OrderSide> {
    match s.to_uppercase().as_str() {
        "BUY" => Some(OrderSide::Buy),
        "SELL" => Some(OrderSide::Sell),
        _ => None,
    }
}

/// 解析有效期类型
fn parse_time_in_force(s: &str) -> Option<TimeInForce> {
    match s.to_uppercase().as_str() {
        "GTC" => Some(TimeInForce::GTC),
        "IOC" => Some(TimeInForce::IOC),
        "FOK" => Some(TimeInForce::FOK),
        "GTX" => Some(TimeInForce::GTX),
        "GTD" => Some(TimeInForce::GTD),
        _ => None,
    }
}

/// 解析订单类型
fn parse_order_type(s: &str) -> Option<OrderType> {
    match s.to_uppercase().as_str() {
        "LIMIT" => Some(OrderType::Limit),
        "MARKET" => Some(OrderType::Market),
        "STOP_LOSS" | "STOPLOSS" => Some(OrderType::StopLoss),
        "STOP_LOSS_LIMIT" | "STOPLOSSLIMIT" => Some(OrderType::StopLossLimit),
        "TAKE_PROFIT" | "TAKEPROFIT" => Some(OrderType::TakeProfit),
        "TAKE_PROFIT_LIMIT" | "TAKEPROFITLIMIT" => Some(OrderType::TakeProfitLimit),
        "LIMIT_MAKER" | "LIMITMAKER" => Some(OrderType::LimitMaker),
        _ => None,
    }
}

/// 解析执行方式
fn parse_execution_method(s: &str) -> Option<ExecutionMethod> {
    match s.to_uppercase().as_str() {
        "LIMIT" => Some(ExecutionMethod::Limit),
        "MARKET" => Some(ExecutionMethod::Market),
        _ => None,
    }
}

/// 解析条件类型
fn parse_conditional_type(s: &str) -> Option<ConditionalType> {
    match s.to_uppercase().as_str() {
        "NONE" => Some(ConditionalType::None),
        "STOP_LOSS" | "STOPLOSS" => Some(ConditionalType::StopLoss),
        "TAKE_PROFIT" | "TAKEPROFIT" => Some(ConditionalType::TakeProfit),
        _ => None,
    }
}

/// 解析订单状态
fn parse_order_status(s: &str) -> Option<OrderStatus> {
    match s.to_uppercase().as_str() {
        "PENDING" => Some(OrderStatus::Pending),
        "CONDITIONAL_PENDING" | "CONDITIONALPENDING" => Some(OrderStatus::ConditionalPending),
        "PARTIALLY_FILLED" | "PARTIALLYFILLED" => Some(OrderStatus::PartiallyFilled),
        "FILLED" => Some(OrderStatus::Filled),
        "CANCELLED" => Some(OrderStatus::Cancelled),
        "REJECTED" => Some(OrderStatus::Rejected),
        "EXPIRED" => Some(OrderStatus::Expired),
        _ => None,
    }
}

/// 解析交易员ID
fn parse_trader_id(s: &str) -> Option<TraderId> {
    // TraderId 通常是 8 字节的数组
    let bytes = s.as_bytes();
    if bytes.len() >= 8 {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes[0..8]);
        Some(TraderId::new(arr))
    } else {
        None
    }
}

impl SpotMatchActor {
    /// 处理变更日志事件
    /// 如果事件是 order.status=pending 则进行撮合处理
    async fn handle_change_log(&self, event_bytes: bytes::Bytes) {
        // 解析 ChangeLogEntry
        let change_log: ChangeLogEntry = match serde_json::from_slice(&event_bytes) {
            Ok(log) => log,
            Err(e) => {
                tracing::error!("Failed to deserialize ChangeLogEntry: {:?}", e);
                return;
            }
        };

        // 检查是否是 SpotOrder 实体
        if change_log.entity_type() != "SpotOrder" {
            return;
        }

        // 检查变更类型是否包含 status 字段更新为 Pending
        let is_pending = match change_log.change_type() {
            ChangeType::Created { fields } | ChangeType::Updated { changed_fields: fields } => {
                fields.iter().any(|field| {
                    field.field_name.as_ref() == "status" && 
                    (field.new_value == "Pending" || field.new_value == "PENDING")
                })
            }
            _ => false,
        };

        if !is_pending {
            return;
        }

        tracing::info!("检测到订单进入 Pending 状态，开始撮合处理: order_id={}", change_log.entity_id());

        // 从 ChangeLogEntry 重构 SpotOrder
        let order = match reconstruct_order_from_changelog(&change_log) {
            Ok(order) => {
                tracing::info!("成功重构订单: order_id={}, trading_pair={:?}, side={:?}, qty={}", 
                    order.order_id, order.trading_pair, order.side, order.total_qty);
                order
            }
            Err(e) => {
                tracing::error!("重构订单失败: {:?}", e);
                return;
            }
        };
        
        // 执行撮合处理
        self.process_match(order).await;
    }

    /// 执行撮合处理
    async fn process_match(&self, mut order: SpotOrder) {
        tracing::info!("执行撮合处理: order_id={}, trading_pair={:?}", 
            order.order_id, order.trading_pair);

        // 调用 handle_match 进行撮合
        // 注意：trade_behavior.handle_match 需要返回 trades
        // 这里假设 trade_behavior 有 handle_match 方法
        
        // 由于 handle_match 是私有方法，我们需要通过其他方式调用
        // 可能的方案：
        // 1. 将 handle_match 改为 pub(crate) 或 pub
        // 2. 通过命令模式调用
        // 3. 直接调用 trade_behavior 的公开方法
        
        // 调用 handle_match2 进行撮合并获取变更日志
        let change_logs = match self.trade_behavior.handle_match2(order.order_id) {
            Ok(logs) => {
                tracing::info!("撮合成功: order_id={}, 生成 {} 条变更日志", order.order_id, logs.len());
                logs
            }
            Err(e) => {
                tracing::error!("撮合失败: order_id={}, error={:?}", order.order_id, e);
                return;
            }
        };

        // 发送变更日志到消息队列
        if !change_logs.is_empty() {
            let bytes_events: Vec<bytes::Bytes> = change_logs
                .iter()
                .filter_map(|log| {
                    serde_json::to_vec(log)
                        .ok()
                        .map(bytes::Bytes::from)
                })
                .collect();

            if !bytes_events.is_empty() {
                match self.queue.send_batch(SpotTopic::EntityChangeLog.name(), bytes_events, None) {
                    Ok(results) => {
                        let success_count = results.iter().filter(|r| r.is_ok()).count();
                        tracing::info!(
                            "成功发送 {}/{} 个变更日志到队列",
                            success_count,
                            change_logs.len()
                        );
                    }
                    Err(e) => {
                        tracing::error!("批量发送变更日志失败: {:?}", e);
                    }
                }
            }
        }

        tracing::info!("撮合处理完成: order_id={}", order.order_id);






    }
}

impl ActorX for SpotMatchActor {
    fn start(self: &Arc<Self>) {
        // 同时订阅变更日志事件（用于接收其他系统事件）
        let self_clone2 = Arc::clone(self);
        tokio::spawn(async move {
            let mut receiver = self_clone2.queue.subscribe(SpotTopic::EntityChangeLog.name(), None);

            while let Ok(event) = receiver.recv().await {
                // 如果事件是 order.status=pending 则进行撮合处理； 并发送trade和order的changelog
                self_clone2.handle_change_log(event).await;
            }
        });
    }
}
