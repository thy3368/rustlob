use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::event_actor::EventActor;
use base_types::handler::event_handler::{EventHandler, EventHandler2};
use diff::diff_types::DomainEvent;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::Message;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::processor::kafka::base::{
    create_kafka_consumer, KafkaConsumerConfig, KafkaProcessorConfig,
};
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_trade_event_handler::NewTradeEventHandler;

pub struct KafkaSettlementEventActor {
    consumer: StreamConsumer,
    handler: Arc<NewTradeEventHandler>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl KafkaSettlementEventActor {
    pub fn new(
        handler: Arc<NewTradeEventHandler>,
        config: KafkaProcessorConfig,
        topic: String,
    ) -> Result<Self, String> {
        let kafka_config =
            KafkaConsumerConfig::new(&config.kafka_brokers, &topic, &config.group_id)
                .with_session_timeout(config.session_timeout_ms)
                .with_auto_commit_interval(config.auto_commit_interval_ms);

        let consumer = create_kafka_consumer(&kafka_config)?;

        Ok(Self { consumer, handler, config, topic })
    }

    #[inline]
    fn into_internal_error(message: impl Into<String>) -> SpotCmdErrorAny {
        SpotCmdErrorAny::Common(CommonError::Internal {
            message: message.into(),
        })
    }

    #[inline]
    fn deserialize_domain_event(bytes: &[u8]) -> Result<DomainEvent<SpotTrade>, SpotCmdErrorAny> {
        serde_json::from_slice(bytes).map_err(|e| {
            Self::into_internal_error(format!("Deserialization error: {}", e))
        })
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn group_id(&self) -> &str {
        &self.config.group_id
    }
}

impl EventActor<DomainEvent<SpotTrade>, SpotCmdErrorAny> for KafkaSettlementEventActor {
    fn recv_event(&mut self) -> Result<Option<DomainEvent<SpotTrade>>, SpotCmdErrorAny> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            Self::into_internal_error(format!("Failed to create Tokio runtime: {}", e))
        })?;

        let message = rt.block_on(self.consumer.recv()).map_err(|e| {
            Self::into_internal_error(format!("Failed to receive message from Kafka: {}", e))
        })?;

        let payload = message.payload().ok_or_else(|| {
            Self::into_internal_error("Received Kafka message without payload")
        })?;

        tracing::info!(
            topic = %self.topic,
            partition = message.partition(),
            offset = message.offset(),
            payload_len = payload.len(),
            "Received Kafka message for settlement event actor"
        );

        let event = Self::deserialize_domain_event(payload)?;

        tracing::info!(
            topic = %self.topic,
            entity_id = %event.change_log().entity_id(),
            "Deserialized settlement domain event"
        );

        Ok(Some(event))
    }

    fn handle_event(&self, event: DomainEvent<SpotTrade>) -> Result<(), SpotCmdErrorAny> {
        self.handler.event_handle(event)?;
        Ok(())
    }
}
