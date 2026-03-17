use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::cqrs::cqrs_types::{CmdResp, ResMetadata};
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler::Handler;
use base_types::Timestamp;
use db_repo::MySqlDbRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::{SpotTradeCmdAny, SpotTradeResAny};
use crate::proc::v2::trade_handlers::account_handler::AccountHandler;
use crate::proc::v2::trade_handlers::oco_handler::OcoHandler;
use crate::proc::v2::trade_handlers::order_handler::OrderHandler;

/// SpotTradeBehaviorV3 实现
///
/// 设计原则：
/// - 主协调器作为薄层路由，将命令分发到对应的 Handler
/// - 每个 Handler 负责一类命令的处理逻辑
/// - 共享依赖通过构造函数注入
pub struct SpotTradeBehaviorV3Impl {
    // 命令处理器
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
    ) -> Self {
        // 初始化各个 Handler
        let order_handler = Arc::new(OrderHandler::new(
            balance_repo.clone(),
            trade_repo.clone(),
            order_repo.clone(),
            lob_repo.clone(),
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

impl Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny> for SpotTradeBehaviorV3Impl {
    async fn handle(
        &self,
        cmd: SpotTradeCmdAny,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        let nonce = 0; // TODO: 从命令元数据中获取

        match cmd {
            // ========== 订单相关命令 ==========
            SpotTradeCmdAny::NewOrder(new_order) => self.order_handler.handle_new_order(new_order),

            SpotTradeCmdAny::TestNewOrder(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp::default()),
                SpotTradeResAny::TestNewOrderEmpty,
            )),

            SpotTradeCmdAny::CancelOrder(cancel_order) => {
                todo!()
                // self.order_handler.handle_cancel_order(cancel_order)
            }

            SpotTradeCmdAny::CancelAllOpenOrders(_) => {
                todo!("Implement cancel all open orders")
            }

            SpotTradeCmdAny::CancelReplaceOrder(_) => {
                todo!("Implement cancel replace order")
            }

            SpotTradeCmdAny::QueryOrder(query_order) => {
                todo!()
                // self.order_handler.handle_query_order(query_order)
            }

            SpotTradeCmdAny::CurrentOpenOrders(_) => {
                todo!("Implement current open orders")
            }

            SpotTradeCmdAny::AllOrders(_) => {
                todo!("Implement all orders")
            }

            // ========== OCO 订单相关命令 ==========
            SpotTradeCmdAny::NewOcoOrder(new_oco) => self.oco_handler.handle_new_oco_order(new_oco),

            SpotTradeCmdAny::NewOtoOrder(_) => {
                todo!("Implement new OTO order")
            }

            SpotTradeCmdAny::NewOtocoOrder(_) => {
                todo!("Implement new OTOCO order")
            }

            SpotTradeCmdAny::CancelOcoOrder(cancel_oco) => {
                self.oco_handler.handle_cancel_oco_order(cancel_oco)
            }

            SpotTradeCmdAny::QueryOcoOrder(query_oco) => {
                self.oco_handler.handle_query_oco_order(query_oco)
            }

            SpotTradeCmdAny::AllOcoOrders(_) => {
                todo!("Implement all OCO orders")
            }

            SpotTradeCmdAny::OpenOcoOrders(_) => {
                todo!("Implement open OCO orders")
            }

            // ========== 账户相关命令 ==========
            SpotTradeCmdAny::Account(account) => self.account_handler.handle_account(account),

            SpotTradeCmdAny::MyTrades(my_trades) => {
                self.account_handler.handle_my_trades(my_trades)
            }

            SpotTradeCmdAny::QueryUnfilledOrderCount(unfilled_count) => {
                self.account_handler.handle_unfilled_order_count(unfilled_count)
            }

            SpotTradeCmdAny::QueryPreventedMatches(_) => {
                todo!("Implement query prevented matches")
            }

            SpotTradeCmdAny::QueryAllocations(_) => {
                todo!("Implement query allocations")
            }

            SpotTradeCmdAny::QueryCommissionRates(_) => {
                todo!("Implement query commission rates")
            }
        }
    }
}
