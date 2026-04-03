use std::sync::Arc;

use base_types::handler::event_actor::EventActor;
use base_types::handler::event_handler::EventHandler;
use diff::ChangeLog;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::Message;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::processor::kafka::base::{
    create_kafka_consumer, deserialize_change_log, KafkaConsumerConfig, KafkaProcessorConfig,
};
use crate::proc::v2::trade_event_handlers::NewOrderPlaceEventHandler::NewOrderPlaceEventHandler;

pub struct MatchingEventActor {
    consumer: StreamConsumer,
    handler: Arc<NewOrderPlaceEventHandler>,
    config: KafkaProcessorConfig,
    topic: String,
}

impl MatchingEventActor {
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

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn group_id(&self) -> &str {
        &self.config.group_id
    }
}

//todo ChangeLog 都改成 DomainEvent<SpotOrder>
impl EventActor<ChangeLog, SpotCmdErrorAny> for MatchingEventActor {
    fn recv_event(&mut self) -> Result<Option<ChangeLog>, SpotCmdErrorAny> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            Self::into_internal_error(format!("Failed to create Tokio runtime: {}", e))
        })?;

        let message = rt.block_on(self.consumer.recv()).map_err(|e| {
            Self::into_internal_error(format!("Failed to receive message from Kafka: {}", e))
        })?;

        let payload = message.payload().ok_or_else(|| {
            Self::into_internal_error("Received Kafka message without payload")
        })?;

        deserialize_change_log(payload).map(Some)
    }

    fn handle_event(&self, event: ChangeLog) -> Result<(), SpotCmdErrorAny> {
        self.handler.event_handle(event)?;
        Ok(())
    }
}
