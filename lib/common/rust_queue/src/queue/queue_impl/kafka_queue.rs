use std::sync::Arc;
use bytes::Bytes;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::broadcast::{Receiver, Sender};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use push::k_line::k_line_types::KLineUpdateEvent;
use crate::queue::queue::{Queue, SendOptions, SubscribeOptions, DefaultQueueConfig, ToBytes, FromBytes};

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
        }
    }
}

/// Kafka 队列实现
/// 用于连接到 Kafka 集群，发送和接收 K 线更新事件
/// 支持多个 topic 和消费组功能
pub struct KafkaQueue {
    config: KafkaConfig,
    producer: Arc<FutureProducer>,
    topic_channels: Arc<std::sync::Mutex<std::collections::HashMap<String, broadcast::Sender<bytes::Bytes>>>>,
}

impl KafkaQueue {
    /// 创建新的 Kafka 队列
    pub fn new_with_config(config: KafkaConfig) -> Self {
        // 创建 Kafka 生产者
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", &config.send_timeout_ms.to_string())
            .set("acks", "1")  // 至少一个副本确认
            .set("linger.ms", "0")  // 立即发送
            .set("retries", "3")
            .create()
            .expect("Failed to create Kafka producer");

        let topic_channels = Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));

        let queue = KafkaQueue {
            config,
            producer: Arc::new(producer),
            topic_channels,
        };


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
    async fn consumer_loop(config: KafkaConfig, topic: String, tx: broadcast::Sender<bytes::Bytes>) -> Result<(), Box<dyn std::error::Error>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.default_group_id)
            .set("auto.offset.reset", "latest")  // 从最新位置开始消费
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
    pub async fn send_async<T: Serialize + ToBytes + Send + Sync + 'static>(
        &self,
        topic: &str,
        event: T,
        options: Option<SendOptions>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let payload = event.to_bytes()?;

        let record = FutureRecord::to(topic)
            .key("kline-update".as_bytes())
            .payload(&payload[..]);

        let timeout = options.map(|o| o.timeout_ms as u64).unwrap_or(self.config.send_timeout_ms as u64);

        match self.producer.send(record, tokio::time::Duration::from_millis(timeout)).await {
            Ok((_, _)) => {
                // 同时发送到本地订阅者
                let tx = self.get_or_create_channel(topic);
                let count = tx.send(payload)?;
                Ok(count)
            }
            Err((e, _)) => Err(Box::new(e)),
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
    fn get_or_create_channel(&self, topic: &str) -> broadcast::Sender<bytes::Bytes> {
        let mut channels = self.topic_channels.lock().unwrap();
        channels.entry(topic.to_string())
            .or_insert_with(|| {
                let buffer_size = if self.config.buffer_size > 0 {
                    self.config.buffer_size
                } else {
                    1024
                };
                let (tx, _) = broadcast::channel(buffer_size);
                // 启动 Kafka 消费者任务
                self.spawn_consumer_task(topic, tx.clone());
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

    fn send<T: Serialize + ToBytes + Send + Sync + 'static + Clone>(
        &self,
        topic: &str,
        event: T,
        options: Option<SendOptions>,
    ) -> Result<usize, SendError<T>> {
        // 背压控制
        if self.should_apply_backpressure(&options) {
            let channel = self.get_or_create_channel(topic);
            let current_subscribers = channel.receiver_count();
            let channel_capacity = self.config.buffer_size.max(1024);

            if current_subscribers == 0 && channel_capacity > 0 {
                tracing::warn!("No subscribers for topic {}, discarding event to prevent buffer overflow", topic);
                return Ok(0);
            }
        }

        // 克隆以便在 async 任务中使用
        let queue = self.clone();
        let event_clone = event.clone();
        let topic_clone = topic.to_string();

        // 异步发送到 Kafka
        tokio::spawn(async move {
            if let Err(e) = queue.send_async(&topic_clone, event_clone, options).await {
                tracing::error!("Failed to send event to Kafka: {}", e);
            }
        });

        // 同步发送到本地订阅者
        let tx = self.get_or_create_channel(topic);
        let payload = event.to_bytes().map_err(|_| SendError(event.clone()))?;
        tx.send(payload).map_err(|_| SendError(event))
    }

    fn subscribe<T: DeserializeOwned + Send + Sync + 'static + Clone>(
        &self,
        topic: &str,
        _options: Option<SubscribeOptions>,
    ) -> Receiver<T> {
        let channel = self.get_or_create_channel(topic);
        let mut rx = channel.subscribe();

        // 创建新的 receiver，内部进行反序列化
        let (tx, rx_out) = broadcast::channel(1024);

        tokio::spawn(async move {
            while let Ok(bytes) = rx.recv().await {
                match T::from_bytes(&bytes) {
                    Ok(event) => {
                        let _ = tx.send(event);
                    }
                    Err(e) => {
                        tracing::error!("Failed to deserialize event: {}", e);
                    }
                }
            }
        });

        rx_out
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
    use std::time::Instant;

    #[test]
    fn test_kafka_config_default() {
        let config = KafkaConfig::default();
        assert_eq!(config.brokers, "localhost:9092");
        assert_eq!(config.send_timeout_ms, 5000);
        assert_eq!(config.recv_timeout_ms, 3000);
    }

    #[test]
    fn test_kafka_config_conversion() {
        let default_config = DefaultQueueConfig::new()
            .with_brokers("test:9092");

        let kafka_config = KafkaConfig::from(default_config);

        assert_eq!(kafka_config.brokers, "test:9092");

    }

    #[test]
    fn test_kafka_config_builder() {
        let config = KafkaConfig::default()
            .with_brokers("localhost:9092,localhost:9093")
            .with_send_timeout(10000)
            .with_recv_timeout(5000);

        assert_eq!(config.brokers, "localhost:9092,localhost:9093");
        assert_eq!(config.send_timeout_ms, 10000);
        assert_eq!(config.recv_timeout_ms, 5000);
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
}
