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

use rand::Rng;
use base_types::{OrderId, AccountId, TradingPair, AssetId, Price, Quantity, OrderSide};
use base_types::exchange::spot::spot_types::{OrderType, TimeInForce, TraderId};

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

    // lob_repo 可以是 EmbeddedLobRepo<SpotOrder> 或者DistributedLobRepo<SpotOrder>
    // 交易对路由 - 静态分发
    lob_repo: L
}


impl<L: MultiSymbolLobRepo<Order = SpotOrder>> SpotTradeBehaviorV2Impl<L> {
    /// 订单预处理 - 负责创建订单、冻结余额和生成事件
    fn pre_process(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 生成订单ID（这里使用简单的时间戳+随机数，实际应该使用更 robust 的生成方式）
        let order_id = OrderId::from((*cmd.timestamp() as u64) << 32 | (rand::random::<u32>() as u64));
        let trader_id = TraderId::default(); //  TODO: 从 metadata 中获取真实的 trader_id
        let account_id = AccountId(1); //  TODO: 从 metadata 中获取真实的 account_id
        let trading_pair = TradingPair {
            base_asset: AssetId(1),
            quote_asset: AssetId(0)
        }; //  TODO: 根据 symbol 解析 trading_pair

        // 根据 NewOrderCmd 创建 SpotOrder
        let mut internal_order = match cmd.order_type() {
            OrderType::Limit => SpotOrder::create_limit(
                order_id,
                trader_id,
                account_id,
                trading_pair,
                *cmd.side(),
                Price::from_f64(cmd.price().unwrap_or(0.0)),
                Quantity::from_f64(cmd.quantity().unwrap_or(0.0)),
                cmd.time_in_force(),
                cmd.new_client_order_id().map(|s| s.to_string()),
            ),
            OrderType::Market => {
                // 市价单处理
                todo!("Market order type not implemented yet")
            }

            OrderType::StopLoss => {
                todo!("Stop order type not implemented yet")
            }
            OrderType::StopLossLimit => {
                todo!("Stop order type not implemented yet")

            }
            OrderType::TakeProfit => {
                todo!("Stop order type not implemented yet")

            }
            OrderType::TakeProfitLimit => {
                todo!("Stop order type not implemented yet")

            }
            OrderType::LimitMaker => {
                todo!("Stop order type not implemented yet")

            }
        };
        
        match internal_order.time_in_force {
            TimeInForce::GTC => {}
            TimeInForce::IOC => {}
            TimeInForce::FOK => {}
            TimeInForce::GTX => {}
            TimeInForce::GTD => {}
        }

        match internal_order.side{
            OrderSide::Buy => {}
            OrderSide::Sell => {}
        }
        // 根据买卖方向冻结相应的余额
        let now = *cmd.timestamp() as u64;
        let frozen_asset_balance_id = internal_order.frozen_asset_balance_id();
        let mut frozen_asset_balance = self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id).unwrap().unwrap();

        // 冻结余额
        internal_order.frozen_margin(&mut frozen_asset_balance, now);

        // TODO: 生成新增/账户冻结 eventlog
        // TODO: 发送eventlog到消息队列，行情对外发布消息
        // TODO: 在db中回放eventlog

        // 生成 NewOrderAck 响应
        let ack = NewOrderAck::new(
            cmd.symbol().to_string(),
            order_id as i64,
            -1, // 不属于任何订单列表
            cmd.new_client_order_id().unwrap_or_else(|| format!("{}", order_id.into())),
            *cmd.timestamp()
        );

        Ok(CmdResp::new(0, SpotTradeResAny::NewOrderAck(ack)))
    }

    /// 处理新订单命令的主方法
    fn handle(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 执行订单预处理
        let ack_result = self.pre_process(cmd)?;

        // TODO: 执行订单匹配逻辑
        // let matches = self.lob_repo.match_order(&internal_order);

        // TODO: 处理匹配结果，生成交易记录

        // 所有数据持久化操作，一次性回放所有事件到数据库
        let all_events: Vec<ChangeLogEntry> = Vec::new();

        let change_log_queue_repo = ChangeLogChannelQueueRepo::new();
        change_log_queue_repo.send_batch(&all_events);

        if !all_events.is_empty() {
            // 回放 matched_order 更新和订单创建事件到各自的 repo
            for event in &all_events {
                // 根据 entity_type 判断回放到哪个 repo
                match event.entity_type().as_str() {
                    "SpotOrder" => {
                        if let Err(e) = self.order_repo.replay_event(event) {
                            log::error!("Failed to replay order event: {:?}", e);
                        }
                    }
                    "SpotTrade" => {
                        if let Err(e) = self.trade_repo.replay_event(event) {
                            log::error!("Failed to replay trade event: {:?}", e);
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

        Ok(ack_result)
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
                // 执行订单处理
                self.handle(new_order)
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
