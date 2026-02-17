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

pub struct SpotSettlementStage {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    kafka_config: KafkaConfig,
}

impl SpotSettlementStage {
    /// 创建新的 SpotSettlementStage
    pub fn new(trade_behavior: Arc<SpotTradeBehaviorV2Impl>, kafka_config: KafkaConfig) -> Self {
        Self { trade_behavior, kafka_config }
    }

    /// 创建并启动 SpotSettlementStage（便捷方法）
    ///
    /// # Arguments
    /// * `trade_behavior` - 结算行为实现
    /// * `kafka_config` - Kafka 配置
    ///
    /// # Example
    /// ```ignore
    /// let stage = SpotSettlementStage::create_and_start(
    ///     trade_behavior,
    ///     KafkaConfig::default_local()
    /// );
    /// ```
    pub fn create_and_start(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        kafka_config: KafkaConfig,
    ) -> Arc<Self> {
        let stage = Arc::new(Self::new(trade_behavior, kafka_config));
        stage.start();
        stage
    }
}

impl ActorX for SpotSettlementStage {
    fn start(self: &Arc<Self>) {
        // 订阅成交变更日志 topic: TradeChangeLog，直接调用 Kafka
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

            let topic = SpotTopic::TradeChangeLog.name();
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
                                    tracing::error!("Failed to deserialize ChangeLogEntry: {:?}", e);
                                    continue;
                                }
                            };

                            // 检查是否是 SpotTrade 实体且为 Created 类型
                            if change_log.entity_type() == "SpotTrade" {
                                let is_created = matches!(change_log.change_type(), ChangeType::Created { .. });

                                if is_created {
                                    tracing::info!(
                                        "Detected new trade created, starting settlement: trade_id={}",
                                        change_log.entity_id()
                                    );

                                    // 解析 trade_id
                                    if let Ok(trade_id) = change_log.entity_id().parse::<u64>() {
                                        // 调用 handle_settlement2 进行结算
                                        match self_clone.trade_behavior.handle_settlement2(trade_id) {
                                            Ok(balance_change_logs) => {
                                                tracing::info!(
                                                    "Settlement successful: trade_id={}, generated {} balance change logs",
                                                    trade_id,
                                                    balance_change_logs.len()
                                                );
                                                // 发送余额变更日志到 BalanceChangeLog topic
                                                send_log_batch(&producer, SpotTopic::BalanceChangeLog.name(), balance_change_logs).await;
                                            }
                                            Err(e) => {
                                                tracing::error!("Settlement failed: trade_id={}, error={:?}", trade_id, e);
                                            }
                                        }
                                    } else {
                                        tracing::error!("Failed to parse trade_id: {}", change_log.entity_id());
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
