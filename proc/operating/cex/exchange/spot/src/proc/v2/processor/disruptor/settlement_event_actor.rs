use base_types::exchange::spot::spot_types::SpotTrade;
use crossbeam_channel::Receiver;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;

use crate::proc::v2::processor::disruptor::inproc_event_actor::InprocEventActor;
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_trade_event_handler::NewTradeEventHandler;

pub type DisruptorSettlementEventActor<R, P> =
    InprocEventActor<DomainEvent<SpotTrade>, NewTradeEventHandler<R, P>>;

impl<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone> DisruptorSettlementEventActor<R, P> {
    pub fn from_parts(
        receiver: Receiver<DomainEvent<SpotTrade>>,
        handler: std::sync::Arc<NewTradeEventHandler<R, P>>,
    ) -> Self {
        InprocEventActor::new(receiver, handler, "disruptor-settlement-event-actor")
    }
}
