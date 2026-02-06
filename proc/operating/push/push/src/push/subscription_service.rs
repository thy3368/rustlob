use std::net::SocketAddr;
use std::sync::Arc;

use immutable_derive::immutable;

use crate::push::connection_types::{ConnectionInfo, ConnectionRepo};
// use serde_json::json;

/// 订阅服务 - 无状态设计，可安全地在多线程间共享
///
/// 该服务只包含不可变的依赖引用，不包含任何运行时状态，
/// 因此可以被多个线程同时访问而无需克隆。
#[immutable]
pub struct SubscriptionService {
    /// 连接管理仓储（不可变引用）
    connection_repo: Arc<ConnectionRepo>,
}

impl SubscriptionService {
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
}
