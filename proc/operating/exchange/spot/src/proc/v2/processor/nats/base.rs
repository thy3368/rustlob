use std::sync::Arc;

use async_nats::Client;
use bytes::Bytes;
use futures::StreamExt;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

#[derive(Debug, Clone)]
pub struct NatsProcessorConfig {
    pub nats_url: String,
}

impl Default for NatsProcessorConfig {
    fn default() -> Self {
        Self {
            nats_url: "nats://localhost:4222".to_string(),
        }
    }
}

impl NatsProcessorConfig {
    pub fn new(nats_url: impl Into<String>) -> Self {
        Self {
            nats_url: nats_url.into(),
        }
    }
}

pub async fn create_nats_client(config: &NatsProcessorConfig) -> Result<Client, String> {
    async_nats::connect(&config.nats_url)
        .await
        .map_err(|e| format!("Failed to connect to NATS at {}: {}", config.nats_url, e))
}

pub fn deserialize_change_log(bytes: &[u8]) -> Result<diff::ChangeLogEntry, SpotCmdErrorAny> {
    serde_json::from_slice(bytes).map_err(|e| {
        tracing::error!(error = ?e, bytes_len = bytes.len(), "Failed to deserialize change log");
        SpotCmdErrorAny::Common(CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        })
    })
}

pub trait NatsProcessor: Send + Sync + 'static {
    fn client(&self) -> &Client;
    fn subject(&self) -> &str;
    fn nats_url(&self) -> &str;

    async fn start(&self) {
        tracing::info!(
            nats_url = %self.nats_url(),
            subject = %self.subject(),
            "Starting NATS processor"
        );

        let subject = self.subject().to_string();
        let mut subscription = self.client().subscribe(subject.clone()).await
            .map_err(|e| format!("Failed to subscribe to {}: {}", subject, e))
            .unwrap();

        while let Some(message) = subscription.next().await {
            let payload: Bytes = message.payload;
            let payload_vec = payload.to_vec();
            if let Err(e) = self.handle_message(&payload_vec).await {
                tracing::error!(error = ?e, "Failed to handle message from NATS");
            }
        }
    }

    fn start_background(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async move {
                self.start().await;
            });
        })
    }

    fn handle_message(&self, payload: &[u8]) -> impl std::future::Future<Output = Result<(), SpotCmdErrorAny>> + Send;
}
