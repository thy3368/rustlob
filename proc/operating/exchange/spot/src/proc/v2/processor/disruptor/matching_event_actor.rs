use base_types::exchange::spot::spot_types::SpotOrder;
use crossbeam_channel::Receiver;
use diff::diff_types::DomainEvent;

use crate::proc::v2::processor::disruptor::inproc_event_actor::InprocEventActor;
use crate::proc::v2::trade_event_handlers::new_order_place_event_handler::NewOrderPlaceEventHandler;

pub type MatchingEventActor = InprocEventActor<DomainEvent<SpotOrder>, NewOrderPlaceEventHandler>;

impl MatchingEventActor {
    pub fn new_matching_actor(
        receiver: Receiver<DomainEvent<SpotOrder>>,
        handler: std::sync::Arc<NewOrderPlaceEventHandler>,
    ) -> Self {
        Self::new(receiver, handler, "disruptor-matching-event-actor")
    }
}
