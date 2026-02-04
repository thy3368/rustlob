use spot_behavior::proc::behavior::v2::spot_user_data_sse_behavior::{
    UserDataStreamEventAny, OutboundAccountPositionEvent, BalanceItem, BalanceUpdateEvent,
    ExecutionReportEvent, ListStatusEvent, EventStreamTerminatedEvent, ExternalLockUpdateEvent,
     OrderType, TimeInForce, ExecutionType, OrderStatus, OrderRejectReason,
    SelfTradePreventionMode, ListOrderItem
};
use serde_json::json;
use futures::SinkExt;
use tokio::sync::mpsc;
use base_types::OrderSide;
use push::push::connection_types::ConnectionRepo;
// use push::push::connection_types::ConnectionRepo;

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
            let stream_msg: UserDataStreamEventAny = self.generate_stream_message();

            // 推送消息给所有连接
            self.broadcast_message(stream_msg).await;
        }
    }

    /// 广播消息给所有连接
    async fn broadcast_message(&self, msg: UserDataStreamEventAny) {
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
    pub async fn send_to_user(&self, user_id: &str, msg: UserDataStreamEventAny) {
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
    fn generate_stream_message(&self) -> UserDataStreamEventAny {
        let counter = self.counter;
        let now = chrono::Utc::now().timestamp_millis();


        if counter % 5 == 0 {
            // 账户位置更新事件
            UserDataStreamEventAny::OutboundAccountPosition(OutboundAccountPositionEvent::new(
                1,
                "outboundAccountPosition".to_string(),
                now,
                now - 1000,
                vec![
                    BalanceItem::new(
                        "BTC".to_string(),
                        format!("{:.8}", 0.5 + counter as f64 * 0.01),
                        format!("{:.8}", 0.1 + counter as f64 * 0.005)
                    ),
                    BalanceItem::new(
                        "ETH".to_string(),
                        format!("{:.6}", 5.0 + counter as f64 * 0.1),
                        format!("{:.6}", 1.0 + counter as f64 * 0.05)
                    ),
                    BalanceItem::new(
                        "USDT".to_string(),
                        format!("{:.2}", 1000.0 + counter as f64 * 50.0),
                        format!("{:.2}", 200.0 + counter as f64 * 10.0)
                    )
                ]
            ))
        } else if counter % 5 == 1 {
            // 余额更新事件
            UserDataStreamEventAny::BalanceUpdate(BalanceUpdateEvent::new(
                1,
                "balanceUpdate".to_string(),
                now,
                "BTC".to_string(),
                format!("{:.8}", 0.01 + counter as f64 * 0.001),
                now + 60000
            ))
        } else if counter % 5 == 2 {
            // 执行报告事件（订单更新）
            UserDataStreamEventAny::ExecutionReport(ExecutionReportEvent::new(
                1,
                "executionReport".to_string(),
                now,
                "BTCUSDT".to_string(),
                format!("test_order_{}", counter),
                if counter % 2 == 0 { OrderSide::Buy } else { OrderSide::Sell },
                OrderType::LIMIT,
                TimeInForce::GTC,
                format!("{:.4}", 0.001 + counter as f64 * 0.0001),
                format!("{:.2}", 45000.0 + counter as f64 * 0.1),
                "0.00".to_string(),
                "0.00".to_string(),
                -1,
                "".to_string(),
                ExecutionType::TRADE,
                OrderStatus::PARTIALLY_FILLED,
                OrderRejectReason::NONE,
                counter as i64,
                format!("{:.4}", 0.0005 + counter as f64 * 0.00005),
                format!("{:.4}", 0.001 + counter as f64 * 0.0001),
                format!("{:.2}", 45000.0 + counter as f64 * 0.1),
                format!("{:.8}", 0.00001 + counter as f64 * 0.000001),
                Some("BTC".to_string()),
                now,
                counter as i64,
                counter as i64,
                true,
                counter % 2 == 0,
                false,
                now - 30000,
                format!("{:.2}", 45.0 + counter as f64 * 0.1),
                format!("{:.2}", 22.5 + counter as f64 * 0.05),
                "0.00".to_string(),
                SelfTradePreventionMode::NONE,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ))
        } else if counter % 5 == 3 {
            // 订单列表状态事件
            UserDataStreamEventAny::ListStatus(ListStatusEvent::new(
                1,
                "listStatus".to_string(),
                now,
                "ETHUSDT".to_string(),
                counter as i64,
                "OCO".to_string(),
                "EXEC_STARTED".to_string(),
                "EXECUTING".to_string(),
                "NONE".to_string(),
                format!("oco_order_{}", counter),
                now,
                vec![
                    ListOrderItem::new(
                        "ETHUSDT".to_string(),
                        counter as i64,
                        format!("oco_order_{}_1", counter)
                    ),
                    ListOrderItem::new(
                        "ETHUSDT".to_string(),
                        counter as i64 + 1,
                        format!("oco_order_{}_2", counter)
                    )
                ]
            ))
        } else {
            // 事件流终止事件
            UserDataStreamEventAny::EventStreamTerminated(EventStreamTerminatedEvent::new(
                1,
                "eventStreamTerminated".to_string(),
                now
            ))
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
