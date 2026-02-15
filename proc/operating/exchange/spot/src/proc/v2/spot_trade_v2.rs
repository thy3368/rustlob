use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::account::error::BalanceError;
use base_types::base_types::TraderId;
use base_types::cqrs::cqrs_types::ResMetadata;
use base_types::exchange::spot::spot_types::{
    ConditionalType, ExecutionMethod, MakerConstraint, OrderType, SpotOrder, SpotTrade, TimeInForce,
};
use base_types::handler::handler::Handler;
use base_types::lob::lob::LobOrder;
use base_types::mark_data::spot::level_types::OrderDelta;
use base_types::spot_topic::SpotTopic;
use base_types::{AccountId, AssetId, OrderId, OrderSide, Price, Quantity, Timestamp, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::{ChangeLogEntry, Entity};
use immutable_derive::immutable;
use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;
use rand::Rng;
use rust_queue::queue::queue::{Queue, ToBytes};
use rust_queue::queue::queue_impl::mpmc_queue::MPMCQueue;

use crate::proc::behavior::spot_trade_behavior::{CmdResp, CommonError, SpotCmdErrorAny};
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
        let trading_pair = cmd.symbol().clone();

        // todo 可以simd优化吗
        let order_type = *cmd.order_type();
        match order_type {
            OrderType::Limit => {
                // 限价单 - 直接使用命令字段创建 SpotOrder，零拷贝
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
                order
            }
            OrderType::Market => {
                // 市价单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价单价格为0
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC, // 市价单默认IOC
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
                order.execution_method = ExecutionMethod::Market;
                order.price = None; // 市价单价格为None
                order
            }
            OrderType::StopLoss => {
                // 止损单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价止损
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
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
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
                order.conditional_type = ConditionalType::StopLoss;
                order.execution_method = ExecutionMethod::Limit;
                order.stop_price = *cmd.stop_price();
                order
            }
            OrderType::TakeProfit => {
                // 止盈单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    Price::from_f64(0.0), // 市价止盈
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::IOC,
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
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
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    cmd.time_in_force().unwrap_or(TimeInForce::GTC),
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
                order.conditional_type = ConditionalType::TakeProfit;
                order.execution_method = ExecutionMethod::Limit;
                order.stop_price = *cmd.stop_price();
                order
            }
            OrderType::LimitMaker => {
                // 限价只挂单
                let mut order = SpotOrder::create_order(
                    order_id,
                    trader_id,
                    trading_pair,
                    cmd.side().clone(),
                    cmd.price().unwrap_or(Price::from_f64(0.0)),
                    cmd.quantity().unwrap_or(Quantity::from_f64(0.0)),
                    TimeInForce::GTX, // GTX = PostOnly
                    cmd.new_client_order_id().clone(),
                    cmd.quote_order_qty().clone(),
                );
                order.order_type = order_type;
                order
            }
        }
    }
}

#[immutable]
pub struct SpotTradeBehaviorV2Impl {
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
    // 交易对路由 - 动态分发
    lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,

    queue: Arc<MPMCQueue>,
}

impl SpotTradeBehaviorV2Impl {
    /// 根据资产ID查找余额ID
    fn queryBalanceId(&self, asset_id: AssetId) -> String {
        // BalanceId format: "account_id:asset_id"
        format!("{:?}:{}", AccountId(1), u32::from(asset_id))
    }

    /// 订单预处理 - 负责创建订单、冻结余额和生成事件
    fn pre_process(&self, cmd: NewOrderCmd) -> Result<(SpotOrder), SpotCmdErrorAny> {
        // 生成订单ID（这里使用简单的时间戳+随机数，实际应该使用更 robust 的生成方式）
        // 根据 NewOrderCmd 创建 SpotOrder
        // todo cmd.clone() 太贵了

        let mut internal_order = SpotOrder::from(cmd);

        match internal_order.conditional_type {
            ConditionalType::None => {}
            ConditionalType::StopLoss => {
                //要专门挂 到特定低价格才挂
                todo!()
            }
            ConditionalType::TakeProfit => {
                //要专门挂 到特定高价格才挂
                todo!()
            }
        }

        let frozen_asset_id = internal_order.frozen_asset_id();
        let frozen_asset_balance_id = self.queryBalanceId(frozen_asset_id);

        //查账户,竞争点
        let mut frozen_asset_balance =
            self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id).unwrap().unwrap();

        //计算逻辑
        let (order_change_log, balance_change_log) = self
            .handle_data(&mut frozen_asset_balance, &mut internal_order)
            .map_err(|e| match e {
                BalanceError::InsufficientAvailable { required, available } => {
                    SpotCmdErrorAny::Common(CommonError::InsufficientBalance {
                        required: required as u64,
                        available: available as u64,
                    })
                }
                _ => SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Balance error: {}", e),
                }),
            })?;

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

        Ok(( internal_order))
    }

    fn handle_data(
        &self,
        balance: &mut Balance,
        order: &mut SpotOrder,
    ) -> Result<(ChangeLogEntry, ChangeLogEntry), BalanceError> {
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

                // Pre-check balance sufficiency before track_update
                let available_raw = balance.available.raw();
                let frozen_raw = frozen_amount.raw();
                if available_raw < frozen_raw {
                    return Err(BalanceError::InsufficientAvailable {
                        required: frozen_raw,
                        available: available_raw,
                    });
                }

                // Generate update log with frozen balance
                let balance_change_log = balance
                    .track_update(|b| {
                        // Safe to unwrap because we pre-checked the balance
                        b.frozen(frozen_amount, order.timestamp).unwrap();
                    })
                    .map_err(|_| BalanceError::Overflow)?;

                // Update order and generate create log
                let order_change_log = order.track_create().unwrap();

                (balance_change_log, order_change_log)
            }
            OrderSide::Sell => {
                // Pre-check balance sufficiency
                let available_raw = balance.available.raw();
                let qty_raw = order.quantity().raw();
                if available_raw < qty_raw {
                    return Err(BalanceError::InsufficientAvailable {
                        required: qty_raw,
                        available: available_raw,
                    });
                }

                // Freeze the base asset balance (quantity)
                let balance_change_log = balance
                    .track_update(|b| {
                        // Safe to unwrap because we pre-checked the balance
                        b.frozen(order.quantity(), order.timestamp).unwrap();
                    })
                    .map_err(|_| BalanceError::Overflow)?;

                // Update order and generate create log
                let order_change_log = order.track_create().unwrap();

                (balance_change_log, order_change_log)
            }
        };

        Ok((balance_change_log, order_change_log))
    }

    /// 处理订单匹配逻辑
    ///
    /// 根据匹配的订单生成交易记录，变更日志已在内部持久化
    fn handle_match(&self, internal_order: &mut SpotOrder) -> Vec<SpotTrade> {
        // TODO: 2.执行订单匹配逻辑
        let (matches, remaining) = self.lob_repo.match_orders(
            internal_order.trading_pair,
            internal_order.side,
            internal_order.price.unwrap(),
            internal_order.quote_order_qty.unwrap(),
        );
        let is_fully_filled = remaining.is_zero();

        let mut trades = Vec::new();

        //todo 对每种TimeInForce 单独处理订单
        match internal_order.time_in_force {
            //持续有效直到取消 - 处理匹配的订单，剩余部分进入订单簿
            TimeInForce::GTC => {
                match internal_order.execution_method {
                    ExecutionMethod::Limit => {
                        // 限价单：处理匹配的订单，剩余部分进入订单簿
                        if let Some(matched_orders) = matches {
                            for matched_order in matched_orders {
                                // 计算成交数量
                                let filled =
                                    internal_order.unfilled_qty.min(matched_order.unfilled_qty);

                                // 计算成交价格
                                let transaction_price =
                                    matched_order.price.unwrap_or(internal_order.price.unwrap());

                                // 生成交易ID
                                let trade_id = (internal_order.timestamp.0 << 32)
                                    | (internal_order.order_id & 0xFFFFFFFF) as u64;

                                // 计算手续费（简化版，使用默认费率）
                                let taker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.001);
                                let maker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.0005);

                                // 创建成交记录
                                let trade = SpotTrade::new(
                                    trade_id,
                                    internal_order.trading_pair,
                                    internal_order.order_id,
                                    matched_order.order_id,
                                    Timestamp::now_as_nanos(),
                                    transaction_price,
                                    filled,
                                    internal_order.side,
                                    taker_commission_qty,
                                    maker_commission_qty,
                                    internal_order.frozen_asset,
                                    10, // taker commission rate in bp
                                    5,  // maker commission rate in bp
                                );
                                trades.push(trade);
                            }
                        }

                        // 如果有剩余数量，将订单添加到LOB
                        if !is_fully_filled {
                            // 将internal_order添加到LOB
                            if let Err(e) = self
                                .lob_repo
                                .add_order(internal_order.trading_pair, internal_order.clone())
                            {
                                log::error!("Failed to add order to LOB: {:?}", e);
                            }
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Pending;
                        } else {
                            // 全部成交
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Filled;
                        }
                    }
                    ExecutionMethod::Market => {
                        // GTC不支持市价单
                        todo!("GTC does not support market orders")
                    }
                }
            }
            TimeInForce::IOC => {
                //立即成交否则取消
                match internal_order.execution_method {
                    ExecutionMethod::Limit => {
                        // 限价单：处理匹配的订单，剩余部分取消
                        if let Some(matched_orders) = matches {
                            for matched_order in matched_orders {
                                // 计算成交数量
                                let filled =
                                    internal_order.unfilled_qty.min(matched_order.unfilled_qty);

                                // 计算成交价格
                                let transaction_price =
                                    matched_order.price.unwrap_or(internal_order.price.unwrap());

                                // 生成交易ID
                                let trade_id = (internal_order.timestamp.0 << 32)
                                    | (internal_order.order_id & 0xFFFFFFFF) as u64;

                                // 计算手续费
                                let taker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.001);
                                let maker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.0005);

                                // 创建成交记录
                                let trade = SpotTrade::new(
                                    trade_id,
                                    internal_order.trading_pair,
                                    internal_order.order_id,
                                    matched_order.order_id,
                                    Timestamp::now_as_nanos(),
                                    transaction_price,
                                    filled,
                                    internal_order.side,
                                    taker_commission_qty,
                                    maker_commission_qty,
                                    internal_order.frozen_asset,
                                    10,
                                    5,
                                );
                                trades.push(trade);
                            }
                        }
                        // 剩余部分自动取消，不进入订单簿
                        if !remaining.is_zero() {
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Cancelled;
                        } else {
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Filled;
                        }
                    }
                    ExecutionMethod::Market => {
                        // 市价单：尽可能成交，剩余部分取消
                        if let Some(matched_orders) = matches {
                            for matched_order in matched_orders {
                                // 计算成交数量
                                let filled =
                                    internal_order.unfilled_qty.min(matched_order.unfilled_qty);

                                // 计算成交价格
                                let transaction_price =
                                    matched_order.price.unwrap_or(internal_order.price.unwrap());

                                // 生成交易ID
                                let trade_id = (internal_order.timestamp.0 << 32)
                                    | (internal_order.order_id & 0xFFFFFFFF) as u64;

                                // 计算手续费
                                let taker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.001);
                                let maker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.0005);

                                // 创建成交记录
                                let trade = SpotTrade::new(
                                    trade_id,
                                    internal_order.trading_pair,
                                    internal_order.order_id,
                                    matched_order.order_id,
                                    Timestamp::now_as_nanos(),
                                    transaction_price,
                                    filled,
                                    internal_order.side,
                                    taker_commission_qty,
                                    maker_commission_qty,
                                    internal_order.frozen_asset,
                                    10,
                                    5,
                                );
                                trades.push(trade);
                            }
                        }
                        internal_order.status =
                            base_types::exchange::spot::spot_types::OrderStatus::Cancelled;
                    }
                }
            }
            TimeInForce::FOK => {
                //全部成交否则取消
                match internal_order.execution_method {
                    ExecutionMethod::Limit => {
                        if is_fully_filled {
                            // 全部成交：处理所有匹配的订单
                            if let Some(matched_orders) = matches {
                                for matched_order in matched_orders {
                                    // 计算成交数量
                                    let filled =
                                        internal_order.unfilled_qty.min(matched_order.unfilled_qty);

                                    // 计算成交价格
                                    let transaction_price = matched_order
                                        .price
                                        .unwrap_or(internal_order.price.unwrap());

                                    // 生成交易ID
                                    let trade_id = (internal_order.timestamp.0 << 32)
                                        | (internal_order.order_id & 0xFFFFFFFF) as u64;

                                    // 计算手续费
                                    let taker_commission_qty =
                                        filled * transaction_price * Quantity::from_f64(0.001);
                                    let maker_commission_qty =
                                        filled * transaction_price * Quantity::from_f64(0.0005);

                                    // 创建成交记录
                                    let trade = SpotTrade::new(
                                        trade_id,
                                        internal_order.trading_pair,
                                        internal_order.order_id,
                                        matched_order.order_id,
                                        Timestamp::now_as_nanos(),
                                        transaction_price,
                                        filled,
                                        internal_order.side,
                                        taker_commission_qty,
                                        maker_commission_qty,
                                        internal_order.frozen_asset,
                                        10,
                                        5,
                                    );
                                    trades.push(trade);
                                }
                            }
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Filled;
                        } else {
                            // 未能全部成交：取消整个订单
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Cancelled;
                        }
                    }
                    ExecutionMethod::Market => {
                        // 市价单：必须全部成交
                        if is_fully_filled {
                            if let Some(matched_orders) = matches {
                                for matched_order in matched_orders {
                                    // 计算成交数量
                                    let filled =
                                        internal_order.unfilled_qty.min(matched_order.unfilled_qty);

                                    // 计算成交价格
                                    let transaction_price = matched_order
                                        .price
                                        .unwrap_or(internal_order.price.unwrap());

                                    // 生成交易ID
                                    let trade_id = (internal_order.timestamp.0 << 32)
                                        | (internal_order.order_id & 0xFFFFFFFF) as u64;

                                    // 计算手续费
                                    let taker_commission_qty =
                                        filled * transaction_price * Quantity::from_f64(0.001);
                                    let maker_commission_qty =
                                        filled * transaction_price * Quantity::from_f64(0.0005);

                                    // 创建成交记录
                                    let trade = SpotTrade::new(
                                        trade_id,
                                        internal_order.trading_pair,
                                        internal_order.order_id,
                                        matched_order.order_id,
                                        Timestamp::now_as_nanos(),
                                        transaction_price,
                                        filled,
                                        internal_order.side,
                                        taker_commission_qty,
                                        maker_commission_qty,
                                        internal_order.frozen_asset,
                                        10,
                                        5,
                                    );
                                    trades.push(trade);
                                }
                            }
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Filled;
                        } else {
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Cancelled;
                        }
                    }
                }
            }
            TimeInForce::GTX => {
                //只做Maker直到成交（Post-Only）
                match internal_order.execution_method {
                    ExecutionMethod::Limit => {
                        if matches.is_some() {
                            // GTX订单不能立即成交，必须作为Maker
                            // 如果有匹配，说明会立即成交，应该拒绝
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Rejected;
                        } else {
                            // 没有匹配，可以作为Maker进入订单簿
                            // TODO: 将internal_order添加到LOB
                        }
                    }
                    ExecutionMethod::Market => {
                        // GTX不支持市价单
                        todo!("GTX does not support market orders")
                    }
                }
            }
            TimeInForce::GTD => {
                //持续有效直到指定日期/时间
                match internal_order.execution_method {
                    ExecutionMethod::Limit => {
                        // 限价单：处理匹配的订单，剩余部分进入订单簿（直到GTD时间）
                        if let Some(matched_orders) = matches {
                            for matched_order in matched_orders {
                                // 计算成交数量
                                let filled =
                                    internal_order.unfilled_qty.min(matched_order.unfilled_qty);

                                // 计算成交价格
                                let transaction_price =
                                    matched_order.price.unwrap_or(internal_order.price.unwrap());

                                // 生成交易ID
                                let trade_id = (internal_order.timestamp.0 << 32)
                                    | (internal_order.order_id & 0xFFFFFFFF) as u64;

                                // 计算手续费
                                let taker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.001);
                                let maker_commission_qty =
                                    filled * transaction_price * Quantity::from_f64(0.0005);

                                    // 创建成交记录
                                    let trade = SpotTrade::new(
                                        trade_id,
                                        internal_order.trading_pair,
                                        internal_order.order_id,
                                        matched_order.order_id,
                                        Timestamp::now_as_nanos(),
                                        transaction_price,
                                        filled,
                                        internal_order.side,
                                        taker_commission_qty,
                                        maker_commission_qty,
                                        internal_order.frozen_asset,
                                        10,
                                        5,
                                    );
                                    trades.push(trade);
                                }
                        }
                        // 如果有剩余数量，将订单添加到LOB
                        if !is_fully_filled {
                            // 将internal_order添加到LOB（带GTD时间约束）
                            if let Err(e) = self
                                .lob_repo
                                .add_order(internal_order.trading_pair, internal_order.clone())
                            {
                                log::error!("Failed to add order to LOB: {:?}", e);
                            }
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Pending;
                        } else {
                            // 全部成交
                            internal_order.status =
                                base_types::exchange::spot::spot_types::OrderStatus::Filled;
                        }
                    }
                    ExecutionMethod::Market => {
                        // GTD不支持市价单
                        todo!("GTD does not support market orders")
                    }
                }
            }
        }

        // 生成新订单的变更日志
        let order_change_log = internal_order.track_create().unwrap();

        if let Err(e) = self.order_repo.replay_event(&order_change_log) {
            log::error!("Failed to replay order event: {:?}", e);
        }

        trades
    }

    /// 处理交易的清算操作
    ///
    /// 根据成交记录更新双方账户余额、扣除手续费、生成变更日志
    fn handle_settlement(&self, trades: Vec<SpotTrade>) -> Vec<ChangeLogEntry> {
        let mut balance_change_logs = Vec::new();

        // 使用第一笔交易的交易对（所有交易应该属于同一交易对）
        let trading_pair = trades.first().map(|t| t.trading_pair);

        // 使用硬编码的账户ID（与 pre_process 中的 queryBalanceId 一致）
        // TODO: 从真实的账户信息中获取
        let taker_account_id = AccountId(1);
        let maker_account_id = AccountId(1);

        for trade in trades {
            let trading_pair = match trading_pair {
                Some(tp) => tp,
                None => continue,
            };
            let base_asset = trading_pair.base_asset();
            let quote_asset = trading_pair.quote_asset();

            // 构建余额ID
            let taker_base_balance_id = format!("{}:{}", taker_account_id.0, u32::from(base_asset));
            let taker_quote_balance_id =
                format!("{}:{}", taker_account_id.0, u32::from(quote_asset));
            let maker_base_balance_id = format!("{}:{}", maker_account_id.0, u32::from(base_asset));
            let maker_quote_balance_id =
                format!("{}:{}", maker_account_id.0, u32::from(quote_asset));
            // 根据 Taker 方向更新余额
            match trade.taker_side {
                OrderSide::Buy => {
                    // Taker 买入：支付 quote 资产，获得 base 资产
                    // Maker 卖出：支付 base 资产，获得 quote 资产

                    // 更新 Taker 的 quote 资产（支付）
                    if let Ok(Some(mut taker_quote_balance)) =
                        self.balance_repo.find_by_id_4_update(&taker_quote_balance_id)
                    {
                        let payment = trade.quantity * trade.price;
                        let log = taker_quote_balance.track_update(|b| {
                            b.frozen2pay(payment, Timestamp::now_as_nanos());
                            b.frozen2pay(trade.taker_commission_qty, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }

                    // 更新 Taker 的 base 资产（获得）
                    if let Ok(Some(mut taker_base_balance)) =
                        self.balance_repo.find_by_id_4_update(&taker_base_balance_id)
                    {
                        let log = taker_base_balance.track_update(|b| {
                            b.add_balance(trade.quantity, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }

                    // 更新 Maker 的 base 资产（支付）
                    if let Ok(Some(mut maker_base_balance)) =
                        self.balance_repo.find_by_id_4_update(&maker_base_balance_id)
                    {
                        let log = maker_base_balance.track_update(|b| {
                            b.frozen2pay(trade.quantity, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }

                    // 更新 Maker 的 quote 资产（获得）
                    if let Ok(Some(mut maker_quote_balance)) =
                        self.balance_repo.find_by_id_4_update(&maker_quote_balance_id)
                    {
                        let proceeds = trade.quantity * trade.price;
                        let log = maker_quote_balance.track_update(|b| {
                            b.add_balance(proceeds, Timestamp::now_as_nanos());
                            b.frozen2pay(trade.maker_commission_qty, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }
                }
                OrderSide::Sell => {
                    // Taker 卖出：支付 base 资产，获得 quote 资产
                    // Maker 买入：支付 quote 资产，获得 base 资产

                    // 更新 Taker 的 base 资产（支付）
                    if let Ok(Some(mut taker_base_balance)) =
                        self.balance_repo.find_by_id_4_update(&taker_base_balance_id)
                    {
                        let log = taker_base_balance.track_update(|b| {
                            b.frozen2pay(trade.quantity, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }

                    // 更新 Taker 的 quote 资产（获得）
                    if let Ok(Some(mut taker_quote_balance)) =
                        self.balance_repo.find_by_id_4_update(&taker_quote_balance_id)
                    {
                        let proceeds = trade.quantity * trade.price;
                        let log = taker_quote_balance.track_update(|b| {
                            b.add_balance(proceeds, Timestamp::now_as_nanos());
                            b.frozen2pay(trade.taker_commission_qty, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }

                    // 更新 Maker 的 quote 资产（支付）
                    if let Ok(Some(mut maker_quote_balance)) =
                        self.balance_repo.find_by_id_4_update(&maker_quote_balance_id)
                    {
                        let payment = trade.quantity * trade.price;
                        let log = maker_quote_balance.track_update(|b| {
                            b.frozen2pay(payment, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }

                    // 更新 Maker 的 base 资产（获得）
                    if let Ok(Some(mut maker_base_balance)) =
                        self.balance_repo.find_by_id_4_update(&maker_base_balance_id)
                    {
                        let log = maker_base_balance.track_update(|b| {
                            b.add_balance(trade.quantity, Timestamp::now_as_nanos());
                            b.frozen2pay(trade.maker_commission_qty, Timestamp::now_as_nanos());
                        });
                        if let Ok(entry) = log {
                            balance_change_logs.push(entry);
                            let _ = self
                                .balance_repo
                                .replay_event(&balance_change_logs.last().unwrap());
                        }
                    }
                }
            }
        }


        // 所有数据持久化操作，一次性回放所有事件到数据库

        // 批量发送事件 - 将 ChangeLogEntry 转换为 Bytes
        let bytes_events: Vec<bytes::Bytes> = balance_change_logs
            .iter()
            .filter_map(|event| serde_json::to_vec(event).ok().map(bytes::Bytes::from))
            .collect();

        let results = self.queue.send_batch(SpotTopic::EntityChangeLog.name(), bytes_events, None);


        balance_change_logs
    }

    /// 处理新订单命令的主方法
    fn handle(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 1.执行订单预处理
        let ( mut internal_order) = self.pre_process(cmd)?;

        // 2.撮合逻辑
        let trades = self.handle_match(&mut internal_order);



        // 3.执行交易的结算操作
        let balance_change_logs = self.handle_settlement(trades);






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

impl Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny> for SpotTradeBehaviorV2Impl {
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

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use base_types::base_types::TraderId;
    use base_types::mark_data::spot::level_types::OrderChangeType;
    use base_types::Price;

    use super::*;

    #[test]
    fn test_orderbook_delta_creation() {
        let delta = OrderDelta {
            symbol_id: 1,
            timestamp: 1234567890,
            sequence: 100,
            change_type: OrderChangeType::Add,
            order_id: 12345,
            side: OrderSide::Buy,
            price: Price::from_raw(50000),
            quantity: Quantity::from_raw(100),
            trader_id: Some(TraderId::default()),
        };

        assert_eq!(delta.symbol_id, 1);
        assert_eq!(delta.change_type, OrderChangeType::Add);
        assert_eq!(delta.order_id, 12345);
    }
}
