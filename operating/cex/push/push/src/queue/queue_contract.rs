use bytes::Bytes;
use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Debug, Clone, thiserror::Error)]
pub enum QueueBatchError {
    #[error("队列批量发送失败: {0}")]
    SendFailed(String),
}

pub type QueueSendResult = Result<usize, broadcast::error::SendError<bytes::Bytes>>;
pub type QueueBatchSendResult = Result<Vec<QueueSendResult>, QueueBatchError>;

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Bytes, Box<dyn std::error::Error>>;
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>;
}

impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Result<Bytes, Box<dyn std::error::Error>> {
        Ok(Bytes::from(serde_json::to_vec(self)?))
    }
}

impl<T: serde::de::DeserializeOwned> FromBytes for T {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    pub require_ack: bool,
    pub timeout_ms: u32,
    pub enable_backpressure: bool,
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

#[derive(Debug, Clone, Default)]
pub struct ChannelConfig {
    pub buffer_size: Option<usize>,
    pub num_partitions: Option<i32>,
    pub replication_factor: Option<i32>,
}

impl ChannelConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = Some(size);
        self
    }

    pub fn with_num_partitions(mut self, num: i32) -> Self {
        self.num_partitions = Some(num);
        self
    }

    pub fn with_replication_factor(mut self, factor: i32) -> Self {
        self.replication_factor = Some(factor);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct SubscribeOptions {
    pub group_id: Option<String>,
    pub from_latest: bool,
    pub timeout_ms: u32,
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

#[derive(Debug, Clone)]
pub struct DefaultQueueConfig {
    pub brokers: String,
    pub default_topic: String,
    pub default_group_id: String,
    pub send_timeout_ms: u32,
    pub recv_timeout_ms: u32,
    pub buffer_size: usize,
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

pub trait Queue {
    fn new() -> Self;

    fn new_with_config(config: impl Into<Self::Config>) -> Self
    where
        Self: Sized;

    fn send(
        &self,
        topic: &str,
        event: bytes::Bytes,
        options: Option<SendOptions>,
    ) -> Result<usize, broadcast::error::SendError<bytes::Bytes>>;

    fn send_batch(
        &self,
        topic: &str,
        events: Vec<bytes::Bytes>,
        options: Option<SendOptions>,
    ) -> QueueBatchSendResult;

    fn subscribe(
        &self,
        topic: &str,
        options: Option<SubscribeOptions>,
    ) -> broadcast::Receiver<bytes::Bytes>;

    fn subscriber_count(&self, topic: &str) -> usize;

    fn get_or_create_channel(
        &self,
        topic: &str,
        config: Option<ChannelConfig>,
    ) -> broadcast::Sender<bytes::Bytes>;

    fn topics(&self) -> Vec<String>;

    type Config;
}
