use base_types::exchange::spot::spot_types::SpotOrder;
use crossbeam_channel::Receiver;
use diff::diff_types::DomainEvent;

use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::v2::processor::disruptor::inproc_event_actor::InprocEventActor;
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_order_place_event_handler::NewOrderPlaceEventHandler;

pub type DisruptorMatchingEventActor<R, P, L> =
    InprocEventActor<DomainEvent<SpotOrder>, NewOrderPlaceEventHandler<R, P, L>>;

impl<R, P, L> DisruptorMatchingEventActor<R, P, L>
where
    R: CmdRepo2 + Send + Sync + 'static,
    P: EventPublisher2 + Send + Sync + 'static,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send + Sync + 'static,
{
    pub fn from_parts(
        receiver: Receiver<DomainEvent<SpotOrder>>,
        handler: std::sync::Arc<NewOrderPlaceEventHandler<R, P, L>>,
    ) -> Self {
        InprocEventActor::new(receiver, handler, "disruptor-matching-event-actor")
    }
}
