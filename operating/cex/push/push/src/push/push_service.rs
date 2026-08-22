use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use diff::ChangeLog;
use entity_derive::immutable;
use serde::Serialize;

use crate::push::connection_types::ConnectionRepo;
use crate::queue::queue_contract::Queue;
use crate::queue::queue_impl::mpmc_queue::MPMCQueue;

/// 推送服务 - 无状态设计，可安全地在多线程间共享
///
/// 该服务只包含不可变的依赖引用，不包含任何运行时状态，
/// 因此可以被多个线程同时访问而无需克隆。
#[immutable]
pub struct PushBehaviorV2Imp {
    /// 连接管理仓储（不可变引用）
    connection_repo: Arc<ConnectionRepo>,
    /// 变更日志仓储（不可变引用）
    change_log_repo: Arc<MPMCQueue>,
}

impl PushBehaviorV2Imp {
    /// 后台运行事件监听循环
    /// 订阅并处理: OrderChangeLog + KLineChangeLog + BalanceChangeLog + TradeChangeLog
    async fn run(self: Arc<Self>) {
        // 订阅所有变更日志Topic
        let order_receiver = self.change_log_repo.subscribe(SpotTopic::OrderChangeLog.name(), None);
        let kline_receiver = self.change_log_repo.subscribe(SpotTopic::KLineChangeLog.name(), None);
        let balance_receiver =
            self.change_log_repo.subscribe(SpotTopic::BalanceChangeLog.name(), None);
        let trade_receiver = self.change_log_repo.subscribe(SpotTopic::TradeChangeLog.name(), None);

        tracing::info!("PushService 已订阅所有变更日志Topic");

        self.spawn_change_log_listener(order_receiver);
        self.spawn_change_log_listener(kline_receiver);
        self.spawn_change_log_listener(balance_receiver);
        self.spawn_change_log_listener(trade_receiver);

        std::future::pending::<()>().await;
    }

    fn spawn_change_log_listener(
        self: &Arc<Self>,
        mut receiver: tokio::sync::broadcast::Receiver<bytes::Bytes>,
    ) {
        let service = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(event) => service.process_change_log_event(event).await,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        tracing::warn!("PushService 订阅滞后，跳过 {} 条消息", skipped);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });
    }

    /// 处理变更日志事件字节流
    async fn process_change_log_event(&self, event: bytes::Bytes) {
        // 将字节转换为 entity_change_log
        let entity_change_log = match serde_json::from_slice::<ChangeLog>(&event) {
            Ok(log) => log,
            Err(e) => {
                tracing::error!("Failed to deserialize event to ChangeLogEntry: {:?}", e);
                return;
            }
        };

        self.handle_event(entity_change_log);
    }

    /// 处理单个变更日志事件
    pub fn handle_event(&self, entity_change_log: ChangeLog) {
        self.handle_events(&[entity_change_log]);
    }

    /// 批量处理变更日志事件（性能优化）
    pub fn handle_events(&self, entity_change_logs: &[ChangeLog]) {
        if entity_change_logs.is_empty() {
            return;
        }

        tracing::debug!("Processing {} events in batch", entity_change_logs.len());

        let mut total_sent = 0;
        let mut total_skipped = 0;
        let mut total_failed = 0;

        // 批量处理：减少日志打印和重复计算
        for event in entity_change_logs {
            // 通过 ConnectionRepo 找到对该事件感兴趣的发送器列表
            let interested_senders: Vec<
                tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>,
            > = self.connection_repo.get_senders_by_entity(event.entity_type(), event.entity_id());

            if interested_senders.is_empty() {
                total_skipped += 1;
                continue;
            }

            // 序列化事件
            let msg_text = match self.serialize_event(event) {
                Ok(text) => text,
                Err(e) => {
                    tracing::error!("Failed to serialize event: {:?}", e);
                    total_failed += 1;
                    continue;
                }
            };

            let ws_msg = axum::extract::ws::Message::Text(msg_text.into());

            // 发送给所有感兴趣的连接
            for sender in interested_senders {
                if sender.send(ws_msg.clone()).is_ok() {
                    total_sent += 1;
                } else {
                    total_failed += 1;
                }
            }
        }

        tracing::debug!(
            "Batch processing complete: {} sent, {} skipped, {} failed",
            total_sent,
            total_skipped,
            total_failed
        );
    }

    /// 序列化单个事件为 JSON
    fn serialize_event(&self, entity_change_log: &ChangeLog) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct PushEventEnvelope {
            stream_type: &'static str,
            data: PushEventData,
        }

        #[derive(Serialize)]
        struct PushEventData {
            entity_id: String,
            entity_type: String,
            change_type: String,
            timestamp: u64,
            sequence: u64,
        }

        serde_json::to_string(&PushEventEnvelope {
            stream_type: "user_data",
            data: PushEventData {
                entity_id: entity_change_log.entity_id().to_string(),
                entity_type: entity_change_log.entity_type().to_string(),
                change_type: format!("{:?}", entity_change_log.change_type()),
                timestamp: *entity_change_log.timestamp(),
                sequence: *entity_change_log.sequence(),
            },
        })
    }

    /// 保留 try_send 方法以保持向后兼容（空实现）
    #[deprecated(note = "This method is deprecated. Events are now processed via subscription.")]
    pub async fn try_send(&self) {
        // 空实现，事件现在通过 subscribe 方法异步处理
    }
}

impl ActorX for PushBehaviorV2Imp {
    /// 启动后台事件监听任务
    ///
    /// 该方法不获取 self 所有权，而是克隆 Arc 引用在后台任务中使用。
    /// 这样可以在启动后台任务后，继续使用当前的服务实例。
    fn start(self: &Arc<Self>) {
        let service = Arc::clone(self);

        tokio::spawn(async move {
            service.run().await;
        });
    }
}
