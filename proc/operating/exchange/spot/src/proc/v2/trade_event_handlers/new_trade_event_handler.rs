use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::event_handler::EventHandler;
use base_types::handler::handler_update::CmdHandlerForUpdate;
use diff::diff_types::DomainEvent;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::trade_cmd_handlers::settlement_handler::{
    DefaultSettlementHandler, SettlementCmd, SettlementResult,
};

pub struct NewTradeEventHandler {
    settlement_handler: Arc<DefaultSettlementHandler>,
}

impl NewTradeEventHandler {
    pub fn new(settlement_handler: Arc<DefaultSettlementHandler>) -> Self {
        Self { settlement_handler }
    }
}

impl EventHandler<DomainEvent<SpotTrade>, SettlementResult, SpotCmdErrorAny>
    for NewTradeEventHandler
{
    fn event_handle(
        &self,
        event: DomainEvent<SpotTrade>,
    ) -> Result<SettlementResult, SpotCmdErrorAny> {
        let (_, trade) = event.into_parts();
        let cmd = SettlementCmd { trades: vec![trade] };
        self.settlement_handler
            .cmd_handle(cmd, |writes, _| writes.clone())
    }
}
