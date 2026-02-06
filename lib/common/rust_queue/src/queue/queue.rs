use bytes::Bytes;
use serde::Serialize;
use tokio::sync::broadcast;

/// 转换为字节的 trait，类似于 Kafka 的 ToBytes
pub trait ToBytes {
    fn to_bytes(&self) -> Result<Bytes, Box<dyn std::error::Error>>;
}

/// 从字节转换的 trait，类似于 Kafka 的 FromBytes
pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>;
}

/// 为实现了 Serialize 的类型自动实现 ToBytes
impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Result<Bytes, Box<dyn std::error::Error>> {
        Ok(Bytes::from(serde_json::to_vec(self)?))
    }
}

/// 为实现了 DeserializeOwned 的类型自动实现 FromBytes
impl<T: serde::de::DeserializeOwned> FromBytes for T {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

/// 队列消息发送选项
#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    /// 是否等待至少一个消费者确认（Kafka 特定）
    pub require_ack: bool,
    /// 发送超时时间（毫秒）
    pub timeout_ms: u32,
    /// 启用背压机制（当队列满时阻塞发送）
    pub enable_backpressure: bool,
    /// 背压超时时间（毫秒）
    pub backpressure_timeout_ms: u32,
}

impl SendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_require_ack(mut self, require: bool) -> Self {
        self.require_ack = require;
        self
    }

    pub fn with_timeout(mut self, ms: u32) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn with_backpressure(mut self, enable: bool) -> Self {
        self.enable_backpressure = enable;
        self
    }

    pub fn with_backpressure_timeout(mut self, ms: u32) -> Self {
        self.backpressure_timeout_ms = ms;
        self
    }
}

/// 队列订阅选项
#[derive(Debug, Clone, Default)]
pub struct SubscribeOptions {
    /// 消费组 ID（用于实现组内消息只消费一次）
    pub group_id: Option<String>,
    /// 是否从最新位置开始消费（仅适用于首次订阅）
    pub from_latest: bool,
    /// 订阅超时时间（毫秒）
    pub timeout_ms: u32,
    /// 缓冲区大小（用于背压控制）
    pub buffer_size: usize,
}

impl SubscribeOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    pub fn with_from_latest(mut self, from_latest: bool) -> Self {
        self.from_latest = from_latest;
        self
    }

    pub fn with_timeout(mut self, ms: u32) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
}

/// 高性能异步队列接口
/// 支持多个 topic 和消费组功能
///
/// # 核心特性
/// - 支持多个 topic 的消息发布和订阅
/// - 每个 topic 可以有不同类型的 Event
/// - 消费组支持：组内消息只消费一次，组间广播
/// - 兼容本地广播（MPMCQueue）和 Kafka 持久化（KafkaQueue）
/// - 背压支持：防止队列过载
pub trait Queue {
    /// 创建新的队列实例
    /// 容量设置为 1024，足以应对高频 K 线更新场景
    fn new() -> Self;

    /// 创建带有自定义配置的队列实例
    /// 支持传递配置参数（如 Kafka brokers 地址等）
    fn new_with_config(config: impl Into<Self::Config>) -> Self
    where
        Self: Sized;

    /// 发送事件到指定 topic
    ///
    /// # 参数
    /// - `topic`: 目标 topic 名称
    /// - `event`: 要发送的事件
    /// - `options`: 发送选项
    ///
    /// # 返回
    /// 返回成功发送到的本地订阅者数量
    fn send(
        &self,
        topic: &str,
        event: bytes::Bytes,
        options: Option<SendOptions>,
    ) -> Result<usize, broadcast::error::SendError<bytes::Bytes>>;

    /// 批量发送事件到指定 topic（高性能优化）
    ///
    /// # 参数
    /// - `topic`: 目标 topic 名称
    /// - `events`: 要发送的事件列表
    /// - `options`: 发送选项
    ///
    /// # 返回
    /// 返回成功发送的事件数量和每个事件的接收者数量
    fn send_batch(
        &self,
        topic: &str,
        events: Vec<bytes::Bytes>,
        options: Option<SendOptions>,
    ) -> Result<Vec<Result<usize, broadcast::error::SendError<bytes::Bytes>>>, ()>;

    /// 订阅指定 topic 的事件
    ///
    /// # 参数
    /// - `topic`: 要订阅的 topic 名称
    /// - `options`: 订阅选项，支持消费组配置
    ///
    /// # 返回
    /// 返回一个接收器，用于异步接收事件
    fn subscribe(
        &self,
        topic: &str,
        options: Option<SubscribeOptions>,
    ) -> broadcast::Receiver<bytes::Bytes>;

    /// 获取指定 topic 的当前订阅者数量
    fn subscriber_count(&self, topic: &str) -> usize;

    fn get_or_create_channel(&self, topic: &str) -> broadcast::Sender<bytes::Bytes>;

    /// 获取所有支持的 topic 列表
    fn topics(&self) -> Vec<String>;

    /// 获取队列配置类型的关联类型
    type Config;
}

/// 默认 Queue 配置类型
#[derive(Debug, Clone)]
pub struct DefaultQueueConfig {
    /// Kafka brokers 地址（逗号分隔）
    pub brokers: String,
    /// 默认 topic 名称
    pub default_topic: String,
    /// 消费组 ID（全局默认值）
    pub default_group_id: String,
    /// 发送超时时间（毫秒）
    pub send_timeout_ms: u32,
    /// 接收超时时间（毫秒）
    pub recv_timeout_ms: u32,
    /// 全局缓冲区大小（用于背压控制）
    pub buffer_size: usize,
    /// 全局启用背压机制
    pub enable_backpressure: bool,
}

impl Default for DefaultQueueConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            default_topic: "kline-updates".to_string(),
            default_group_id: "kline-aggregator-group".to_string(),
            send_timeout_ms: 5000,
            recv_timeout_ms: 3000,
            buffer_size: 1024,
            enable_backpressure: false,
        }
    }
}

impl DefaultQueueConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_brokers(mut self, brokers: impl Into<String>) -> Self {
        self.brokers = brokers.into();
        self
    }

    pub fn with_default_topic(mut self, topic: impl Into<String>) -> Self {
        self.default_topic = topic.into();
        self
    }

    pub fn with_default_group_id(mut self, group_id: impl Into<String>) -> Self {
        self.default_group_id = group_id.into();
        self
    }

    pub fn with_send_timeout(mut self, ms: u32) -> Self {
        self.send_timeout_ms = ms;
        self
    }

    pub fn with_recv_timeout(mut self, ms: u32) -> Self {
        self.recv_timeout_ms = ms;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_backpressure(mut self, enable: bool) -> Self {
        self.enable_backpressure = enable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_options_builder() {
        let options = SendOptions::new()
            .with_require_ack(true)
            .with_timeout(10000)
            .with_backpressure(true)
            .with_backpressure_timeout(5000);

        assert!(options.require_ack);
        assert_eq!(options.timeout_ms, 10000);
        assert!(options.enable_backpressure);
        assert_eq!(options.backpressure_timeout_ms, 5000);
    }

    #[tokio::test]
    async fn test_subscribe_options_builder() {
        let options = SubscribeOptions::new()
            .with_group_id("test-group-123")
            .with_from_latest(false)
            .with_timeout(5000)
            .with_buffer_size(2048);

        assert_eq!(options.group_id, Some("test-group-123".to_string()));
        assert!(!options.from_latest);
        assert_eq!(options.timeout_ms, 5000);
        assert_eq!(options.buffer_size, 2048);
    }

    #[tokio::test]
    async fn test_default_config_builder() {
        let config = DefaultQueueConfig::new()
            .with_brokers("localhost:9092,localhost:9093")
            .with_send_timeout(3000)
            .with_recv_timeout(2000)
            .with_buffer_size(4096)
            .with_backpressure(true);

        assert_eq!(config.brokers, "localhost:9092,localhost:9093");
        assert_eq!(config.send_timeout_ms, 3000);
        assert_eq!(config.recv_timeout_ms, 2000);
        assert_eq!(config.buffer_size, 4096);
        assert!(config.enable_backpressure);
    }
}
