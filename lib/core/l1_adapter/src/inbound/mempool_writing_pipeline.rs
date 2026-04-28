use std::marker::PhantomData;

use cmd_handler::use_case_def::DomainEventPipeline;
use l1_core::{MempoolPort, ReceiveAndAdmitTransactionsError, ReceiveAndAdmitTransactionsEvents};

pub struct MempoolWritingPipeline<E> {
    mempool: Box<dyn MempoolPort>,
    _events: PhantomData<E>,
}

impl<E> MempoolWritingPipeline<E> {
    pub fn new(mempool: Box<dyn MempoolPort>) -> Self {
        Self { mempool, _events: PhantomData }
    }
}

impl DomainEventPipeline<ReceiveAndAdmitTransactionsEvents, ReceiveAndAdmitTransactionsError>
    for MempoolWritingPipeline<ReceiveAndAdmitTransactionsEvents>
{
    fn persist(
        &self,
        _events: &ReceiveAndAdmitTransactionsEvents,
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }

    fn replay(
        &self,
        events: &ReceiveAndAdmitTransactionsEvents,
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        self.mempool
            .add_requests(events.admitted_requests.clone())
            .map_err(|e| ReceiveAndAdmitTransactionsError::LoadStateFailed(format!("{:?}", e)))
    }

    fn publish(
        &self,
        _events: &ReceiveAndAdmitTransactionsEvents,
    ) -> Result<(), ReceiveAndAdmitTransactionsError> {
        Ok(())
    }
}
