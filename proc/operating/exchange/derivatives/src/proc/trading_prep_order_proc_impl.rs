use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock}
};

// Clean Architecture: 引入 MySqlDbRepo 和相关接口
// Base types
use base_types::{AccountId, AssetId, PositionSide, PrepPosition, Side as BaseSide, Timestamp, TradingPair};
use base_types::account::balance::Balance;
use base_types::exchange::prep::prep_order::PrepOrder;
use db_repo::{CmdRepo, MySqlDbRepo, QueryRepo};
// Event Sourcing: Entity trait for track_update
use diff::{ChangeLogEntry, Entity};
use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;
// LOB 仓储接口
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::trading_prep_order_behavior::{
    AccountBalance, AccountInfo, CancelAllOrdersCmd, CancelAllOrdersResult, CancelOrderCmd,
    CancelOrderResult, ClosePositionCmd, ClosePositionResult, FundingFeeRecord, FundingRateRecord,
    MarkPriceInfo, ModifyOrderCmd, ModifyOrderResult, OpenPositionCmd, OpenPositionResult,
    OrderBookSnapshot, OrderId, OrderQueryResult, PerpOrderExchBehavior, PerpOrderExchQueryProc, PrepCmdError,
    PrepTrade, Price, Quantity, QueryAccountBalanceCmd, QueryAccountInfoCmd, QueryFundingFeeCmd,
    QueryFundingRateHistoryCmd, QueryMarkPriceCmd, QueryOrderBookCmd, QueryOrderCmd,
    QueryPositionCmd, QueryTradesCmd, SetLeverageCmd, SetLeverageResult, SetMarginTypeCmd,
    SetMarginTypeResult, SetPositionModeCmd, SetPositionModeResult, Side, TradeId, TradesQueryResult
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

    lob_repo: EmbeddedLobRepo<PrepOrder>,

    /// 账户ID（固定账户）
    account_id: AccountId,

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
        trade_repo: MySqlDbRepo<PrepTrade>, order_repo: MySqlDbRepo<PrepOrder>, lob_repo: EmbeddedLobRepo<PrepOrder>,
        account_id: AccountId
    ) -> Self {
        Self {
            balance_repo,
            position_repo,
            trade_repo,
            order_repo,
            lob_repo,
            account_id,
            match_seq: Arc::new(RwLock::new(0))
        }
    }


    /// 扣减余额（冻结保证金、支付手续费）
    ///
    /// # 参数
    /// - `amount`: 扣减金额（u64）
    ///
    /// # 返回
    /// - `Ok(())`: 扣减成功
    /// - `Err(InsufficientBalance)`: 余额不足
    fn deduct_balance(&self, amount: Price, now: Timestamp) -> Result<(), PrepCmdError> {
        // 获取变更事件
        let event = self.deduct_balance2(amount, now)?;

        // 回放事件到数据库
        self.balance_repo
            .replay_event(&event)
            .map_err(|e| PrepCmdError::MatchingEngineError(format!("Failed to replay balance event: {:?}", e)))?;

        Ok(())
    }

    /// 生成扣减余额变更事件（内部实现）
    fn deduct_balance2(&self, amount: Price, now: Timestamp) -> Result<diff::ChangeLogEntry, PrepCmdError> {
        // TODO: 从数据库获取当前余额并验证充足
        // 然后生成变更事件
        // let event = diff::ChangeLogEntry::new(
        //     "Balance".to_string(),
        //     format!("{}:USDT", self.account_id.0),
        //     "DebitFrozen".to_string(),
        //     serde_json::json!({"amount": amount.to_f64()})
        // );
        todo!()
    }

    /// 增加余额
    fn add_balance(&self, amount: Price, now: Timestamp) {
        // TODO: 实现增加余额逻辑
        log::debug!("Adding balance: {}", amount.to_f64());
    }

    /// 获取当前余额
    fn get_balance(&self) -> Price {
        // TODO: 从数据库获取当前余额
        Price::default()
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
        match self.position_repo.find_one_by_condition(PrepPosition::empty(trading_pair, PositionSide::Both)) {
            Ok(Some(position)) => position,
            Ok(None) | Err(_) => {
                // 如果数据库中不存在，返回空持仓
                PrepPosition::empty(trading_pair, PositionSide::Both)
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
        let mut position = PrepPosition::empty(trading_pair, PositionSide::Both);
        modify_fn(&mut position);
        // 实际应调用 self.save_position(position)
    }

    /// 检查持仓是否存在
    fn has_position(&self, trading_pair: TradingPair) -> bool {
        // 从数据库查询是否存在该持仓
        match self.position_repo.find_one_by_condition(PrepPosition::empty(trading_pair, PositionSide::Both)) {
            Ok(Some(position)) => position.has_position(),
            Ok(None) | Err(_) => false
        }
    }


    /// 获取当前时间戳（纳秒）
    #[inline]
    fn now(&self) -> Timestamp {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64
    }


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
        position_side: crate::proc::trading_prep_order_behavior::PositionSide, quantity: Quantity, avg_price: Price,
        leverage: u8
    ) {
        // 获取或创建持仓（通过 self.position_repo）
        let mut position = self.get_position(trading_pair);

        // ========================================================================
        // Track position 生成log event, then replay_event
        // ========================================================================
        match position.track_update(|p: &mut PrepPosition| {
            // 调用position的update方法更新所有信息（数量、均价、杠杆、保证金、PnL等）
            p.add(quantity, avg_price, leverage, _side, position_side);
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


    fn handle_limit_order3(&self, cmd: OpenPositionCmd) -> Result<OpenPositionResult, PrepCmdError> {
        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCmdError::ValidationError)?;

        // todo time_in_force 没有用
        let order_id = self.generate_order_id();

        // 1 创建订单
        let mut internal_order = PrepOrder::pending(
            order_id.clone(),
            self.account_id,
            cmd.trading_pair,
            cmd.side,
            cmd.order_type,
            cmd.quantity,
            cmd.price,
            cmd.leverage
        );


        let balance_id = format!("{}:{}", self.account_id.0, cmd.trading_pair.base_asset.0);
        let mut balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
            Some(b) => b,
            None => todo!() // todo 应该报错
        };

        let now = self.now();
        // 2 风控检查 - 余额检查并冻结保证金
        internal_order.frozen_margin(balance.clone(), now);


        // 匹配
        let matched_orders = self.lob_repo.match_orders(
            cmd.trading_pair,
            cmd.side,
            cmd.price.unwrap_or_else(|| Price::from_f64(50000.0)),
            cmd.quantity
        );

        // 获取或创建持仓（通过 self.position_repo）
        let mut position = self.get_position(cmd.trading_pair);


        if (matched_orders.is_some()) {
            // 如果匹配
            let mut trades = Vec::new();
            if let Some(matched) = matched_orders {
                // matched_order 的状态也要同步变更，生成 log event 放在一个数据里
                for matched_order in matched {
                    let mut matched_position = self.get_position(matched_order.trading_pair);

                    let balance_id = format!("{}:{}", matched_order.account_id.0, cmd.trading_pair.base_asset.0);
                    let mut matched_balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
                        Some(b) => b,
                        None => todo!() // todo 应该报错
                    };

                    let mut matched_order_mut = matched_order.clone();
                    let trade = internal_order.make_trade(
                        &mut matched_order_mut,
                        &mut matched_balance,
                        &mut matched_position,
                        &mut balance,
                        &mut position,
                        now
                    );

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
            // 回放 matched_order 更新和 v1 创建事件到各自的 repo
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
                            log::error!("Failed to replay v1 event: {:?}", e);
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

impl PerpOrderExchBehavior for PrepMatchingService {
    /// 处理开仓命令
    ///
    /// # 流程
    /// 1. 验证命令有效性
    /// 2. 风控检查（余额、杠杆配置）
    /// 3. 生成订单ID
    /// 4. 根据订单类型进行撮合
    /// 5. 更新持仓和余额
    /// 6. 返回撮合结果
    fn open_position(&self, cmd: OpenPositionCmd) -> Result<OpenPositionResult, PrepCmdError> {
        self.handle_limit_order3(cmd)
    }

    fn close_position(&self, cmd: ClosePositionCmd) -> Result<ClosePositionResult, PrepCmdError> {
        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCmdError::ValidationError)?;

        // ========================================================================
        // 2. 查询持仓并克隆数据
        // ========================================================================
        let position = self.get_position(cmd.trading_pair);

        // 验证持仓存在
        if !self.has_position(cmd.trading_pair) {
            return Err(PrepCmdError::InsufficientPosition);
        }

        // 验证持仓方向
        if position.position_side != cmd.position_side {
            return Err(PrepCmdError::InsufficientPosition);
        }

        // 验证持仓数量
        if !position.has_position() {
            return Err(PrepCmdError::InsufficientPosition);
        }

        // 确定平仓数量
        let close_qty = cmd.quantity.unwrap_or(position.quantity);
        if close_qty > position.quantity {
            return Err(PrepCmdError::InsufficientPosition);
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
            crate::proc::trading_prep_order_behavior::PositionSide::Long => {
                // 多仓平仓盈亏 = (平仓价 - 开仓价) × 数量
                (close_price - entry_price) * qty
            }
            crate::proc::trading_prep_order_behavior::PositionSide::Short => {
                // 空仓平仓盈亏 = (开仓价 - 平仓价) × 数量
                (entry_price - close_price) * qty
            }
            crate::proc::trading_prep_order_behavior::PositionSide::Both => {
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
        let margin_return_price = Price::from_f64(margin_return);
        self.add_balance(margin_return_price, now);

        // 结算盈亏（可能为负）
        if realized_pnl >= 0.0 {
            // 盈利入账
            let pnl_price = Price::from_f64(realized_pnl);
            self.add_balance(pnl_price, now);
        } else {
            // 亏损扣除
            let loss_price = Price::from_f64(-realized_pnl);
            self.deduct_balance(loss_price, now).unwrap_or_else(|_| {
                log::error!("Failed to deduct loss {} for position {:?}", -realized_pnl, cmd.trading_pair);
            });
        }

        // 扣除手续费
        self.deduct_balance(fee, now).unwrap_or_else(|_| {
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

    fn cancel_order(&self, cmd: CancelOrderCmd) -> Result<CancelOrderResult, PrepCmdError> {
        // 1. 先从元数据中获取订单信息 2. order.cancel

        let balance_id = format!("{}:{}", self.account_id.0, cmd.trading_pair.quote_asset.0);
        let mut balance = match self.balance_repo.find_by_id(&balance_id).ok().flatten() {
            Some(b) => b,
            None => todo!() // todo 应该报错
        };


        self.lob_repo.remove_order(TradingPair::USDT_USDT, cmd.order_id);

        let order = self.lob_repo.find_order_mut(TradingPair::USDT_USDT, cmd.order_id).unwrap();

        order.cancel(&mut balance, self.now());

        todo!()
    }

    fn modify_order(&self, cmd: ModifyOrderCmd) -> Result<ModifyOrderResult, PrepCmdError> {
        cmd.validate().map_err(PrepCmdError::ValidationError)?;

        // TODO: 需要实现修改LOB中的订单
        // 修改订单需要：
        // 1. 从LOB移除旧订单
        // 2. 以新价格/数量重新添加订单
        // 3. 更新元数据


        todo!()
    }

    fn cancel_all_orders(&self, cmd: CancelAllOrdersCmd) -> Result<CancelAllOrdersResult, PrepCmdError> {
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
            let total_refund_price = Price::from_raw(total_refund_u64 as i64);
            self.add_balance(total_refund_price, now);
        }

        Ok(CancelAllOrdersResult::new(cancelled_ids, failed_count))
    }

    // todo 设计某持仓的杠杆会影响保证金
    fn set_leverage(&self, cmd: SetLeverageCmd) -> Result<SetLeverageResult, PrepCmdError> {
        cmd.validate().map_err(PrepCmdError::ValidationError)?;

        // 获取当前持仓信息，以便获取旧的杠杆倍数
        let position = self.get_position(cmd.trading_pair);
        let old_leverage = position.leverage;

        // 获取当前余额
        let balance = self.get_balance();

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

    fn set_margin_type(&self, cmd: SetMarginTypeCmd) -> Result<SetMarginTypeResult, PrepCmdError> {
        Ok(SetMarginTypeResult {
            trading_pair: cmd.trading_pair,
            margin_type: cmd.margin_type,
            success: true
        })
    }

    fn set_position_mode(&self, cmd: SetPositionModeCmd) -> Result<SetPositionModeResult, PrepCmdError> {
        Ok(SetPositionModeResult {
            dual_side: cmd.dual_side,
            success: true
        })
    }
}

impl PerpOrderExchQueryProc for PrepMatchingService {
    fn query_order(&self, _cmd: QueryOrderCmd) -> Result<OrderQueryResult, PrepCmdError> {
        // TODO: 需要从LOB获取完整订单信息
        // 当前简化实现：返回空结果
        Ok(OrderQueryResult::default())
    }

    fn query_position(&self, cmd: QueryPositionCmd) -> Result<PrepPosition, PrepCmdError> {
        Ok(self.get_position(cmd.trading_pair))
    }

    fn query_order_book(&self, _cmd: QueryOrderBookCmd) -> Result<OrderBookSnapshot, PrepCmdError> {
        // 简化实现：返回空订单簿
        Ok(OrderBookSnapshot::empty(_cmd.trading_pair))
    }

    fn query_trades(&self, _cmd: QueryTradesCmd) -> Result<TradesQueryResult, PrepCmdError> {
        // 简化实现：返回空结果
        Ok(TradesQueryResult::empty())
    }

    fn query_account_balance(&self, cmd: QueryAccountBalanceCmd) -> Result<Vec<AccountBalance>, PrepCmdError> {
        let balance = self.get_balance();

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

    fn query_account_info(&self, _cmd: QueryAccountInfoCmd) -> Result<AccountInfo, PrepCmdError> {
        let balance = self.get_balance();

        // Mock 实现：返回空的持仓列表
        let positions_vec: Vec<PrepPosition> = vec![];

        Ok(AccountInfo::new(balance, Price::from_raw(0), Price::from_raw(0), balance, positions_vec, Vec::new()))
    }

    fn query_mark_price(&self, cmd: QueryMarkPriceCmd) -> Result<Vec<MarkPriceInfo>, PrepCmdError> {
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
        &self, _cmd: QueryFundingRateHistoryCmd
    ) -> Result<Vec<FundingRateRecord>, PrepCmdError> {
        // 简化实现：返回空历史
        Ok(Vec::new())
    }

    fn query_funding_fee(&self, _cmd: QueryFundingFeeCmd) -> Result<Vec<FundingFeeRecord>, PrepCmdError> {
        // 简化实现：返回空记录
        Ok(Vec::new())
    }
}
