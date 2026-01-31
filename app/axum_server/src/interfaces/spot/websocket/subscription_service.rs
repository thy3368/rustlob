use std::{net::SocketAddr, sync::Arc, time::Duration};

use db_repo::{adapter::change_log_queue_repo::ChangeLogChannelQueueRepo, core::queue_repo::ChangeLogQueueRepo};
use immutable_derive::immutable;
use serde_json::json;

use crate::interfaces::spot::websocket::connection_types::{ConnectionInfo, ConnectionRepo};

/// 订阅服务 - 无状态设计，可安全地在多线程间共享
///
/// 该服务只包含不可变的依赖引用，不包含任何运行时状态，
/// 因此可以被多个线程同时访问而无需克隆。
#[immutable]
pub struct SubscriptionService {
    /// 连接管理仓储（不可变引用）
    connection_repo: Arc<ConnectionRepo>,
    /// 变更日志仓储（不可变引用）
    change_log_repo: Arc<ChangeLogChannelQueueRepo>
}

impl SubscriptionService {
    /// 创建新的订阅服务实例
    ///
    /// # 参数
    /// - `connection_repo`: 连接管理仓储
    /// - `change_log_repo`: 变更日志仓储
    pub fn new(connection_repo: Arc<ConnectionRepo>, change_log_repo: Arc<ChangeLogChannelQueueRepo>) -> Self {
        Self {
            connection_repo,
            change_log_repo
        }
    }

    /// 启动后台事件轮询任务
    ///
    /// 该方法不获取 self 所有权，而是克隆 Arc 引用在后台任务中使用。
    /// 这样可以在启动后台任务后，继续使用当前的服务实例。
    ///
    /// # 参数
    /// - `interval`: 轮询事件的时间间隔
    pub fn start(self: &Arc<Self>, interval: Duration) {
        let service = Arc::clone(self);

        tokio::spawn(async move {
            service.run(interval).await;
        });
    }

    /// 后台运行事件轮询循环
    ///
    /// # 参数
    /// - `interval`: 轮询事件的时间间隔
    async fn run(&self, interval: Duration) {
        let mut interval_timer = tokio::time::interval(interval);

        loop {
            interval_timer.tick().await;
            self.try_send().await;
        }
    }

    /// 添加新连接
    ///
    /// # 参数
    /// - `conn_info`: 连接信息
    pub async fn add_connection(&self, conn_info: ConnectionInfo) {
        self.connection_repo.add_connection(conn_info).await;
    }

    /// 移除连接
    ///
    /// # 参数
    /// - `client_addr`: 客户端地址
    pub async fn remove_connection(&self, client_addr: SocketAddr) {
        self.connection_repo.remove_connection(client_addr).await;
    }

    /// 尝试发送待处理的事件给相关订阅者
    ///
    /// 该方法：
    /// 1. 从变更日志仓储轮询新事件
    /// 2. 找到对每个事件感兴趣的连接
    /// 3. 将事件序列化为 JSON 并发送给这些连接
    pub async fn try_send(&self) {
        // 轮询事件（使用固定超时 100ms）
        let events = match self.change_log_repo.poll(Duration::from_millis(100)) {
            Ok(events) => events,
            Err(e) => {
                tracing::error!("Failed to poll change log events: {:?}", e);
                return;
            }
        };

        for event in events {
            tracing::debug!(
                "Processing event: entity_type={}, entity_id={}, change_type={:?}",
                event.entity_type,
                event.entity_id,
                event.change_type
            );

            // 通过 ConnectionRepo 找到对该 event 感兴趣的发送器列表
            let interested_senders: Vec<tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>> =
                self.connection_repo.get_senders_by_entity(&event.entity_type, &event.entity_id).await;

            if interested_senders.is_empty() {
                tracing::trace!(
                    "No connections interested in event: entity_type={}, entity_id={}",
                    event.entity_type,
                    event.entity_id
                );
                continue; // 没有感兴趣的连接，直接丢弃该消息
            }

            // 序列化事件为 JSON 消息
            let msg_text = match serde_json::to_string(&json!({
                "stream_type": "user_data",
                "data": {
                    "entity_id": event.entity_id,
                    "entity_type": event.entity_type,
                    "change_type": format!("{:?}", event.change_type),
                    "timestamp": event.timestamp,
                    "sequence": event.sequence
                }
            })) {
                Ok(text) => text,
                Err(e) => {
                    tracing::error!("Failed to serialize event to JSON: {:?}", e);
                    continue;
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
                event.entity_type,
                event.entity_id
            );
        }
    }
}
