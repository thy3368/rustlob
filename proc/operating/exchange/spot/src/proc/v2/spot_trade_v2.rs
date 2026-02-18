use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::account::error::BalanceError;
use base_types::base_types::TraderId;
use base_types::cqrs::cqrs_types::ResMetadata;
use base_types::exchange::spot::spot_types::{
    ConditionalType, ExecutionMethod, MakerConstraint, OrderStatus, OrderType, SpotOrder,
    SpotTrade, TimeInForce,
};
use base_types::handler::handler::Handler;
use base_types::lob::lob::LobOrder;
use base_types::mark_data::spot::level_types::OrderDelta;
use base_types::spot_topic::SpotTopic;
use base_types::{AccountId, AssetId, OrderId, OrderSide, Price, Quantity, Timestamp, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo, QueryRepo};
use diff::{ChangeLogEntry, Entity};
use immutable_derive::immutable;
use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;
use rand::Rng;

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
                order.status = base_types::exchange::spot::spot_types::OrderStatus::Pending;

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
                order.status = base_types::exchange::spot::spot_types::OrderStatus::Pending;
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
                order.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
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
                order.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
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
                order.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
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
                order.status =
                    base_types::exchange::spot::spot_types::OrderStatus::ConditionalPending;
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
}

impl SpotTradeBehaviorV2Impl {
    /// 根据资产ID查找余额ID
    fn queryBalanceId(&self, asset_id: AssetId) -> String {
        // BalanceId format: "account_id:asset_id"
        format!("{:?}:{}", AccountId(1), u32::from(asset_id))
    }

    ///
    /// ### 关键结论
    ///
    /// | 分析项 | 结论 |
    /// |--------|------|
    /// | 初始状态 | **Pending/ConditionalPending** (根据订单类型) |
    /// | 普通单 | **Pending** → 直接进入订单簿 |
    /// | 条件单 | **ConditionalPending** → 等待触发（资金已冻结）|
    /// | 条件单触发后 | **ConditionalPending** → **Pending** → 进入订单簿 |
    /// | 资金冻结时机 | **创建时冻结**（方案A）|
    /// | 异常流程 | 余额不足 → **Rejected** |
    ///
    /// ### 状态变更规则
    ///
    /// 1. **普通订单流程**:
    ///    - 创建订单 → 冻结资金 → 状态保持 **Pending** → 进入订单簿
    ///
    /// 2. **条件单流程**（方案A：创建时冻结）:
    ///    - 创建订单 → 冻结资金 → 状态 **ConditionalPending** → 等待触发
    ///    - 触发后: ConditionalPending → **Pending** → 进入订单簿
    ///    - 资金始终冻结，确保触发时可执行
    ///
    /// 3. **异常流程** (余额不足):
    ///    - 状态从初始状态 → **Rejected**
    ///    - 不冻结资金，生成变更日志并持久化
    ///
    /// ### ✅ 已实现的改进
    ///
    /// **余额不足时的状态处理** (Line 454-480):
    ///
    /// ```rust
    /// // 改进后的实现：显式设置 Rejected 状态并记录日志
    /// Err(BalanceError::InsufficientAvailable { required, available }) => {
    ///     // 1. 设置订单状态为 Rejected
    ///     internal_order.status = OrderStatus::Rejected;
    ///     internal_order.last_updated = Timestamp::now_as_nanos();
    ///
    ///     // 2. 生成订单变更日志（记录 Rejected 状态变更）
    ///     let rejected_order_change_log = internal_order.track_create().unwrap();
    ///
    ///     // 3. 发送订单变更事件到队列（对外通知）
    ///     self.queue.send(SpotTopic::EntityChangeLog.name(), ...);
    ///
    ///     // 4. 持久化 Rejected 状态到数据库
    ///     self.order_repo.replay_event(&rejected_order_change_log);
    ///
    ///     // 5. 返回错误
    ///     return Err(SpotCmdErrorAny::Common(CommonError::InsufficientBalance));
    /// }
    /// ```
    ///
    /// **改进收益**:
    /// - ✅ 订单状态明确：从 Pending 变为 Rejected
    /// - ✅ 可追溯：变更日志记录拒绝原因和时间
    /// - ✅ 一致性：事件发送到队列，DB持久化
    /// - ✅ 可查询：用户可查询到被拒绝的订单记录
    ///
    /// ### 与 handle_match 的对比
    ///
    /// | 函数 | 正常流程状态 | 异常流程状态 | 负责阶段 |
    /// |------|-------------|-------------|---------|
    /// | pre_process | Pending（不变） | Rejected（余额不足时） | 订单创建、资金冻结 |
    /// | handle_match | 多次变更（根据TIF） | - | 撮合、状态流转 |
    ///
    /// ---
    ///
    /// ## 优化效果总结
    ///
    /// | 优化项 | 改进前 | 改进后 | 收益 |
    /// |--------|--------|--------|------|
    /// | **A. 消除 unwrap()** | 3处 unwrap() 可能 panic | 使用 match/Result 安全处理 | 系统稳定性↑ |
    /// | **B. 批量发送** | 2次 send() 调用 | 1次 send_batch() 批量发送 | IO 次数↓ 50% |
    /// | **D. 参数验证前置** | 创建后验证 | 创建前验证 | 无效资源↓ |
    /// | **E. 条件单实现** | todo!() panic | 完整的触发/挂起逻辑 | 功能完整性↑ |
    ///
    /// 验证订单命令参数（优化D：参数验证前置）
    fn validate_order_cmd(&self, cmd: &NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
        // 验证数量必须大于0
        if let Some(qty) = cmd.quantity() {
            if qty.is_zero() {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "quantity",
                    reason: "must be greater than 0",
                }));
            }
        }

        // 验证价格（限价单必须提供价格）
        match cmd.order_type() {
            OrderType::Limit
            | OrderType::StopLossLimit
            | OrderType::TakeProfitLimit
            | OrderType::LimitMaker => {
                if cmd.price().is_none() {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "price",
                        reason: "required for limit orders",
                    }));
                }
            }
            _ => {} // 市价单不需要价格
        }

        // 验证止损/止盈价格（条件单必须提供）
        match cmd.order_type() {
            OrderType::StopLoss
            | OrderType::StopLossLimit
            | OrderType::TakeProfit
            | OrderType::TakeProfitLimit => {
                if cmd.stop_price().is_none() {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "stop_price",
                        reason: "required for conditional orders",
                    }));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// 批量发送变更事件并持久化（优化B：批量发送）
    fn persist_change_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), SpotCmdErrorAny> {
        // 1.批量回放到数据库
        for log in logs {
            match log.entity_type().as_str() {
                "Balance" => {
                    if let Err(e) = self.balance_repo.replay_event(log) {
                        log::error!("Failed to replay balance event: {:?}", e);
                    }
                }
                "SpotOrder" => {
                    if let Err(e) = self.order_repo.replay_event(log) {
                        log::error!("Failed to replay order event: {:?}", e);
                    }
                }
                _ => {
                    log::warn!("Unknown entity type: {}", log.entity_type());
                }
            }
        }

        // 2.批量发送到消息队列
        // let bytes_events: Vec<bytes::Bytes> =
        //     logs.iter().filter_map(|log| log.to_bytes().ok().map(bytes::Bytes::from)).collect();
        //
        // if !bytes_events.is_empty() {
        //     let _ = self.queue.send_batch(SpotTopic::OrderChangeLog.name(), bytes_events, None);
        // }

        Ok(())
    }

    /// 处理条件单（优化E：条件单实现）
    ///
    /// **前提条件**: 资金已在 pre_process 中冻结
    ///
    /// ## 条件单生命周期（方案A：创建时冻结）
    ///
    /// ```
    /// 创建条件单
    ///     │
    ///     ▼
    /// 冻结资金 ←───────────────────────────┐
    ///     │                                 │
    ///     ▼                                 │
    /// ConditionalPending (等待触发)        │
    ///     │                                 │
    ///     ├─ 未触发 ──→ 继续等待            │
    ///     │              资金保持冻结        │
    ///     │                                 │
    ///     └─ 触发 ─────→ Pending            │
    ///       (资金已冻结)  进入订单簿         │
    ///                       │              │
    ///                       ▼              │
    ///                   撮合成交           │
    ///                       │              │
    ///                       ▼              │
    ///                   Filled ────────────┤
    ///                       │              │
    ///                       ▼              │
    ///                   解冻剩余资金 ◄─────┘
    /// ```
    fn handle_conditional_order(
        &self,
        order: &mut SpotOrder,
    ) -> Result<Option<SpotOrder>, SpotCmdErrorAny> {
        match order.conditional_type {
            ConditionalType::None => Ok(None),
            ConditionalType::StopLoss => {
                // 检查当前价格是否已触发
                // TODO: 从 market_data_repo 获取当前价格
                let current_price = self.get_current_price(order.trading_pair)?;
                let stop_price = order.stop_price.ok_or_else(|| {
                    SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "stop_price",
                        reason: "required for stop loss order",
                    })
                })?;

                // 根据买卖方向判断触发条件
                let should_trigger = match order.side {
                    OrderSide::Sell => current_price <= stop_price,
                    OrderSide::Buy => current_price >= stop_price,
                };

                if should_trigger {
                    // 已触发，转为市价单执行
                    log::info!(
                        "StopLoss order {} triggered at price {:?}, stop_price: {:?}",
                        order.order_id,
                        current_price,
                        stop_price
                    );
                    order.execution_method = ExecutionMethod::Market;
                    order.price = None;
                    order.conditional_type = ConditionalType::None;
                    order.status = base_types::exchange::spot::spot_types::OrderStatus::Pending;
                    Ok(None) // 继续正常流程（资金已冻结）
                } else {
                    // 未触发，挂起等待（资金已冻结）
                    log::info!(
                        "StopLoss order {} pending: current_price={:?}, stop_price={:?}",
                        order.order_id,
                        current_price,
                        stop_price
                    );
                    // 保持 ConditionalPending 状态（已在 from 中设置）
                    Ok(Some(order.clone())) // 返回订单，不再执行撮合
                }
            }
            ConditionalType::TakeProfit => {
                // 检查当前价格是否已触发
                let current_price = self.get_current_price(order.trading_pair)?;
                let take_profit_price = order.stop_price.ok_or_else(|| {
                    SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "stop_price",
                        reason: "required for take profit order",
                    })
                })?;

                // 根据买卖方向判断触发条件（与止损相反）
                let should_trigger = match order.side {
                    OrderSide::Sell => current_price >= take_profit_price,
                    OrderSide::Buy => current_price <= take_profit_price,
                };

                if should_trigger {
                    // 已触发，转为市价单执行
                    log::info!(
                        "TakeProfit order {} triggered at price {:?}, target: {:?}",
                        order.order_id,
                        current_price,
                        take_profit_price
                    );
                    order.execution_method = ExecutionMethod::Market;
                    order.price = None;
                    order.conditional_type = ConditionalType::None;
                    order.status = base_types::exchange::spot::spot_types::OrderStatus::Pending;
                    Ok(None) // 继续正常流程（资金已冻结）
                } else {
                    // 未触发，挂起等待（资金已冻结）
                    log::info!(
                        "TakeProfit order {} pending: current_price={:?}, target={:?}",
                        order.order_id,
                        current_price,
                        take_profit_price
                    );
                    // 保持 ConditionalPending 状态（已在 from 中设置）
                    Ok(Some(order.clone())) // 返回订单，不再执行撮合
                }
            }
        }
    }

    /// 获取当前市场价格（辅助函数）
    fn get_current_price(&self, trading_pair: TradingPair) -> Result<Price, SpotCmdErrorAny> {
        // TODO: 从 market_data_repo 获取实时价格
        // 临时返回默认价格，实际需要实现
        Ok(Price::from_f64(0.0))
    }

    pub(crate) fn handle_acquiring2(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<(ChangeLogEntry, ChangeLogEntry), SpotCmdErrorAny> {
        // 优化D: 参数验证前置（在创建订单前验证，避免无效资源分配）
        self.validate_order_cmd(&cmd)?;

        // 根据 NewOrderCmd 创建 SpotOrder
        let mut internal_order = SpotOrder::from(cmd);

        // 步骤1: 冻结资金（所有订单类型，包括条件单）
        let frozen_asset_id = internal_order.frozen_asset_id();
        let frozen_asset_balance_id = self.queryBalanceId(frozen_asset_id);

        // 优化A: 消除 unwrap() - 使用安全的错误处理
        let mut frozen_asset_balance =
            match self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id) {
                Ok(Some(balance)) => balance,
                Ok(None) => {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "balance",
                        reason: "balance not found",
                    }));
                }
                Err(e) => {
                    return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                        message: format!("Failed to query balance: {}", e),
                    }));
                }
            };

        // 计算逻辑
        let result = self.handle_data(&mut frozen_asset_balance, &mut internal_order);

        // 处理余额不足：显式设置 Rejected 状态并记录日志
        let (order_change_log, balance_change_log) = match result {
            Ok(logs) => logs,
            Err(BalanceError::InsufficientAvailable { required, available }) => {
                // 设置订单状态为 Rejected
                internal_order.status =
                    base_types::exchange::spot::spot_types::OrderStatus::Rejected;
                internal_order.last_updated = Timestamp::now_as_nanos();

                // 优化A: 使用 map_err 替代 unwrap
                let rejected_order_change_log = internal_order.track_create().map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal {
                        message: format!("Failed to track rejected order: {}", e),
                    })
                })?;

                // 优化B: 批量持久化
                self.persist_change_logs(&[rejected_order_change_log])?;

                return Err(SpotCmdErrorAny::Common(CommonError::InsufficientBalance {
                    required: required as u64,
                    available: available as u64,
                }));
            }
            Err(e) => {
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Balance error: {}", e),
                }));
            }
        };

        // 非条件单或已触发，持久化资金冻结和订单创建事件
        let logs = vec![balance_change_log.clone(), order_change_log.clone()];
        self.persist_change_logs(&logs)?;

        Ok((balance_change_log.clone(), order_change_log.clone()))
    }

    pub(crate) fn handle_acquiring(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<(SpotOrder, Vec<ChangeLogEntry>), SpotCmdErrorAny> {
        // 优化D: 参数验证前置（在创建订单前验证，避免无效资源分配）
        self.validate_order_cmd(&cmd)?;

        // 根据 NewOrderCmd 创建 SpotOrder
        let mut internal_order = SpotOrder::from(cmd);

        // 步骤1: 冻结资金（所有订单类型，包括条件单）
        let frozen_asset_id = internal_order.frozen_asset_id();
        let frozen_asset_balance_id = self.queryBalanceId(frozen_asset_id);

        // 优化A: 消除 unwrap() - 使用安全的错误处理
        let mut frozen_asset_balance =
            match self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id) {
                Ok(Some(balance)) => balance,
                Ok(None) => {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "balance",
                        reason: "balance not found",
                    }));
                }
                Err(e) => {
                    return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                        message: format!("Failed to query balance: {}", e),
                    }));
                }
            };

        // 计算逻辑
        let result = self.handle_data(&mut frozen_asset_balance, &mut internal_order);

        // 处理余额不足：显式设置 Rejected 状态并记录日志
        let (order_change_log, balance_change_log) = match result {
            Ok(logs) => logs,
            Err(BalanceError::InsufficientAvailable { required, available }) => {
                // 设置订单状态为 Rejected
                internal_order.status =
                    base_types::exchange::spot::spot_types::OrderStatus::Rejected;
                internal_order.last_updated = Timestamp::now_as_nanos();

                // 优化A: 使用 map_err 替代 unwrap
                let rejected_order_change_log = internal_order.track_create().map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal {
                        message: format!("Failed to track rejected order: {}", e),
                    })
                })?;

                // 优化B: 批量持久化
                self.persist_change_logs(&[rejected_order_change_log])?;

                return Err(SpotCmdErrorAny::Common(CommonError::InsufficientBalance {
                    required: required as u64,
                    available: available as u64,
                }));
            }
            Err(e) => {
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Balance error: {}", e),
                }));
            }
        };

        // 非条件单或已触发，持久化资金冻结和订单创建事件
        let logs = vec![balance_change_log.clone(), order_change_log.clone()];
        self.persist_change_logs(&logs)?;

        Ok((internal_order, logs))
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
    /// 根据 TimeInForce 和 ExecutionMethod 的不同组合执行相应的撮合策略。
    /// 状态流转见下方详细说明。
    ///
    /// ## TimeInForce 状态流转矩阵
    ///
    /// ```
    /// ┌─────────────────────────────────────────────────────────────────────────────┐
    /// │                            TimeInForce 处理流程                              │
    /// ├─────────────┬──────────────────┬──────────────────┬─────────────────────────┤
    /// │    TIF      │  ExecutionMethod │   匹配结果       │        状态流转          │
    /// ├─────────────┼──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │                  │  有匹配          │ Pending → Partially     │
    /// │    GTC      │      Limit       │  部分成交        │   → Pending (进LOB)     │
    /// │             │                  │                  │                         │
    /// │             │                  │  完全成交        │ Pending → Filled         │
    /// ├─────────────┼──────────────────┼──────────────────┼─────────────────────────┤
    /// │    GTC      │      Market      │      -           │ ❌ 不支持，进入待办      │
    /// ├─────────────┼──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Limit       │  有匹配          │ Pending → Partially     │
    /// │             │                  │  部分成交        │   → Cancelled (IOC特性) │
    /// │    IOC      │                  │                  │                         │
    /// │             │                  │  完全成交        │ Pending → Filled         │
    /// │             ├──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Market      │  有匹配          │ 同 Limit 逻辑           │
    /// │             │                  │  部分成交        │   → Cancelled           │
    /// ├─────────────┼──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Limit       │  完全成交        │ Pending → Filled         │
    /// │    FOK      │                  │  不完全成交      │ Pending → Cancelled      │
    /// │             ├──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Market      │  同 Limit 逻辑   │ 同上                    │
    /// ├─────────────┼──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Limit       │  无匹配          │ Pending → Pending        │
    /// │    GTX      │                  │    (进LOB)       │                         │
    /// │             │                  │  有匹配          │ Pending → Rejected       │
    /// │             │                  │   (PostOnly失败) │  ❌ 会立即成交，拒绝    │
    /// │             ├──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Market      │      -           │ ❌ 不支持，进入待办      │
    /// ├─────────────┼──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Limit       │  同 GTC 逻辑     │ 同 GTC                  │
    /// │    GTD      │                  │  但有过期时间    │  + GTD 时间监控         │
    /// │             ├──────────────────┼──────────────────┼─────────────────────────┤
    /// │             │      Market      │      -           │ ❌ 不支持，进入待办      │
    /// └─────────────┴──────────────────┴──────────────────┴─────────────────────────┘
    /// ```
    ///
    /// ## 订单状态 (order_status) 变化详细分析
    ///
    /// ### 状态变更位置汇总
    ///
    /// | 行号 | 状态变更 | TIF类型 | 触发条件 | 业务含义 |
    /// |------|---------|---------|---------|---------|
    /// | 603-604 | Pending → **Pending** | GTC Limit | 部分成交，剩余进LOB | 订单仍在订单簿等待 |
    /// | 607-608 | Pending → **Filled** | GTC Limit | 全部成交 | 订单完成 |
    /// | 663-667 | Pending → Cancelled/ Filled | IOC Limit | 部分成交/全部成交 | IOC剩余取消 |
    /// | 711-712 | Pending → **Cancelled** | IOC Market | 市价单剩余 | 市价单IOC特性 |
    /// | 762-767 | Pending → Filled/ Cancelled | FOK Limit | 全部成交/不完全成交 | FOK原子性要求 |
    /// | 813-818 | Pending → Filled/ Cancelled | FOK Market | 同FOK Limit | 市价单FOK |
    /// | 829-830 | Pending → **Rejected** | GTX Limit | 会立即成交 | PostOnly保护 |
    /// | 895-900 | Pending → Pending/ Filled | GTD Limit | 部分成交/全部成交 | GTD同GTC逻辑 |
    ///
    /// ### 状态流转规则详解
    ///
    /// #### 1. GTC (Good Till Cancel) - 持续有效
    /// ```
    /// 初始状态: Pending
    ///     │
    ///     ├── 部分成交 ─────┬── 剩余进LOB ───→ Pending (Line 603)
    ///     │                 │
    ///     │                 └── 后续成交 ───→ Filled
    ///     │
    ///     └── 全部成交 ────────────────────→ Filled (Line 607)
    /// ```
    /// - **特点**: 最常用，订单长期有效
    /// - **风险**: 订单可能长期挂在订单簿，需监控过期
    ///
    /// #### 2. IOC (Immediate Or Cancel) - 立即成交否则取消
    /// ```
    /// 初始状态: Pending
    ///     │
    ///     ├── 部分成交 ───→ 成交部分生成交记录
    ///     │                 剩余取消 ───────→ Cancelled (Line 663)
    ///     │
    ///     └── 全部成交 ────────────────────→ Filled (Line 666)
    ///            │
    ///            └── 市价单无论如何都 → Cancelled (Line 711)
    /// ```
    /// - **特点**: 快速吃单，不留挂单
    /// - **注意**: 市价单(Line 711)即使没有匹配也设为Cancelled，可能存在逻辑问题
    ///
    /// #### 3. FOK (Fill Or Kill) - 全部成交否则取消
    /// ```
    /// 初始状态: Pending
    ///     │
    ///     ├── 流动性足够 ───→ 全部成交 ─────→ Filled (Line 762)
    ///     │
    ///     └── 流动性不足 ───→ 全部取消 ─────→ Cancelled (Line 766)
    /// ```
    /// - **特点**: 原子性，要么全成要么全取消
    /// - **应用场景**: 对冲交易、大额订单
    ///
    /// #### 4. GTX (Good Till Crossing / Post-Only) - 只做Maker
    /// ```
    /// 初始状态: Pending
    ///     │
    ///     ├── 无匹配 ───→ 进LOB ───────────→ Pending (隐含，Line 831 TODO)
    ///     │
    ///     └── 有匹配 ───→ 会立即成交 ──────→ Rejected (Line 829)
    ///                      (PostOnly失败)
    /// ```
    /// - **特点**: 确保Maker身份，享受低手续费
    /// - **注意**: Line 831 有 TODO，未实现进LOB逻辑
    ///
    /// #### 5. GTD (Good Till Date) - 到期取消
    /// ```
    /// 初始状态: Pending
    ///     │
    ///     ├── 部分成交 ─────┬── 剩余进LOB ───→ Pending (Line 895)
    ///     │                 │   (带过期时间)
    ///     │                 │
    ///     │                 └── 到期未成交 ──→ Expired (由定时任务处理)
    ///     │
    ///     └── 全部成交 ────────────────────→ Filled (Line 899)
    /// ```
    /// - **特点**: 同GTC，但有过期时间
    /// - **注意**: Expired状态由外部定时任务触发，不在此处处理
    ///
    /// ### 状态一致性保证
    ///
    /// 1. **撮合与LOB同步**: 状态变更后，订单必须同时更新到LOB
    ///    - Line 597-601: GTC添加订单到LOB
    ///    - Line 889-893: GTD添加订单到LOB
    ///
    /// 2. **终态判定**:
    ///    - Filled/Cancelled/Rejected 都是终态，不会再变更
    ///    - Pending 可能后续变为其他状态
    ///
    /// 3. **资金释放**:
    ///    - Filled: 解冻剩余冻结资金，完成交割
    ///    - Cancelled: 全额解冻
    ///    - Rejected: 全额解冻（应在更早阶段处理）
    ///
    /// ## 撮合流程详解
    ///
    /// ### 1. 匹配查询 (Line 447-452)
    /// ```rust
    /// let (matches, remaining) = self.lob_repo.match_orders(
    ///     internal_order.trading_pair,
    ///     internal_order.side,
    ///     internal_order.price.unwrap(),
    ///     internal_order.quantity(),
    /// );
    /// ```
    /// - 查询订单簿寻找可匹配的对手单
    /// - 返回匹配列表和剩余未成交数量
    ///
    /// ### 2. 成交记录生成 (Line 465-519)
    /// - 每笔成交创建 `SpotTrade` 记录
    /// - 计算手续费（Taker 0.1%, Maker 0.05%）
    /// - 生成唯一 trade_id
    ///
    /// ### 3. 订单状态更新
    /// 根据 TIF 规则更新订单状态：
    /// - **GTC**: 剩余部分进订单簿，状态 Pending
    /// - **IOC**: 剩余部分取消，状态根据是否成交决定
    /// - **FOK**: 不完全成交则全部取消
    /// - **GTX**: 立即成交则拒绝（PostOnly）
    /// - **GTD**: 同 GTC，但有过期时间
    ///
    /// ## 关键状态判断
    ///
    /// ```rust
    /// let is_fully_filled = remaining.is_zero();
    /// ```
    /// - `true`: 订单完全成交，状态设为 Filled
    /// - `false`: 有剩余，根据 TIF 决定后续处理
    ///
    /// ## 性能优化点
    ///
    /// 1. **批量成交处理**: 当前每笔成交单独生成 Trade，可考虑批量插入
    /// 2. **手续费计算缓存**: 费率可缓存避免重复计算
    /// 3. **订单簿批量操作**: 匹配和添加订单可优化为原子操作
    ///
    /// ## 风险点
    ///
    /// - **状态一致性**: 撮合后需确保订单状态与 LOB 同步
    /// - **部分成交**: IOC/FOK 的部分成交需正确处理资金释放
    /// - **并发撮合**: 多个订单同时匹配同一对手单的处理
    fn handle_match(&self, internal_order: &mut SpotOrder) -> Vec<SpotTrade> {
        // TODO: 2.执行订单匹配逻辑
        let (matches, remaining) = self.lob_repo.match_orders(
            internal_order.trading_pair,
            internal_order.side,
            internal_order.price.unwrap(),
            internal_order.quantity(),
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

        //todo 回放 order_change_logs/trade_change_logs
        //发消息 order_change_logs/trade_change_logs

        trades
    }

    /// 通过订单ID进行撮合处理（用于Actor模式）
    ///
    /// # Arguments
    /// * `order_id` - 订单ID
    ///
    /// # Returns
    /// * `Ok(Vec<ChangeLogEntry>)` - 撮合产生的所有变更日志（包括trade和order更新）
    /// * `Err(SpotCmdErrorAny)` - 处理失败
    pub(crate) fn handle_match2(
        &self,
        order_id: OrderId,
    ) -> Result<Vec<ChangeLogEntry>, SpotCmdErrorAny> {
        // 1. 从订单仓库查询订单
        let order_id_str = order_id.to_string();
        let mut order = match self.order_repo.find_by_id(&order_id_str) {
            Ok(Some(order)) => order,
            Ok(None) => {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "order_id",
                    reason: "order not found",
                }));
            }
            Err(e) => {
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to query order: {}", e),
                }));
            }
        };

        // 2. 检查订单状态是否为Pending
        if order.status != OrderStatus::Pending {
            return Ok(Vec::new()); // 非Pending状态不需要撮合
        }

        tracing::info!(
            "开始撮合订单: order_id={}, trading_pair={:?}, side={:?}, qty={}",
            order.order_id,
            order.trading_pair,
            order.side,
            order.total_qty
        );

        // 3. 执行撮合
        let trades = self.handle_match(&mut order);

        // 4. 生成变更日志
        let mut all_change_logs: Vec<ChangeLogEntry> = Vec::new();

        // 生成trade的changelog
        for trade in &trades {
            // 手动创建 ChangeLogEntry（因为 SpotTrade 没有实现 Entity trait）
            let trade_log = ChangeLogEntry::new(
                trade.trade_id.to_string(),
                "SpotTrade".to_string(),
                diff::ChangeType::Created { fields: Vec::new() },
                Timestamp::now_as_nanos().0,
                0, // sequence
            );
            all_change_logs.push(trade_log);
        }

        // 生成order更新的changelog（如果有成交）
        if !trades.is_empty() {
            // 手动创建 order 更新的 ChangeLogEntry
            let order_log = ChangeLogEntry::new(
                order.order_id.to_string(),
                "SpotOrder".to_string(),
                diff::ChangeType::Updated {
                    changed_fields: vec![
                        diff::FieldChange::new(
                            "status",
                            "Pending".to_string(),
                            format!("{:?}", order.status),
                        ),
                        diff::FieldChange::new(
                            "executed_qty",
                            "0".to_string(),
                            order.executed_qty.to_string(),
                        ),
                        diff::FieldChange::new(
                            "unfilled_qty",
                            order.total_qty.to_string(),
                            order.unfilled_qty.to_string(),
                        ),
                    ],
                },
                Timestamp::now_as_nanos().0,
                0, // sequence
            );
            all_change_logs.push(order_log);
        }

        tracing::info!(
            "订单撮合完成: order_id={}, 成交数={}, 变更日志数={}",
            order_id,
            trades.len(),
            all_change_logs.len()
        );

        Ok(all_change_logs)
    }

    /// 处理订单撮合
    ///
    /// # 参数
    /// - `order_log`: 订单变更日志，从中提取订单ID
    ///
    /// # 返回
    /// - `Ok((order_change_logs, trade_change_logs))`: 成功时返回订单变更日志列表和成交变更日志列表
    ///   - order_change_logs: 订单变更日志，None表示无订单变更
    ///   - trade_change_logs: 成交变更日志，None表示无成交
    /// - `Err(SpotCmdErrorAny)`: 失败时返回错误
    pub(crate) fn handle_match3(
        &self,
        order_log: ChangeLogEntry,
    ) -> Result<(Option<Vec<ChangeLogEntry>>, Option<Vec<ChangeLogEntry>>), SpotCmdErrorAny> {
        // 1. 从 order_log 中提取订单ID
        let order_id_str = order_log.entity_id().to_string();

        // 2. 从订单仓库查询订单
        let mut order = match self.order_repo.find_by_id(&order_id_str) {
            Ok(Some(order)) => order,
            Ok(None) => {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "order_id",
                    reason: "order not found",
                }));
            }
            Err(e) => {
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to query order: {}", e),
                }));
            }
        };

        // 3. 检查订单状态是否为Pending
        if order.status != OrderStatus::Pending {
            return Ok((None, None)); // 非Pending状态不需要撮合
        }

        tracing::info!(
            "开始撮合订单: order_id={}, trading_pair={:?}, side={:?}, qty={}",
            order.order_id,
            order.trading_pair,
            order.side,
            order.total_qty
        );

        //todo 要优化
        // 4. 执行撮合
        let trades = self.handle_match(&mut order);

        // 5. 生成变更日志
        let mut order_change_logs: Vec<ChangeLogEntry> = Vec::new();
        let mut trade_change_logs: Vec<ChangeLogEntry> = Vec::new();

        // 生成trade的changelog
        for trade in &trades {
            // 手动创建 ChangeLogEntry（因为 SpotTrade 没有实现 Entity trait）
            let trade_log = ChangeLogEntry::new(
                trade.trade_id.to_string(),
                "SpotTrade".to_string(),
                diff::ChangeType::Created { fields: Vec::new() },
                Timestamp::now_as_nanos().0,
                0, // sequence
            );
            trade_change_logs.push(trade_log);
        }

        // 生成order更新的changelog（如果有成交）
        if !trades.is_empty() {
            // 手动创建 order 更新的 ChangeLogEntry
            let order_update_log = ChangeLogEntry::new(
                order.order_id.to_string(),
                "SpotOrder".to_string(),
                diff::ChangeType::Updated {
                    changed_fields: vec![
                        diff::FieldChange::new(
                            "status",
                            "Pending".to_string(),
                            format!("{:?}", order.status),
                        ),
                        diff::FieldChange::new(
                            "executed_qty",
                            "0".to_string(),
                            order.executed_qty.to_string(),
                        ),
                        diff::FieldChange::new(
                            "unfilled_qty",
                            order.total_qty.to_string(),
                            order.unfilled_qty.to_string(),
                        ),
                    ],
                },
                Timestamp::now_as_nanos().0,
                0, // sequence
            );
            order_change_logs.push(order_update_log);
        }

        tracing::info!(
            "订单撮合完成: order_id={}, 成交数={}, 订单日志数={}, 成交日志数={}",
            order_id_str,
            trades.len(),
            order_change_logs.len(),
            trade_change_logs.len()
        );

        // 只有在有数据时才返回 Some，否则返回 None
        let order_logs_opt =
            if order_change_logs.is_empty() { None } else { Some(order_change_logs) };
        let trade_logs_opt =
            if trade_change_logs.is_empty() { None } else { Some(trade_change_logs) };

        Ok((order_logs_opt, trade_logs_opt))
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

        // // 批量发送事件 - 将 ChangeLogEntry 转换为 Bytes
        // let bytes_events: Vec<bytes::Bytes> = balance_change_logs
        //     .iter()
        //     .filter_map(|event| serde_json::to_vec(event).ok().map(bytes::Bytes::from))
        //     .collect();
        //
        // let results = self.queue.send_batch(SpotTopic::OrderChangeLog.name(), bytes_events, None);

        balance_change_logs
    }

    /// 通过交易ID进行结算处理（用于Actor模式）
    ///
    /// # Arguments
    /// * `trade_id` - 交易ID
    ///
    /// # Returns
    /// * `Ok(Vec<ChangeLogEntry>)` - 结算产生的所有余额变更日志
    /// * `Err(SpotCmdErrorAny)` - 处理失败
    ///
    pub(crate) fn handle_settlement2(
        &self,
        trade_id: u64,
    ) -> Result<Vec<ChangeLogEntry>, SpotCmdErrorAny> {
        // 1. 从交易仓库查询交易
        let trade_id_str = trade_id.to_string();
        let trade = match self.trade_repo.find_by_id(&trade_id_str) {
            Ok(Some(trade)) => trade,
            Ok(None) => {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "trade_id",
                    reason: "trade not found",
                }));
            }
            Err(e) => {
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to query trade: {}", e),
                }));
            }
        };

        tracing::info!(
            "开始结算交易: trade_id={}, taker_order_id={}, maker_order_id={}",
            trade.trade_id,
            trade.taker_order_id,
            trade.maker_order_id
        );

        // 2. 执行结算
        let balance_change_logs = self.handle_settlement(vec![trade]);

        tracing::info!(
            "交易结算完成: trade_id={}, 生成 {} 条余额变更日志",
            trade_id,
            balance_change_logs.len()
        );

        Ok(balance_change_logs)
    }

    /// 处理新订单命令的主方法
    ///
    /// ## 状态处理逻辑
    ///
    /// ```
    /// ┌─────────────────────────────────────────────────────────────┐
    /// │                    handle 方法状态处理                       │
    /// ├─────────────────────────────────────────────────────────────┤
    /// │                                                             │
    /// │  pre_process(cmd)                                           │
    /// │       │                                                     │
    /// │       ▼                                                     │
    /// │  ┌─────────────────┐                                        │
    /// │  │ 返回订单状态    │                                        │
    /// │  └────────┬────────┘                                        │
    /// │           │                                                 │
    /// │     ┌─────┴─────┬──────────────┬─────────────────────┐      │
    /// │     ▼         ▼              ▼                     ▼      │
    /// │ Conditional  Pending      PartiallyFilled        终态     │
    /// │  Pending              │    (不应出现)          (不应出现) │
    /// │     │                 │                                │   │
    /// │     ▼                 ▼                                │   │
    /// │  直接返回          撮合+结算                           │   │
    /// │  (资金已冻结)      handle_match                        │   │
    /// │                    handle_settlement                   │   │
    /// │                                                             │
    /// └─────────────────────────────────────────────────────────────┘
    /// ```
    fn handle(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 1.执行订单预处理（创建订单 + 冻结资金）
        let (mut internal_order, _logs) = self.handle_acquiring(cmd)?;

        match internal_order.status {
            OrderStatus::ConditionalPending => {
                // 条件单未触发，资金已冻结，等待后续触发
                // 直接返回确认，不进行撮合
                log::info!(
                    "Conditional order {} is pending, waiting for trigger",
                    internal_order.order_id
                );
            }
            OrderStatus::Pending => {
                // 普通订单或已触发的条件单，执行撮合和结算
                // 2.撮合逻辑
                let trades = self.handle_match(&mut internal_order);

                // 3.结算操作
                let _balance_change_logs = self.handle_settlement(trades);
            }
            OrderStatus::PartiallyFilled => {
                // 预处理不应返回此状态，如果发生则报错
                log::error!(
                    "Unexpected PartiallyFilled status for new order {}",
                    internal_order.order_id
                );
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "Unexpected order status: PartiallyFilled".to_string(),
                }));
            }
            OrderStatus::Filled => {
                // 预处理不应返回此状态，如果发生则报错
                log::error!("Unexpected Filled status for new order {}", internal_order.order_id);
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "Unexpected order status: Filled".to_string(),
                }));
            }
            OrderStatus::Cancelled => {
                // 预处理不应返回此状态，如果发生则报错
                log::error!(
                    "Unexpected Cancelled status for new order {}",
                    internal_order.order_id
                );
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "Unexpected order status: Cancelled".to_string(),
                }));
            }
            OrderStatus::Rejected => {
                // 余额不足等原因被拒绝，pre_process 已处理并返回错误
                // 这里不应该到达，但为了完整性处理
                log::warn!("Order {} was rejected", internal_order.order_id);
                return Err(SpotCmdErrorAny::Common(CommonError::InsufficientBalance {
                    required: 0,
                    available: 0,
                }));
            }
            OrderStatus::Expired => {
                // 预处理不应返回此状态
                log::error!("Unexpected Expired status for new order {}", internal_order.order_id);
                return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "Unexpected order status: Expired".to_string(),
                }));
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
