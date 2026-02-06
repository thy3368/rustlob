use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use pingora::upstreams::peer::HttpPeer;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// 用户路由配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRouteConfig {
    /// 用户ID到后端IP列表的映射
    pub partition_ips: HashMap<usize, Vec<String>>,
    pub num_partitions: usize,
    /// 默认后端地址（当用户未配置时使用）
    pub default_backend: String,
}

impl Default for UserRouteConfig {
    fn default() -> Self {
        // let mut user_partition = HashMap::new();
        let mut partition_ips = HashMap::new();
        // 示例配置：不同用户路由到不同的后端服务器
        partition_ips.insert(1, vec!["127.0.0.1:3001".to_string(), "127.0.0.1:3002".to_string()]);
        partition_ips.insert(2, vec!["127.0.0.1:3003".to_string(), "127.0.0.1:3004".to_string()]);
        partition_ips.insert(3, vec!["127.0.0.1:3001".to_string()]);
        partition_ips.insert(4, vec!["127.0.0.1:3002".to_string()]);

        UserRouteConfig {
            partition_ips,
            num_partitions: 10,
            default_backend: "127.0.0.1:3001".to_string(),
        }
    }
}

/// 用户路由器 - 根据用户ID选择后端服务器
pub struct UserRouter {
    config: Arc<RwLock<UserRouteConfig>>,
    /// 轮询索引，用于负载均衡（用户ID -> 当前索引）
    round_robin_index: Arc<RwLock<HashMap<String, usize>>>,
}

impl UserRouter {
    /// 创建新的用户路由器
    pub fn new(config: UserRouteConfig) -> Self {
        UserRouter {
            config: Arc::new(RwLock::new(config)),
            round_robin_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn select_partition(&self, user_id: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        user_id.hash(&mut hasher);
        let hash = hasher.finish();

        let config = self.config.read().await;

        //todo fix 语法错
        (hash % config.num_partitions as u64) + 1
    }

    /// 根据用户ID选择后端服务器（轮询负载均衡）
    pub async fn select_backend(&self, user_id: &str) -> HttpPeer {
        let config = self.config.read().await;

        if let partition = self.select_partition(user_id).await {
            let backends = config.partition_ips.get(&partition).unwrap();
            if backends.is_empty() {
                return self.create_peer(&config.default_backend);
            }

            // 获取或初始化轮询索引
            let mut indices = self.round_robin_index.write().await;
            let index = indices.entry(user_id.to_string()).or_insert(0);

            // 选择后端（轮询）
            let backend = &backends[*index % backends.len()];
            *index = (*index + 1) % backends.len();

            self.create_peer(backend)
        } else {
            // 用户未配置，使用默认后端
            self.create_peer(&config.default_backend)
        }
    }

    /// 更新路由配置（热更新）
    pub async fn update_config(&self, new_config: UserRouteConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    /// 创建 HttpPeer
    fn create_peer(&self, addr: &str) -> HttpPeer {
        HttpPeer::new(addr, false, "localhost".to_string())
    }
}

/// 从 HTTP 请求中提取用户ID
pub struct UserIdExtractor;

impl UserIdExtractor {
    /// 从 JSON 请求体中提取 user_id 或 trader_id
    ///
    /// 支持的字段名：
    /// - user_id
    /// - userId
    /// - trader_id
    /// - traderId
    /// - uid
    pub fn extract_from_json(body: &[u8]) -> Option<String> {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(body) {
            // 尝试多个可能的字段名
            let possible_fields =
                ["user_id", "userId", "trader_id", "traderId", "uid", "accountId", "account_id"];

            for field in &possible_fields {
                if let Some(user_id) = json.get(field) {
                    if let Some(user_id_str) = user_id.as_str() {
                        return Some(user_id_str.to_string());
                    } else if let Some(user_id_num) = user_id.as_u64() {
                        return Some(user_id_num.to_string());
                    }
                }
            }
        }
        None
    }

    /// 从 HTTP 请求头中提取用户ID
    ///
    /// 支持的请求头：
    /// - X-User-Id
    /// - X-Trader-Id
    /// - Authorization (Bearer token中的用户ID)
    pub fn extract_from_headers(headers: &str) -> Option<String> {
        for line in headers.lines() {
            if line.to_lowercase().starts_with("x-user-id:") {
                return Some(line.split(':').nth(1)?.trim().to_string());
            }
            if line.to_lowercase().starts_with("x-trader-id:") {
                return Some(line.split(':').nth(1)?.trim().to_string());
            }
        }
        None
    }

    /// 从 URL 查询参数中提取用户ID
    /// 例如：/api/spot/v2/?user_id=alice
    pub fn extract_from_query(url: &str) -> Option<String> {
        if let Some(query_start) = url.find('?') {
            let query = &url[query_start + 1..];
            for param in query.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    if key == "user_id" || key == "userId" || key == "trader_id" {
                        return Some(value.to_string());
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use pingora_core::upstreams::peer::Peer;

    use super::*;

    #[tokio::test]
    async fn test_user_router_selection() {
        let config = UserRouteConfig::default();
        let router = UserRouter::new(config);

        // 测试已配置用户
        let peer1 = router.select_backend("alice").await;
        assert!(peer1.address().to_string().contains("3001"));

        let peer2 = router.select_backend("bob").await;
        assert!(peer2.address().to_string().contains("3002"));

        // 测试未配置用户（使用默认后端）
        let peer_default = router.select_backend("unknown_user").await;
        assert!(peer_default.address().to_string().contains("3001"));
    }

    async fn test_user_router_selection2() {
        let config = UserRouteConfig::default();
        let router = UserRouter::new(config);

        // 测试已配置用户
        let peer1 = router.select_backend("alice").await;
        assert!(peer1.address().to_string().contains("3001"));

        let peer2 = router.select_backend("bob").await;
        assert!(peer2.address().to_string().contains("3002"));

        // 测试未配置用户（使用默认后端）
        let peer_default = router.select_backend("unknown_user").await;
        assert!(peer_default.address().to_string().contains("3001"));
    }

    #[tokio::test]
    async fn test_round_robin() {
        let config = UserRouteConfig::default();
        let router = UserRouter::new(config);

        // user_1 有两个后端，测试轮询
        let peer1 = router.select_backend("user_1").await;
        let peer2 = router.select_backend("user_1").await;
        let peer3 = router.select_backend("user_1").await;

        // 应该轮询在 3001 和 3002 之间
        assert!(
            peer1.address().to_string().contains("3001")
                || peer1.address().to_string().contains("3002")
        );
    }

    #[test]
    fn test_extract_user_id_from_json() {
        let json_body = r#"{"user_id": "alice", "price": 50000}"#;
        let user_id = UserIdExtractor::extract_from_json(json_body.as_bytes());
        assert_eq!(user_id, Some("alice".to_string()));

        let json_body2 = r#"{"trader_id": "bob", "quantity": 10}"#;
        let user_id2 = UserIdExtractor::extract_from_json(json_body2.as_bytes());
        assert_eq!(user_id2, Some("bob".to_string()));

        let json_body3 = r#"{"uid": 12345}"#;
        let user_id3 = UserIdExtractor::extract_from_json(json_body3.as_bytes());
        assert_eq!(user_id3, Some("12345".to_string()));
    }

    #[test]
    fn test_extract_user_id_from_headers() {
        let headers = "Content-Type: application/json\nX-User-Id: alice\nAccept: */*";
        let user_id = UserIdExtractor::extract_from_headers(headers);
        assert_eq!(user_id, Some("alice".to_string()));
    }

    #[test]
    fn test_extract_user_id_from_query() {
        let url = "/api/spot/v2/?user_id=alice&symbol=BTCUSDT";
        let user_id = UserIdExtractor::extract_from_query(url);
        assert_eq!(user_id, Some("alice".to_string()));
    }
}
