use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use diff::{ChangeLogEntry, ChangeType};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::proc::v2::actor::kafka_config::{send_log_batch, KafkaConfig};
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

pub struct SpotMarketDataStage {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    kafka_config: KafkaConfig,
}

impl SpotMarketDataStage {
    /// 创建新的 SpotMatchActor
    pub fn new(trade_behavior: Arc<SpotTradeBehaviorV2Impl>, kafka_config: KafkaConfig) -> Self {
        Self { trade_behavior, kafka_config }
    }

    /// 创建并启动 SpotMatchActor（便捷方法）
    /// 
    /// # Arguments
    /// * `trade_behavior` - 撮合行为实现
    /// * `kafka_config` - Kafka 配置
    /// 
    /// # Example
    /// ```ignore
    /// let actor = SpotMatchActor::create_and_start(
    ///     trade_behavior,
    ///     KafkaConfig::default_local()
    /// );
    /// ```
    pub fn create_and_start(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        kafka_config: KafkaConfig,
    ) -> Arc<Self> {
        let actor = Arc::new(Self::new(trade_behavior, kafka_config));
        actor.start();
        actor
    }

    /// 发送变更日志到 Kafka
    async fn send_change_logs(
        producer: &FutureProducer,
        order_change_logs: Vec<ChangeLogEntry>,
        trade_change_logs: Vec<ChangeLogEntry>,
    ) {
        // 发送订单变更日志到 OrderChangeLog topic
        if !order_change_logs.is_empty() {
            send_log_batch(producer, SpotTopic::OrderChangeLog.name(), order_change_logs).await;
        }
        // 发送成交变更日志到 TradeChangeLog topic
        if !trade_change_logs.is_empty() {
            send_log_batch(producer, SpotTopic::TradeChangeLog.name(), trade_change_logs).await;
        }
    }
}

impl ActorX for SpotMarketDataStage {
    fn start(self: &Arc<Self>) {
        // 订阅订单变更日志 topic: OrderChangeLog，直接调用 Kafka
        let self_clone = Arc::clone(self);
        tokio::spawn(async move {
            // 创建 Kafka 消费者
            let consumer: StreamConsumer = ClientConfig::new()
                .set("bootstrap.servers", &self_clone.kafka_config.brokers)
                .set("group.id", &self_clone.kafka_config.group_id)
                .set("auto.offset.reset", "latest")
                .set("enable.auto.commit", "true")
                .set("auto.commit.interval.ms", "5000")
                .create()
                .expect("Failed to create Kafka consumer");

            // 创建 Kafka 生产者（启用自动批量发送优化）
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &self_clone.kafka_config.brokers)
                .set("message.timeout.ms", "5000")
                .set("acks", "1")
                .set("linger.ms", "5") // 延迟5ms批量发送
                .set("batch.size", "32768") // 批量大小32KB
                .set("compression.type", "lz4") // LZ4压缩
                .set("retries", "3")
                .set("max.in.flight.requests.per.connection", "5")
                .create()
                .expect("Failed to create Kafka producer");

            let topic = SpotTopic::OrderChangeLog.name();
            if let Err(e) = consumer.subscribe(&[topic]) {
                tracing::error!("Failed to subscribe to topic {}: {}", topic, e);
                return;
            }

            tracing::info!("Kafka consumer subscribed to topic: {}", topic);

            loop {
                match consumer.recv().await {
                    Ok(msg) => {
                        if let Some(payload) = msg.payload() {
                            // 解析 ChangeLogEntry
                            let change_log: ChangeLogEntry = match serde_json::from_slice(payload) {
                                Ok(log) => log,
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to deserialize ChangeLogEntry: {:?}",
                                        e
                                    );
                                    continue;
                                }
                            };

                            // 检查是否是 SpotOrder 实体且状态为 Pending
                            if change_log.entity_type() == "SpotOrder" {
                                let is_pending = match change_log.change_type() {
                                    ChangeType::Created { fields }
                                    | ChangeType::Updated { changed_fields: fields } => {
                                        fields.iter().any(|field| {
                                            field.field_name.as_ref() == "status"
                                                && (field.new_value == "Pending"
                                                    || field.new_value == "PENDING")
                                        })
                                    }
                                    _ => false,
                                };

                                if is_pending {
                                    tracing::info!(
                                        "Detected order entering Pending state, starting match processing: order_id={}",
                                        change_log.entity_id()
                                    );

                                    // 调用 handle_match3 进行撮合处理
                                    match self_clone.trade_behavior.handle_match3(change_log) {
                                        Ok((order_change_logs, trade_change_logs)) => {
                                            // 发送变更日志到对应的 Kafka topics
                                            //todo 发送market_data changelog

                                            Self::send_change_logs(
                                                &producer,
                                                order_change_logs.unwrap_or_default(),
                                                trade_change_logs.unwrap_or_default(),
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            tracing::error!("Match processing failed: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Kafka consumer error: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use diff::FieldChange;

    use super::*;

    /// 创建测试用的 ChangeLogEntry
    fn create_test_change_log(entity_id: &str, entity_type: &str, status: &str) -> ChangeLogEntry {
        let fields = vec![FieldChange::new(
            Cow::from("status"),
            String::from("Created"),
            String::from(status),
        )];

        ChangeLogEntry::new(
            entity_id.to_string(),
            entity_type.to_string(),
            ChangeType::Updated { changed_fields: fields },
            1234567890u64,
            1u64,
        )
    }

    /// 测试 KafkaConfig 创建
    #[test]
    fn test_kafka_config_new() {
        let config = KafkaConfig::new("localhost:9092", "test-group");
        assert_eq!(config.brokers, "localhost:9092");
        assert_eq!(config.group_id, "test-group");
    }

    /// 测试 KafkaConfig 默认本地配置
    #[test]
    fn test_kafka_config_default_local() {
        let config = KafkaConfig::default_local();
        assert_eq!(config.brokers, "localhost:9092");
        assert!(!config.group_id.is_empty());
    }

    /// 测试 SpotMatchActor 创建
    #[test]
    fn test_spot_match_actor_creation() {
        // 注意：这里使用 mock 或简单的 trade_behavior
        // 实际测试需要适当的 SpotTradeBehaviorV2Impl mock
        // 这里仅演示结构
    }

    /// 测试检查订单是否为 Pending 状态 - 是 Pending
    #[test]
    fn test_is_pending_status_true() {
        let change_log = create_test_change_log("order-123", "SpotOrder", "Pending");

        let is_pending = match change_log.change_type() {
            ChangeType::Created { fields } | ChangeType::Updated { changed_fields: fields } => {
                fields.iter().any(|field| {
                    field.field_name.as_ref() == "status"
                        && (field.new_value == "Pending" || field.new_value == "PENDING")
                })
            }
            _ => false,
        };

        assert!(is_pending, "Expected order to be pending");
    }

    /// 测试检查订单是否为 Pending 状态 - 不是 Pending
    #[test]
    fn test_is_pending_status_false() {
        let change_log = create_test_change_log("order-123", "SpotOrder", "Filled");

        let is_pending = match change_log.change_type() {
            ChangeType::Created { fields } | ChangeType::Updated { changed_fields: fields } => {
                fields.iter().any(|field| {
                    field.field_name.as_ref() == "status"
                        && (field.new_value == "Pending" || field.new_value == "PENDING")
                })
            }
            _ => false,
        };

        assert!(!is_pending, "Expected order not to be pending");
    }

    /// 测试检查非 SpotOrder 实体类型
    #[test]
    fn test_non_spot_order_entity() {
        let change_log = create_test_change_log("trade-123", "SpotTrade", "Pending");
        assert_ne!(change_log.entity_type(), "SpotOrder");
    }

    /// 测试检查 SpotOrder 实体类型
    #[test]
    fn test_spot_order_entity() {
        let change_log = create_test_change_log("order-123", "SpotOrder", "Pending");
        assert_eq!(change_log.entity_type(), "SpotOrder");
    }

    /// 测试变更日志序列化和反序列化
    #[test]
    fn test_changelog_serialization() {
        let change_log = create_test_change_log("order-123", "SpotOrder", "Pending");

        // 序列化
        let serialized = serde_json::to_vec(&change_log).expect("Failed to serialize");
        assert!(!serialized.is_empty());

        // 反序列化
        let deserialized: ChangeLogEntry =
            serde_json::from_slice(&serialized).expect("Failed to deserialize");

        assert_eq!(change_log.entity_id(), deserialized.entity_id());
        assert_eq!(change_log.entity_type(), deserialized.entity_type());
    }

    /// 测试 send_change_logs 方法 - 使用 mock producer
    /// 注意：由于 FutureProducer 难以 mock，这个测试展示了概念
    /// 实际项目中可以使用 trait 抽象 producer 以便测试
    #[tokio::test]
    async fn test_send_change_logs_empty() {
        // 当传入 None 时不应该报错
        // 这里仅测试方法可以调用（不依赖真实 Kafka）
        // SpotMatchActor::send_change_logs(&mock_producer, None, None).await;
    }

    /// 示例：创建并启动 SpotMatchActor
    /// 
    /// # Example
    /// ```ignore
    /// use std::sync::Arc;
    /// use spot_behavior::proc::v2::actor::spot_trade_match_actor2::{SpotMatchActor, KafkaConfig};
    /// use spot_behavior::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;
    /// use base_types::actor_x::ActorX;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // 1. 创建 SpotTradeBehaviorV2Impl（需要根据实际业务初始化）
    ///     let trade_behavior = Arc::new(SpotTradeBehaviorV2Impl::new());
    ///
    ///     // 2. 创建 Kafka 配置
    ///     let kafka_config = KafkaConfig::new(
    ///         "localhost:9092",           // Kafka brokers
    ///         "spot-match-actor-group"    // Consumer group id
    ///     );
    ///
    ///     // 3. 创建 SpotMatchActor
    ///     let actor = Arc::new(SpotMatchActor::new(trade_behavior, kafka_config));
    ///
    ///     // 4. 启动 Actor（开始消费 Kafka 消息并撮合）
    ///     actor.start();
    ///
    ///     // 5. 保持程序运行
    ///     tokio::signal::ctrl_c().await.unwrap();
    /// }
    /// ```
    #[test]
    fn test_spot_match_actor_creation_and_start_example() {
        // 这是一个文档示例，展示了如何创建和启动 SpotMatchActor
        // 实际使用时需要提供真实的 SpotTradeBehaviorV2Impl 和 Kafka 环境
        
        // 创建 Kafka 配置
        let kafka_config = KafkaConfig::default_local();
        
        // 验证配置正确
        assert_eq!(kafka_config.brokers, "localhost:9092");
        assert!(!kafka_config.group_id.is_empty());
        
        // 注意：实际启动需要以下步骤：
        // 1. 创建 SpotTradeBehaviorV2Impl 实例
        // 2. 调用 SpotMatchActor::new(trade_behavior, kafka_config)
        // 3. 调用 actor.start() 启动消费者
    }
}
