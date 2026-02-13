use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::base_types::TraderId;
use base_types::cqrs::cqrs_types::ResMetadata;
use base_types::exchange::spot::spot_types::{
    ConditionalType, ExecutionMethod, MakerConstraint, OrderType, SpotOrder, SpotTrade, TimeInForce,
};
use base_types::handler::handler::Handler;
use base_types::lob::lob::LobOrder;
use base_types::spot_topic::SpotTopic;
use base_types::{AccountId, AssetId, OrderId, OrderSide, Price, Quantity, Timestamp, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::{ChangeLogEntry, Entity};
use immutable_derive::immutable;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;
use rand::Rng;
use rust_queue::queue::queue::{Queue, ToBytes};
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_market_data_sse_behavior::SpotMarketDataStreamAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    NewOrderAck, NewOrderCmd, SpotTradeCmdAny, SpotTradeResAny,
};
use crate::proc::behavior::v2::spot_user_data_sse_behavior::UserDataStreamEventAny;
use crate::proc::v2::id_repo::order_next_id;

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
                SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                )
            }
            OrderType::Market => {
                // 市价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价单价格为0
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC, // 市价单默认IOC
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.execution_method = ExecutionMethod::Market;
                order.price = None; // 市价单价格为None
                order
            }
            OrderType::StopLoss => {
                // 止损单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价止损
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.conditional_type = ConditionalType::StopLoss;
                order.stop_price = *cmd.stop_price();
                order.execution_method = ExecutionMethod::Market;
                order.price = None;
                order
            }
            OrderType::StopLossLimit => {
                // 止损限价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.conditional_type = ConditionalType::StopLoss;
                order.stop_price = *cmd.stop_price();
                order
            }
            OrderType::TakeProfit => {
                // 止盈单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价止盈
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.conditional_type = ConditionalType::TakeProfit;
                order.stop_price = *cmd.stop_price();
                order.execution_method = ExecutionMethod::Market;
                order.price = None;
                order
            }
            OrderType::TakeProfitLimit => {
                // 止盈限价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.conditional_type = ConditionalType::TakeProfit;
                order.stop_price = *cmd.stop_price();
                order
            }
            OrderType::LimitMaker => {
                // 限价只挂单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    account_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::GTX, // GTX = PostOnly
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
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
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    // uid路由
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    // uid路由
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,

    // uid路由
    user_data_repo: Arc<MySqlDbRepo<SpotOrder>>,

    // 交易对路由
    market_data_repo: Arc<MySqlDbRepo<SpotOrder>>,

    // lob_repo 可以是 EmbeddedLobRepo<SpotOrder> 或者DistributedLobRepo<SpotOrder>
    // 交易对路由 - 静态分发
    lob_repo: L,

    queue: Arc<MPMCQueue>,
}

impl<L: MultiSymbolLobRepo<Order = SpotOrder>> SpotTradeBehaviorV2Impl<L> {
    /// 根据资产ID查找余额ID
    fn queryBalanceId(&self, asset_id: AssetId) -> String {
        // BalanceId format: "account_id:asset_id"
        format!("{:?}:{}", AccountId(1), u32::from(asset_id))
    }

    /// 订单预处理 - 负责创建订单、冻结余额和生成事件
    fn pre_process(&self, cmd: NewOrderCmd) -> (Balance, SpotOrder) {
        // 生成订单ID（这里使用简单的时间戳+随机数，实际应该使用更 robust 的生成方式）
        // 根据 NewOrderCmd 创建 SpotOrder
        // todo cmd.clone() 太贵了
        let mut internal_order = SpotOrder::from(cmd);

        // 根据买卖方向冻结相应的资产余额：买则冻结计算资产，卖则冻结基础资产
        let asset_id = match internal_order.side() {
            OrderSide::Buy => internal_order.trading_pair.quote_asset(),
            OrderSide::Sell => internal_order.trading_pair.base_asset(),
        };

        // todo 根据资产查找余额id
        let frozen_asset_balance_id = self.queryBalanceId(asset_id);

        //查账户,竞争点
        let mut frozen_asset_balance =
            self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id).unwrap().unwrap();

        //计算逻辑
        let (order_change_log, balance_change_log) =
            self.handle_data(&mut frozen_asset_balance, &mut internal_order);

        // TODO: 发送eventlog到消息队列，user data/market data对外发布消息
        // TODO: 在db中回放eventlog
        //持久消息队列
        self.queue.send(
            SpotTopic::EntityChangeLog.name(),
            balance_change_log.to_bytes().unwrap(),
            None,
        );
        self.queue.send(
            SpotTopic::EntityChangeLog.name(),
            order_change_log.to_bytes().unwrap(),
            None,
        );

        //数据库回放以便持久
        if let Err(e) = self.balance_repo.replay_event(&balance_change_log) {
            log::error!("Failed to replay balance event: {:?}", e);
        }
        if let Err(e) = self.order_repo.replay_event(&order_change_log) {
            log::error!("Failed to replay order event: {:?}", e);
        }

        (frozen_asset_balance, internal_order)
    }

    fn handle_data(
        &self,
        balance: &mut Balance,
        order: &mut SpotOrder,
    ) -> (ChangeLogEntry, ChangeLogEntry) {
        let (balance_change_log, order_change_log) = match order.side() {
            OrderSide::Buy => {
                // Freeze the quote asset balance
                // Priority: use quote_order_qty if available (market orders), otherwise use price * quantity
                let frozen_amount = if let Some(quote_qty) = order.quote_order_qty {
                    // Market order with explicit quote amount
                    quote_qty
                } else {
                    // Limit order: use price * quantity
                    order.price.unwrap_or(Price::from_f64(0.0)) * order.quantity()
                };

                // Generate update log with frozen balance
                let balance_change_log = balance
                    .track_update(|b| {
                        b.frozen(frozen_amount, order.timestamp);
                    })
                    .unwrap();

                //todo order有需要改的吗？
                // Update order and generate create log
                let order_change_log = order.track_create().unwrap();

                (balance_change_log, order_change_log)
            }
            OrderSide::Sell => {
                // Freeze the base asset balance (quantity)
                let balance_change_log = balance
                    .track_update(|b| {
                        b.frozen(order.quantity(), order.timestamp);
                    })
                    .unwrap();

                // Update order and generate create log
                let order_change_log = order.track_create().unwrap();

                (balance_change_log, order_change_log)
            }
        };

        (balance_change_log, order_change_log)
    }

    /// 处理新订单命令的主方法
    fn handle(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 执行订单预处理
        let (mut frozen_asset_balance, mut internal_order) = self.pre_process(cmd);
        // TODO: 执行订单匹配逻辑
        let matches = self.lob_repo.match_order(&internal_order);

        let (
            trades,
            trade_change_logs,
            order_change_log,
            matches_change_log,
            base_balance_change_log,
            quote_balance_change_log,
            base_balance_change_logs,
            quote_balance_change_logs,
        ) = self.handle_data2(&mut frozen_asset_balance, &mut internal_order, &mut matches);

        // TODO: 处理匹配结果，生成交易记录

        // 所有数据持久化操作，一次性回放所有事件到数据库
        let all_events: Vec<ChangeLogEntry> = Vec::new();

        // self.

        // let change_log_queue_repo = ChangeLogChannelQueueRepo::new();
        // change_log_queue_repo.send_batch(&all_events);

        // 批量发送事件 - 将 ChangeLogEntry 转换为 Bytes
        let bytes_events: Vec<bytes::Bytes> = all_events
            .iter()
            .filter_map(|event| serde_json::to_vec(event).ok().map(bytes::Bytes::from))
            .collect();

        let results = self.queue.send_batch(SpotTopic::EntityChangeLog.name(), bytes_events, None);

        // 检查发送结果
        if let Ok(send_results) = results {
            for (index, result) in send_results.iter().enumerate() {
                match result {
                    Ok(count) => {
                        if *count == 0 {
                            log::warn!("Event {} sent but no subscribers received it", index);
                        } else {
                            log::debug!("Event {} received by {} subscribers", index, count);
                        }
                    }
                    Err(e) => log::error!("Failed to send event {}: {:?}", index, e),
                }
            }
        } else {
            log::error!("Failed to send batch events");
        }

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

        let ack = NewOrderAck::new(
            internal_order.trading_pair,
            internal_order.order_id,
            -1, // 不属于任何订单列表
            internal_order.client_order_id,
            internal_order.timestamp,
        );

        Ok(CmdResp::new(
            ResMetadata::new(0, false, internal_order.timestamp),
            SpotTradeResAny::NewOrderAck(ack),
        ))
    }
}

impl<L: MultiSymbolLobRepo<Order = SpotOrder>>
    Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny> for SpotTradeBehaviorV2Impl<L>
{
    async fn handle(
        &self,
        cmd: SpotTradeCmdAny,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 使用固定的 nonce 值，实际应用中应该从命令元数据中获取
        let nonce = 0;

        match cmd {
            SpotTradeCmdAny::NewOrder(new_order) => {
                // 执行订单处理
                self.handle(new_order)
            }

            SpotTradeCmdAny::TestNewOrder(_) => Ok(CmdResp::new(
                ResMetadata::new(nonce, false, Timestamp::default()),
                SpotTradeResAny::TestNewOrderEmpty,
            )),
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
