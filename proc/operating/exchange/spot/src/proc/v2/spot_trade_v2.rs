use base_types::{
    account::balance::Balance,
    exchange::spot::spot_types::{SpotOrder, SpotTrade},
    handler::handler::Handler
};
use db_repo::MySqlDbRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::spot_trade_behavior_v2::{SpotTradeCmdAny, SpotTradeResAny}
};

pub struct SpotTradeBehaviorV2Impl<L: MultiSymbolLobRepo<Order = SpotOrder>> {
    // uid路由
    pub balance_repo: MySqlDbRepo<Balance>,
    // uid路由
    pub trade_repo: MySqlDbRepo<SpotTrade>,
    // uid路由
    pub order_repo: MySqlDbRepo<SpotOrder>,
    // uid路由
    pub user_data_repo: MySqlDbRepo<SpotOrder>,
    // 交易对路由
    pub market_data_repo: MySqlDbRepo<SpotOrder>,
    // lob_repo 可以是 EmbeddedLobRepo<SpotOrder> 或者DistributedLobRepo<SpotOrder>
    // 交易对路由 - 静态分发
    pub lob_repo: L
}


impl<L: MultiSymbolLobRepo<Order = SpotOrder>> SpotTradeBehaviorV2Impl<L> {
    pub fn new(
        balance_repo: MySqlDbRepo<Balance>, trade_repo: MySqlDbRepo<SpotTrade>, order_repo: MySqlDbRepo<SpotOrder>,
        user_data_repo: MySqlDbRepo<SpotOrder>, market_data_repo: MySqlDbRepo<SpotOrder>, lob_repo: L
    ) -> Self {
        Self {
            balance_repo,
            trade_repo,
            order_repo,
            user_data_repo,
            market_data_repo,
            lob_repo
        }
    }
}


impl<L: MultiSymbolLobRepo<Order = SpotOrder>> Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny>
    for SpotTradeBehaviorV2Impl<L>
{
    async fn handle(&self, cmd: SpotTradeCmdAny) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 使用固定的 nonce 值，实际应用中应该从命令元数据中获取
        let nonce = 0;

        match cmd {
            SpotTradeCmdAny::NewOrder(_) => {
                // todo
                // todo
                // todo 匹配 通知
                Ok(CmdResp::new(nonce, SpotTradeResAny::TestNewOrderEmpty))
            }

            SpotTradeCmdAny::TestNewOrder(_) => Ok(CmdResp::new(nonce, SpotTradeResAny::TestNewOrderEmpty)),
            SpotTradeCmdAny::CancelOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::CancelAllOpenOrders(_) => {
                todo!()
            }
            SpotTradeCmdAny::CancelReplaceOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::QueryOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::CurrentOpenOrders(_) => {
                todo!()
            }
            SpotTradeCmdAny::AllOrders(_) => {
                todo!()
            }
            SpotTradeCmdAny::NewOcoOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::NewOtoOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::NewOtocoOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::CancelOcoOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::QueryOcoOrder(_) => {
                todo!()
            }
            SpotTradeCmdAny::AllOcoOrders(_) => {
                todo!()
            }
            SpotTradeCmdAny::OpenOcoOrders(_) => {
                todo!()
            }
            SpotTradeCmdAny::Account(_) => {
                todo!()
            }
            SpotTradeCmdAny::MyTrades(_) => {
                todo!()
            }
            SpotTradeCmdAny::QueryUnfilledOrderCount(_) => {
                todo!()
            }
            SpotTradeCmdAny::QueryPreventedMatches(_) => {
                todo!()
            }
            SpotTradeCmdAny::QueryAllocations(_) => {
                todo!()
            }
            SpotTradeCmdAny::QueryCommissionRates(_) => {
                todo!()
            }
        }
    }
}
