use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock}
};

// Clean Architecture: 引入 BalanceRepo 和 PositionRepo 接口
use account::domain::entity::{AccountId, AssetId, Timestamp};
use account::domain::repo::{BalanceRepo, PositionRepo};

use crate::proc::trading_prep_order_proc::{
    AccountBalance, AccountInfo, CancelAllOrdersCommand, CancelAllOrdersResult, CancelOrderCommand, CancelOrderResult,
    ClosePositionCommand, ClosePositionResult, FundingFeeRecord, FundingRateRecord, MarkPriceInfo, ModifyOrderCommand,
    ModifyOrderResult, OpenPositionCommand, OpenPositionResult, OrderBookSnapshot, OrderId, OrderQueryResult,
    OrderStatus, OrderType, PerpOrderExchProc, PerpOrderExchQueryProc, PositionInfo, PrepCommandError, Price, Quantity,
    QueryAccountBalanceCommand, QueryAccountInfoCommand, QueryFundingFeeCommand, QueryFundingRateHistoryCommand,
    QueryMarkPriceCommand, QueryOrderBookCommand, QueryOrderCommand, QueryPositionCommand, QueryTradesCommand,
    SetLeverageCommand, SetLeverageResult, SetMarginTypeCommand, SetMarginTypeResult, SetPositionModeCommand,
    SetPositionModeResult, Side, Symbol, Trade, TradeId, TradesQueryResult
};

/// 本地撮合引擎服务
///
/// 遵循Clean Architecture原则的撮合引擎实现
/// - 依赖注入：通过 BalanceRepo 接口管理余额
/// - 依赖注入：通过 PositionRepo 接口管理持仓
/// - 支持价格-时间优先的订单匹配
/// - 维护持仓、余额、订单状态
pub struct MatchingService<R: BalanceRepo, P: PositionRepo<PositionInfo>> {
    /// 余额仓储（依赖注入）
    balance_repo: Arc<Mutex<R>>,

    /// 持仓仓储（依赖注入）
    position_repo: Arc<Mutex<P>>,

    /// 账户ID（固定账户）
    account_id: AccountId,

    /// 资产ID（USDT）
    asset_id: AssetId,

    /// 订单映射（订单ID -> 订单信息）
    orders: Arc<RwLock<HashMap<OrderId, InternalOrder>>>,
    /// 杠杆配置（交易对 -> 杠杆倍数）
    leverage_config: Arc<RwLock<HashMap<Symbol, u8>>>,
    /// 撮合序列号（用于追踪撮合顺序）
    match_seq: Arc<RwLock<u64>>
}

/// 内部订单状态（扩展字段用于撮合引擎）
#[derive(Debug, Clone)]
struct InternalOrder {
    order_id: OrderId,
    symbol: Symbol,
    side: Side,
    order_type: OrderType,
    quantity: Quantity,
    price: Option<Price>,
    filled_quantity: Quantity,
    status: OrderStatus,
    created_at: u64,
    /// 冻结的保证金金额（用于订单取消时归还）
    frozen_margin: Price
}

impl<R: BalanceRepo, P: PositionRepo<PositionInfo>> MatchingService<R, P> {
    /// 创建新的撮合服务实例
    ///
    /// # 参数
    /// - `balance_repo`: 余额仓储实现（依赖注入）
    /// - `position_repo`: 持仓仓储实现（依赖注入）
    /// - `account_id`: 账户ID
    /// - `asset_id`: 资产ID（如 USDT）
    pub fn new(balance_repo: R, position_repo: P, account_id: AccountId, asset_id: AssetId) -> Self {
        Self {
            balance_repo: Arc::new(Mutex::new(balance_repo)),
            position_repo: Arc::new(Mutex::new(position_repo)),
            account_id,
            asset_id,
            orders: Arc::new(RwLock::new(HashMap::new())),
            leverage_config: Arc::new(RwLock::new(HashMap::new())),
            match_seq: Arc::new(RwLock::new(0))
        }
    }

    /// 获取当前余额
    ///
    /// # 返回
    /// 可用余额（u64 原始值）
    ///
    /// # 说明
    /// 从 BalanceRepo 获取余额，如果不存在返回 0
    fn get_balance(&self) -> u64 {
        let repo = self.balance_repo.lock().unwrap();
        repo.get(self.account_id, self.asset_id).map(|b| b.available).unwrap_or(0)
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
        let mut repo = self.balance_repo.lock().unwrap();

        let balance = repo.get_or_create(self.account_id, self.asset_id, now);
        if balance.available < amount {
            return Err(PrepCommandError::InsufficientBalance);
        }
        balance.deduct_balance(amount, now);

        Ok(())
    }

    /// 增加余额（归还保证金、盈利入账）
    ///
    /// # 参数
    /// - `amount`: 增加金额（u64）
    fn add_balance(&self, amount: u64, now: Timestamp) {
        let mut repo = self.balance_repo.lock().unwrap();
        let balance = repo.get_or_create(self.account_id, self.asset_id, now);
        balance.add_balance(amount, now);
    }

    // ========================================================================
    // PositionRepo 辅助方法
    // ========================================================================

    /// 获取持仓信息
    ///
    /// # 参数
    /// - `symbol`: 交易对
    ///
    /// # 返回
    /// 持仓信息，如果不存在返回空持仓
    fn get_position(&self, symbol: Symbol) -> PositionInfo {
        let repo = self.position_repo.lock().unwrap();
        repo.get(symbol).cloned().unwrap_or_else(|| PositionInfo::empty(symbol, account::PositionSide::Both))
    }

    /// 保存持仓信息
    ///
    /// # 参数
    /// - `position`: 持仓信息
    fn save_position(&self, position: PositionInfo) {
        let mut repo = self.position_repo.lock().unwrap();
        repo.save(position);
    }

    /// 修改持仓（如果存在）
    ///
    /// # 参数
    /// - `symbol`: 交易对
    /// - `modify_fn`: 修改函数
    fn modify_position<F>(&self, symbol: Symbol, modify_fn: F)
    where
        F: FnOnce(&mut PositionInfo)
    {
        let mut repo = self.position_repo.lock().unwrap();
        if let Some(position) = repo.get_mut(symbol) {
            modify_fn(position);
        }
    }

    /// 检查持仓是否存在
    fn has_position(&self, symbol: Symbol) -> bool {
        let repo = self.position_repo.lock().unwrap();
        repo.exists(symbol)
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

    /// 模拟市价单成交
    ///
    /// # 参数
    /// - `order_id`: 订单ID
    /// - `cmd`: 开仓命令
    ///
    /// # 返回
    /// 成交明细列表
    ///
    /// # 说明
    /// 市价单立即以估算价格成交（实际应查询订单簿）
    /// 这里简化处理，使用固定价格模拟成交
    fn simulate_market_fill(&self, order_id: &OrderId, cmd: &OpenPositionCommand) -> Vec<Trade> {
        // 简化：使用固定价格模拟市价成交
        // 实际实现应查询订单簿获取最优价格
        let fill_price = match cmd.side {
            Side::Buy => Price::from_f64(50000.0), // 买单使用卖一价
            Side::Sell => Price::from_f64(49990.0) // 卖单使用买一价
        };

        // 计算手续费 (0.04% Taker费率)
        let notional = fill_price.to_f64() * cmd.quantity.to_f64();
        let fee = Price::from_f64(notional * 0.0004);

        vec![Trade::new(
            TradeId::generate(),
            order_id.clone(),
            cmd.symbol,
            cmd.side,
            fill_price,
            cmd.quantity,
            fee,
            Symbol::new("USDT"),
            false // 市价单为Taker
        )]
    }

    /// 模拟限价单撮合
    ///
    /// # 参数
    /// - `order_id`: 订单ID
    /// - `cmd`: 开仓命令
    ///
    /// # 返回
    /// (是否成交, 成交明细列表)
    ///
    /// # 说明
    /// 简化实现：50%概率立即成交，50%概率挂单等待
    fn simulate_limit_fill(&self, order_id: &OrderId, cmd: &OpenPositionCommand) -> (bool, Vec<Trade>) {
        // 简化：随机决定是否立即成交
        // 实际实现应查询订单簿判断是否能撮合
        let should_fill = rand::random::<bool>();

        if !should_fill {
            return (false, Vec::new());
        }

        let fill_price = cmd.price.unwrap();
        let notional = fill_price.to_f64() * cmd.quantity.to_f64();
        let fee = Price::from_f64(notional * 0.0002); // 0.02% Maker费率

        let trade = Trade::new(
            TradeId::generate(),
            order_id.clone(),
            cmd.symbol,
            cmd.side,
            fill_price,
            cmd.quantity,
            fee,
            Symbol::new("USDT"),
            true // 限价单为Maker
        );

        (true, vec![trade])
    }

    /// 更新或创建持仓
    ///
    /// # 参数
    /// - `symbol`: 交易对
    /// - `_side`: 订单方向（仅用于向后兼容）
    /// - `position_side`: 持仓方向
    /// - `quantity`: 成交数量
    /// - `avg_price`: 成交均价
    /// - `leverage`: 杠杆倍数
    fn update_position(
        &self, symbol: Symbol, _side: Side, position_side: crate::proc::trading_prep_order_proc::PositionSide,
        quantity: Quantity, avg_price: Price, leverage: u8
    ) {
        // 获取或创建持仓
        let mut position = self.get_position(symbol);
        if !self.has_position(symbol) {
            position = PositionInfo::empty(symbol, position_side);
        }

        // 更新持仓数量和均价
        let old_qty = position.quantity.to_f64();
        let old_price = position.entry_price.to_f64();
        let new_qty_val = quantity.to_f64();
        let new_price = avg_price.to_f64();

        // 计算新的持仓均价（加权平均）
        let total_cost = old_qty * old_price + new_qty_val * new_price;
        let total_qty = old_qty + new_qty_val;

        position.quantity = Quantity::from_f64(total_qty);
        position.entry_price =
            if total_qty > 0.0 { Price::from_f64(total_cost / total_qty) } else { Price::from_raw(0) };
        position.leverage = leverage;
        position.mark_price = avg_price;
        position.updated_at =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

        // 计算保证金 = (持仓价值) / 杠杆倍数
        let notional = position.entry_price.to_f64() * position.quantity.to_f64();
        position.margin = Price::from_f64(notional / leverage as f64);

        // 计算未实现盈亏
        position.unrealized_pnl = self.calculate_unrealized_pnl(&position);

        // 计算强平价格
        position.liquidation_price = self.calculate_liquidation_price(&position);

        // 保存持仓
        self.save_position(position);
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
    fn calculate_unrealized_pnl(&self, position: &PositionInfo) -> Price {
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
    fn calculate_liquidation_price(&self, position: &PositionInfo) -> Option<Price> {
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
}

impl<R: BalanceRepo, P: PositionRepo<PositionInfo>> PerpOrderExchProc for MatchingService<R, P> {
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
        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // ========================================================================
        // 2. 风控检查 - 杠杆配置
        // ========================================================================
        let leverage = {
            let config = self.leverage_config.read().unwrap();
            *config.get(&cmd.symbol).unwrap_or(&cmd.leverage)
        };

        // 如果杠杆配置不存在，使用命令中的杠杆并保存配置
        if leverage == cmd.leverage {
            let mut config = self.leverage_config.write().unwrap();
            config.insert(cmd.symbol, cmd.leverage);
        }

        // ========================================================================
        // 3. 风控检查 - 余额检查并冻结保证金
        // ========================================================================
        let estimate_price = cmd.price.unwrap_or_else(|| Price::from_f64(50000.0));
        let required_margin = self.calculate_required_margin(estimate_price, cmd.quantity, leverage);

        // 检查余额并立即扣除保证金（冻结效果）
        let now = self.now();
        let required_margin_u64 = Self::price_to_u64(required_margin);

        // 使用 BalanceRepo 扣除保证金
        self.deduct_balance(required_margin_u64, now)?;

        // ========================================================================
        // 4. 生成订单ID
        // ========================================================================
        let order_id = OrderId::generate();

        // ========================================================================
        // 5. 根据订单类型进行撮合
        // ========================================================================
        let (status, trades): (OrderStatus, Vec<Trade>) = match cmd.order_type {
            OrderType::Market => {
                // 市价单立即成交
                let trades = self.simulate_market_fill(&order_id, &cmd);
                (OrderStatus::Filled, trades)
            }
            OrderType::Limit => {
                // 限价单尝试撮合
                let (filled, trades) = self.simulate_limit_fill(&order_id, &cmd);
                if filled {
                    (OrderStatus::Filled, trades)
                } else {
                    // 未成交，进入订单簿
                    (OrderStatus::Submitted, Vec::new())
                }
            }
        };

        // ========================================================================
        // 6. 保存订单记录
        // ========================================================================
        let filled_quantity = if status == OrderStatus::Filled { cmd.quantity } else { Quantity::from_raw(0) };

        let internal_order = InternalOrder {
            order_id: order_id.clone(),
            symbol: cmd.symbol,
            side: cmd.side,
            order_type: cmd.order_type,
            quantity: cmd.quantity,
            price: cmd.price,
            filled_quantity,
            status,
            created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
            frozen_margin: required_margin // 保存冻结的保证金金额
        };

        {
            let mut orders = self.orders.write().unwrap();
            orders.insert(order_id.clone(), internal_order);
        }

        // ========================================================================
        // 7. 如果成交，更新持仓和余额
        // ========================================================================
        if status == OrderStatus::Filled && !trades.is_empty() {
            // 计算成交均价和总数量
            let mut total_notional = 0.0;
            let mut total_quantity = 0.0;

            for trade in &trades {
                let notional = trade.price.to_f64() * trade.quantity.to_f64();
                total_notional += notional;
                total_quantity += trade.quantity.to_f64();
            }

            let avg_price = if total_quantity > 0.0 {
                Price::from_f64(total_notional / total_quantity)
            } else {
                Price::from_raw(0)
            };

            let total_qty = Quantity::from_f64(total_quantity);

            // 更新持仓（使用命令中的持仓方向）
            self.update_position(cmd.symbol, cmd.side, cmd.position_side, total_qty, avg_price, leverage);

            // 扣除手续费
            let total_fee: f64 = trades.iter().map(|t| t.fee.to_f64()).sum();
            let total_fee_u64 = Self::price_to_u64(Price::from_f64(total_fee));
            let now = self.now();

            // 使用 BalanceRepo 扣除手续费
            self.deduct_balance(total_fee_u64, now).unwrap_or_else(|_| {
                // 手续费扣除失败（理论上不应该发生，因为保证金已经冻结）
                log::error!("Failed to deduct fee {} for order {:?}", total_fee, order_id);
            });

            // 获取撮合序列号
            let match_seq = self.next_match_seq();

            // 返回成交结果
            Ok(OpenPositionResult::filled(order_id, trades, match_seq))
        } else {
            // 未成交或部分成交
            Ok(OpenPositionResult::accepted(order_id))
        }
    }

    fn close_position(&self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        // ========================================================================
        // 1. 命令验证
        // ========================================================================
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // ========================================================================
        // 2. 查询持仓并克隆数据
        // ========================================================================
        let position = self.get_position(cmd.symbol);

        // 验证持仓存在
        if !self.has_position(cmd.symbol) {
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
        let order_id = OrderId::generate();

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

        let trade = Trade::new(
            TradeId::generate(),
            order_id.clone(),
            cmd.symbol,
            cmd.side,
            fill_price,
            close_qty,
            fee,
            Symbol::new("USDT"),
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
            // 完全平仓 - 移除持仓
            let mut repo = self.position_repo.lock().unwrap();
            repo.remove(cmd.symbol);
        } else {
            // 部分平仓 - 减少数量，保证金按比例减少
            self.modify_position(cmd.symbol, |position| {
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
                log::error!("Failed to deduct loss {} for position {:?}", -realized_pnl, cmd.symbol);
            });
        }

        // 扣除手续费
        let fee_u64 = Self::price_to_u64(fee);
        self.deduct_balance(fee_u64, now).unwrap_or_else(|_| {
            log::error!("Failed to deduct fee {} for close position {:?}", fee.to_f64(), cmd.symbol);
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
        let mut orders = self.orders.write().unwrap();

        if let Some(order) = orders.get_mut(&cmd.order_id) {
            if order.status.is_final() {
                return Ok(CancelOrderResult::failed(cmd.order_id, order.status));
            }

            // 归还冻结的保证金（仅限未成交或部分成交的订单）
            let unfilled_quantity = order.quantity.to_f64() - order.filled_quantity.to_f64();
            if unfilled_quantity > 0.0 {
                let refund_ratio = unfilled_quantity / order.quantity.to_f64();
                let refund_margin = Price::from_f64(order.frozen_margin.to_f64() * refund_ratio);
                let refund_u64 = Self::price_to_u64(refund_margin);

                let now = self.now();
                self.add_balance(refund_u64, now);
            }

            order.status = OrderStatus::Cancelled;
            Ok(CancelOrderResult::success(cmd.order_id))
        } else {
            Err(PrepCommandError::OrderNotFound(cmd.order_id.as_str().to_string()))
        }
    }

    fn modify_order(&self, cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        let mut orders = self.orders.write().unwrap();

        if let Some(order) = orders.get_mut(&cmd.order_id) {
            if order.status.is_final() {
                return Ok(ModifyOrderResult::failed(cmd.order_id));
            }

            if let Some(new_price) = cmd.new_price {
                order.price = Some(new_price);
            }

            if let Some(new_qty) = cmd.new_quantity {
                order.quantity = new_qty;
            }

            Ok(ModifyOrderResult::success(cmd.order_id, cmd.new_price, cmd.new_quantity))
        } else {
            Err(PrepCommandError::OrderNotFound(cmd.order_id.as_str().to_string()))
        }
    }

    fn cancel_all_orders(&self, cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        let mut orders = self.orders.write().unwrap();
        let mut cancelled_ids = Vec::new();
        let mut failed_count = 0;
        let mut total_refund_u64 = 0u64;

        for (order_id, order) in orders.iter_mut() {
            let should_cancel = match (&cmd.symbol, &cmd.position_side) {
                (None, None) => true,
                (Some(sym), None) => order.symbol == *sym,
                (None, Some(_)) => true, // 简化：忽略position_side过滤
                (Some(sym), Some(_)) => order.symbol == *sym
            };

            if should_cancel && !order.status.is_final() {
                // 归还保证金
                let unfilled_quantity = order.quantity.to_f64() - order.filled_quantity.to_f64();
                if unfilled_quantity > 0.0 {
                    let refund_ratio = unfilled_quantity / order.quantity.to_f64();
                    let refund_margin = order.frozen_margin.to_f64() * refund_ratio;
                    total_refund_u64 += Self::price_to_u64(Price::from_f64(refund_margin));
                }

                order.status = OrderStatus::Cancelled;
                cancelled_ids.push(order_id.clone());
            } else if should_cancel {
                failed_count += 1;
            }
        }

        // 归还总保证金
        if total_refund_u64 > 0 {
            let now = self.now();
            self.add_balance(total_refund_u64, now);
        }

        Ok(CancelAllOrdersResult::new(cancelled_ids, failed_count))
    }

    fn set_leverage(&self, cmd: SetLeverageCommand) -> Result<SetLeverageResult, PrepCommandError> {
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        let mut config = self.leverage_config.write().unwrap();
        let old_leverage = *config.get(&cmd.symbol).unwrap_or(&1);
        config.insert(cmd.symbol, cmd.leverage);

        // 获取当前余额
        let balance_u64 = self.get_balance();
        let balance = Self::u64_to_price(balance_u64);

        Ok(SetLeverageResult {
            symbol: cmd.symbol,
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
            symbol: cmd.symbol,
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

impl<R: BalanceRepo, P: PositionRepo<PositionInfo>> PerpOrderExchQueryProc for MatchingService<R, P> {
    fn query_order(&self, cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
        let orders = self.orders.read().unwrap();

        if let Some(order) = orders.get(&cmd.order_id) {
            let (avg_price, filled_quantity) = if order.status == OrderStatus::Filled {
                (order.price, order.filled_quantity)
            } else {
                (None, Quantity::from_raw(0))
            };

            Ok(OrderQueryResult {
                order_id: order.order_id.clone(),
                symbol: order.symbol,
                side: order.side,
                order_type: order.order_type,
                status: order.status,
                quantity: order.quantity,
                price: order.price,
                filled_quantity,
                avg_price,
                position_side: crate::proc::trading_prep_order_proc::PositionSide::Long,
                created_at: order.created_at,
                updated_at: order.created_at
            })
        } else {
            Err(PrepCommandError::OrderNotFound(cmd.order_id.as_str().to_string()))
        }
    }

    fn query_position(&self, cmd: QueryPositionCommand) -> Result<PositionInfo, PrepCommandError> {
        Ok(self.get_position(cmd.symbol))
    }

    fn query_order_book(&self, cmd: QueryOrderBookCommand) -> Result<OrderBookSnapshot, PrepCommandError> {
        // 简化实现：返回空订单簿
        Ok(OrderBookSnapshot::empty(cmd.symbol))
    }

    fn query_trades(&self, cmd: QueryTradesCommand) -> Result<TradesQueryResult, PrepCommandError> {
        // 简化实现：返回空结果
        Ok(TradesQueryResult::empty())
    }

    fn query_account_balance(&self, cmd: QueryAccountBalanceCommand) -> Result<Vec<AccountBalance>, PrepCommandError> {
        let balance_u64 = self.get_balance();
        let balance = Self::u64_to_price(balance_u64);

        let account_balance = AccountBalance::new(
            cmd.asset.unwrap_or_else(|| Symbol::new("USDT")),
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

        let repo = self.position_repo.lock().unwrap();
        let positions_vec: Vec<PositionInfo> = repo.get_all();

        Ok(AccountInfo::new(balance, Price::from_raw(0), Price::from_raw(0), balance, positions_vec, Vec::new()))
    }

    fn query_mark_price(&self, cmd: QueryMarkPriceCommand) -> Result<Vec<MarkPriceInfo>, PrepCommandError> {
        // 简化实现：返回模拟标记价格
        let symbol = cmd.symbol.unwrap_or_else(|| Symbol::new("BTCUSDT"));

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
        &self, cmd: QueryFundingRateHistoryCommand
    ) -> Result<Vec<FundingRateRecord>, PrepCommandError> {
        // 简化实现：返回空历史
        Ok(Vec::new())
    }

    fn query_funding_fee(&self, cmd: QueryFundingFeeCommand) -> Result<Vec<FundingFeeRecord>, PrepCommandError> {
        // 简化实现：返回空记录
        Ok(Vec::new())
    }
}
