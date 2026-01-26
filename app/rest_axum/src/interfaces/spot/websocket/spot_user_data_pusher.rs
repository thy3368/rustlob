use spot_behavior::proc::behavior::v2::spot_user_data_sse_behavior::{
    UserDataStreamEvent, OutboundAccountPositionEvent, BalanceItem, BalanceUpdateEvent,
    ExecutionReportEvent, ListStatusEvent, EventStreamTerminatedEvent, ExternalLockUpdateEvent,
    OrderSide, OrderType, TimeInForce, ExecutionType, OrderStatus, OrderRejectReason,
    SelfTradePreventionMode, ListOrderItem
};
use serde_json::json;
use futures::SinkExt;
use tokio::sync::mpsc;

use crate::interfaces::spot::websocket::connection_types::ConnectionRepo;

/// UserDataStreamEvent 消息推送器
pub struct SpotUserDataPusher {
    connection_repo: std::sync::Arc<ConnectionRepo>,
    interval: tokio::time::Duration,
    counter: u64
}

impl SpotUserDataPusher {
    /// 创建新的 SpotUserDataPusher 实例
    pub fn new(connection_repo: std::sync::Arc<ConnectionRepo>) -> Self {
        Self {
            connection_repo,
            interval: tokio::time::Duration::from_secs(8),
            counter: 0
        }
    }

    /// 设置消息推送间隔（秒）
    pub fn with_interval(mut self, seconds: u64) -> Self {
        self.interval = tokio::time::Duration::from_secs(seconds);
        self
    }

    /// 启动消息推送任务
    pub fn start(self) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    /// 运行消息推送循环
    async fn run(mut self) {
        let mut interval = tokio::time::interval(self.interval);

        loop {
            interval.tick().await;
            self.counter += 1;

            // 模拟生成不同类型的用户数据消息
            let stream_msg: UserDataStreamEvent = self.generate_stream_message();

            // 推送消息给所有连接
            self.broadcast_message(stream_msg).await;
        }
    }

    /// 广播消息给所有连接
    async fn broadcast_message(&self, msg: UserDataStreamEvent) {
        let all_senders = self.connection_repo.get_all_senders().await;
        let msg_text = serde_json::to_string(&json!({
            "stream_type": "user_data",
            "data": msg
        })).unwrap();

        for sender in all_senders {
            let _ = sender.send(axum::extract::ws::Message::Text(msg_text.clone().into()));
        }
    }

    /// 向指定用户推送消息
    pub async fn send_to_user(&self, user_id: &str, msg: UserDataStreamEvent) {
        let user_senders = self.connection_repo.get_senders_by_user(user_id).await;
        let msg_text = serde_json::to_string(&json!({
            "stream_type": "user_data",
            "data": msg
        })).unwrap();

        for sender in user_senders {
            let _ = sender.send(axum::extract::ws::Message::Text(msg_text.clone().into()));
        }
    }



    /// 生成模拟的 UserDataStreamEvent 消息
    fn generate_stream_message(&self) -> UserDataStreamEvent {
        let counter = self.counter;
        let now = chrono::Utc::now().timestamp_millis();

        if counter % 5 == 0 {
            // 账户位置更新事件
            UserDataStreamEvent::OutboundAccountPosition(OutboundAccountPositionEvent {
                subscription_id: 1,
                event_type: "outboundAccountPosition".to_string(),
                event_time: now,
                last_account_update_time: now - 1000,
                balances: vec![
                    BalanceItem {
                        asset: "BTC".to_string(),
                        free: format!("{:.8}", 0.5 + counter as f64 * 0.01),
                        locked: format!("{:.8}", 0.1 + counter as f64 * 0.005)
                    },
                    BalanceItem {
                        asset: "ETH".to_string(),
                        free: format!("{:.6}", 5.0 + counter as f64 * 0.1),
                        locked: format!("{:.6}", 1.0 + counter as f64 * 0.05)
                    },
                    BalanceItem {
                        asset: "USDT".to_string(),
                        free: format!("{:.2}", 1000.0 + counter as f64 * 50.0),
                        locked: format!("{:.2}", 200.0 + counter as f64 * 10.0)
                    }
                ]
            })
        } else if counter % 5 == 1 {
            // 余额更新事件
            UserDataStreamEvent::BalanceUpdate(BalanceUpdateEvent {
                subscription_id: 1,
                event_type: "balanceUpdate".to_string(),
                event_time: now,
                asset: "BTC".to_string(),
                balance_delta: format!("{:.8}", 0.01 + counter as f64 * 0.001),
                clear_time: now + 60000
            })
        } else if counter % 5 == 2 {
            // 执行报告事件（订单更新）
            UserDataStreamEvent::ExecutionReport(ExecutionReportEvent {
                subscription_id: 1,
                event_type: "executionReport".to_string(),
                event_time: now,
                symbol: "BTCUSDT".to_string(),
                client_order_id: format!("test_order_{}", counter),
                side: if counter % 2 == 0 { OrderSide::BUY } else { OrderSide::SELL },
                order_type: OrderType::LIMIT,
                time_in_force: TimeInForce::GTC,
                order_quantity: format!("{:.4}", 0.001 + counter as f64 * 0.0001),
                order_price: format!("{:.2}", 45000.0 + counter as f64 * 0.1),
                stop_price: "0.00".to_string(),
                iceberg_quantity: "0.00".to_string(),
                order_list_id: -1,
                original_client_order_id: "".to_string(),
                current_execution_type: ExecutionType::TRADE,
                current_order_status: OrderStatus::PARTIALLY_FILLED,
                order_reject_reason: OrderRejectReason::NONE,
                order_id: counter as i64,
                last_executed_quantity: format!("{:.4}", 0.0005 + counter as f64 * 0.00005),
                cumulative_filled_quantity: format!("{:.4}", 0.001 + counter as f64 * 0.0001),
                last_executed_price: format!("{:.2}", 45000.0 + counter as f64 * 0.1),
                commission_amount: format!("{:.8}", 0.00001 + counter as f64 * 0.000001),
                commission_asset: Some("BTC".to_string()),
                transaction_time: now,
                trade_id: counter as i64,
                execution_id: counter as i64,
                is_on_book: true,
                is_maker: counter % 2 == 0,
                ignore: false,
                order_creation_time: now - 30000,
                cumulative_quote_transacted_quantity: format!("{:.2}", 45.0 + counter as f64 * 0.1),
                last_quote_transacted_quantity: format!("{:.2}", 22.5 + counter as f64 * 0.05),
                quote_order_quantity: "0.00".to_string(),
                self_trade_prevention_mode: SelfTradePreventionMode::NONE,
                trailing_delta: None,
                trailing_time: None,
                strategy_id: None,
                strategy_type: None,
                prevented_match_id: None,
                prevented_quantity: None,
                last_prevented_quantity: None,
                trade_group_id: None,
                counter_order_id: None,
                counter_symbol: None,
                prevented_execution_quantity: None,
                prevented_execution_price: None,
                prevented_execution_quote_quantity: None,
                working_time: None,
                match_type: None,
                allocation_id: None,
                working_floor: None,
                used_sor: None,
                pegged_price_type: None,
                pegged_offset_type: None,
                pegged_offset_value: None,
                pegged_price: None
            })
        } else if counter % 5 == 3 {
            // 订单列表状态事件
            UserDataStreamEvent::ListStatus(ListStatusEvent {
                subscription_id: 1,
                event_type: "listStatus".to_string(),
                event_time: now,
                symbol: "ETHUSDT".to_string(),
                order_list_id: counter as i64,
                contingency_type: "OCO".to_string(),
                list_status_type: "EXEC_STARTED".to_string(),
                list_order_status: "EXECUTING".to_string(),
                list_reject_reason: "NONE".to_string(),
                list_client_order_id: format!("oco_order_{}", counter),
                transaction_time: now,
                orders: vec![
                    ListOrderItem {
                        symbol: "ETHUSDT".to_string(),
                        order_id: counter as i64,
                        client_order_id: format!("oco_order_{}_1", counter)
                    },
                    ListOrderItem {
                        symbol: "ETHUSDT".to_string(),
                        order_id: counter as i64 + 1,
                        client_order_id: format!("oco_order_{}_2", counter)
                    }
                ]
            })
        } else {
            // 事件流终止事件
            UserDataStreamEvent::EventStreamTerminated(EventStreamTerminatedEvent {
                subscription_id: 1,
                event_type: "eventStreamTerminated".to_string(),
                event_time: now
            })
        }
    }
}

/// 便捷函数：创建并启动 SpotUserDataPusher
pub fn start_spot_user_data_pusher(connection_repo: std::sync::Arc<ConnectionRepo>) {
    SpotUserDataPusher::new(connection_repo).start();
}

/// 便捷函数：创建并启动带有自定义间隔的 SpotUserDataPusher
pub fn start_spot_user_data_pusher_with_interval(
    connection_repo: std::sync::Arc<ConnectionRepo>, interval_seconds: u64
) {
    SpotUserDataPusher::new(connection_repo).with_interval(interval_seconds).start();
}
