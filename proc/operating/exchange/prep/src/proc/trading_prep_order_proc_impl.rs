use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock}
};

// Clean Architecture: 引入 MySqlDbRepo 和相关接口
use account::domain::entity::{AccountId, AssetId, Balance, Timestamp};
// Base types
use base_types::{Side as BaseSide, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo, QueryRepo};
// Event Sourcing: Entity trait for track_update
use diff::{ChangeLogEntry, Entity};
use lob_repo::adapter::standalone_lob_repo::StandaloneLobRepo;
// LOB 仓储接口
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::{
    prep_types::PrepOrder,
    trading_prep_order_proc::{
        AccountBalance, AccountInfo, CancelAllOrdersCommand, CancelAllOrdersResult, CancelOrderCommand,
        CancelOrderResult, ClosePositionCommand, ClosePositionResult, FundingFeeRecord, FundingRateRecord,
        MarkPriceInfo, ModifyOrderCommand, ModifyOrderResult, OpenPositionCommand, OpenPositionResult,
        OrderBookSnapshot, OrderId, OrderQueryResult, OrderStatus, OrderType, PerpOrderExchProc,
        PerpOrderExchQueryProc, PrepCommandError, PrepPosition, PrepTrade, Price, Quantity, QueryAccountBalanceCommand,
        QueryAccountInfoCommand, QueryFundingFeeCommand, QueryFundingRateHistoryCommand, QueryMarkPriceCommand,
        QueryOrderBookCommand, QueryOrderCommand, QueryPositionCommand, QueryTradesCommand, SetLeverageCommand,
        SetLeverageResult, SetMarginTypeCommand, SetMarginTypeResult, SetPositionModeCommand, SetPositionModeResult,
        Side, TradeId, TradesQueryResult
    }
};

/// 本地撮合引擎服务
///
/// 遵循Clean Architecture原则的撮合引擎实现
/// - 依赖注入：通过 MySqlDbRepo<Balance> 管理余额
/// - 依赖注入：通过 MySqlDbRepo<PositionInfo> 管理持仓
/// - 依赖注入：通过 MultiSymbolLobRepo 管理订单簿
/// - 支持价格-时间优先的订单匹配
/// - 维护持仓、余额、订单状态
pub struct PrepMatchingService {
    /// 余额仓储（依赖注入）
    balance_repo: MySqlDbRepo<Balance>,

    /// 持仓仓储（依赖注入）
    position_repo: MySqlDbRepo<PrepPosition>,

    trade_repo: MySqlDbRepo<PrepTrade>,

    order_repo: MySqlDbRepo<PrepOrder>,

    lob_repo: StandaloneLobRepo<PrepOrder>,

    /// 账户ID（固定账户）
    account_id: AccountId,

    /// 资产ID（USDT）
    asset_id: AssetId,

    /// 撮合序列号（用于追踪撮合顺序）
    match_seq: Arc<RwLock<u64>>
}


impl PrepMatchingService {
    /// 创建新的撮合服务实例
    ///
    /// # 参数
    /// - `balance_repo`: 余额仓储实现（依赖注入）
    /// - `position_repo`: 持仓仓储实现（依赖注入）
    /// - `trade_repo`: 成交仓储实现（依赖注入）
    /// - `order_repo`: 订单仓储实现（依赖注入）
    /// - `lob_repo`: LOB 仓储实现（依赖注入）
    /// - `account_id`: 账户ID
    /// - `asset_id`: 资产ID（如 USDT）
    pub fn new(
        balance_repo: MySqlDbRepo<Balance>, position_repo: MySqlDbRepo<PrepPosition>,
        trade_repo: MySqlDbRepo<PrepTrade>, order_repo: MySqlDbRepo<PrepOrder>, lob_repo: StandaloneLobRepo<PrepOrder>,
        account_id: AccountId, asset_id: AssetId
    ) -> Self {
        Self {
            balance_repo,
            position_repo,
            trade_repo,
            order_repo,
            lob_repo,
            account_id,
            asset_id,
            match_seq: Arc::new(RwLock::new(0))
        }
    }

    /// 获取当前余额
    ///
    /// # 返回
    /// 可用余额（u64 原始值）
    ///
    /// # 说明
    /// 从 MySqlDbRepo 获取余额，如果不存在返回 0
    fn get_balance(&self) -> u64 {
        let balance_id = format!("{}:{}", self.account_id.0, self.asset_id.0);
        self.balance_repo.find_by_id(&balance_id).ok().flatten().map(|b| b.available).unwrap_or(0)
    }

    /// 扣减余额（冻结保证金、支付手续费）
    ///
    /// # 参数
    /// - `amount`: 扣减金额（u64）
    ///
    /// # 返回
    /// - `Ok(())`: 扣减成功
    /// - `Err(InsufficientBalance)`: 余额不足
    fn deduct_balance(&self, amount: u64, now: Timestamp) -> Result<(), PrepCommandError> {
        // 获取变更事件
        let event = self.deduct_balance2(amount, now)?;

        // 回放事件到数据库
        self.balance_repo
            .replay_event(&event)
            .map_err(|e| PrepCommandError::MatchingEngineError(format!("Failed to replay balance event: {:?}", e)))?;

        Ok(())
    }

    fn deduct_balance2(&self, amount: u64, now: Timestamp) -> Result<ChangeLogEntry, PrepCommandError> {
        let balance_id = format!("{}:{}", self.account_id.0, self.asset_id.0);

        // 获取或创建余额
        let mut balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
            Some(b) => b,
            None => Balance::new(self.account_id, self.asset_id, now) // todo 应该报错
        };

        if balance.available < amount {
            return Err(PrepCommandError::InsufficientBalance);
        }

        // 在 track_update 闭包内修改 balance，生成正确的变更事件，然后回放到数据库
        let event = balance
            .track_update(|b: &mut Balance| {
                b.deduct_balance(amount, now);
            })
            .map_err(|e| PrepCommandError::MatchingEngineError(format!("Failed to track balance update: {:?}", e)))?;


        Ok(event)
    }

    /// 增加余额（归还保证金、盈利入账）
    ///
    /// # 参数
    /// - `amount`: 增加金额（u64）
    fn add_balance(&self, amount: u64, now: Timestamp) {
        let balance_id = format!("{}:{}", self.account_id.0, self.asset_id.0);

        // 获取或创建余额
        let mut balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
            Some(b) => b,
            None => Balance::new(self.account_id, self.asset_id, now)
        };

        // 在 track_update 闭包内修改 balance，生成正确的变更事件，然后回放到数据库
        match balance.track_update(|b| {
            b.add_balance(amount, now);
        }) {
            Ok(event) => {
                // 回放事件到数据库
                if let Err(e) = self.balance_repo.replay_event(&event) {
                    log::error!("Failed to replay balance update event: {:?}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to track balance update for {}: {:?}", balance_id, e);
            }
        }
    }

    // ========================================================================
    // PositionRepo 辅助方法
    // ========================================================================

    /// 获取持仓信息
    ///
    /// # 参数
    /// - `trading_pair`: 交易对
    ///
    /// # 返回
    /// 持仓信息，如果不存在返回空持仓
    fn get_position(&self, trading_pair: TradingPair) -> PrepPosition {
        // 从数据库按 trading_pair 查询
        match self.position_repo.find_one_by_condition(PrepPosition::empty(trading_pair, account::PositionSide::Both)) {
            Ok(Some(position)) => position,
            Ok(None) | Err(_) => {
                // 如果数据库中不存在，返回空持仓
                PrepPosition::empty(trading_pair, account::PositionSide::Both)
            }
        }
    }


    /// 修改持仓（如果存在）
    ///
    /// # 参数
    /// - `trading_pair`: 交易对
    /// - `modify_fn`: 修改函数
    fn modify_position<F>(&self, trading_pair: TradingPair, modify_fn: F)
    where
        F: FnOnce(&mut PrepPosition)
    {
        // 当前 mock 实现：创建空持仓，修改它，然后丢弃
        // 在实际实现中，应该从数据库加载、修改、保存
        let mut position = PrepPosition::empty(trading_pair, account::PositionSide::Both);
        modify_fn(&mut position);
        // 实际应调用 self.save_position(position)
    }

    /// 检查持仓是否存在
    fn has_position(&self, trading_pair: TradingPair) -> bool {
        // 从数据库查询是否存在该持仓
        match self.position_repo.find_one_by_condition(PrepPosition::empty(trading_pair, account::PositionSide::Both)) {
            Ok(Some(position)) => position.has_position(),
            Ok(None) | Err(_) => false
        }
    }


    /// 获取当前时间戳（纳秒）
    #[inline]
    fn now(&self) -> Timestamp {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
    }

    /// Price 转 u64（假设 Price 内部是 8 位小数）
    ///
    /// # 说明
    /// Price 内部存储为 raw 值，通常是整数表示（如 100_000_000 = 1.0）
    /// 这里假设 Price 的精度与 u64 精度一致
    #[inline]
    fn price_to_u64(price: Price) -> u64 { price.raw() as u64 }

    /// u64 转 Price
    #[inline]
    fn u64_to_price(value: u64) -> Price { Price::from_raw(value as i64) }

    /// 计算所需保证金
    ///
    /// # 参数
    /// - `price`: 订单价格（市价单使用估算价格）
    /// - `quantity`: 订单数量
    /// - `leverage`: 杠杆倍数
    ///
    /// # 返回
    /// 所需保证金金额
    ///
    /// # 计算公式
    /// ```text
    /// 保证金 = (价格 × 数量) / 杠杆倍数
    /// ```
    fn calculate_required_margin(&self, price: Price, quantity: Quantity, leverage: u8) -> Price {
        let notional = price.to_f64() * quantity.to_f64();
        let margin = notional / leverage as f64;
        Price::from_f64(margin)
    }

    /// 获取下一个撮合序列号
    fn next_match_seq(&self) -> u64 {
        let mut seq = self.match_seq.write().unwrap();
        *seq += 1;
        *seq
    }

    /// 生成订单ID
    ///
    /// # 返回
    /// 基于时间戳的唯一订单ID
    fn generate_order_id(&self) -> OrderId {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
    }


    /// 更新或创建持仓
    ///
    /// # 参数
    /// - `trading_pair`: 交易对
    /// - `_side`: 订单方向（仅用于向后兼容）
    /// - `position_side`: 持仓方向
    /// - `quantity`: 成交数量
    /// - `avg_price`: 成交均价
    /// - `leverage`: 杠杆倍数
    fn update_position(
        &self, trading_pair: TradingPair, _side: Side,
        position_side: crate::proc::trading_prep_order_proc::PositionSide, quantity: Quantity, avg_price: Price,
        leverage: u8
    ) {
        // 获取或创建持仓（通过 self.position_repo）
        let mut position = self.get_position(trading_pair);
        if !self.has_position(trading_pair) {
            position = PrepPosition::empty(trading_pair, position_side);
        }

        // 更新持仓数量和均价
        let old_qty = position.quantity.to_f64();
        let old_price = position.entry_price.to_f64();
        let new_qty_val = quantity.to_f64();
        let new_price = avg_price.to_f64();

        // 计算新的持仓均价（加权平均）
        let total_cost = old_qty * old_price + new_qty_val * new_price;
        let total_qty = old_qty + new_qty_val;

        // ========================================================================
        // Track position 生成log event, then replay_event
        // ========================================================================
        match position.track_update(|p: &mut PrepPosition| {
            p.quantity = Quantity::from_f64(total_qty);
            p.entry_price = if total_qty > 0.0 { Price::from_f64(total_cost / total_qty) } else { Price::from_raw(0) };
            p.leverage = leverage;
            p.mark_price = avg_price;
            p.updated_at =
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

            // 计算保证金 = (持仓价值) / 杠杆倍数
            let notional = p.entry_price.to_f64() * p.quantity.to_f64();
            p.margin = Price::from_f64(notional / leverage as f64);

            // 计算未实现盈亏
            p.unrealized_pnl = self.calculate_unrealized_pnl(p);

            // 计算强平价格
            p.liquidation_price = self.calculate_liquidation_price(p);
        }) {
            Ok(event) => {
                // 回放事件到数据库
                if let Err(e) = self.position_repo.replay_event(&event) {
                    log::error!("Failed to replay position event: {:?}", e);
                }
            }
            Err(e) => {
                log::error!("Failed to track position update: {:?}", e);
            }
        }
    }

    /// 计算未实现盈亏
    ///
    /// # 参数
    /// - `position`: 持仓信息
    ///
    /// # 返回
    /// 未实现盈亏金额
    ///
    /// # 计算公式
    /// ```text
    /// 多仓未实现盈亏 = (标记价格 - 开仓均价) × 持仓数量
    /// 空仓未实现盈亏 = (开仓均价 - 标记价格) × 持仓数量
    /// ```
    fn calculate_unrealized_pnl(&self, position: &PrepPosition) -> Price {
        if !position.has_position() {
            return Price::from_raw(0);
        }

        let entry = position.entry_price.to_f64();
        let mark = position.mark_price.to_f64();
        let qty = position.quantity.to_f64();

        let pnl = match position.position_side {
            crate::proc::trading_prep_order_proc::PositionSide::Long => (mark - entry) * qty,
            crate::proc::trading_prep_order_proc::PositionSide::Short => (entry - mark) * qty,
            crate::proc::trading_prep_order_proc::PositionSide::Both => {
                // 单向持仓模式，根据数量符号判断
                (mark - entry) * qty
            }
        };

        Price::from_f64(pnl)
    }

    /// 计算强平价格
    ///
    /// # 参数
    /// - `position`: 持仓信息
    ///
    /// # 返回
    /// 强平价格（如果有持仓）
    ///
    /// # 计算公式
    /// ```text
    /// 多仓强平价 = 开仓均价 × (1 - 1/杠杆倍数 + 维持保证金率)
    /// 空仓强平价 = 开仓均价 × (1 + 1/杠杆倍数 - 维持保证金率)
    ///
    /// 其中维持保证金率假设为 0.4% (Binance标准)
    /// ```
    fn calculate_liquidation_price(&self, position: &PrepPosition) -> Option<Price> {
        if !position.has_position() {
            return None;
        }

        const MAINTENANCE_MARGIN_RATE: f64 = 0.004; // 0.4% 维持保证金率
        let entry = position.entry_price.to_f64();
        let leverage = position.leverage as f64;

        let liq_price = match position.position_side {
            crate::proc::trading_prep_order_proc::PositionSide::Long => {
                // 多仓：价格下跌到此价格时强平
                entry * (1.0 - 1.0 / leverage + MAINTENANCE_MARGIN_RATE)
            }
            crate::proc::trading_prep_order_proc::PositionSide::Short => {
                // 空仓：价格上涨到此价格时强平
                entry * (1.0 + 1.0 / leverage - MAINTENANCE_MARGIN_RATE)
            }
            crate::proc::trading_prep_order_proc::PositionSide::Both => {
                // 单向模式，暂时按多仓处理
                entry * (1.0 - 1.0 / leverage + MAINTENANCE_MARGIN_RATE)
            }
        };

        Some(Price::from_f64(liq_price.max(0.0)))
    }


    // todo 1,3 4个order 4个trade,4个balance,4个持仓

    /// 处理限价单开仓 v2 版本 - 直接返回 OpenPositionResult
    ///
    /// # 参数
    /// - `cmd`: 开仓命令
    ///
    /// # 返回
    /// - `Ok(OpenPositionResult)`: 包含订单信息和成交详情的完整结果
    /// - `Err(PrepCommandError)`: 处理过程中的错误
    ///
    /// # 流程
    /// 1. 验证命令有效性
    /// 2. 检查和保存杠杆配置
    /// 3. 检查余额并冻结保证金
    /// 4. 生成订单ID
    /// 5. 从 LOB 匹配现有订单
    /// 6. 处理成交或未成交的情况
    /// 7. 更新持仓和余额
    /// 8. 返回完整结果
    // todo 完成handle_limit_order2
    fn handle_limit_order2(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // ========================================================================
        // 2. 风控检查 - 杠杆配置
        // ========================================================================
        // 直接使用命令中的杠杆倍数，不使用缓存
        let leverage = cmd.leverage;

        // ========================================================================
        // 3. 风控检查 - 余额检查并冻结保证金
        // ========================================================================
        let estimate_price = cmd.price.unwrap_or_else(|| Price::from_f64(50000.0));
        let required_margin = self.calculate_required_margin(estimate_price, cmd.quantity, leverage);

        // 检查余额并立即扣除保证金（冻结效果）
        let now = self.now();
        let required_margin_u64 = Self::price_to_u64(required_margin);

        let event = self.deduct_balance2(required_margin_u64, now)?;
        // 回放事件到数据库
        self.balance_repo
            .replay_event(&event)
            .map_err(|e| PrepCommandError::MatchingEngineError(format!("Failed to replay balance event: {:?}", e)))?;

        // ========================================================================
        // 4. 生成订单ID
        // ========================================================================
        let order_id = self.generate_order_id();

        // ========================================================================
        // 5. 从 LOB 匹配现有订单
        // ========================================================================
        let mut generated_trades = Vec::new();
        let mut total_filled = 0.0;
        let mut remaining_qty = cmd.quantity.to_f64();

        let matched_orders = self.lob_repo.match_orders(
            cmd.trading_pair,
            cmd.side,
            cmd.price.unwrap_or_else(|| Price::from_f64(50000.0)),
            cmd.quantity
        );

        // 用于收集所有需要回放的事件
        let mut all_events = Vec::new();

        if let Some(matched) = matched_orders {
            // matched_order 的状态也要同步变更，生成 log event 放在一个数据里
            for matched_order in matched {
                if remaining_qty <= 0.0 {
                    break;
                }

                // 计算当前成交数量
                let fill_amount = remaining_qty.min(matched_order.quantity.to_f64());
                let fill_qty_obj = Quantity::from_f64(fill_amount);
                let match_price = matched_order.price.unwrap_or_else(|| Price::from_f64(50000.0));

                // 计算成交金额和手续费（限价单为 Maker，费率 0.02%）
                let notional = match_price.to_f64() * fill_amount;
                let fee = Price::from_f64(notional * 0.0002);

                // 创建成交记录
                let trade = PrepTrade::new(
                    TradeId::generate(),
                    order_id.clone(),
                    order_id.clone(),
                    cmd.trading_pair,
                    cmd.side,
                    match_price,
                    fill_qty_obj,
                    fee,
                    AssetId::USDT,
                    true // Maker
                );

                generated_trades.push(trade.clone());

                // 创建 matched_order 的副本以便修改并生成事件
                let mut updated_order = matched_order.clone();
                let old_filled = updated_order.filled_quantity.to_f64();
                let new_filled_qty = Quantity::from_f64(old_filled + fill_amount);
                updated_order.filled_quantity = new_filled_qty;

                // 如果完全成交，更新状态为 Filled
                let new_order_status = if (updated_order.quantity.to_f64() - new_filled_qty.to_f64()).abs() <= 0.0001 {
                    OrderStatus::Filled
                } else {
                    OrderStatus::PartiallyFilled
                };
                updated_order.status = new_order_status.clone();

                // 生成 matched_order 的更新事件
                let filled_copy = new_filled_qty;
                let status_copy = new_order_status.clone();
                if let Ok(event) = updated_order.track_update(|order| {
                    order.filled_quantity = filled_copy;
                    order.status = status_copy.clone();
                }) {
                    all_events.push(event);
                }

                // 生成 trade 的创建事件
                if let Ok(event) = trade.track_create() {
                    all_events.push(event);
                }

                total_filled += fill_amount;
                remaining_qty -= fill_amount;
            }
        }

        // ========================================================================
        // 6. 确定最终订单状态
        // ========================================================================
        let filled_qty = Quantity::from_f64(total_filled);

        // ========================================================================
        // 7. 如果有成交，更新持仓和扣除手续费
        // ========================================================================
        if total_filled > 0.0 && !generated_trades.is_empty() {
            // 扣除手续费
            let total_fee: f64 = generated_trades.iter().map(|t| t.fee.to_f64()).sum();
            let total_fee_u64 = Self::price_to_u64(Price::from_f64(total_fee));
            let now = self.now();

            // todo
            self.deduct_balance(total_fee_u64, now).unwrap_or_else(|_| {
                log::error!("Failed to deduct fee {} for order {:?}", total_fee, order_id);
            });

            // 计算成交均价
            let mut total_notional = 0.0;
            for trade in &generated_trades {
                total_notional += trade.price.to_f64() * trade.quantity.to_f64();
            }
            let avg_price = Price::from_f64(total_notional / total_filled);

            // 更新持仓
            self.update_position(cmd.trading_pair, cmd.side, cmd.position_side, filled_qty, avg_price, leverage);

            // 获取撮合序列号
            let match_seq = self.next_match_seq();

            // ========================================================================
            // 8. 创建内部订单对象
            // ========================================================================
            let internal_order = if remaining_qty > 0.0 {
                // 部分成交情况
                PrepOrder {
                    order_id: order_id.clone(),
                    trading_pair: cmd.trading_pair,
                    side: cmd.side,
                    order_type: cmd.order_type,
                    quantity: cmd.quantity,
                    price: cmd.price,
                    filled_quantity: filled_qty,
                    status: OrderStatus::PartiallyFilled,
                    created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                        as u64,
                    frozen_margin: required_margin
                }
            } else {
                // 完全成交情况
                PrepOrder {
                    order_id: order_id.clone(),
                    trading_pair: cmd.trading_pair,
                    side: cmd.side,
                    order_type: cmd.order_type,
                    quantity: cmd.quantity,
                    price: cmd.price,
                    filled_quantity: cmd.quantity,
                    status: OrderStatus::Filled,
                    created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                        as u64,
                    frozen_margin: Price::from_raw(0)
                }
            };

            // 1. 如果有未成交部分，挂单到 LOB
            if remaining_qty > 0.0 {
                let _ = self.lob_repo.add_order(cmd.trading_pair, internal_order.clone());
            }

            // 2. 生成 internal_order 的创建事件，加入 all_events
            if let Ok(event) = internal_order.track_create() {
                all_events.push(event);
            }

            // 3. 一次性回放所有事件到数据库
            if !all_events.is_empty() {
                // 回放 matched_order 更新和 trade 创建事件到各自的 repo
                for event in &all_events {
                    // 根据 entity_type 判断回放到哪个 repo
                    match event.entity_type.as_str() {
                        "PrepOrder" => {
                            if let Err(e) = self.order_repo.replay_event(event) {
                                log::error!("Failed to replay order event: {:?}", e);
                            }
                        }
                        "PrepTrade" => {
                            if let Err(e) = self.trade_repo.replay_event(event) {
                                log::error!("Failed to replay trade event: {:?}", e);
                            }
                        }
                        _ => {}
                    }
                }
            }

            // 4. 返回相应的结果
            if remaining_qty > 0.0 {
                Ok(OpenPositionResult::partially_filled(order_id, generated_trades, match_seq))
            } else {
                Ok(OpenPositionResult::filled(order_id, generated_trades, match_seq))
            }
        } else {
            // 无成交情况
            // ========================================================================
            // 8. 创建内部订单对象（未成交，需要挂单等待）
            // ========================================================================
            let internal_order = PrepOrder {
                order_id: order_id.clone(),
                trading_pair: cmd.trading_pair,
                side: cmd.side,
                order_type: cmd.order_type,
                quantity: cmd.quantity,
                price: cmd.price,
                filled_quantity: Quantity::from_raw(0),
                status: OrderStatus::Submitted,
                created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                    as u64,
                frozen_margin: required_margin
            };

            // 挂单到 LOB
            let _ = self.lob_repo.add_order(cmd.trading_pair, internal_order.clone());

            // 生成 internal_order 的创建事件，加入 all_events
            if let Ok(event) = internal_order.track_create() {
                all_events.push(event);
            }

            // 一次性回放所有事件到数据库
            if !all_events.is_empty() {
                for event in &all_events {
                    match event.entity_type.as_str() {
                        "PrepOrder" => {
                            if let Err(e) = self.order_repo.replay_event(event) {
                                log::error!("Failed to replay order event: {:?}", e);
                            }
                        }
                        _ => {}
                    }
                }
            }

            // 返回已接受状态（待撮合）
            Ok(OpenPositionResult::accepted(order_id))
        }
    }

    fn handle_limit_order3(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        // /// 等待提交
        // Pending = 1,
        // /// 已提交
        // Submitted = 2,
        // /// 部分成交
        // PartiallyFilled = 3,
        // /// 完全成交
        // Filled = 4,
        // /// 已取消
        // Cancelled = 5,
        // /// 已拒绝
        // Rejected = 6

        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCommandError::ValidationError)?;


        let order_id = self.generate_order_id();

        // 1 创建订单
        let mut internal_order = PrepOrder::Pending(
            order_id.clone(),
            cmd.trading_pair,
            cmd.side,
            cmd.order_type,
            cmd.quantity,
            cmd.price,
            cmd.leverage
        );


        // 3. 风控检查 - 余额检查并冻结保证金
        let balance_id = format!("{}:{}", self.account_id.0, self.asset_id.0);
        let mut balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
            Some(b) => b,
            None => todo!() // todo 应该报错
        };

        let now = self.now();
        internal_order.frozen_margin(balance, now);


        // todo 如果冻结失败 balance 则变成 Rejected internal_order.change2rejected

        // 匹配
        let matched_orders = self.lob_repo.match_orders(
            cmd.trading_pair,
            cmd.side,
            cmd.price.unwrap_or_else(|| Price::from_f64(50000.0)),
            cmd.quantity
        );

        if (!matched_orders.is_none()) {
            // 如果匹配
            let mut trades = Vec::new();
            if let Some(matched) = matched_orders {
                // matched_order 的状态也要同步变更，生成 log event 放在一个数据里
                for matched_order in matched {
                    let trade = internal_order.make_trade(matched_order);

                    // todo 更新持仓和资金
                    trades.push(trade);
                    if internal_order.is_all_filled() {
                        break;
                    }
                }
            }
        } else {
            // 如果完全没有匹配
            internal_order.change2submit();
        }


        // 没成交挂单
        if (!internal_order.is_all_filled()) {
            let _ = self.lob_repo.add_order(cmd.trading_pair, internal_order.clone());
        }


        // 所有数据持久化操作
        let mut all_events: Vec<ChangeLogEntry> = Vec::new();

        // 3. 一次性回放所有事件到数据库
        if !all_events.is_empty() {
            // 回放 matched_order 更新和 trade 创建事件到各自的 repo
            for event in &all_events {
                // 根据 entity_type 判断回放到哪个 repo
                // todo 增加balance position
                match event.entity_type.as_str() {
                    "PrepOrder" => {
                        if let Err(e) = self.order_repo.replay_event(event) {
                            log::error!("Failed to replay order event: {:?}", e);
                        }
                    }
                    "PrepTrade" => {
                        if let Err(e) = self.trade_repo.replay_event(event) {
                            log::error!("Failed to replay trade event: {:?}", e);
                        }
                    }
                    _ => {}
                }
            }
        }


        // todo 每个prep order的处理是一个状态机
        // 1, 新建订单 为Pending

        todo!()
    }
}

impl PerpOrderExchProc for PrepMatchingService {
    /// 处理开仓命令
    ///
    /// # 流程
    /// 1. 验证命令有效性
    /// 2. 风控检查（余额、杠杆配置）
    /// 3. 生成订单ID
    /// 4. 根据订单类型进行撮合
    /// 5. 更新持仓和余额
    /// 6. 返回撮合结果
    fn open_position(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        self.handle_limit_order2(cmd)
    }

    fn close_position(&self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // ========================================================================
        // 2. 查询持仓并克隆数据
        // ========================================================================
        let position = self.get_position(cmd.trading_pair);

        // 验证持仓存在
        if !self.has_position(cmd.trading_pair) {
            return Err(PrepCommandError::InsufficientPosition);
        }

        // 验证持仓方向
        if position.position_side != cmd.position_side {
            return Err(PrepCommandError::InsufficientPosition);
        }

        // 验证持仓数量
        if !position.has_position() {
            return Err(PrepCommandError::InsufficientPosition);
        }

        // 确定平仓数量
        let close_qty = cmd.quantity.unwrap_or(position.quantity);
        if close_qty > position.quantity {
            return Err(PrepCommandError::InsufficientPosition);
        }

        // ========================================================================
        // 3. 生成平仓订单ID
        // ========================================================================
        let order_id = self.generate_order_id();

        // ========================================================================
        // 4. 模拟平仓成交
        // ========================================================================
        let fill_price = match cmd.side {
            Side::Buy => Price::from_f64(50000.0), // 平空用买，使用卖一价
            Side::Sell => Price::from_f64(49990.0) // 平多用卖，使用买一价
        };

        // 计算手续费 (0.04% Taker费率)
        let notional = fill_price.to_f64() * close_qty.to_f64();
        let fee = Price::from_f64(notional * 0.0004);

        let trade = PrepTrade::new(
            TradeId::generate(),
            order_id.clone(),
            order_id.clone(),
            cmd.trading_pair,
            cmd.side,
            fill_price,
            close_qty,
            fee,
            AssetId::USDT,
            false // 市价单为Taker
        );

        // ========================================================================
        // 5. 计算已实现盈亏
        // ========================================================================
        let entry_price = position.entry_price.to_f64();
        let close_price = fill_price.to_f64();
        let qty = close_qty.to_f64();

        let realized_pnl = match position.position_side {
            crate::proc::trading_prep_order_proc::PositionSide::Long => {
                // 多仓平仓盈亏 = (平仓价 - 开仓价) × 数量
                (close_price - entry_price) * qty
            }
            crate::proc::trading_prep_order_proc::PositionSide::Short => {
                // 空仓平仓盈亏 = (开仓价 - 平仓价) × 数量
                (entry_price - close_price) * qty
            }
            crate::proc::trading_prep_order_proc::PositionSide::Both => {
                // 单向模式，根据side判断
                if cmd.side == Side::Sell {
                    (close_price - entry_price) * qty
                } else {
                    (entry_price - close_price) * qty
                }
            }
        };

        let realized_pnl_price = Price::from_f64(realized_pnl);

        // ========================================================================
        // 6. 更新持仓
        // ========================================================================
        let is_full_close = close_qty == position.quantity;

        if is_full_close {
            // 完全平仓 - 移除持仓（mock 实现）
            // 在实际实现中，应从数据库删除持仓
        } else {
            // 部分平仓 - 减少数量，保证金按比例减少
            self.modify_position(cmd.trading_pair, |position| {
                let close_ratio = close_qty.to_f64() / position.quantity.to_f64();

                position.quantity = Quantity::from_f64(position.quantity.to_f64() - close_qty.to_f64());
                position.margin = Price::from_f64(position.margin.to_f64() * (1.0 - close_ratio));
                position.updated_at =
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            });
        }

        // ========================================================================
        // 7. 更新账户余额
        // ========================================================================
        let now = self.now();

        // 计算归还的保证金
        let margin_return = if is_full_close {
            position.margin.to_f64()
        } else {
            let close_ratio = close_qty.to_f64() / position.quantity.to_f64();
            position.margin.to_f64() * close_ratio
        };

        // 归还保证金
        let margin_return_u64 = Self::price_to_u64(Price::from_f64(margin_return));
        self.add_balance(margin_return_u64, now);

        // 结算盈亏（可能为负）
        if realized_pnl >= 0.0 {
            // 盈利入账
            let pnl_u64 = Self::price_to_u64(Price::from_f64(realized_pnl));
            self.add_balance(pnl_u64, now);
        } else {
            // 亏损扣除
            let loss_u64 = Self::price_to_u64(Price::from_f64(-realized_pnl));
            self.deduct_balance(loss_u64, now).unwrap_or_else(|_| {
                log::error!("Failed to deduct loss {} for position {:?}", -realized_pnl, cmd.trading_pair);
            });
        }

        // 扣除手续费
        let fee_u64 = Self::price_to_u64(fee);
        self.deduct_balance(fee_u64, now).unwrap_or_else(|_| {
            log::error!("Failed to deduct fee {} for close position {:?}", fee.to_f64(), cmd.trading_pair);
        });

        // ========================================================================
        // 8. 获取撮合序列号
        // ========================================================================
        let match_seq = self.next_match_seq();

        // ========================================================================
        // 9. 返回平仓结果
        // ========================================================================
        Ok(ClosePositionResult::filled(order_id, vec![trade], realized_pnl_price, match_seq))
    }

    fn cancel_order(&self, cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
        // 先从元数据中获取订单信息

        todo!()
    }

    fn modify_order(&self, cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // TODO: 需要实现修改LOB中的订单
        // 修改订单需要：
        // 1. 从LOB移除旧订单
        // 2. 以新价格/数量重新添加订单
        // 3. 更新元数据


        todo!()
    }

    fn cancel_all_orders(&self, cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        // TODO: 需要遍历LOB获取所有订单
        // 当前简化实现：只处理元数据
        let mut cancelled_ids: Vec<OrderId> = Vec::new();
        let mut failed_count = 0;
        let mut total_refund_u64 = 0u64;

        // TODO: 实现批量取消逻辑
        // for (order_id, metadata) in metadata_map.iter_mut() {
        //     // 从LOB获取订单信息以判断symbol
        //     // ...
        //     if should_cancel && !metadata.status.is_final() {
        //         metadata.status = OrderStatus::Cancelled;
        //         cancelled_ids.push(order_id.clone());
        //     }
        // }

        // 归还总保证金
        if total_refund_u64 > 0 {
            let now = self.now();
            self.add_balance(total_refund_u64, now);
        }

        Ok(CancelAllOrdersResult::new(cancelled_ids, failed_count))
    }

    // todo 设计某持仓的杠杆会影响保证金
    fn set_leverage(&self, cmd: SetLeverageCommand) -> Result<SetLeverageResult, PrepCommandError> {
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // 获取当前持仓信息，以便获取旧的杠杆倍数
        let position = self.get_position(cmd.trading_pair);
        let old_leverage = position.leverage;

        // 获取当前余额
        let balance_u64 = self.get_balance();
        let balance = Self::u64_to_price(balance_u64);

        Ok(SetLeverageResult {
            trading_pair: cmd.trading_pair,
            old_leverage,
            new_leverage: cmd.leverage,
            position_margin_change: Price::from_raw(0),
            available_balance: balance,
            liquidation_price: None,
            max_open_quantity: Quantity::from_f64(1000.0)
        })
    }

    fn set_margin_type(&self, cmd: SetMarginTypeCommand) -> Result<SetMarginTypeResult, PrepCommandError> {
        Ok(SetMarginTypeResult {
            trading_pair: cmd.trading_pair,
            margin_type: cmd.margin_type,
            success: true
        })
    }

    fn set_position_mode(&self, cmd: SetPositionModeCommand) -> Result<SetPositionModeResult, PrepCommandError> {
        Ok(SetPositionModeResult {
            dual_side: cmd.dual_side,
            success: true
        })
    }
}

impl PerpOrderExchQueryProc for PrepMatchingService {
    fn query_order(&self, cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
        // TODO: 需要从LOB获取完整订单信息
        // 当前简化实现：从元数据获取部分信息


        todo!()
    }

    fn query_position(&self, cmd: QueryPositionCommand) -> Result<PrepPosition, PrepCommandError> {
        Ok(self.get_position(cmd.trading_pair))
    }

    fn query_order_book(&self, _cmd: QueryOrderBookCommand) -> Result<OrderBookSnapshot, PrepCommandError> {
        // 简化实现：返回空订单簿
        Ok(OrderBookSnapshot::empty(_cmd.trading_pair))
    }

    fn query_trades(&self, _cmd: QueryTradesCommand) -> Result<TradesQueryResult, PrepCommandError> {
        // 简化实现：返回空结果
        Ok(TradesQueryResult::empty())
    }

    fn query_account_balance(&self, cmd: QueryAccountBalanceCommand) -> Result<Vec<AccountBalance>, PrepCommandError> {
        let balance_u64 = self.get_balance();
        let balance = Self::u64_to_price(balance_u64);

        let account_balance = AccountBalance::new(
            cmd.asset.unwrap_or_else(|| AssetId::USDT),
            balance,
            balance,
            Price::from_raw(0),
            Price::from_raw(0),
            Price::from_raw(0)
        );

        Ok(vec![account_balance])
    }

    fn query_account_info(&self, _cmd: QueryAccountInfoCommand) -> Result<AccountInfo, PrepCommandError> {
        let balance_u64 = self.get_balance();
        let balance = Self::u64_to_price(balance_u64);

        // Mock 实现：返回空的持仓列表
        let positions_vec: Vec<PrepPosition> = vec![];

        Ok(AccountInfo::new(balance, Price::from_raw(0), Price::from_raw(0), balance, positions_vec, Vec::new()))
    }

    fn query_mark_price(&self, cmd: QueryMarkPriceCommand) -> Result<Vec<MarkPriceInfo>, PrepCommandError> {
        // 简化实现：返回模拟标记价格
        let symbol = cmd.trading_pair.unwrap_or_else(|| TradingPair::USDT_USDT);

        let mark_price = MarkPriceInfo::new(
            symbol,
            Price::from_f64(50000.0),
            Price::from_f64(49995.0),
            Price::from_f64(0.0001),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
                + 8 * 3600 * 1000,
            Price::from_f64(50005.0)
        );

        Ok(vec![mark_price])
    }

    fn query_funding_rate_history(
        &self, _cmd: QueryFundingRateHistoryCommand
    ) -> Result<Vec<FundingRateRecord>, PrepCommandError> {
        // 简化实现：返回空历史
        Ok(Vec::new())
    }

    fn query_funding_fee(&self, _cmd: QueryFundingFeeCommand) -> Result<Vec<FundingFeeRecord>, PrepCommandError> {
        // 简化实现：返回空记录
        Ok(Vec::new())
    }
}
