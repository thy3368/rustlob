use base_types::{
    account::balance::Balance,
    exchange::spot::spot_types::{SpotOrder, SpotTrade},
    handler::handler::Handler
};
use db_repo::{
    adapter::change_log_queue_repo::ChangeLogChannelQueueRepo, core::queue_repo::ChangeLogQueueRepo, CmdRepo,
    MySqlDbRepo
};
use diff::ChangeLogEntry;
use immutable_derive::immutable;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::{
        spot_market_data_sse_behavior::SpotMarketDataStreamAny,
        spot_trade_behavior_v2::{NewOrderCmd, SpotTradeCmdAny, SpotTradeResAny},
        spot_user_data_sse_behavior::UserDataStreamEventAny
    }
};
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderAck;

#[immutable]
pub struct SpotTradeBehaviorV2Impl<L: MultiSymbolLobRepo<Order = SpotOrder>> {
    // uid路由
    balance_repo: MySqlDbRepo<Balance>,
    // uid路由
    trade_repo: MySqlDbRepo<SpotTrade>,
    // uid路由
    order_repo: MySqlDbRepo<SpotOrder>,

    // uid路由
    user_data_repo: MySqlDbRepo<SpotOrder>,

    // 交易对路由
    market_data_repo: MySqlDbRepo<SpotOrder>,

    // // todo?
    // pub change_log_repo: MySqlDbRepo<ChangeLogEntry>,
    // // uid路由
    // pub user_data_update_repo: MySqlDbRepo<UserDataStreamEventAny>,
    // // 交易对路由
    // pub market_data_update_repo: MySqlDbRepo<SpotMarketDataStreamAny>,

    // lob_repo 可以是 EmbeddedLobRepo<SpotOrder> 或者DistributedLobRepo<SpotOrder>
    // 交易对路由 - 静态分发
    lob_repo: L
}


impl<L: MultiSymbolLobRepo<Order = SpotOrder>> SpotTradeBehaviorV2Impl<L> {
    // pub fn new(
    //     balance_repo: MySqlDbRepo<Balance>, trade_repo: MySqlDbRepo<SpotTrade>,
    // order_repo: MySqlDbRepo<SpotOrder>,     user_data_repo:
    // MySqlDbRepo<SpotOrder>, market_data_repo: MySqlDbRepo<SpotOrder>, lob_repo: L
    // ) -> Self {
    //     Self {
    //         balance_repo,
    //         trade_repo,
    //         order_repo,
    //         user_data_repo,
    //         market_data_repo,
    //         lob_repo
    //     }
    // }

    fn dj(&self, cmd: NewOrderCmd) -> Result<CmdResp<NewOrderAck>, SpotCmdErrorAny>{


        //todo 生成 spot order
        let mut internal_order = SpotOrder::create_limit(
            order_id,
            limit_order.trader,
            limit_order.account_id,
            limit_order.trading_pair,
            limit_order.side,
            limit_order.price,
            limit_order.quantity,
            limit_order.time_in_force,
            limit_order.client_order_id,
        );

        //数据竞争点是 余额账户 需要加锁
        //todo 根据买卖单 分别冻结不同的余额账户
        // 生成新增/账户冻结 eventlog
        // 发送eventlog到消息对列，行情对外发布消息
        //todo 在db中回放eventlog
        // 对order进行撮合操作


        let mut frozen_asset_balance = self.balance_repo.find_by_id(&"xxxxx").unwrap().unwrap();
        let mut base_asset_balance = self.balance_repo.find_by_id(&"base_asset_balance_id").unwrap().unwrap();

    }
    fn handle(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {







        // 所有数据持久化操作，一次性回放所有事件到数据库
        let all_events: Vec<ChangeLogEntry> = Vec::new();


        let change_log_queue_repo = ChangeLogChannelQueueRepo::new();
        change_log_queue_repo.send_batch(&all_events);


        if !all_events.is_empty() {
            // 回放 matched_order 更新和 v1 创建事件到各自的 repo
            for event in &all_events {
                // 根据 entity_type 判断回放到哪个 repo
                // todo 增加balance position
                match event.entity_type().as_str() {
                    "SpotOrder" => {
                        if let Err(e) = self.order_repo.replay_event(event) {
                            log::error!("Failed to replay order event: {:?}", e);
                        }
                    }
                    "SpotTrade" => {
                        if let Err(e) = self.trade_repo.replay_event(event) {
                            log::error!("Failed to replay v1 event: {:?}", e);
                        }
                    }
                    "Balance" => {
                        if let Err(e) = self.balance_repo.replay_event(event) {
                            log::error!("Failed to replay balance event: {:?}", e);
                        }
                    }

                    _ => {}
                }
            }
        }

        todo!()
    }
}


impl<L: MultiSymbolLobRepo<Order = SpotOrder>> Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny>
    for SpotTradeBehaviorV2Impl<L>
{
    async fn handle(&self, cmd: SpotTradeCmdAny) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 使用固定的 nonce 值，实际应用中应该从命令元数据中获取
        let nonce = 0;

        match cmd {
            SpotTradeCmdAny::NewOrder(new_order) => {
                // todo
                // todo
                // todo 匹配 通知

                // 生成user data
                // 生成market data

                self.handle(new_order)

                // Ok(CmdResp::new(nonce, SpotTradeResAny::TestNewOrderEmpty))
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
