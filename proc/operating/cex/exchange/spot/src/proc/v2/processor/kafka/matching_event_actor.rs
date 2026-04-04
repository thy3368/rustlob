use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::handler::event_actor::EventActor;
use base_types::handler::event_handler::EventHandler;
use diff::diff_types::DomainEvent;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::Message;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::processor::kafka::base::{
    create_kafka_consumer, KafkaConsumerConfig, KafkaProcessorConfig,
};
use crate::proc::v2::trade_event_handlers::new_order_place_event_handler::NewOrderPlaceEventHandler;

pub struct KafkaMatchingEventActor {
    consumer: StreamConsumer,
    handler: Arc<NewOrderPlaceEventHandler>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl KafkaMatchingEventActor {
    pub fn new(
        handler: Arc<NewOrderPlaceEventHandler>,
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
    fn deserialize_domain_event(bytes: &[u8]) -> Result<DomainEvent<SpotOrder>, SpotCmdErrorAny> {
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

impl EventActor<DomainEvent<SpotOrder>, SpotCmdErrorAny> for KafkaMatchingEventActor {
    fn recv_event(&mut self) -> Result<Option<DomainEvent<SpotOrder>>, SpotCmdErrorAny> {
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
            "Received Kafka message for matching event actor"
        );

        let event = Self::deserialize_domain_event(payload)?;

        tracing::info!(
            topic = %self.topic,
            entity_id = %event.change_log().entity_id(),
            "Deserialized matching domain event"
        );

        Ok(Some(event))
    }

    fn handle_event(&self, event: DomainEvent<SpotOrder>) -> Result<(), SpotCmdErrorAny> {
        self.handler.event_handle(event)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_domain_event_example() {
        let payload = br#"{\"change_log\":{\"entity_id\":\"order-1\",\"entity_type\":\"SpotOrder\",\"change_type\":\"Created\",\"timestamp\":1,\"sequence\":1},\"state\":null}"#;
        let result = KafkaMatchingEventActor::deserialize_domain_event(payload);
        assert!(result.is_err());
    }

    // Usage example:
    // let order_repo = Arc::new(order_repo);
    // let matching_handler = Arc::new(matching_handler);
    // let event_handler = Arc::new(NewOrderPlaceEventHandler::new(order_repo, matching_handler));
    // let config = KafkaProcessorConfig::new("localhost:9092", "matching-event-actor-group");
    // let mut actor = KafkaMatchingEventActor::new(event_handler, config, "spot-order-events".to_string())?;
    // actor.run()?;
}
