use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::handler::event_handler::EventHandler2;
use base_types::handler::handler_update2::CmdHandlerForUpdate2;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::match_order_handler::{
    MatchCmd, MatchOrderCmdHandler,
};

pub struct NewOrderPlaceEventHandler<
    R: CmdRepo2,
    P: EventPublisher,
    L: MultiSymbolLobRepo<Order = SpotOrder>,
> {
    matching_handler: Arc<MatchOrderCmdHandler<R, P, L>>,
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo<Order = SpotOrder>>
    NewOrderPlaceEventHandler<R, P, L>
{
    pub fn new(matching_handler: Arc<MatchOrderCmdHandler<R, P, L>>) -> Self {
        Self { matching_handler }
    }
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo<Order = SpotOrder>>
    EventHandler2<DomainEvent<SpotOrder>, SpotCmdErrorAny> for NewOrderPlaceEventHandler<R, P, L>
{
    fn event_handle(&self, event: DomainEvent<SpotOrder>) -> Result<(), SpotCmdErrorAny> {
        let cmd = MatchCmd {
            taker_order: event.object().clone(),
        };
        let _ = self.matching_handler.cmd_handle(cmd)?;
        Ok(())
    }
}
