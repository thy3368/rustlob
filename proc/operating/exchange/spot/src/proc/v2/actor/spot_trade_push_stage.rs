use std::sync::Arc;

use base_types::actor_x::ActorX;
use base_types::spot_topic::SpotTopic;
use diff::ChangeLogEntry;
use push::push::push_service::PushBehaviorV2Imp;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;

use crate::proc::v2::actor::kafka_config::KafkaConfig;

pub struct SpotPushStage {
    /// 推送服务，负责将变更日志推送到客户端
    push_service: Arc<PushBehaviorV2Imp>,
    kafka_config: KafkaConfig,
}

impl SpotPushStage {
    /// 创建新的 SpotPushStage
    pub fn new(push_service: Arc<PushBehaviorV2Imp>, kafka_config: KafkaConfig) -> Self {
        Self { push_service, kafka_config }
    }

    /// 创建并启动 SpotPushStage（便捷方法）
    pub fn create_and_start(
        push_service: Arc<PushBehaviorV2Imp>,
        kafka_config: KafkaConfig,
    ) -> Arc<Self> {
        let actor = Arc::new(Self::new(push_service, kafka_config));
        actor.start();
        actor
    }
}

impl ActorX for SpotPushStage {
    fn start(self: &Arc<Self>) {
        // 订阅变更日志 topics: TradeChangeLog, BalanceChangeLog, OrderChangeLog, KLineChangeLog
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

            let topics = vec![
                SpotTopic::TradeChangeLog.name(),
                SpotTopic::BalanceChangeLog.name(),
                SpotTopic::OrderChangeLog.name(),
                SpotTopic::KLineChangeLog.name(),
            ];

            if let Err(e) = consumer.subscribe(&topics) {
                tracing::error!("Failed to subscribe to topics {:?}: {}", topics, e);
                return;
            }

            tracing::info!("Kafka consumer subscribed to topics: {:?}", topics);

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

                            tracing::debug!(
                                "Received event from topic {}: entity_type={}, entity_id={}",
                                msg.topic(),
                                change_log.entity_type(),
                                change_log.entity_id()
                            );

                            // 批量处理变更日志，推送到客户端
                            self_clone.push_service.handle_events(&[change_log]);
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
