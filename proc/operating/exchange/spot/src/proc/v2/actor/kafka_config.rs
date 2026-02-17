use diff::ChangeLogEntry;
use rdkafka::producer::{FutureProducer, FutureRecord};

/// Kafka 配置结构体
#[derive(Debug, Clone)]
pub struct KafkaConfig {
    /// Kafka brokers 地址
    pub brokers: String,
    /// 消费者组 ID
    pub group_id: String,
}

impl KafkaConfig {
    /// 创建新的 Kafka 配置
    pub fn new(brokers: impl Into<String>, group_id: impl Into<String>) -> Self {
        Self { brokers: brokers.into(), group_id: group_id.into() }
    }

    /// 使用默认本地配置
    pub fn default_local() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            group_id: "spot-match-actor-group".to_string(),
        }
    }
}

/// 发送单条变更日志到 Kafka
pub async fn send_single_log(
    producer: &FutureProducer,
    topic: &str,
    log: &ChangeLogEntry,
) -> Result<(), Box<dyn std::error::Error>> {
    match serde_json::to_vec(log) {
        Ok(payload) => {
            let record = FutureRecord::to(topic)
                .key(log.entity_id().as_bytes())
                .payload(&payload);
            match producer.send(record, std::time::Duration::from_secs(5)).await {
                Ok(_) => {
                    tracing::debug!("Sent log to {}: entity_id={}", topic, log.entity_id());
                    Ok(())
                }
                Err((e, _)) => {
                    tracing::error!("Failed to send log to {}: {}", topic, e);
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to serialize log for {}: {:?}", topic, e);
            Err(Box::new(e))
        }
    }
}

/// 发送变更日志列表到指定 topic
pub async fn send_log_batch(producer: &FutureProducer, topic: &str, logs: Vec<ChangeLogEntry>) {
    for log in logs {
        if let Err(e) = send_single_log(producer, topic, &log).await {
            tracing::error!("Failed to send log in batch to {}: {}", topic, e);
        }
    }
}
