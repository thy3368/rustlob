use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::event_handler::EventHandler2;
use cmd_handler::CmdHandlerForUpdate3;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;

use crate::proc::behavior::v2::spot_trade_error::SpotCmdErrorAny;
use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::sett_order_handler::{
    SettOrderCmdHandler, SettlementCmd,
};

pub struct NewTradeEventHandler<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone> {
    settlement_handler: Arc<SettOrderCmdHandler<R, P>>,
}

impl<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone> NewTradeEventHandler<R, P> {
    pub fn new(settlement_handler: Arc<SettOrderCmdHandler<R, P>>) -> Self {
        Self { settlement_handler }
    }
}

impl<R: CmdRepo2 + Clone, P: EventPublisher2 + Clone>
    EventHandler2<DomainEvent<SpotTrade>, SpotCmdErrorAny> for NewTradeEventHandler<R, P>
{
    fn event_handle(&self, event: DomainEvent<SpotTrade>) -> Result<(), SpotCmdErrorAny> {
        let (_, trade) = event.into_parts();
        let cmd = SettlementCmd { trades: vec![trade] };
        let _ = self.settlement_handler.cmd_handle(
            cmd,
            self.settlement_handler.repo.clone(),
            self.settlement_handler.publisher.clone(),
        )?;
        Ok(())
    }
}
