use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::cqrs::cqrs_types::{CmdResp, ResMetadata};
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler::Handler;
use base_types::Timestamp;
use db_repo::MySqlDbRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    SpotTradeCmd, SpotTradeCmdOrQuery, SpotTradeResAny,
};
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;
use crate::proc::v2::trade_handlers::account_handler::AccountHandler;
use crate::proc::v2::trade_handlers::oco_handler::OcoHandler;
use crate::proc::v2::trade_handlers::order_handler::OrderHandler;

pub struct SpotTradeBehaviorV3Impl {
    order_handler: Arc<OrderHandler>,
    oco_handler: Arc<OcoHandler>,
    account_handler: Arc<AccountHandler>,
}

impl SpotTradeBehaviorV3Impl {
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

impl Handler<SpotTradeCmdOrQuery, SpotTradeResAny, SpotCmdErrorAny> for SpotTradeBehaviorV3Impl {
    async fn handle(
        &self,
        cmd: SpotTradeCmdOrQuery,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        let nonce = 0;

        match cmd {
            // ========== 订单相关命令 ==========
            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::NewOrder(new_order)) => {
                self.order_handler.handle_post(new_order)
            }

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::TestNewOrder(_)) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp::default()),
                SpotTradeResAny::TestNewOrderEmpty,
            )),

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::CancelOrder(cancel_order)) => {
                todo!()
            }

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::CancelAllOpenOrders(_)) => {
                todo!("Implement cancel all open orders")
            }

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::CancelReplaceOrder(_)) => {
                todo!("Implement cancel replace order")
            }

            // ========== OCO 订单相关命令 ==========
            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::NewOcoOrder(new_oco)) => {
                self.oco_handler.handle_new_oco_order(new_oco)
            }

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::NewOtoOrder(_)) => {
                todo!("Implement new OTO order")
            }

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::NewOtocoOrder(_)) => {
                todo!("Implement new OTOCO order")
            }

            SpotTradeCmdOrQuery::Cmd(SpotTradeCmd::CancelOcoOrder(cancel_oco)) => {
                self.oco_handler.handle_cancel_oco_order(cancel_oco)
            }

            // ========== 查询相关 ==========
            SpotTradeCmdOrQuery::Query(q) => {
                use crate::proc::behavior::v2::spot_trade_behavior_v2::SpotTradeQuery;
                match q {
                    SpotTradeQuery::QueryOrder(query_order) => {
                        todo!()
                    }
                    SpotTradeQuery::CurrentOpenOrders(_) => {
                        todo!("Implement current open orders")
                    }
                    SpotTradeQuery::AllOrders(_) => {
                        todo!("Implement all orders")
                    }
                    SpotTradeQuery::QueryOcoOrder(query_oco) => {
                        self.oco_handler.handle_query_oco_order(query_oco)
                    }
                    SpotTradeQuery::AllOcoOrders(_) => {
                        todo!("Implement all OCO orders")
                    }
                    SpotTradeQuery::OpenOcoOrders(_) => {
                        todo!("Implement open OCO orders")
                    }
                    SpotTradeQuery::Account(account) => {
                        self.account_handler.handle_account(account)
                    }
                    SpotTradeQuery::MyTrades(my_trades) => {
                        self.account_handler.handle_my_trades(my_trades)
                    }
                    SpotTradeQuery::QueryUnfilledOrderCount(unfilled_count) => {
                        self.account_handler.handle_unfilled_order_count(unfilled_count)
                    }
                    SpotTradeQuery::QueryPreventedMatches(_) => {
                        todo!("Implement query prevented matches")
                    }
                    SpotTradeQuery::QueryAllocations(_) => {
                        todo!("Implement query allocations")
                    }
                    SpotTradeQuery::QueryCommissionRates(_) => {
                        todo!("Implement query commission rates")
                    }
                }
            }
        }
    }
}
