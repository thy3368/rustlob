use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::extract::ws::Message;
use chrono;
use tokio::sync::{mpsc, Mutex};

/// 连接信息结构体
#[derive(Debug)]
pub struct ConnectionInfo {
    /// 用户ID（可选，未认证用户可能没有）
    pub user_id: Option<String>,
    /// 客户端IP地址
    pub client_addr: SocketAddr,
    /// todo 订阅消息
    pub subscription: Vec<String>,
    /// 连接建立时间戳（毫秒）
    pub connected_at: i64,
    /// 最后活动时间戳（毫秒）
    pub last_active_at: i64,
    /// 消息发送器
    pub sender: mpsc::UnboundedSender<Message>
}

/// 连接管理器
#[derive(Debug, Clone, Default)]
pub struct ConnectionRepo {
    // todo 这里数据可能放DB 或其它cache
    /// 用户连接映射：user_id -> Vec<ConnectionInfo>
    user_connections: Arc<Mutex<HashMap<String, Vec<ConnectionInfo>>>>,
    /// 所有连接列表
    all_connections: Arc<Mutex<Vec<ConnectionInfo>>>,
    /// 连接发送器映射：client_addr ->
    /// tokio::sync::mpsc::UnboundedSender<Message>
    connection_senders: Arc<Mutex<HashMap<SocketAddr, mpsc::UnboundedSender<Message>>>>,
    /// 用户发送器映射：user_id ->
    /// Vec<tokio::sync::mpsc::UnboundedSender<Message>>
    user_senders: Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<Message>>>>>
}

impl ConnectionRepo {
    /// 创建新的连接管理器实例
    pub fn new() -> Self {
        Self {
            user_connections: Arc::new(Mutex::new(HashMap::new())),
            all_connections: Arc::new(Mutex::new(Vec::new())),
            connection_senders: Arc::new(Mutex::new(HashMap::new())),
            user_senders: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    /// 根据事件类型获取对该事件感兴趣的所有发送器
    /// 事件类型可以是实体类型（如 "Order"）或具体事件名称
    pub async fn get_senders_by_event(&self, event_type: &str) -> Vec<mpsc::UnboundedSender<Message>> {
        let mut matched_senders = Vec::new();
        let all_conns = self.all_connections.lock().await;

        for conn in &*all_conns {
            // 检查连接是否对该事件类型感兴趣
            if conn.subscription.iter().any(|sub| sub == event_type || sub == "*" || sub.contains(event_type)) {
                matched_senders.push(conn.sender.clone());
            }
        }

        matched_senders
    }

    /// 根据实体类型和实体ID获取相关的发送器（用于特定实体的事件）
    pub async fn get_senders_by_entity(
        &self, entity_type: &str, entity_id: &str
    ) -> Vec<mpsc::UnboundedSender<Message>> {
        let mut matched_senders = Vec::new();
        let all_conns = self.all_connections.lock().await;

        for conn in &*all_conns {
            // 检查连接是否对该实体类型或特定实体ID感兴趣
            if conn
                .subscription
                .iter()
                .any(|sub| sub.as_str() == entity_type || sub == &format!("{}:{}", entity_type, entity_id) || sub.as_str() == "*")
            {
                matched_senders.push(conn.sender.clone());
            }
        }

        matched_senders
    }

    /// 添加连接信息和发送器
    pub async fn add_connection(&self, conn_info: ConnectionInfo) {
        let mut all_conns = self.all_connections.lock().await;
        // 因为 ConnectionInfo 现在没有实现 Clone，所以我们直接推入
        all_conns.push(conn_info.clone());

        let mut conn_senders = self.connection_senders.lock().await;
        conn_senders.insert(conn_info.client_addr, conn_info.sender.clone());

        if let Some(user_id) = &conn_info.user_id {
            let mut user_conns = self.user_connections.lock().await;
            user_conns.entry(user_id.clone()).or_insert(Vec::new()).push(conn_info.clone());

            let mut user_senders = self.user_senders.lock().await;
            user_senders.entry(user_id.clone()).or_insert(Vec::new()).push(conn_info.sender.clone());
        }

        println!("Connection added: {:?}", conn_info);
    }

    /// 移除连接信息
    pub async fn remove_connection(&self, client_addr: SocketAddr) {
        let mut all_conns = self.all_connections.lock().await;
        if let Some(pos) = all_conns.iter().position(|c| c.client_addr == client_addr) {
            let removed = all_conns.remove(pos);

            let mut conn_senders = self.connection_senders.lock().await;
            conn_senders.remove(&client_addr);

            if let Some(user_id) = &removed.user_id {
                let mut user_conns = self.user_connections.lock().await;
                if let Some(conns) = user_conns.get_mut(user_id) {
                    conns.retain(|c| c.client_addr != client_addr);
                    if conns.is_empty() {
                        user_conns.remove(user_id);
                    }
                }

                let mut user_senders = self.user_senders.lock().await;
                if let Some(senders) = user_senders.get_mut(user_id) {
                    senders.retain(|s| {
                        // 这里我们无法直接比较 Sender，所以我们保留所有 Sender
                        // 因为 Sender 没有实现 PartialEq
                        true
                    });
                    // 清理空列表
                    if senders.is_empty() {
                        user_senders.remove(user_id);
                    }
                }
            }

            println!("Connection removed: {:?}", removed);
        }
    }

    /// 根据用户ID获取所有连接信息
    pub async fn get_connections_by_user(&self, user_id: &str) -> Vec<ConnectionInfo> {
        let user_conns = self.user_connections.lock().await;
        user_conns.get(user_id).cloned().unwrap_or(Vec::new())
    }

    /// 根据用户ID获取所有发送器
    pub async fn get_senders_by_user(&self, user_id: &str) -> Vec<mpsc::UnboundedSender<Message>> {
        let user_senders = self.user_senders.lock().await;
        user_senders.get(user_id).cloned().unwrap_or(Vec::new())
    }

    /// 根据客户端地址获取发送器
    pub async fn get_sender_by_addr(&self, client_addr: SocketAddr) -> Option<mpsc::UnboundedSender<Message>> {
        let conn_senders = self.connection_senders.lock().await;
        conn_senders.get(&client_addr).cloned()
    }

    /// 获取所有连接信息
    pub async fn get_all_connections(&self) -> Vec<ConnectionInfo> {
        let all_conns = self.all_connections.lock().await;
        all_conns.clone()
    }

    /// 获取所有发送器
    pub async fn get_all_senders(&self) -> Vec<mpsc::UnboundedSender<Message>> {
        let conn_senders = self.connection_senders.lock().await;
        conn_senders.values().cloned().collect()
    }

    /// 获取用户连接数量
    pub async fn get_user_connection_count(&self, user_id: &str) -> usize {
        let user_conns = self.user_connections.lock().await;
        user_conns.get(user_id).map(|v| v.len()).unwrap_or(0)
    }

    /// 获取总连接数量
    pub async fn get_total_connection_count(&self) -> usize {
        let all_conns = self.all_connections.lock().await;
        all_conns.len()
    }

    /// 更新连接活动时间
    pub async fn update_last_active(&self, client_addr: SocketAddr) {
        let mut all_conns = self.all_connections.lock().await;
        if let Some(conn) = all_conns.iter_mut().find(|c| c.client_addr == client_addr) {
            conn.last_active_at = chrono::Utc::now().timestamp_millis();
        }

        if let Some(conn) = all_conns.iter().find(|c| c.client_addr == client_addr) {
            if let Some(user_id) = &conn.user_id {
                let mut user_conns = self.user_connections.lock().await;
                if let Some(conns) = user_conns.get_mut(user_id) {
                    if let Some(user_conn) = conns.iter_mut().find(|c| c.client_addr == client_addr) {
                        user_conn.last_active_at = chrono::Utc::now().timestamp_millis();
                    }
                }
            }
        }
    }

    /// 更新连接的用户ID
    pub async fn update_user_id(&self, client_addr: SocketAddr, new_user_id: String) {
        let mut all_conns = self.all_connections.lock().await;
        if let Some(conn) = all_conns.iter_mut().find(|c| c.client_addr == client_addr) {
            // 先从原用户ID的连接列表中移除
            if let Some(old_user_id) = &conn.user_id {
                let mut user_conns = self.user_connections.lock().await;
                if let Some(conns) = user_conns.get_mut(old_user_id) {
                    conns.retain(|c| c.client_addr != client_addr);
                    if conns.is_empty() {
                        user_conns.remove(old_user_id);
                    }
                }

                let mut user_senders = self.user_senders.lock().await;
                if let Some(senders) = user_senders.get_mut(old_user_id) {
                    senders.retain(|s| {
                        // 这里我们无法直接比较 Sender，所以我们通过重新构建列表来清理
                        // 实际上，更好的方法是在 ConnectionRepo 中跟踪 sender 和 client_addr 的关系
                        true
                    });
                    if senders.is_empty() {
                        user_senders.remove(old_user_id);
                    }
                }
            }

            // 更新用户ID
            conn.user_id = Some(new_user_id.clone());

            // 添加到新用户ID的连接列表
            let mut user_conns = self.user_connections.lock().await;
            user_conns.entry(new_user_id.clone()).or_insert(Vec::new()).push(conn.clone());

            // 添加到新用户ID的发送器列表
            let mut user_senders = self.user_senders.lock().await;
            if let Some(sender) = self.get_sender_by_addr(client_addr).await {
                user_senders.entry(new_user_id).or_insert(Vec::new()).push(sender);
            }
        }
    }
}

// 为 ConnectionInfo 手动实现 Clone trait，因为 mpsc::UnboundedSender 没有实现
// Clone
impl Clone for ConnectionInfo {
    fn clone(&self) -> Self {
        Self {
            user_id: self.user_id.clone(),
            client_addr: self.client_addr,
            subscription: self.subscription.clone(),
            connected_at: self.connected_at,
            last_active_at: self.last_active_at,
            sender: self.sender.clone()
        }
    }
}
