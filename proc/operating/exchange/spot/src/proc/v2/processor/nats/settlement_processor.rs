use std::sync::Arc;

use async_nats::Client;
use futures::StreamExt;
use diff::ChangeLog;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::trade_cmd_handlers::settlement_handler::{SettlementHandler, SettlementResult};
use crate::proc::v2::processor::nats::base::{NatsProcessor, NatsProcessorConfig};
use crate::proc::v2::trade_cmd_handlers::change_log_store::deserialize_change_log;

pub struct NatsSettlementProcessor {
    client: Arc<Client>,
    handler: Arc<dyn SettlementHandler>,
    config: NatsProcessorConfig,
    subject: String,
}

impl NatsSettlementProcessor {
    pub fn new(
        handler: Arc<dyn SettlementHandler>,
        config: NatsProcessorConfig,
        subject: String,
    ) -> Result<Self, String> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create tokio runtime: {}", e))?;
        
        let client = rt.block_on(async_nats::connect(&config.nats_url))
            .map_err(|e| format!("Failed to connect to NATS at {}: {}", config.nats_url, e))?;

        Ok(Self {
            client: Arc::new(client),
            handler,
            config,
            subject,
        })
    }
}

impl NatsProcessor for NatsSettlementProcessor {
    fn client(&self) -> &Client {
        &self.client
    }

    fn subject(&self) -> &str {
        &self.subject
    }

    fn nats_url(&self) -> &str {
        &self.config.nats_url
    }

    async fn handle_message(&self, payload: &[u8]) -> Result<(), SpotCmdErrorAny> {
        let trade_log: ChangeLog = deserialize_change_log(payload)?;

        tracing::debug!(
            entity_id = %trade_log.entity_id(),
            change_type = ?trade_log.change_type(),
            "Received trade log from NATS"
        );

        let trade = self.handler.reconstruct_trade(&trade_log)?;
        let settlement_result = self.handler.settle_trade(&trade)?;

        if settlement_result.success && !settlement_result.balance_logs.is_empty() {
            self.handler.publish_balance_logs(&settlement_result.balance_logs);
        }

        Ok(())
    }
}

pub struct NatsSettlementProcessorFactory;

impl NatsSettlementProcessorFactory {
    pub fn create_and_start(
        handler: Arc<dyn SettlementHandler>,
        config: NatsProcessorConfig,
        subject: String,
    ) -> Result<tokio::task::JoinHandle<()>, String> {
        let processor = NatsSettlementProcessor::new(handler, config, subject)?;
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