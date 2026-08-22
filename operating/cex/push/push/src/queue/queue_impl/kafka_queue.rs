use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::queue::queue_contract::{
    ChannelConfig, DefaultQueueConfig, Queue, QueueBatchSendResult, SendOptions, SubscribeOptions,
};
use crate::queue::queue_impl::mpmc_queue::MPMCQueue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub default_group_id: String,
    pub send_timeout_ms: i32,
    pub recv_timeout_ms: i32,
    pub buffer_size: usize,
    pub enable_backpressure: bool,
    pub default_num_partitions: i32,
    pub default_replication_factor: i32,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            default_group_id: "kline-aggregator-group".to_string(),
            send_timeout_ms: 5000,
            recv_timeout_ms: 3000,
            buffer_size: 1024,
            enable_backpressure: false,
            default_num_partitions: 3,
            default_replication_factor: 1,
        }
    }
}

impl KafkaConfig {
    pub fn with_num_partitions(mut self, num_partitions: i32) -> Self {
        self.default_num_partitions = num_partitions;
        self
    }

    pub fn with_replication_factor(mut self, replication_factor: i32) -> Self {
        self.default_replication_factor = replication_factor;
        self
    }
}

impl From<DefaultQueueConfig> for KafkaConfig {
    fn from(config: DefaultQueueConfig) -> Self {
        Self {
            brokers: config.brokers,
            default_group_id: config.default_group_id,
            send_timeout_ms: config.send_timeout_ms as i32,
            recv_timeout_ms: config.recv_timeout_ms as i32,
            buffer_size: config.buffer_size,
            enable_backpressure: config.enable_backpressure,
            default_num_partitions: 3,
            default_replication_factor: 1,
        }
    }
}

impl From<KafkaConfig> for DefaultQueueConfig {
    fn from(config: KafkaConfig) -> Self {
        Self {
            brokers: config.brokers,
            default_topic: "kline-updates".to_string(),
            default_group_id: config.default_group_id,
            send_timeout_ms: config.send_timeout_ms.max(0) as u32,
            recv_timeout_ms: config.recv_timeout_ms.max(0) as u32,
            buffer_size: config.buffer_size,
            enable_backpressure: config.enable_backpressure,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KafkaQueue {
    inner: Arc<MPMCQueue>,
}

impl KafkaQueue {
    pub fn from_config(config: KafkaConfig) -> Self {
        let inner = Arc::new(MPMCQueue::from_config(config.clone().into()));
        Self { inner }
    }

    fn inner(&self) -> &MPMCQueue {
        &self.inner
    }
}

impl Queue for KafkaQueue {
    type Config = KafkaConfig;

    fn new() -> Self {
        Self::from_config(KafkaConfig::default())
    }

    fn new_with_config(config: impl Into<Self::Config>) -> Self {
        Self::from_config(config.into())
    }

    fn send(
        &self,
        topic: &str,
        event: bytes::Bytes,
        options: Option<SendOptions>,
    ) -> Result<usize, broadcast::error::SendError<bytes::Bytes>> {
        self.inner().send(topic, event, options)
    }

    fn send_batch(
        &self,
        topic: &str,
        events: Vec<bytes::Bytes>,
        options: Option<SendOptions>,
    ) -> QueueBatchSendResult {
        self.inner().send_batch(topic, events, options)
    }

    fn subscribe(
        &self,
        topic: &str,
        options: Option<SubscribeOptions>,
    ) -> broadcast::Receiver<bytes::Bytes> {
        self.inner().subscribe(topic, options)
    }

    fn subscriber_count(&self, topic: &str) -> usize {
        self.inner().subscriber_count(topic)
    }

    fn get_or_create_channel(
        &self,
        topic: &str,
        config: Option<ChannelConfig>,
    ) -> broadcast::Sender<bytes::Bytes> {
        self.inner().get_or_create_channel(topic, config)
    }

    fn topics(&self) -> Vec<String> {
        self.inner().topics()
    }
}
