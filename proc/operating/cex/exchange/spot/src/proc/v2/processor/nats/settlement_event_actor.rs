use base_types::exchange::spot::spot_types::SpotTrade;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::nats::nats_event_actor::{NatsEventActor, NatsProcessorConfig};
use crate::proc::v2::trade_cmd_handlers::v3::event_handler::new_trade_event_handler::NewTradeEventHandler;

pub type NatsSettlementEventActor<R, P> =
    NatsEventActor<DomainEvent<SpotTrade>, NewTradeEventHandler<R, P>>;

impl<R: CmdRepo2, P: EventPublisher2> NatsSettlementEventActor<R, P> {
    pub fn from_parts(
        config: NatsProcessorConfig,
        subject: String,
        handler: std::sync::Arc<NewTradeEventHandler<R, P>>,
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
    serde_json::from_slice(bytes).map_err(|e| {
        SpotCmdErrorAny::Common(crate::proc::behavior::spot_trade_behavior::CommonError::Internal {
            message: format!("Deserialization error: {}", e),
        })
    })
}
