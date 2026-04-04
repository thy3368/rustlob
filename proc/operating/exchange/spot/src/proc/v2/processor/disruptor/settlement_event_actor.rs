use base_types::exchange::spot::spot_types::SpotTrade;
use crossbeam_channel::Receiver;
use diff::diff_types::DomainEvent;

use crate::proc::v2::processor::disruptor::inproc_event_actor::InprocEventActor;
use crate::proc::v2::trade_event_handlers::new_trade_event_handler::NewTradeEventHandler;

pub type DisruptorSettlementEventActor =
    InprocEventActor<DomainEvent<SpotTrade>, NewTradeEventHandler>;

impl DisruptorSettlementEventActor {
    pub fn from_parts(
        receiver: Receiver<DomainEvent<SpotTrade>>,
        handler: std::sync::Arc<NewTradeEventHandler>,
    ) -> Self {
        InprocEventActor::new(receiver, handler, "disruptor-settlement-event-actor")
    }
}
