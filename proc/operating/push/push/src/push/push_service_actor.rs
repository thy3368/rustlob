use std::{net::SocketAddr, sync::Arc, time::Duration};

use actix::prelude::*;
use actix_rt;
use base_types::spot_topic::SpotTopic;
use core_affinity::CoreId;
use diff::ChangeLogEntry;
use immutable_derive::immutable;
use num_cpus;
use rust_queue::queue::{queue::Queue, queue_impl::mpmc_queue::MPMCQueue};
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::push::connection_types::{ConnectionInfo, ConnectionRepo};
use crate::push::thread_binding::CpuAffinityActor;
// use futures_util::stream::once;
/// 推送服务 - 无状态设计，可安全地在多线程间共享
///
/// 该服务只包含不可变的依赖引用，不包含任何运行时状态，
/// 因此可以被多个线程同时访问而无需克隆。
pub struct PushService {
    /// 连接管理仓储（不可变引用）
    connection_repo: Arc<ConnectionRepo>,
    /// 变更日志仓储（不可变引用）
    change_log_repo: Arc<MPMCQueue>,
    /// CPU 亲和性核心 ID
    core_id: Option<CoreId>,
}


impl PushService {
    /// 创建新的 PushService 实例
    pub fn new(connection_repo: Arc<ConnectionRepo>, change_log_repo: Arc<MPMCQueue>) -> Self {
        Self {
            connection_repo,
            change_log_repo,
            core_id: None,
        }
    }

    /// 创建带有 CPU 亲和性的 PushService 实例
    pub fn with_affinity(
        connection_repo: Arc<ConnectionRepo>,
        change_log_repo: Arc<MPMCQueue>,
        core_id: CoreId,
    ) -> Self {
        Self {
            connection_repo,
            change_log_repo,
            core_id: Some(core_id),
        }
    }

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
        let mut receiver = self.change_log_repo.subscribe::<ChangeLogEntry>(SpotTopic::EntityChangeLog.name(), None);

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


impl Actor for PushService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // 设置 CPU 亲和性
        if let Some(core_id) = self.core_id {
            #[cfg(target_os = "linux")]
            {
                if core_affinity::set_for_current(core_id) {
                    tracing::info!("PushService 成功绑定到 CPU 核心: {}", core_id.id);
                } else {
                    tracing::error!("PushService 绑定到 CPU 核心失败: {}", core_id.id);
                }
            }

            #[cfg(not(target_os = "linux"))]
            {
                tracing::warn!("CPU 亲和性仅在 Linux 上支持");
            }
        }

        // 在 Actor 启动时，启动事件监听任务
        let connection_repo = Arc::clone(&self.connection_repo);
        let mut receiver = self.change_log_repo.subscribe::<ChangeLogEntry>(
            SpotTopic::EntityChangeLog.name(),
            None
        );

        actix_rt::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                // 我们需要再次创建一个服务实例，因为不能直接引用外部的 self
                let temp_service = PushService {
                    connection_repo: Arc::clone(&connection_repo),
                    change_log_repo: Arc::new(MPMCQueue::new()), // 这里我们只需要一个空的队列，因为我们不需要发送消息
                    core_id: None,
                };
                temp_service.process_event(event).await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use actix::Actor;
    use core_affinity;
    use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

    use super::*;


    #[actix::test]
    async fn test_push_service_actor_start() {
        // 创建测试所需的依赖
        let connection_repo = Arc::new(ConnectionRepo::default());
        let change_log_repo = Arc::new(MPMCQueue::new());

        // 创建 PushService 实例
        let push_service = PushService::new(connection_repo, change_log_repo);

        // 测试作为 Actor 启动
        let addr = push_service.start();

        // 验证 Actor 是否连接
        assert!(addr.connected());
    }

    #[actix::test]
    async fn test_push_service_with_affinity() {
        // 创建测试所需的依赖
        let connection_repo = Arc::new(ConnectionRepo::default());
        let change_log_repo = Arc::new(MPMCQueue::new());

        // 获取可用的核心 ID
        if let Some(core_ids) = core_affinity::get_core_ids() {
            // 创建带有 CPU 亲和性的 PushService 实例
            let push_service = PushService::with_affinity(
                connection_repo,
                change_log_repo,
                core_ids[0],
            );

            // 测试作为 Actor 启动
            let addr = push_service.start();

            // 验证 Actor 是否连接
            assert!(addr.connected());
        }
    }
}
