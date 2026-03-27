use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler::Handler2;
use db_repo::MySqlDbRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    NewOrderAck, NewOrderCmd, SpotTradeCmd, SpotTradeCmdOrQuery, SpotTradeResAny,
};
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;
use crate::proc::v2::trade_handlers::account_handler::AccountHandler;
use crate::proc::v2::trade_handlers::oco_handler::OcoHandler;
use crate::proc::v2::trade_handlers::order_handler::OrderHandler;

pub struct SpotTradeBehaviorV4Impl {
    order_handler: Arc<OrderHandler>,
    oco_handler: Arc<OcoHandler>,
    account_handler: Arc<AccountHandler>,
}

impl SpotTradeBehaviorV4Impl {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        let order_handler = Arc::new(OrderHandler::new(
            balance_repo.clone(),
            trade_repo.clone(),
            order_repo.clone(),
            lob_repo.clone(),
            event_publisher.clone(),
        ));

        let oco_handler = Arc::new(OcoHandler::new(
            balance_repo.clone(),
            trade_repo.clone(),
            order_repo.clone(),
            lob_repo.clone(),
        ));

        let account_handler = Arc::new(AccountHandler::new(
            balance_repo.clone(),
            trade_repo.clone(),
            order_repo.clone(),
        ));

        Self { order_handler, oco_handler, account_handler }
    }
}

impl Handler2<NewOrderCmd, NewOrderAck, SpotCmdErrorAny> for SpotTradeBehaviorV4Impl {
    fn handle(&self, cmd: NewOrderCmd) -> Result<NewOrderAck, SpotCmdErrorAny> {
        self.order_handler.accept_new_order(cmd)
    }
}
impl Handler2<SpotTradeCmdOrQuery, SpotTradeResAny, SpotCmdErrorAny> for SpotTradeBehaviorV4Impl {
    fn handle(&self, cmd: SpotTradeCmdOrQuery) -> Result<SpotTradeResAny, SpotCmdErrorAny> {
        match cmd {
            SpotTradeCmdOrQuery::Cmd(cmd) => match cmd {
                SpotTradeCmd::NewOrder(cmd) => {
                    let ack = <Self as Handler2<NewOrderCmd, NewOrderAck, SpotCmdErrorAny>>::handle(self, cmd)?;
                    Ok(SpotTradeResAny::NewOrderAck(ack))
                }
                SpotTradeCmd::TestNewOrder(_) => todo!(),
                SpotTradeCmd::CancelOrder(_) => todo!(),
                SpotTradeCmd::CancelAllOpenOrders(_) => todo!(),
                SpotTradeCmd::CancelReplaceOrder(_) => todo!(),
                SpotTradeCmd::NewOcoOrder(_) => todo!(),
                SpotTradeCmd::NewOtoOrder(_) => todo!(),
                SpotTradeCmd::NewOtocoOrder(_) => todo!(),
                SpotTradeCmd::CancelOcoOrder(_) => todo!(),
            },
            SpotTradeCmdOrQuery::Query(_query) => todo!(),
        }
    }
}
