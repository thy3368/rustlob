use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::sync::broadcast;

use crate::queue::queue_contract::{
    ChannelConfig, DefaultQueueConfig, Queue, QueueBatchSendResult, SendOptions, SubscribeOptions,
};

#[derive(Debug, Clone)]
pub struct MPMCQueue {
    topic_channels: Arc<RwLock<HashMap<String, broadcast::Sender<bytes::Bytes>>>>,
    config: DefaultQueueConfig,
}

impl MPMCQueue {
    pub fn from_config(config: DefaultQueueConfig) -> Self {
        Self { topic_channels: Arc::new(RwLock::new(HashMap::new())), config }
    }

    fn should_apply_backpressure(&self, options: &Option<SendOptions>) -> bool {
        options
            .as_ref()
            .map(|opts| opts.enable_backpressure)
            .unwrap_or(self.config.enable_backpressure)
    }

    fn read_channels(
        &self,
    ) -> std::sync::RwLockReadGuard<'_, HashMap<String, broadcast::Sender<bytes::Bytes>>> {
        match self.topic_channels.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn write_channels(
        &self,
    ) -> std::sync::RwLockWriteGuard<'_, HashMap<String, broadcast::Sender<bytes::Bytes>>> {
        match self.topic_channels.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

impl Queue for MPMCQueue {
    type Config = DefaultQueueConfig;

    fn new() -> Self {
        Self::from_config(DefaultQueueConfig::default())
    }

    fn new_with_config(config: impl Into<Self::Config>) -> Self {
        Self::from_config(config.into())
    }

    fn send(
        &self,
        topic: &str,
        event: bytes::Bytes,
        options: Option<SendOptions>,
    ) -> Result<usize, broadcast::error::SendError<bytes::Bytes>> {
        let sender = self.get_or_create_channel(topic, None);

        if self.should_apply_backpressure(&options) && sender.receiver_count() == 0 {
            tracing::warn!(
                "No subscribers for topic {}, discarding event to prevent buffer overflow",
                topic
            );
            return Ok(0);
        }

        sender.send(event)
    }

    fn send_batch(
        &self,
        topic: &str,
        events: Vec<bytes::Bytes>,
        options: Option<SendOptions>,
    ) -> QueueBatchSendResult {
        let channel = self.get_or_create_channel(topic, None);
        let mut results = Vec::with_capacity(events.len());
        let apply_backpressure = self.should_apply_backpressure(&options);
        let has_subscribers = channel.receiver_count() > 0;

        for event in events {
            if apply_backpressure && !has_subscribers {
                tracing::warn!(
                    "No subscribers for topic {}, discarding event to prevent buffer overflow",
                    topic
                );
                results.push(Ok(0));
                continue;
            }

            results.push(channel.send(event));
        }

        Ok(results)
    }

    fn subscribe(
        &self,
        topic: &str,
        options: Option<SubscribeOptions>,
    ) -> broadcast::Receiver<bytes::Bytes> {
        let channel_config =
            options.as_ref().map(|opts| ChannelConfig::new().with_buffer_size(opts.buffer_size));
        self.get_or_create_channel(topic, channel_config).subscribe()
    }

    fn subscriber_count(&self, topic: &str) -> usize {
        self.read_channels().get(topic).map(|channel| channel.receiver_count()).unwrap_or(0)
    }

    fn get_or_create_channel(
        &self,
        topic: &str,
        config: Option<ChannelConfig>,
    ) -> broadcast::Sender<bytes::Bytes> {
        if let Some(existing) = self.read_channels().get(topic) {
            return existing.clone();
        }

        let buffer_size =
            config.and_then(|c| c.buffer_size).unwrap_or_else(|| self.config.buffer_size.max(1));
        let (tx, _) = broadcast::channel(buffer_size.max(1));

        let mut channels = self.write_channels();
        channels.entry(topic.to_string()).or_insert_with(|| tx.clone()).clone()
    }

    fn topics(&self) -> Vec<String> {
        self.read_channels().keys().cloned().collect()
    }
}

impl Default for MPMCQueue {
    fn default() -> Self {
        Self::new()
    }
}
