use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::handler::event_actor::EventRecvActor;
use base_types::handler::event_handler::{EventHandler, EventHandler2};
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::Message;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::processor::kafka::base::{
    create_kafka_consumer, KafkaConsumerConfig, KafkaProcessorConfig,
};
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_order_place_event_handler::NewOrderPlaceEventHandler;

pub struct KafkaMatchingEventActor<
    R: CmdRepo2 + Clone,
    P: EventPublisher2 + Clone,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> {
    consumer: StreamConsumer,
    handler: Arc<NewOrderPlaceEventHandler<R, P, L>>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone, L: MultiSymbolLobRepo<Order = SpotOrder> + Send>
    KafkaMatchingEventActor<R, P, L>
{
    pub fn new(
        handler: Arc<NewOrderPlaceEventHandler<R, P, L>>,
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
        SpotCmdErrorAny::Common(CommonError::Internal { message: message.into() })
    }

    #[inline]
    fn deserialize_domain_event(bytes: &[u8]) -> Result<DomainEvent<SpotOrder>, SpotCmdErrorAny> {
        serde_json::from_slice(bytes)
            .map_err(|e| Self::into_internal_error(format!("Deserialization error: {}", e)))
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn group_id(&self) -> &str {
        &self.config.group_id
    }
}

impl<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone, L: MultiSymbolLobRepo<Order = SpotOrder> + Send>
    EventRecvActor<DomainEvent<SpotOrder>, SpotCmdErrorAny> for KafkaMatchingEventActor<R, P, L>
{
    fn recv_event(&mut self) -> Result<Option<DomainEvent<SpotOrder>>, SpotCmdErrorAny> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            Self::into_internal_error(format!("Failed to create Tokio runtime: {}", e))
        })?;

        let message = rt.block_on(self.consumer.recv()).map_err(|e| {
            Self::into_internal_error(format!("Failed to receive message from Kafka: {}", e))
        })?;

        let payload = message
            .payload()
            .ok_or_else(|| Self::into_internal_error("Received Kafka message without payload"))?;

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
mod tests {}
