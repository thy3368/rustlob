use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::entity::PendingRequest;

/// 内存池端口 - 定义写入和读取 pending requests 的接口
pub trait MempoolPort: Send + Sync {
    /// 将准入的请求写入内存池
    fn add_requests(&self, requests: Vec<PendingRequest>) -> Result<(), MempoolError>;

    /// 按数量拉取待处理请求（用于区块执行）
    fn fetch_requests(&self, limit: usize) -> Result<Vec<PendingRequest>, MempoolError>;

    /// 获取当前池中的请求数量
    fn len(&self) -> usize;

    /// 检查池是否为空
    fn is_empty(&self) -> bool;

    /// 清空内存池
    fn clear(&self);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MempoolError {
    PoolFull,
    StorageError(String),
}

/// 基于内存的 Mempool 实现
/// 使用 Arc<Mutex<>> 支持多线程并发访问
#[derive(Debug, Clone, Default)]
pub struct InMemoryMempool {
    inner: Arc<Mutex<HashMap<String, PendingRequest>>>,
}

impl InMemoryMempool {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MempoolPort for InMemoryMempool {
    fn add_requests(&self, requests: Vec<PendingRequest>) -> Result<(), MempoolError> {
        let mut pool = self.inner.lock().map_err(|e| {
            MempoolError::StorageError(format!("Lock poisoned: {}", e))
        })?;

        for request in requests {
            pool.insert(request.request_id.clone(), request);
        }

        Ok(())
    }

    fn fetch_requests(&self, limit: usize) -> Result<Vec<PendingRequest>, MempoolError> {
        let mut pool = self.inner.lock().map_err(|e| {
            MempoolError::StorageError(format!("Lock poisoned: {}", e))
        })?;

        let keys_to_remove: Vec<String> = pool.keys().take(limit).cloned().collect();
        let mut requests = Vec::with_capacity(keys_to_remove.len());
        for key in keys_to_remove {
            if let Some(request) = pool.remove(&key) {
                requests.push(request);
            }
        }

        Ok(requests)
    }

    fn len(&self) -> usize {
        self.inner
            .lock()
            .map(|pool| pool.len())
            .unwrap_or(0)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&self) {
        if let Ok(mut pool) = self.inner.lock() {
            pool.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pending_request(id: &str) -> PendingRequest {
        PendingRequest {
            request_id: id.to_string(),
            performer: "acct-1".to_string(),
            action_type: "order".to_string(),
            payload_hash: format!("payload-{}", id),
        }
    }

    #[test]
    fn add_and_fetch_requests() {
        let mempool = InMemoryMempool::new();

        // Add requests
        mempool.add_requests(vec![
            pending_request("req-1"),
            pending_request("req-2"),
        ]).unwrap();

        assert_eq!(mempool.len(), 2);

        // Fetch all
        let fetched = mempool.fetch_requests(10).unwrap();
        assert_eq!(fetched.len(), 2);
        assert!(mempool.is_empty());
    }

    #[test]
    fn fetch_with_limit() {
        let mempool = InMemoryMempool::new();

        mempool.add_requests(vec![
            pending_request("req-1"),
            pending_request("req-2"),
            pending_request("req-3"),
        ]).unwrap();

        // Fetch only 2
        let fetched = mempool.fetch_requests(2).unwrap();
        assert_eq!(fetched.len(), 2);
        assert_eq!(mempool.len(), 1); // 1 remaining
    }

    #[test]
    fn fetch_drains_pool() {
        let mempool = InMemoryMempool::new();

        mempool.add_requests(vec![pending_request("req-1")]).unwrap();
        assert_eq!(mempool.len(), 1);

        mempool.fetch_requests(10).unwrap();
        assert_eq!(mempool.len(), 0);
    }
}
