use base_types::exchange::spot::spot_types::SpotTrade;
use diff::diff_types::DomainEvent;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::nats::nats_event_actor::{NatsEventActor, NatsProcessorConfig};
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_trade_event_handler::NewTradeEventHandler;

pub type NatsSettlementEventActor =
    NatsEventActor<DomainEvent<SpotTrade>, NewTradeEventHandler>;

impl NatsSettlementEventActor {
    pub fn from_parts(
        config: NatsProcessorConfig,
        subject: String,
        handler: std::sync::Arc<NewTradeEventHandler>,
    ) -> Result<Self, String> {
        NatsEventActor::new(
            config,
            subject,
            handler,
            "nats-settlement-event-actor",
            deserialize_settlement_domain_event,
        )
    }
}

fn deserialize_settlement_domain_event(
    bytes: &[u8],
) -> Result<DomainEvent<SpotTrade>, SpotCmdErrorAny> {
    serde_json::from_slice(bytes).map_err(|e| SpotCmdErrorAny::Common(
        crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        },
    ))
}
