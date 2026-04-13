use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler::CmdHandler;
use db_repo::MySqlDbRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::v2::spot_trade_error::SpotApiErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior::{
    NewOrderAck, NewOrderCmd, SpotTradeCmd, SpotTradeCmdOrQuery, SpotTradeResAny,
};
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;

pub struct SpotTradeBehaviorV4Impl {}

impl SpotTradeBehaviorV4Impl {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {}
    }
}

impl CmdHandler<NewOrderCmd, NewOrderAck, SpotApiErrorAny> for SpotTradeBehaviorV4Impl {
    fn cmd_handle(&self, cmd: NewOrderCmd) -> Result<NewOrderAck, SpotApiErrorAny> {
        todo!()
    }
}
impl CmdHandler<SpotTradeCmdOrQuery, SpotTradeResAny, SpotApiErrorAny> for SpotTradeBehaviorV4Impl {
    fn cmd_handle(&self, cmd: SpotTradeCmdOrQuery) -> Result<SpotTradeResAny, SpotApiErrorAny> {
        match cmd {
            SpotTradeCmdOrQuery::Cmd(cmd) => match cmd {
                SpotTradeCmd::NewOrder(cmd) => {
                    let ack =
                        <Self as CmdHandler<NewOrderCmd, NewOrderAck, SpotApiErrorAny>>::cmd_handle(
                            self, cmd,
                        )?;
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
