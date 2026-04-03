use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use diff::ChangeLog;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;
use crate::proc::v2::processor::nats::base::{create_nats_client, deserialize_change_log, NatsProcessor, NatsProcessorConfig};
use crate::proc::v2::trade_cmd_handlers::matching_handler::{MatchResult, MatchingHandler};

pub struct NatsMatchingProcessor {
    client: Arc<async_nats::Client>,
    matching_engine: Arc<MatchingHandler>,
    event_publisher: Arc<dyn EventPublisher>,
    config: NatsProcessorConfig,
    subject: String,
}

impl NatsMatchingProcessor {
    pub fn new(
        matching_engine: Arc<MatchingHandler>,
        event_publisher: Arc<dyn EventPublisher>,
        config: NatsProcessorConfig,
        subject: String,
    ) -> Result<Self, String> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;
        let client = rt.block_on(create_nats_client(&config))?;

        Ok(Self {
            client: Arc::new(client),
            matching_engine,
            event_publisher,
            config,
            subject,
        })
    }
}

impl NatsProcessor for NatsMatchingProcessor {
    fn client(&self) -> &async_nats::Client {
        &self.client
    }

    fn subject(&self) -> &str {
        &self.subject
    }

    fn nats_url(&self) -> &str {
        &self.config.nats_url
    }

    async fn handle_message(&self, payload: &[u8]) -> Result<(), SpotCmdErrorAny> {
        let order_log: ChangeLog = deserialize_change_log(payload)?;

        tracing::debug!(
            entity_id = %order_log.entity_id(),
            change_type = ?order_log.change_type(),
            "Received order change log from NATS"
        );

        let order = self.reconstruct_order(&order_log)?;
        let match_result = self.matching_engine.match_order(order)?;

        self.publish_match_result(match_result).await
    }
}

impl NatsMatchingProcessor {
    fn reconstruct_order(&self, order_log: &ChangeLog) -> Result<SpotOrder, SpotCmdErrorAny> {
        tracing::warn!(
            entity_id = %order_log.entity_id(),
            "Order reconstruction not implemented yet"
        );

        Err(SpotCmdErrorAny::Common(
            crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
                message: "Order reconstruction not implemented".to_string(),
            },
        ))
    }

    async fn publish_match_result(&self, match_result: MatchResult) -> Result<(), SpotCmdErrorAny> {
        if !match_result.order_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_order_logs(&match_result.order_logs) {
                tracing::error!(count = match_result.order_logs.len(), error = ?e, "Failed to publish order logs");
            }
        }

        if !match_result.trade_logs.is_empty() {
            if let Err(e) = self.event_publisher.publish_trade_logs(&match_result.trade_logs) {
                tracing::error!(count = match_result.trade_logs.len(), error = ?e, "Failed to publish trade logs");
            }
        }

        Ok(())
    }
}

pub struct NatsMatchingProcessorFactory;

impl NatsMatchingProcessorFactory {
    pub fn create_and_start(
        matching_engine: Arc<MatchingHandler>,
        event_publisher: Arc<dyn EventPublisher>,
        config: NatsProcessorConfig,
        subject: String,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor = NatsMatchingProcessor::new(matching_engine, event_publisher, config, subject)?;
        Ok(Arc::new(processor).start_background())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = NatsProcessorConfig::default();
        assert_eq!(config.nats_url, "nats://localhost:4222");
    }
}
