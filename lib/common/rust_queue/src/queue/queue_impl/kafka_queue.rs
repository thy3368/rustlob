use std::sync::Arc;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::types::RDKafkaErrorCode;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::queue::queue::{ChannelConfig, DefaultQueueConfig, Queue, SendOptions, SubscribeOptions};

/// Kafka 配置（兼容 Queue trait 的 Config 关联类型）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    /// Kafka brokers 地址 (逗号分隔)
    pub brokers: String,
    /// 消费组 ID（全局默认值）
    pub default_group_id: String,
    /// 发送超时 (毫秒)
    pub send_timeout_ms: i32,
    /// 接收超时 (毫秒)
    pub recv_timeout_ms: i32,
    /// 全局缓冲区大小（用于背压控制）
    pub buffer_size: usize,
    /// 全局启用背压机制
    pub enable_backpressure: bool,
    /// 默认分区数（用于 topic 创建）
    pub default_num_partitions: i32,
    /// 默认副本数（用于 topic 创建）
    pub default_replication_factor: i32,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        KafkaConfig {
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

impl From<DefaultQueueConfig> for KafkaConfig {
    fn from(config: DefaultQueueConfig) -> Self {
        KafkaConfig {
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

/// Kafka 队列实现
/// 用于连接到 Kafka 集群，发送和接收 K 线更新事件
/// 支持多个 topic 和消费组功能
pub struct KafkaQueue {
    config: KafkaConfig,
    producer: Arc<FutureProducer>,
    topic_channels:
        Arc<std::sync::Mutex<std::collections::HashMap<String, broadcast::Sender<bytes::Bytes>>>>,
}

impl KafkaQueue {
    /// 创建新的 Kafka 队列
    pub fn new_with_config(config: KafkaConfig) -> Self {
        // 创建 Kafka 生产者
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", &config.send_timeout_ms.to_string())
            .set("acks", "1") // 至少一个副本确认
            .set("linger.ms", "0") // 立即发送
            .set("retries", "3")
            .create()
            .expect("Failed to create Kafka producer");

        let topic_channels = Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));

        let queue = KafkaQueue { config, producer: Arc::new(producer), topic_channels };

        queue
    }

    /// 使用默认配置创建新的 Kafka 队列
    pub fn new() -> Self {
        Self::new_with_config(KafkaConfig::default())
    }

    /// 检查是否需要背压控制
    fn should_apply_backpressure(&self, options: &Option<SendOptions>) -> bool {
        match options {
            Some(opts) => opts.enable_backpressure,
            None => self.config.enable_backpressure,
        }
    }

    /// 启动 Kafka 消费者任务
    fn spawn_consumer_task(&self, topic: &str, tx: broadcast::Sender<bytes::Bytes>) {
        let config = self.config.clone();
        let topic = topic.to_string();

        tokio::spawn(async move {
            if let Err(e) = Self::consumer_loop(config, topic, tx).await {
                tracing::error!("Kafka consumer loop failed: {}", e);
            }
        });
    }

    /// Kafka 消费者循环
    async fn consumer_loop(
        config: KafkaConfig,
        topic: String,
        tx: broadcast::Sender<bytes::Bytes>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.default_group_id)
            .set("auto.offset.reset", "latest") // 从最新位置开始消费
            .set("enable.auto.commit", "true")
            .set("auto.commit.interval.ms", "5000")
            .create()
            .expect("Failed to create Kafka consumer");

        consumer.subscribe(&[&topic])?;

        tracing::info!("Kafka consumer subscribed to topic: {}", topic);

        loop {
            match consumer.recv().await {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        // 直接转发字节数据，不做反序列化
                        let _ = tx.send(bytes::Bytes::copy_from_slice(payload));
                    }
                }
                Err(e) => {
                    tracing::error!("Kafka consumer error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// 异步发送事件到 Kafka
    pub async fn send_async(
        &self,
        topic: &str,
        event: bytes::Bytes,
        options: Option<SendOptions>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let record = FutureRecord::to(topic).key("kline-update".as_bytes()).payload(&event[..]);

        let timeout =
            options.map(|o| o.timeout_ms as u64).unwrap_or(self.config.send_timeout_ms as u64);

        match self.producer.send(record, tokio::time::Duration::from_millis(timeout)).await {
            Ok((..)) => {
                // 同时发送到本地订阅者
                let tx = self.get_or_create_channel(topic, None);
                let count = tx.send(event)?;
                Ok(count)
            }
            Err((e, _)) => Err(Box::new(e)),
        }
    }

    /// 创建 Kafka topic（如果不存在）
    async fn create_topic_if_not_exists(
        &self,
        topic: &str,
        config: Option<ChannelConfig>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", &self.config.brokers)
            .create()?;

        // 优先使用传入的配置，其次使用全局默认配置
        let num_partitions = config
            .as_ref()
            .and_then(|c| c.num_partitions)
            .unwrap_or(self.config.default_num_partitions);
        let replication_factor = config
            .as_ref()
            .and_then(|c| c.replication_factor)
            .unwrap_or(self.config.default_replication_factor);

        let new_topic = NewTopic {
            name: topic,
            num_partitions,
            replication: TopicReplication::Fixed(replication_factor),
            config: vec![],
        };

        let opts = AdminOptions::new();

        match admin_client.create_topics(&[new_topic], &opts).await {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(_) => {
                            tracing::info!(
                                "Kafka topic '{}' created successfully (partitions: {}, replication: {})",
                                topic, num_partitions, replication_factor
                            );
                        }
                        Err((name, code)) => {
                            if code == RDKafkaErrorCode::TopicAlreadyExists {
                                tracing::debug!("Kafka topic '{}' already exists", name);
                            } else {
                                tracing::warn!("Failed to create topic '{}': {:?}", name, code);
                            }
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                tracing::warn!("Failed to create topic '{}': {}", topic, e);
                Ok(()) // 即使创建失败也继续，因为发送时可能会自动创建
            }
        }
    }
}

impl Queue for KafkaQueue {
    type Config = KafkaConfig;

    /// 创建新的 Kafka 队列
    fn new() -> Self {
        Self::new()
    }

    /// 获取或创建 topic 对应的 broadcast channel
    fn get_or_create_channel(
        &self,
        topic: &str,
        config: Option<ChannelConfig>,
    ) -> broadcast::Sender<bytes::Bytes> {
        let mut channels = self.topic_channels.lock().unwrap();
        channels
            .entry(topic.to_string())
            .or_insert_with(|| {
                // 优先使用传入的配置，其次使用全局配置
                let buffer_size = config
                    .as_ref()
                    .and_then(|c| c.buffer_size)
                    .unwrap_or_else(|| {
                        if self.config.buffer_size > 0 {
                            self.config.buffer_size
                        } else {
                            1024
                        }
                    });
                let (tx, _) = broadcast::channel(buffer_size);
                let topic_str = topic.to_string();
                // 异步创建 Kafka topic（如果不存在）
                let queue = self.clone();
                let topic_for_async = topic_str.clone();
                let channel_config_for_topic = config.clone();
                tokio::spawn(async move {
                    if let Err(e) = queue.create_topic_if_not_exists(&topic_for_async, channel_config_for_topic).await {
                        tracing::warn!("Failed to create topic '{}': {}", topic_for_async, e);
                    }
                });
                // 启动 Kafka 消费者任务
                self.spawn_consumer_task(&topic_str, tx.clone());
                tx
            })
            .clone()
    }

    fn new_with_config(config: impl Into<Self::Config>) -> Self
    where
        Self: Sized,
    {
        Self::new_with_config(config.into())
    }

    fn send(
        &self,
        topic: &str,
        event: bytes::Bytes,
        options: Option<SendOptions>,
    ) -> Result<usize, broadcast::error::SendError<bytes::Bytes>> {
        // 背压控制
        if self.should_apply_backpressure(&options) {
            let channel = self.get_or_create_channel(topic, None);
            let current_subscribers = channel.receiver_count();
            let channel_capacity = self.config.buffer_size.max(1024);

            if current_subscribers == 0 && channel_capacity > 0 {
                tracing::warn!(
                    "No subscribers for topic {}, discarding event to prevent buffer overflow",
                    topic
                );
                return Ok(0);
            }
        }

        // 发送到本地订阅者
        let tx = self.get_or_create_channel(topic, None);
        let result = tx.send(event.clone());

        // 异步发送到 Kafka
        let event_clone = event.clone();
        let topic_clone = topic.to_string();
        let queue = self.clone();
        let options_clone = options.clone();

        tokio::spawn(async move {
            if let Err(e) = queue.send_async(&topic_clone, event_clone, options_clone).await {
                tracing::error!("Failed to send event to Kafka: {}", e);
            }
        });

        result
    }

    fn send_batch(
        &self,
        topic: &str,
        events: Vec<bytes::Bytes>,
        options: Option<SendOptions>,
    ) -> Result<Vec<Result<usize, broadcast::error::SendError<bytes::Bytes>>>, ()> {
        let mut results = Vec::with_capacity(events.len());
        let tx = self.get_or_create_channel(topic, None);

        // 背压控制
        let apply_backpressure = self.should_apply_backpressure(&options);
        let has_subscribers = tx.receiver_count() > 0;
        let channel_capacity = self.config.buffer_size.max(1024);

        // 克隆事件向量以便在异步任务中使用
        let events_clone = events.clone();

        // 先处理本地发送，同步执行
        for event in events {
            if apply_backpressure && !has_subscribers && channel_capacity > 0 {
                tracing::warn!(
                    "No subscribers for topic {}, discarding event to prevent buffer overflow",
                    topic
                );
                results.push(Ok(0));
                continue;
            }

            match tx.send(event) {
                Ok(count) => results.push(Ok(count)),
                Err(e) => results.push(Err(e)),
            }
        }

        // 异步发送到 Kafka，整个批量只创建一个任务
        let queue = self.clone();
        let topic_clone = topic.to_string();
        let options_clone = options.clone();

        tokio::spawn(async move {
            for event in events_clone {
                let event_clone = event.clone();
                let topic_clone = topic_clone.clone();
                let options_clone = options_clone.clone();

                if let Err(e) = queue.send_async(&topic_clone, event_clone, options_clone).await {
                    tracing::error!("Failed to send event to Kafka: {}", e);
                }
            }
        });

        Ok(results)
    }

    fn subscribe(
        &self,
        topic: &str,
        options: Option<SubscribeOptions>,
    ) -> broadcast::Receiver<bytes::Bytes> {
        // 从 SubscribeOptions 中提取 buffer_size 并创建 ChannelConfig
        let channel_config = options.as_ref().map(|opts| {
            ChannelConfig::new().with_buffer_size(opts.buffer_size)
        });
        let channel = self.get_or_create_channel(topic, channel_config);
        let rx = channel.subscribe();

        rx
    }

    fn subscriber_count(&self, topic: &str) -> usize {
        if let Ok(channels) = self.topic_channels.try_lock() {
            if let Some(channel) = channels.get(topic) {
                return channel.receiver_count();
            }
        }
        0
    }

    fn topics(&self) -> Vec<String> {
        if let Ok(channels) = self.topic_channels.try_lock() {
            channels.keys().cloned().collect()
        } else {
            vec![]
        }
    }
}

impl Clone for KafkaQueue {
    fn clone(&self) -> Self {
        KafkaQueue {
            config: self.config.clone(),
            producer: self.producer.clone(),
            topic_channels: self.topic_channels.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_config_default() {
        let config = KafkaConfig::default();
        assert_eq!(config.brokers, "localhost:9092");
        assert_eq!(config.send_timeout_ms, 5000);
        assert_eq!(config.recv_timeout_ms, 3000);
        assert_eq!(config.default_num_partitions, 3);
        assert_eq!(config.default_replication_factor, 1);
    }

    #[test]
    fn test_kafka_config_conversion() {
        let default_config = DefaultQueueConfig::new().with_brokers("test:9092");

        let kafka_config = KafkaConfig::from(default_config);

        assert_eq!(kafka_config.brokers, "test:9092");
        assert_eq!(kafka_config.default_num_partitions, 3);
        assert_eq!(kafka_config.default_replication_factor, 1);
    }

    #[test]
    fn test_kafka_config_builder() {
        let config = KafkaConfig::default()
            .with_brokers("localhost:9092,localhost:9093")
            .with_send_timeout(10000)
            .with_recv_timeout(5000)
            .with_num_partitions(10)
            .with_replication_factor(3);

        assert_eq!(config.brokers, "localhost:9092,localhost:9093");
        assert_eq!(config.send_timeout_ms, 10000);
        assert_eq!(config.recv_timeout_ms, 5000);
        assert_eq!(config.default_num_partitions, 10);
        assert_eq!(config.default_replication_factor, 3);
    }
}

impl KafkaConfig {
    pub fn with_brokers(mut self, brokers: impl Into<String>) -> Self {
        self.brokers = brokers.into();
        self
    }

    pub fn with_send_timeout(mut self, ms: i32) -> Self {
        self.send_timeout_ms = ms;
        self
    }

    pub fn with_recv_timeout(mut self, ms: i32) -> Self {
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

    pub fn with_num_partitions(mut self, num: i32) -> Self {
        self.default_num_partitions = num;
        self
    }

    pub fn with_replication_factor(mut self, factor: i32) -> Self {
        self.default_replication_factor = factor;
        self
    }
}
