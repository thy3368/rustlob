use base_types::exchange::spot::spot_types::SpotOrder;
use diff::diff_types::DomainEvent;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::nats::nats_event_actor::{NatsEventActor, NatsProcessorConfig};
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_order_place_event_handler::NewOrderPlaceEventHandler;

pub type NatsMatchingEventActor =
    NatsEventActor<DomainEvent<SpotOrder>, NewOrderPlaceEventHandler>;

impl NatsMatchingEventActor {
    pub fn from_parts(
        config: NatsProcessorConfig,
        subject: String,
        handler: std::sync::Arc<NewOrderPlaceEventHandler>,
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
    serde_json::from_slice(bytes).map_err(|e| SpotCmdErrorAny::Common(
        crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        },
    ))
}
