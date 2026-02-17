use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use diff::ChangeLogEntry;
use push::k_line::k_line_service::KLineBehaviorV2Imp;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;

use crate::proc::v2::actor::kafka_config::KafkaConfig;

pub struct SpotKLineStage {
    /// K线服务，负责聚合交易数据生成K线
    kline_service: Arc<KLineBehaviorV2Imp>,
    kafka_config: KafkaConfig,
}

impl SpotKLineStage {
    /// 创建新的 SpotKLineStage
    pub fn new(kline_service: Arc<KLineBehaviorV2Imp>, kafka_config: KafkaConfig) -> Self {
        Self { kline_service, kafka_config }
    }

    /// 创建并启动 SpotKLineStage（便捷方法）
    pub fn create_and_start(
        kline_service: Arc<KLineBehaviorV2Imp>,
        kafka_config: KafkaConfig,
    ) -> Arc<Self> {
        let actor = Arc::new(Self::new(kline_service, kafka_config));
        actor.start();
        actor
    }
}

impl ActorX for SpotKLineStage {
    fn start(self: &Arc<Self>) {
        // 订阅成交变更日志 topic: TradeChangeLog
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
                                    tracing::error!(
                                        "Failed to deserialize ChangeLogEntry: {:?}",
                                        e
                                    );
                                    continue;
                                }
                            };

                            // 检查是否是 SpotTrade 实体且为创建事件
                            if change_log.entity_type() == "SpotTrade" {
                                match change_log.change_type() {
                                    diff::ChangeType::Created { .. } => {
                                        tracing::debug!(
                                            "Detected trade created event: trade_id={}",
                                            change_log.entity_id()
                                        );

                                        // 批量处理交易变更日志，聚合生成K线
                                        self_clone.kline_service.handle_events(&[change_log]);
                                    }
                                    _ => {
                                        // 忽略其他类型的变更事件
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
