use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::extract::ws::Message;
use chrono;
use immutable_derive::immutable;
use tokio::sync::{mpsc, RwLock};

/// 连接信息结构体
#[derive(Debug)]
pub struct ConnectionInfo {
    /// 用户ID（可选，未认证用户可能没有）
    pub user_id: Option<String>,
    /// 客户端IP地址
    pub client_addr: SocketAddr,
    /// 订阅主题列表
    ///
    /// 支持的订阅模式：
    /// - `"*"` - 订阅所有事件
    /// - `"Order"` - 订阅所有订单事件
    /// - `"Order:123"` - 订阅特定订单的事件
    pub subscription: Vec<String>,
    /// 连接建立时间戳（毫秒）
    pub connected_at: i64,
    /// 最后活动时间戳（毫秒）
    pub last_active_at: i64,
    /// 消息发送器
    pub sender: mpsc::UnboundedSender<Message>
}

// 为 ConnectionInfo 手动实现 Clone trait
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


/// 连接仓储 - 有状态服务
///
/// 该仓储负责在内存中管理所有 WebSocket 连接的状态。
/// 虽然它是有状态的，但通过 Arc<RwLock> 实现了高效的多线程共享访问。
///
/// # 设计说明
///
/// ConnectionRepo 是一个**有状态的仓储服务**，这是合理的，因为：
/// 1. 它的核心职责就是管理连接的运行时状态
/// 2. 内存中的状态管理比数据库查询快得多，符合低延迟要求
/// 3. 使用 RwLock 而非 Mutex 优化了读多写少的场景
///
/// # 性能优化
///
/// - 使用 `RwLock` 代替 `Mutex`，允许多个并发读取
/// - 数据结构冗余存储，用空间换时间（避免重复遍历）
/// - 所有方法都是 `&self`，支持高效共享
#[derive(Debug, Clone, Default)]
#[immutable]
pub struct ConnectionRepo {
    /// 用户连接映射：user_id -> Vec<ConnectionInfo>
    /// 用于快速查找某个用户的所有连接
    user_connections: Arc<RwLock<HashMap<String, Vec<ConnectionInfo>>>>,

    /// 所有连接列表（按 client_addr 索引）
    /// 用于快速查找和遍历所有连接
    all_connections: Arc<RwLock<HashMap<SocketAddr, ConnectionInfo>>>,

    /// 连接发送器映射：client_addr -> Sender
    /// 用于快速向特定连接发送消息
    connection_senders: Arc<RwLock<HashMap<SocketAddr, mpsc::UnboundedSender<Message>>>>,

    /// 用户发送器映射：user_id -> Vec<Sender>
    /// 用于快速向某个用户的所有连接发送消息
    user_senders: Arc<RwLock<HashMap<String, Vec<mpsc::UnboundedSender<Message>>>>>
}

impl ConnectionRepo {
    /// 创建新的连接管理器实例

    /// 根据事件类型获取对该事件感兴趣的所有发送器
    ///
    /// # 参数
    /// - `event_type`: 事件类型（如 "Order"）
    ///
    /// # 性能优化
    /// 使用读锁，允许多个并发查询
    pub async fn get_senders_by_event(&self, event_type: &str) -> Vec<mpsc::UnboundedSender<Message>> {
        let mut matched_senders = Vec::new();
        let all_conns = self.all_connections.read().await;

        for conn in all_conns.values() {
            if conn.subscription.iter().any(|sub| sub == event_type || sub == "*" || sub.contains(event_type)) {
                matched_senders.push(conn.sender.clone());
            }
        }

        matched_senders
    }

    /// 根据实体类型和实体ID获取相关的发送器
    ///
    /// # 参数
    /// - `entity_type`: 实体类型（如 "Order"）
    /// - `entity_id`: 实体ID（如 "123"）
    ///
    /// # 性能优化
    /// 使用读锁，允许多个并发查询
    pub async fn get_senders_by_entity(
        &self, entity_type: &str, entity_id: &str
    ) -> Vec<mpsc::UnboundedSender<Message>> {
        let mut matched_senders = Vec::new();
        let all_conns = self.all_connections.read().await;

        let specific_topic = format!("{}:{}", entity_type, entity_id);

        for conn in all_conns.values() {
            if conn
                .subscription
                .iter()
                .any(|sub| sub.as_str() == entity_type || sub == &specific_topic || sub.as_str() == "*")
            {
                matched_senders.push(conn.sender.clone());
            }
        }

        matched_senders
    }

    /// 添加新连接
    ///
    /// # 参数
    /// - `conn_info`: 连接信息
    ///
    /// # 性能考虑
    /// 虽然需要写锁，但添加连接的频率远低于查询，因此总体性能良好
    pub async fn add_connection(&self, conn_info: ConnectionInfo) {
        let client_addr = conn_info.client_addr;

        // 添加到所有连接映射
        {
            let mut all_conns = self.all_connections.write().await;
            all_conns.insert(client_addr, conn_info.clone());
        }

        // 添加到发送器映射
        {
            let mut conn_senders = self.connection_senders.write().await;
            conn_senders.insert(client_addr, conn_info.sender.clone());
        }

        // 如果有用户ID，添加到用户相关映射
        if let Some(user_id) = &conn_info.user_id {
            {
                let mut user_conns = self.user_connections.write().await;
                user_conns.entry(user_id.clone()).or_insert_with(Vec::new).push(conn_info.clone());
            }

            {
                let mut user_senders = self.user_senders.write().await;
                user_senders.entry(user_id.clone()).or_insert_with(Vec::new).push(conn_info.sender.clone());
            }
        }

        tracing::info!(
            client_addr = %client_addr,
            user_id = ?conn_info.user_id,
            subscriptions = ?conn_info.subscription,
            "Connection added"
        );
    }

    /// 移除连接
    ///
    /// # 参数
    /// - `client_addr`: 客户端地址
    pub async fn remove_connection(&self, client_addr: SocketAddr) {
        // 从所有连接映射中移除并获取连接信息
        let removed_conn = {
            let mut all_conns = self.all_connections.write().await;
            all_conns.remove(&client_addr)
        };

        if let Some(conn) = removed_conn {
            // 从发送器映射中移除
            {
                let mut conn_senders = self.connection_senders.write().await;
                conn_senders.remove(&client_addr);
            }

            // 如果有用户ID，从用户相关映射中移除
            if let Some(user_id) = &conn.user_id {
                {
                    let mut user_conns = self.user_connections.write().await;
                    if let Some(conns) = user_conns.get_mut(user_id) {
                        conns.retain(|c| c.client_addr != client_addr);
                        if conns.is_empty() {
                            user_conns.remove(user_id);
                        }
                    }
                }

                {
                    let mut user_senders = self.user_senders.write().await;
                    if let Some(senders) = user_senders.get_mut(user_id) {
                        // 注意：由于 Sender 没有实现 PartialEq，我们无法精确移除
                        // 这里简单地保留所有发送器，依赖 mpsc 的自动清理机制
                        // 当接收端关闭时，发送操作会失败
                        senders.clear();
                        user_senders.remove(user_id);
                    }
                }
            }

            tracing::info!(
                client_addr = %client_addr,
                user_id = ?conn.user_id,
                "Connection removed"
            );
        }
    }

    /// 根据用户ID获取所有连接信息
    pub async fn get_connections_by_user(&self, user_id: &str) -> Vec<ConnectionInfo> {
        let user_conns = self.user_connections.read().await;
        user_conns.get(user_id).cloned().unwrap_or_default()
    }

    /// 根据用户ID获取所有发送器
    pub async fn get_senders_by_user(&self, user_id: &str) -> Vec<mpsc::UnboundedSender<Message>> {
        let user_senders = self.user_senders.read().await;
        user_senders.get(user_id).cloned().unwrap_or_default()
    }

    /// 根据客户端地址获取发送器
    pub async fn get_sender_by_addr(&self, client_addr: SocketAddr) -> Option<mpsc::UnboundedSender<Message>> {
        let conn_senders = self.connection_senders.read().await;
        conn_senders.get(&client_addr).cloned()
    }

    /// 获取所有连接信息
    pub async fn get_all_connections(&self) -> Vec<ConnectionInfo> {
        let all_conns = self.all_connections.read().await;
        all_conns.values().cloned().collect()
    }

    /// 获取所有发送器
    pub async fn get_all_senders(&self) -> Vec<mpsc::UnboundedSender<Message>> {
        let conn_senders = self.connection_senders.read().await;
        conn_senders.values().cloned().collect()
    }

    /// 获取用户连接数量
    pub async fn get_user_connection_count(&self, user_id: &str) -> usize {
        let user_conns = self.user_connections.read().await;
        user_conns.get(user_id).map(|v| v.len()).unwrap_or(0)
    }

    /// 获取总连接数量
    pub async fn get_total_connection_count(&self) -> usize {
        let all_conns = self.all_connections.read().await;
        all_conns.len()
    }

    /// 更新连接活动时间
    ///
    /// # 性能考虑
    /// 这是一个写操作，但频率相对较低
    pub async fn update_last_active(&self, client_addr: SocketAddr) {
        let now = chrono::Utc::now().timestamp_millis();
        let user_id_opt = {
            let mut all_conns = self.all_connections.write().await;
            if let Some(conn) = all_conns.get_mut(&client_addr) {
                conn.last_active_at = now;
                conn.user_id.clone()
            } else {
                None
            }
        };

        // 同步更新用户连接映射中的时间戳
        if let Some(user_id) = user_id_opt {
            let mut user_conns = self.user_connections.write().await;
            if let Some(conns) = user_conns.get_mut(&user_id) {
                if let Some(user_conn) = conns.iter_mut().find(|c| c.client_addr == client_addr) {
                    user_conn.last_active_at = now;
                }
            }
        }
    }

    /// 更新连接的用户ID（用于认证后绑定用户）
    ///
    /// # 参数
    /// - `client_addr`: 客户端地址
    /// - `new_user_id`: 新的用户ID
    pub async fn update_user_id(&self, client_addr: SocketAddr, new_user_id: String) {
        let mut all_conns = self.all_connections.write().await;

        if let Some(conn) = all_conns.get_mut(&client_addr) {
            let old_user_id = conn.user_id.clone();

            // 从旧用户ID的映射中移除
            if let Some(old_id) = &old_user_id {
                drop(all_conns); // 释放 all_conns 的锁以避免死锁

                {
                    let mut user_conns = self.user_connections.write().await;
                    if let Some(conns) = user_conns.get_mut(old_id) {
                        conns.retain(|c| c.client_addr != client_addr);
                        if conns.is_empty() {
                            user_conns.remove(old_id);
                        }
                    }
                }

                {
                    let mut user_senders = self.user_senders.write().await;
                    user_senders.remove(old_id);
                }

                // 重新获取锁
                all_conns = self.all_connections.write().await;
            }

            // 更新连接的用户ID
            if let Some(conn) = all_conns.get_mut(&client_addr) {
                conn.user_id = Some(new_user_id.clone());
                let conn_clone = conn.clone();

                drop(all_conns); // 释放锁

                // 添加到新用户ID的映射
                {
                    let mut user_conns = self.user_connections.write().await;
                    user_conns.entry(new_user_id.clone()).or_insert_with(Vec::new).push(conn_clone.clone());
                }

                {
                    let mut user_senders = self.user_senders.write().await;
                    user_senders.entry(new_user_id.clone()).or_insert_with(Vec::new).push(conn_clone.sender.clone());
                }

                tracing::info!(
                    client_addr = %client_addr,
                    old_user_id = ?old_user_id,
                    new_user_id = %new_user_id,
                    "User ID updated"
                );
            }
        }
    }
}
