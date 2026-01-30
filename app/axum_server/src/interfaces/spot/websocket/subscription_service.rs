use std::{net::SocketAddr, sync::Arc};

use db_repo::{adapter::change_log_queue_repo::ChangeLogChannelQueueRepo, core::queue_repo::ChangeLogQueueRepo};
use serde_json::json;

use crate::interfaces::spot::websocket::connection_types::{ConnectionInfo, ConnectionRepo};

#[derive(Clone)]
// todo 将 SubscriptionService 设计成无状态 从而可被多线程访问，也不需要clone
pub struct SubscriptionService {
    pub connection_repo: Arc<ConnectionRepo>,
    pub change_log_repo: Arc<ChangeLogChannelQueueRepo>,

    /// 轮询事件的时间间隔（毫秒）
    pub interval: std::time::Duration
}


impl SubscriptionService {
    async fn run(self) {
        let mut interval = tokio::time::interval(self.interval);

        loop {
            interval.tick().await;
            self.try_send().await;
        }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    pub(crate) fn new(connection_repo: Arc<ConnectionRepo>, change_log_repo: Arc<ChangeLogChannelQueueRepo>) -> Self {
        Self {
            connection_repo,
            change_log_repo,
            interval: std::time::Duration::from_millis(100) // 默认100ms轮询间隔
        }
    }

    pub async fn add_connection(&self, conn_info: ConnectionInfo) {
        // 添加到连接管理器（使用 mpsc Sender）
        self.connection_repo.add_connection(conn_info).await;
    }

    pub async fn remove_connection(&self, client_addr: SocketAddr) {
        self.connection_repo.remove_connection(client_addr).await;
    }

    pub async fn try_send(&self) {
        // 轮询事件（使用默认超时）
        let events = self.change_log_repo.poll(std::time::Duration::from_millis(100)).unwrap();

        for event in events {
            println!(
                "Processing event: entity_type={}, entity_id={}, change_type={:?}",
                event.entity_type, event.entity_id, event.change_type
            );

            // 通过 ConnectionRepo 找到对该 event 感兴趣的发送器列表
            let interested_senders: Vec<tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>> =
                self.connection_repo.get_senders_by_entity(&event.entity_type, &event.entity_id).await;

            if interested_senders.is_empty() {
                println!(
                    "No connections interested in event: entity_type={}, entity_id={}",
                    event.entity_type, event.entity_id
                );
                continue; // 没有感兴趣的连接，直接丢掉该消息
            }

            // 序列化事件为 JSON 消息
            let msg_text = serde_json::to_string(&json!({
                "stream_type": "user_data",
                "data": {
                    "entity_id": event.entity_id,
                    "entity_type": event.entity_type,
                    "change_type": format!("{:?}", event.change_type),
                    "timestamp": event.timestamp,
                    "sequence": event.sequence
                }
            }))
            .unwrap();

            let ws_msg = axum::extract::ws::Message::Text(msg_text.into());

            // 发送消息给所有感兴趣的连接
            let sender_count = interested_senders.len();
            for sender in interested_senders {
                if sender.send(ws_msg.clone()).is_err() {
                    println!("Failed to send message to WebSocket connection");
                    // 发送失败，可能连接已断开，不需要特殊处理
                }
            }

            println!(
                "Sent event to {} connections: entity_type={}, entity_id={}",
                sender_count, event.entity_type, event.entity_id
            );
        }
    }
}
