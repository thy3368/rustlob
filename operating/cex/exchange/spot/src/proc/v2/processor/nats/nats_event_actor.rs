use std::sync::Arc;

use async_nats::{Client, Subscriber};
use base_types::handler::event_actor::EventRecvActor;
use base_types::handler::event_handler::EventHandler;
use futures::StreamExt;
use tokio::runtime::Runtime;

use crate::proc::behavior::v2::spot_trade_error::{CommonError, SpotApiErrorAny};

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

struct NatsSubscription {
    _client: Client,
    subscriber: Subscriber,
}

impl NatsSubscription {
    fn connect(runtime: &Runtime, nats_url: &str, subject: &str) -> Result<Self, String> {
        let client = runtime
            .block_on(async_nats::connect(nats_url))
            .map_err(|e| format!("Failed to connect to NATS at {}: {}", nats_url, e))?;
        let subscriber = runtime
            .block_on(client.subscribe(subject.to_owned()))
            .map_err(|e| format!("Failed to subscribe to NATS subject {}: {}", subject, e))?;

        Ok(Self {
            _client: client,
            subscriber,
        })
    }

    #[inline]
    fn recv(
        &mut self,
        runtime: &Runtime,
        subject: &str,
    ) -> Result<async_nats::Message, SpotApiErrorAny> {
        runtime
            .block_on(self.subscriber.next())
            .ok_or_else(|| NatsEventActor::<(), ()>::into_closed_subscription_error(subject))
    }
}

pub struct NatsEventActor<E, H> {
    runtime: Runtime,
    subscription: NatsSubscription,
    subject: String,
    handler: Arc<H>,
    actor_name: &'static str,
    deserialize: fn(&[u8]) -> Result<E, SpotApiErrorAny>,
}

impl<E, H> NatsEventActor<E, H> {
    pub fn new(
        config: NatsProcessorConfig,
        subject: String,
        handler: Arc<H>,
        actor_name: &'static str,
        deserialize: fn(&[u8]) -> Result<E, SpotApiErrorAny>,
    ) -> Result<Self, String> {
        let runtime = Self::build_runtime()?;
        let subscription = NatsSubscription::connect(&runtime, &config.nats_url, &subject)?;

        Ok(Self {
            runtime,
            subscription,
            subject,
            handler,
            actor_name,
            deserialize,
        })
    }

    #[inline]
    fn build_runtime() -> Result<Runtime, String> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create tokio runtime: {}", e))
    }

    #[inline]
    fn into_closed_subscription_error(subject: &str) -> SpotApiErrorAny {
        SpotApiErrorAny::Common(CommonError::Internal {
            message: format!("NATS subscription closed for subject {}", subject),
        })
    }

    pub fn actor_name(&self) -> &str {
        self.actor_name
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl<E, H> EventRecvActor<E, SpotApiErrorAny> for NatsEventActor<E, H>
where
    E: Send + Sync + 'static,
    H: EventHandler<E, (), SpotApiErrorAny> + Send + Sync + 'static,
{
    fn recv_event(&mut self) -> Result<Option<E>, SpotApiErrorAny> {
        let message = self.subscription.recv(&self.runtime, &self.subject)?;

        tracing::info!(
            actor = self.actor_name,
            subject = %self.subject,
            payload_len = message.payload.len(),
            "Received NATS event"
        );

        let event = (self.deserialize)(&message.payload)?;
        Ok(Some(event))
    }

    fn handle_event(&self, event: E) -> Result<(), SpotApiErrorAny> {
        tracing::info!(
            actor = self.actor_name,
            subject = %self.subject,
            "Handling NATS event"
        );

        self.handler.event_handle(event)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use base_types::handler::event_handler::EventHandler;

    use super::*;

    struct NoopHandler;

    impl EventHandler<(), (), SpotApiErrorAny> for NoopHandler {
        fn event_handle(&self, _event: ()) -> Result<(), SpotApiErrorAny> {
            Ok(())
        }
    }

    fn deserialize_unit(_bytes: &[u8]) -> Result<(), SpotApiErrorAny> {
        Ok(())
    }

    #[test]
    fn nats_event_actor_type_checks() {
        let _ctor: fn(
            NatsProcessorConfig,
            String,
            Arc<NoopHandler>,
            &'static str,
            fn(&[u8]) -> Result<(), SpotApiErrorAny>,
        ) -> Result<NatsEventActor<(), NoopHandler>, String> = NatsEventActor::new;

        let _deserialize = deserialize_unit as fn(&[u8]) -> Result<(), SpotApiErrorAny>;
    }
}
