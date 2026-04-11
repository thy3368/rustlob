use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::lob::lob::LobOrder;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::nats::nats_event_actor::{NatsEventActor, NatsProcessorConfig};
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_order_place_event_handler::NewOrderPlaceEventHandler;

pub type NatsMatchingEventActor<R, P, L> =
    NatsEventActor<DomainEvent<SpotOrder>, NewOrderPlaceEventHandler<R, P, L>>;

impl<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone, L: MultiSymbolLobRepo<Order = SpotOrder>>
    NatsMatchingEventActor<R, P, L>
{
    pub fn from_parts(
        config: NatsProcessorConfig,
        subject: String,
        handler: std::sync::Arc<NewOrderPlaceEventHandler<R, P, L>>,
    ) -> Result<Self, String> {
        NatsEventActor::new(
            config,
            subject,
            handler,
            "nats-matching-event-actor",
            deserialize_matching_domain_event,
        )
    }
}

fn deserialize_matching_domain_event(
    bytes: &[u8],
) -> Result<DomainEvent<SpotOrder>, SpotCmdErrorAny> {
    serde_json::from_slice(bytes).map_err(|e| {
        SpotCmdErrorAny::Common(crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        })
    })
}
