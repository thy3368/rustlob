use std::sync::Arc;

use base_types::handler::event_actor::EventActor;
use base_types::handler::event_handler::EventHandler2;
use crossbeam_channel::Receiver;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

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
    fn into_internal_error(message: impl Into<String>) -> SpotCmdErrorAny {
        SpotCmdErrorAny::Common(CommonError::Internal {
            message: message.into(),
        })
    }

    pub fn actor_name(&self) -> &str {
        self.actor_name
    }
}

impl<E, H> EventActor<E, SpotCmdErrorAny> for InprocEventActor<E, H>
where
    E: Send + Sync + 'static,
    H: EventHandler2<E, SpotCmdErrorAny> + Send + Sync + 'static,
{
    fn recv_event(&mut self) -> Result<Option<E>, SpotCmdErrorAny> {
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

    fn handle_event(&self, event: E) -> Result<(), SpotCmdErrorAny> {
        tracing::info!(
            actor = self.actor_name,
            "Handling inproc/disruptor event"
        );

        self.handler.event_handle(event)?;
        Ok(())
    }
}
