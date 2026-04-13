use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::handler::event_handler::EventHandler2;
use cmd_handler::CmdHandlerForUpdate3;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::v2::spot_trade_error::SpotCmdErrorAny;
use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::match_order_handler::{
    MatchCmd, MatchOrderCmdHandler,
};

pub struct NewOrderPlaceEventHandler<
    R: CmdRepo2 + Clone,
    P: EventPublisher2 + Clone,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> {
    matching_handler: Arc<MatchOrderCmdHandler<R, P, L>>,
}

impl<
    R: CmdRepo2 + Clone,
    P: EventPublisher2 + Clone,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> NewOrderPlaceEventHandler<R, P, L>
{
    pub fn new(matching_handler: Arc<MatchOrderCmdHandler<R, P, L>>) -> Self {
        Self { matching_handler }
    }
}

impl<
    R: CmdRepo2 + Clone,
    P: EventPublisher2 + Clone,
    L: MultiSymbolLobRepo<Order = SpotOrder> + Send,
> EventHandler2<DomainEvent<SpotOrder>, SpotCmdErrorAny> for NewOrderPlaceEventHandler<R, P, L>
{
    fn event_handle(&self, event: DomainEvent<SpotOrder>) -> Result<(), SpotCmdErrorAny> {
        let cmd = MatchCmd {
            taker_order: event.object().clone(),
        };
        let _ = self.matching_handler.cmd_handle(
            cmd,
            self.matching_handler.repo.clone(),
            self.matching_handler.publisher.clone(),
        )?;
        Ok(())
    }
}
