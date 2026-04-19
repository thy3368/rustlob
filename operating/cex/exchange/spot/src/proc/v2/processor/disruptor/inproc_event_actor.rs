use std::sync::Arc;

use base_types::handler::event_actor::EventRecvActor;
use base_types::handler::event_handler::EventHandler2;
use crossbeam_channel::Receiver;

use crate::proc::behavior::v2::spot_trade_error::{CommonError, SpotApiErrorAny};

pub struct InprocEventActor<E, H> {
    receiver: Receiver<E>,
    handler: Arc<H>,
    actor_name: &'static str,
}

impl<E, H> InprocEventActor<E, H> {
    pub fn new(receiver: Receiver<E>, handler: Arc<H>, actor_name: &'static str) -> Self {
        Self {
            receiver,
            handler,
            actor_name,
        }
    }

    #[inline]
    fn into_internal_error(message: impl Into<String>) -> SpotApiErrorAny {
        SpotApiErrorAny::Common(CommonError::Internal {
            message: message.into(),
        })
    }

    pub fn actor_name(&self) -> &str {
        self.actor_name
    }
}

impl<E, H> EventRecvActor<E, SpotApiErrorAny> for InprocEventActor<E, H>
where
    E: Send + Sync + 'static,
    H: EventHandler2<E, SpotApiErrorAny> + Send + Sync + 'static,
{
    fn recv_event(&mut self) -> Result<Option<E>, SpotApiErrorAny> {
        let event = self.receiver.recv().map_err(|e| {
            Self::into_internal_error(format!(
                "Failed to receive message from inproc/disruptor channel: {}",
                e
            ))
        })?;

        tracing::info!(
            actor = self.actor_name,
            "Received inproc/disruptor event"
        );

        Ok(Some(event))
    }

    fn handle_event(&self, event: E) -> Result<(), SpotApiErrorAny> {
        tracing::info!(
            actor = self.actor_name,
            "Handling inproc/disruptor event"
        );

        self.handler.event_handle(event)?;
        Ok(())
    }
}
