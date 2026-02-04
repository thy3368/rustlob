use base_types::{
    account::balance::Balance,
    base_types::TraderId,
    cqrs::cqrs_types::ResMetadata,
    exchange::spot::spot_types::{
        ConditionalType, ExecutionMethod, MakerConstraint, OrderType, SpotOrder, SpotTrade, TimeInForce
    },
    handler::handler::Handler,
    AccountId, AssetId, OrderId, OrderSide, Price, Quantity, Timestamp, TradingPair
};
use db_repo::{
    adapter::change_log_queue_repo::ChangeLogChannelQueueRepo, core::queue_repo::ChangeLogQueueRepo, CmdRepo,
    MySqlDbRepo
};
use diff::ChangeLogEntry;
use immutable_derive::immutable;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;
use rand::Rng;
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::{
    behavior::{
        spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
        v2::{
            spot_market_data_sse_behavior::SpotMarketDataStreamAny,
            spot_trade_behavior_v2::{NewOrderAck, NewOrderCmd, SpotTradeCmdAny, SpotTradeResAny},
            spot_user_data_sse_behavior::UserDataStreamEventAny
        }
    },
    v2::id_repo::order_next_id
};

// 方案1：直接在 Command 上实现 Entity 转换（零拷贝）
impl From<NewOrderCmd> for SpotOrder {
    #[inline(always)]
    fn from(cmd: NewOrderCmd) -> Self {
        let order_id = order_next_id as u64;


        let trader_id = TraderId::default(); // TODO: 从 metadata 中获取真实的 trader_id
        let account_id = AccountId(1); // TODO: 从 metadata 中获取真实的 account_id
        let trading_pair = cmd.symbol().clone();

        // todo 可以simd优化吗
        match cmd.order_type() {
            OrderType::Limit => {
                // 限价单 - 直接使用命令字段创建 SpotOrder，零拷贝
                SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    cmd.price().cloned().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone()
                )
            }
            OrderType::Market => {
                // 市价单
                let mut order = SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    Price::from_f64(0.0), // 市价单价格为0
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC, // 市价单默认IOC
                    cmd.new_client_order_id().clone()
                );
                order.execution_method = ExecutionMethod::Market;
                order.price = None; // 市价单价格为None
                order
            }
            OrderType::StopLoss => {
                // 止损单
                let mut order = SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    Price::from_f64(0.0), // 市价止损
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone()
                );
                order.conditional_type = ConditionalType::StopLoss;
                order.stop_price = cmd.stop_price().cloned();
                order.execution_method = ExecutionMethod::Market;
                order.price = None;
                order
            }
            OrderType::StopLossLimit => {
                // 止损限价单
                let mut order = SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    cmd.price().cloned().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone()
                );
                order.conditional_type = ConditionalType::StopLoss;
                order.stop_price = cmd.stop_price().cloned();
                order
            }
            OrderType::TakeProfit => {
                // 止盈单
                let mut order = SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    Price::from_f64(0.0), // 市价止盈
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone()
                );
                order.conditional_type = ConditionalType::TakeProfit;
                order.stop_price = cmd.stop_price().cloned();
                order.execution_method = ExecutionMethod::Market;
                order.price = None;
                order
            }
            OrderType::TakeProfitLimit => {
                // 止盈限价单
                let mut order = SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    cmd.price().cloned().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone()
                );
                order.conditional_type = ConditionalType::TakeProfit;
                order.stop_price = cmd.stop_price().cloned();
                order
            }
            OrderType::LimitMaker => {
                // 限价只挂单
                let mut order = SpotOrder::create_limit(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    *cmd.side(),
                    cmd.price().cloned().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().cloned().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::GTX, // GTX = PostOnly
                    cmd.new_client_order_id().clone()
                );
                order.maker_constraint = MakerConstraint::PostOnly;
                order
            }
        }
    }
}


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
    lob_repo: L,

    queue: MPMCQueue
}


impl<L: MultiSymbolLobRepo<Order = SpotOrder>> SpotTradeBehaviorV2Impl<L> {
    /// 订单预处理 - 负责创建订单、冻结余额和生成事件
    fn pre_process(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 生成订单ID（这里使用简单的时间戳+随机数，实际应该使用更 robust 的生成方式）

        // 根据 NewOrderCmd 创建 SpotOrder
        let mut internal_order = SpotOrder::from(cmd.clone());

        match internal_order.side {
            OrderSide::Buy => {}
            OrderSide::Sell => {}
        }
        // 根据买卖方向冻结相应的余额
        let now = *cmd.timestamp() as u64;
        let frozen_asset_balance_id = internal_order.frozen_asset_balance_id();
        let mut frozen_asset_balance =
            self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id).unwrap().unwrap();

        // frozen_asset_balance.frozen_margin_4(internal_order);
        // 冻结余额
        internal_order.frozen_margin(&mut frozen_asset_balance, Timestamp::now_as_nanos());

        // TODO: 生成新增/账户冻结 eventlog
        // TODO: 发送eventlog到消息队列，行情对外发布消息
        // TODO: 在db中回放eventlog

        match internal_order.time_in_force {
            TimeInForce::GTC => {}
            TimeInForce::IOC => {}
            TimeInForce::FOK => {}
            TimeInForce::GTX => {}
            TimeInForce::GTD => {}
        }

        // 生成 NewOrderAck 响应
        let order_id = order_next_id as u64;

        let ack = NewOrderAck::new(
            cmd.symbol().clone(),
            order_id,
            -1, // 不属于任何订单列表
            cmd.new_client_order_id().as_ref().unwrap().parse().unwrap(),
            *cmd.timestamp()
        );

        Ok(CmdResp::new(ResMetadata::new(0, false, *cmd.timestamp() as u64), SpotTradeResAny::NewOrderAck(ack)))
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

            SpotTradeCmdAny::TestNewOrder(_) => Ok(CmdResp::new(ResMetadata::new(nonce, false, 0), SpotTradeResAny::TestNewOrderEmpty)),
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
