use std::sync::Arc;

use base_types::exchange::spot::spot_types::SpotOrder;
use base_types::handler::event_handler::EventHandler;
use base_types::handler::handler_update::CmdHandlerForUpdate;
use db_repo::{MySqlDbRepo, QueryRepo};
use diff::ChangeLog;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::trade_cmd_handlers::matching_handler::{
    MatchCmd, MatchResult, MatchingHandler,
};

pub struct NewOrderPlaceEventHandler {
    //todo order_repo相关操作 应该收签到command handler
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,
    matching_handler: Arc<MatchingHandler>,
}

impl NewOrderPlaceEventHandler {
    pub fn new(
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        matching_handler: Arc<MatchingHandler>,
    ) -> Self {
        Self { order_repo, matching_handler }
    }

    fn load_taker_order(&self, event: &ChangeLog) -> Result<SpotOrder, SpotCmdErrorAny> {
        let order_id = event.entity_id();
        self.order_repo
            .find_by_id(order_id)
            .map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to load order {}: {}", order_id, e),
                })
            })?
            .ok_or_else(|| {
                SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "order_id",
                    reason: "order not found",
                })
            })
    }
}

//todo 处理change log 变成 DomainEvent<SpotOrder>
impl EventHandler<ChangeLog, MatchResult, SpotCmdErrorAny> for NewOrderPlaceEventHandler {
    fn event_handle(&self, event: ChangeLog) -> Result<MatchResult, SpotCmdErrorAny> {
        let taker_order = self.load_taker_order(&event)?;
        let cmd = MatchCmd { taker_order };
        self.matching_handler.cmd_handle(cmd, |writes, _| writes.clone())
    }
}
