use std::{net::SocketAddr, sync::Arc, time::Duration};
use serde::de::DeserializeOwned;
use immutable_derive::immutable;
use serde_json::json;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;
use crate::push::connection_types::{ConnectionInfo, ConnectionRepo};
use diff::ChangeLogEntry;
use rust_queue::queue::queue::Queue;

/// 推送服务 - 无状态设计，可安全地在多线程间共享
///
/// 该服务只包含不可变的依赖引用，不包含任何运行时状态，
/// 因此可以被多个线程同时访问而无需克隆。
#[immutable]
pub struct PushService {
    /// 连接管理仓储（不可变引用）
    connection_repo: Arc<ConnectionRepo>,
    /// 变更日志仓储（不可变引用）
    change_log_repo: Arc<MPMCQueue>
}


impl PushService {
    /// 启动后台事件监听任务
    ///
    /// 该方法不获取 self 所有权，而是克隆 Arc 引用在后台任务中使用。
    /// 这样可以在启动后台任务后，继续使用当前的服务实例。
    pub fn start(self: &Arc<Self>) {
        let service = Arc::clone(self);

        tokio::spawn(async move {
            service.run().await;
        });
    }

    /// 后台运行事件监听循环
    async fn run(&self) {
        // 订阅变更日志事件
        let mut receiver = self.change_log_repo.subscribe::<ChangeLogEntry>(
            "entity_change_log",
            None
        );

        // 持续监听事件
        while let Ok(event) = receiver.recv().await {
            self.process_event(event).await;
        }
    }

    /// 处理单个变更日志事件
    async fn process_event(&self, event: ChangeLogEntry) {
        tracing::debug!(
            "Processing event: entity_type={}, entity_id={}, change_type={:?}",
            event.entity_type(),
            event.entity_id(),
            event.change_type()
        );

        // 通过 ConnectionRepo 找到对该 event 感兴趣的发送器列表
        let interested_senders: Vec<tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>> =
            self.connection_repo.get_senders_by_entity(&event.entity_type(), &event.entity_id()).await;

        if interested_senders.is_empty() {
            tracing::trace!(
                "No connections interested in event: entity_type={}, entity_id={}",
                event.entity_type(),
                event.entity_id()
            );
            return; // 没有感兴趣的连接，直接返回
        }

        // 序列化事件为 JSON 消息
        let msg_text = match serde_json::to_string(&json!({
            "stream_type": "user_data",
            "data": {
                "entity_id": event.entity_id(),
                "entity_type": event.entity_type(),
                "change_type": format!("{:?}", event.change_type()),
                "timestamp": event.timestamp(),
                "sequence": event.sequence()
            }
        })) {
            Ok(text) => text,
            Err(e) => {
                tracing::error!("Failed to serialize event to JSON: {:?}", e);
                return;
            }
        };

        let ws_msg = axum::extract::ws::Message::Text(msg_text.into());

        // 发送消息给所有感兴趣的连接
        let sender_count = interested_senders.len();
        let mut success_count = 0;

        for sender in interested_senders {
            if sender.send(ws_msg.clone()).is_ok() {
                success_count += 1;
            } else {
                tracing::debug!("Failed to send message to WebSocket connection (connection may be closed)");
            }
        }

        tracing::debug!(
            "Sent event to {}/{} connections: entity_type={}, entity_id={}",
            success_count,
            sender_count,
            event.entity_type(),
            event.entity_id()
        );
    }

    /// 保留 try_send 方法以保持向后兼容（空实现）
    #[deprecated(note = "This method is deprecated. Events are now processed via subscription.")]
    pub async fn try_send(&self) {
        // 空实现，事件现在通过 subscribe 方法异步处理
    }
}
