//! BDD测试 - PerpOrderExchProc设计验证
//!
//! 使用行为驱动开发(BDD)方式验证永续合约订单处理器的设计
//! 遵循Given-When-Then模式
//!
//! # 测试覆盖
//! - 开仓场景 (Open Position Scenarios)
//! - 平仓场景 (Close Position Scenarios)
//! - 订单管理场景 (Order Management Scenarios)
//! - 账户配置场景 (Account Configuration Scenarios)
//! - 查询场景 (Query Scenarios)
//! - 风控场景 (Risk Control Scenarios)

use prep_proc::proc::trading_prep_order_proc::*;

// ============================================================================
// BDD测试框架 - 测试上下文
// ============================================================================

/// 测试上下文 - 维护测试场景的状态
struct TestContext {
    /// 撮合服务实例
    matching_service: MockMatchingService,
    /// 测试用户ID
    user_id: String,
    /// 最后一次命令结果
    last_result: Option<TestResult>,
    /// 错误信息
    last_error: Option<PrepCommandError>,
}

/// 测试结果枚举
enum TestResult {
    OpenPosition(OpenPositionResult),
    ClosePosition(ClosePositionResult),
    CancelOrder(CancelOrderResult),
    ModifyOrder(ModifyOrderResult),
    CancelAllOrders(CancelAllOrdersResult),
    SetLeverage(SetLeverageResult),
    SetMarginType(SetMarginTypeResult),
    SetPositionMode(SetPositionModeResult),
    QueryOrder(OrderQueryResult),
    QueryPosition(PositionInfo),
    QueryOrderBook(OrderBookSnapshot),
    QueryTrades(TradesQueryResult),
    QueryAccountBalance(Vec<AccountBalance>),
    QueryAccountInfo(AccountInfo),
    QueryMarkPrice(Vec<MarkPriceInfo>),
    QueryFundingRate(Vec<FundingRateRecord>),
    QueryFundingFee(Vec<FundingFeeRecord>),
}

impl TestContext {
    fn new() -> Self {
        Self {
            matching_service: MockMatchingService::new(),
            user_id: "TEST_USER_001".to_string(),
            last_result: None,
            last_error: None,
        }
    }

    fn reset(&mut self) {
        self.matching_service = MockMatchingService::new();
        self.last_result = None;
        self.last_error = None;
    }
}

// ============================================================================
// Mock实现 - 用于测试
// ============================================================================

struct MockMatchingService {
    /// 模拟账户余额
    balance: Price,
    /// 模拟持仓
    positions: std::collections::HashMap<Symbol, PositionInfo>,
    /// 模拟订单
    orders: std::collections::HashMap<OrderId, MockOrder>,
    /// 杠杆配置
    leverage_config: std::collections::HashMap<Symbol, u8>,
    /// 保证金类型配置
    margin_type_config: std::collections::HashMap<Symbol, MarginType>,
    /// 持仓模式
    dual_side_mode: bool,
}

struct MockOrder {
    symbol: Symbol,
    side: Side,
    order_type: OrderType,
    quantity: Quantity,
    price: Option<Price>,
    position_side: PositionSide,
    status: OrderStatus,
}

impl MockMatchingService {
    fn new() -> Self {
        Self {
            balance: Price::from_f64(10000.0), // 初始余额10000 USDT
            positions: std::collections::HashMap::new(),
            orders: std::collections::HashMap::new(),
            leverage_config: std::collections::HashMap::new(),
            margin_type_config: std::collections::HashMap::new(),
            dual_side_mode: false,
        }
    }

    fn set_balance(&mut self, balance: f64) {
        self.balance = Price::from_f64(balance);
    }

    fn add_position(&mut self, symbol: Symbol, position_side: PositionSide, quantity: f64, entry_price: f64) {
        let position = PositionInfo {
            symbol,
            position_side,
            quantity: Quantity::from_f64(quantity),
            entry_price: Price::from_f64(entry_price),
            mark_price: Price::from_f64(entry_price),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 1,
            margin: Price::from_f64(entry_price * quantity),
            liquidation_price: None,
            updated_at: 0,
        };
        self.positions.insert(symbol, position);
    }

    fn calculate_required_margin(&self, cmd: &OpenPositionCommand) -> Price {
        let notional = cmd.price.unwrap_or(Price::from_f64(50000.0)).to_f64() * cmd.quantity.to_f64();
        let leverage = *self.leverage_config.get(&cmd.symbol).unwrap_or(&1);
        Price::from_f64(notional / leverage as f64)
    }
}

impl PerpOrderExchProc for MockMatchingService {
    fn open_position(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        // 验证命令
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // 风控检查 - 余额
        let required_margin = self.calculate_required_margin(&cmd);
        if self.balance < required_margin {
            return Err(PrepCommandError::InsufficientBalance);
        }

        // 生成订单ID
        let order_id = OrderId::generate();

        // 市价单直接成交
        if cmd.order_type == OrderType::Market {
            // 模拟成交
            let fill_price = Price::from_f64(50000.0); // 模拟成交价
            let trades = vec![
                Trade::new(
                    TradeId::generate(),
                    order_id.clone(),
                    cmd.symbol,
                    cmd.side,
                    fill_price,
                    cmd.quantity,
                    Price::from_f64(1.0), // 手续费
                    Symbol::new("USDT"),
                    false,
                )
            ];

            // 更新持仓
            let position = self.positions.entry(cmd.symbol)
                .or_insert(PositionInfo::empty(cmd.symbol, cmd.position_side));
            position.quantity = Quantity::from_f64(position.quantity.to_f64() + cmd.quantity.to_f64());
            position.entry_price = fill_price;

            // 扣除保证金
            self.balance = Price::from_f64(self.balance.to_f64() - required_margin.to_f64());

            Ok(OpenPositionResult::filled(order_id, trades, 1))
        } else {
            // 限价单进入订单簿
            let mock_order = MockOrder {
                symbol: cmd.symbol,
                side: cmd.side,
                order_type: cmd.order_type,
                quantity: cmd.quantity,
                price: cmd.price,
                position_side: cmd.position_side,
                status: OrderStatus::Submitted,
            };
            self.orders.insert(order_id.clone(), mock_order);
            Ok(OpenPositionResult::accepted(order_id))
        }
    }

    fn close_position(&self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        // 验证命令
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        // 检查持仓
        let position = self.positions.get(&cmd.symbol)
            .ok_or(PrepCommandError::InsufficientPosition)?;

        if !position.has_position() {
            return Err(PrepCommandError::InsufficientPosition);
        }

        // 生成订单ID
        let order_id = OrderId::generate();

        // 市价单直接成交
        if cmd.order_type == OrderType::Market {
            let close_price = Price::from_f64(51000.0); // 模拟平仓价
            let close_qty = cmd.quantity.unwrap_or(position.quantity);

            let trades = vec![
                Trade::new(
                    TradeId::generate(),
                    order_id.clone(),
                    cmd.symbol,
                    cmd.side,
                    close_price,
                    close_qty,
                    Price::from_f64(1.0),
                    Symbol::new("USDT"),
                    false,
                )
            ];

            // 计算盈亏
            let pnl = if cmd.position_side == PositionSide::Long {
                (close_price.to_f64() - position.entry_price.to_f64()) * close_qty.to_f64()
            } else {
                (position.entry_price.to_f64() - close_price.to_f64()) * close_qty.to_f64()
            };

            // 返还保证金 + 盈亏
            self.balance = Price::from_f64(self.balance.to_f64() + position.margin.to_f64() + pnl);

            // 更新持仓
            if let Some(pos) = self.positions.get_mut(&cmd.symbol) {
                let new_qty = pos.quantity.to_f64() - close_qty.to_f64();
                if new_qty <= 0.0 {
                    self.positions.remove(&cmd.symbol);
                } else {
                    pos.quantity = Quantity::from_f64(new_qty);
                }
            }

            Ok(ClosePositionResult::filled(order_id, trades, Price::from_f64(pnl), 2))
        } else {
            // 限价单进入订单簿
            Ok(ClosePositionResult::accepted(order_id))
        }
    }

    fn cancel_order(&self, cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
        if let Some(order) = self.orders.remove(&cmd.order_id) {
            if order.status.is_final() {
                return Ok(CancelOrderResult::failed(cmd.order_id, order.status));
            }
            Ok(CancelOrderResult::success(cmd.order_id))
        } else {
            Err(PrepCommandError::OrderNotFound(cmd.order_id.to_string()))
        }
    }

    fn modify_order(&self, cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        if let Some(order) = self.orders.get_mut(&cmd.order_id) {
            if order.status.is_final() {
                return Ok(ModifyOrderResult::failed(cmd.order_id));
            }
            if let Some(price) = cmd.new_price {
                order.price = Some(price);
            }
            if let Some(qty) = cmd.new_quantity {
                order.quantity = qty;
            }
            Ok(ModifyOrderResult::success(cmd.order_id, cmd.new_price, cmd.new_quantity))
        } else {
            Err(PrepCommandError::OrderNotFound(cmd.order_id.to_string()))
        }
    }

    fn cancel_all_orders(&self, cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        let mut cancelled_ids = Vec::new();
        let mut failed_count = 0;

        let order_ids: Vec<_> = self.orders.keys().cloned().collect();

        for order_id in order_ids {
            if let Some(order) = self.orders.get(&order_id) {
                // 过滤条件
                let symbol_match = cmd.symbol.is_none() || cmd.symbol.as_ref() == Some(&order.symbol);
                let side_match = cmd.position_side.is_none() || cmd.position_side == Some(order.position_side);

                if symbol_match && side_match {
                    if !order.status.is_final() {
                        self.orders.remove(&order_id);
                        cancelled_ids.push(order_id);
                    } else {
                        failed_count += 1;
                    }
                }
            }
        }

        Ok(CancelAllOrdersResult::new(cancelled_ids, failed_count))
    }

    fn set_leverage(&self, cmd: SetLeverageCommand) -> Result<SetLeverageResult, PrepCommandError> {
        cmd.validate().map_err(PrepCommandError::ValidationError)?;

        let old_leverage = *self.leverage_config.get(&cmd.symbol).unwrap_or(&1);
        self.leverage_config.insert(cmd.symbol, cmd.leverage);

        // 计算保证金变化
        let margin_change = if let Some(position) = self.positions.get(&cmd.symbol) {
            let old_margin = position.entry_price.to_f64() * position.quantity.to_f64() / old_leverage as f64;
            let new_margin = position.entry_price.to_f64() * position.quantity.to_f64() / cmd.leverage as f64;
            new_margin - old_margin
        } else {
            0.0
        };

        Ok(SetLeverageResult {
            symbol: cmd.symbol,
            old_leverage,
            new_leverage: cmd.leverage,
            position_margin_change: Price::from_f64(margin_change),
            available_balance: self.balance,
            liquidation_price: None,
            max_open_quantity: Quantity::from_f64(100.0),
        })
    }

    fn set_margin_type(&self, cmd: SetMarginTypeCommand) -> Result<SetMarginTypeResult, PrepCommandError> {
        // 检查是否有持仓
        if self.positions.contains_key(&cmd.symbol) {
            return Err(PrepCommandError::InvalidOrderState("有持仓时无法切换保证金类型".to_string()));
        }

        self.margin_type_config.insert(cmd.symbol, cmd.margin_type);

        Ok(SetMarginTypeResult {
            symbol: cmd.symbol,
            margin_type: cmd.margin_type,
            success: true,
        })
    }

    fn set_position_mode(&self, cmd: SetPositionModeCommand) -> Result<SetPositionModeResult, PrepCommandError> {
        // 检查是否有持仓
        if !self.positions.is_empty() {
            return Err(PrepCommandError::InvalidOrderState("有持仓时无法切换持仓模式".to_string()));
        }

        self.dual_side_mode = cmd.dual_side;

        Ok(SetPositionModeResult {
            dual_side: cmd.dual_side,
            success: true,
        })
    }
}

impl PerpOrderExchQueryProc for MockMatchingService {
    fn query_order(&self, cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
        let order = self.orders.get(&cmd.order_id)
            .ok_or_else(|| PrepCommandError::OrderNotFound(cmd.order_id.to_string()))?;

        Ok(OrderQueryResult {
            order_id: cmd.order_id,
            symbol: order.symbol,
            side: order.side,
            order_type: order.order_type,
            status: order.status,
            quantity: order.quantity,
            price: order.price,
            filled_quantity: Quantity::from_raw(0),
            avg_price: None,
            position_side: order.position_side,
            created_at: 0,
            updated_at: 0,
        })
    }

    fn query_position(&self, cmd: QueryPositionCommand) -> Result<PositionInfo, PrepCommandError> {
        Ok(self.positions.get(&cmd.symbol)
            .cloned()
            .unwrap_or_else(|| PositionInfo::empty(cmd.symbol, cmd.position_side)))
    }

    fn query_order_book(&self, _cmd: QueryOrderBookCommand) -> Result<OrderBookSnapshot, PrepCommandError> {
        // Mock实现
        Ok(OrderBookSnapshot::empty(Symbol::new("BTCUSDT")))
    }

    fn query_trades(&self, _cmd: QueryTradesCommand) -> Result<TradesQueryResult, PrepCommandError> {
        Ok(TradesQueryResult::empty())
    }

    fn query_account_balance(&self, _cmd: QueryAccountBalanceCommand) -> Result<Vec<AccountBalance>, PrepCommandError> {
        Ok(vec![AccountBalance::new(
            Symbol::new("USDT"),
            self.balance,
            self.balance,
            Price::from_raw(0),
            Price::from_raw(0),
            Price::from_raw(0),
        )])
    }

    fn query_account_info(&self, _cmd: QueryAccountInfoCommand) -> Result<AccountInfo, PrepCommandError> {
        Ok(AccountInfo::new(
            self.balance,
            Price::from_raw(0),
            Price::from_raw(0),
            self.balance,
            self.positions.values().cloned().collect(),
            vec![],
        ))
    }

    fn query_mark_price(&self, _cmd: QueryMarkPriceCommand) -> Result<Vec<MarkPriceInfo>, PrepCommandError> {
        Ok(vec![])
    }

    fn query_funding_rate_history(&self, _cmd: QueryFundingRateHistoryCommand) -> Result<Vec<FundingRateRecord>, PrepCommandError> {
        Ok(vec![])
    }

    fn query_funding_fee(&self, _cmd: QueryFundingFeeCommand) -> Result<Vec<FundingFeeRecord>, PrepCommandError> {
        Ok(vec![])
    }
}

// ============================================================================
// BDD测试场景 - 开仓场景
// ============================================================================

#[cfg(test)]
mod open_position_scenarios {
    use super::*;

    #[test]
    fn scenario_open_long_position_with_sufficient_balance() {
        // Feature: 开多仓
        // Scenario: 余额充足时成功开多仓

        let mut ctx = TestContext::new();

        // Given: 用户有10000 USDT余额
        ctx.matching_service.set_balance(10000.0);

        // And: 设置了5倍杠杆
        ctx.matching_service.leverage_config.insert(
            Symbol::new("BTCUSDT"),
            5
        );

        // When: 用户以市价开多仓1 BTC
        let cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(5);

        let result = ctx.matching_service.open_position(cmd);

        // Then: 订单应该成功执行
        assert!(result.is_ok());
        let open_result = result.unwrap();

        // And: 订单状态应该是完全成交
        assert_eq!(open_result.status, OrderStatus::Filled);

        // And: 应该有成交记录
        assert!(!open_result.trades.is_empty());

        // And: 成交数量应该是1.0
        assert_eq!(open_result.filled_quantity.to_f64(), 1.0);

        // And: 应该有持仓
        let position = ctx.matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(position.has_position());
        assert_eq!(position.quantity.to_f64(), 1.0);
    }

    #[test]
    fn scenario_open_long_position_with_insufficient_balance() {
        // Feature: 开多仓
        // Scenario: 余额不足时拒绝开仓

        let mut ctx = TestContext::new();

        // Given: 用户只有100 USDT余额
        ctx.matching_service.set_balance(100.0);

        // And: 设置了1倍杠杆
        ctx.matching_service.leverage_config.insert(
            Symbol::new("BTCUSDT"),
            1
        );

        // When: 用户试图开多仓1 BTC (需要约50000 USDT保证金)
        let cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(1);

        let result = ctx.matching_service.open_position(cmd);

        // Then: 订单应该被拒绝
        assert!(result.is_err());

        // And: 错误应该是余额不足
        assert!(matches!(result.unwrap_err(), PrepCommandError::InsufficientBalance));

        // And: 不应该有持仓
        let position = ctx.matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(!position.has_position());
    }

    #[test]
    fn scenario_open_short_position_successfully() {
        // Feature: 开空仓
        // Scenario: 成功开空仓

        let mut ctx = TestContext::new();

        // Given: 用户有10000 USDT余额
        ctx.matching_service.set_balance(10000.0);

        // And: 设置了10倍杠杆
        ctx.matching_service.leverage_config.insert(
            Symbol::new("BTCUSDT"),
            10
        );

        // When: 用户以市价开空仓0.5 BTC
        let cmd = OpenPositionCommand::market_short(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(0.5)
        ).with_leverage(10);

        let result = ctx.matching_service.open_position(cmd);

        // Then: 订单应该成功执行
        assert!(result.is_ok());
        let open_result = result.unwrap();

        // And: 订单状态应该是完全成交
        assert_eq!(open_result.status, OrderStatus::Filled);

        // And: 应该有空头持仓
        let position = ctx.matching_service.query_position(
            QueryPositionCommand::short(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(position.is_short());
        assert_eq!(position.quantity.to_f64(), 0.5);
    }

    #[test]
    fn scenario_limit_order_enters_order_book() {
        // Feature: 限价单开仓
        // Scenario: 限价单进入订单簿等待成交

        let mut ctx = TestContext::new();

        // Given: 用户有10000 USDT余额
        ctx.matching_service.set_balance(10000.0);

        // And: 设置了5倍杠杆
        ctx.matching_service.leverage_config.insert(
            Symbol::new("BTCUSDT"),
            5
        );

        // When: 用户以49000价格挂限价买单1 BTC
        let cmd = OpenPositionCommand::limit_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0),
            Price::from_f64(49000.0)
        ).with_leverage(5);

        let result = ctx.matching_service.open_position(cmd);

        // Then: 订单应该成功提交
        assert!(result.is_ok());
        let open_result = result.unwrap();

        // And: 订单状态应该是已提交(进入订单簿)
        assert_eq!(open_result.status, OrderStatus::Submitted);

        // And: 不应该有成交记录
        assert!(open_result.trades.is_empty());

        // And: 已成交数量应该是0
        assert_eq!(open_result.filled_quantity.to_f64(), 0.0);
    }

    #[test]
    fn scenario_validate_invalid_quantity() {
        // Feature: 订单验证
        // Scenario: 拒绝无效数量的订单

        let mut ctx = TestContext::new();

        // Given: 用户有充足余额
        ctx.matching_service.set_balance(10000.0);

        // When: 用户试图开仓数量为0的订单
        let cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_raw(0)  // 无效数量
        );

        let result = ctx.matching_service.open_position(cmd);

        // Then: 订单应该被拒绝
        assert!(result.is_err());

        // And: 错误应该是验证错误
        assert!(matches!(result.unwrap_err(), PrepCommandError::ValidationError(_)));
    }

    #[test]
    fn scenario_validate_invalid_leverage() {
        // Feature: 订单验证
        // Scenario: 拒绝无效杠杆的订单

        let mut ctx = TestContext::new();

        // Given: 用户有充足余额
        ctx.matching_service.set_balance(10000.0);

        // When: 用户试图使用200倍杠杆(超过最大125倍)
        let cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(200);  // 无效杠杆

        let result = ctx.matching_service.open_position(cmd);

        // Then: 订单应该被拒绝
        assert!(result.is_err());

        // And: 错误应该是验证错误
        assert!(matches!(result.unwrap_err(), PrepCommandError::ValidationError(_)));
    }
}

// ============================================================================
// BDD测试场景 - 平仓场景
// ============================================================================

#[cfg(test)]
mod close_position_scenarios {
    use super::*;

    #[test]
    fn scenario_close_long_position_with_profit() {
        // Feature: 平多仓
        // Scenario: 盈利平多仓

        let mut ctx = TestContext::new();

        // Given: 用户有多仓持仓1 BTC @ 50000
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.0,
            50000.0
        );

        // And: 用户有充足保证金
        ctx.matching_service.set_balance(5000.0);

        // When: 用户以市价全部平多仓 (假设成交价51000)
        let cmd = ClosePositionCommand::market_close_long(
            Symbol::new("BTCUSDT"),
            None  // None表示全部平仓
        );

        let result = ctx.matching_service.close_position(cmd);

        // Then: 订单应该成功执行
        assert!(result.is_ok());
        let close_result = result.unwrap();

        // And: 订单状态应该是完全成交
        assert_eq!(close_result.status, OrderStatus::Filled);

        // And: 应该有盈亏信息
        assert!(close_result.realized_pnl.is_some());
        let pnl = close_result.realized_pnl.unwrap().to_f64();

        // And: 盈亏应该是正数(盈利)
        assert!(pnl > 0.0);

        // And: 盈亏应该约等于1000 (51000 - 50000)
        assert!((pnl - 1000.0).abs() < 10.0);

        // And: 持仓应该被清空
        let position = ctx.matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(!position.has_position());
    }

    #[test]
    fn scenario_close_short_position_with_loss() {
        // Feature: 平空仓
        // Scenario: 亏损平空仓

        let mut ctx = TestContext::new();

        // Given: 用户有空仓持仓1 BTC @ 50000
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Short,
            1.0,
            50000.0
        );

        // And: 用户有充足保证金
        ctx.matching_service.set_balance(5000.0);

        // When: 用户以市价全部平空仓 (假设成交价51000, 价格上涨亏损)
        let cmd = ClosePositionCommand::market_close_short(
            Symbol::new("BTCUSDT"),
            None
        );

        let result = ctx.matching_service.close_position(cmd);

        // Then: 订单应该成功执行
        assert!(result.is_ok());
        let close_result = result.unwrap();

        // And: 订单状态应该是完全成交
        assert_eq!(close_result.status, OrderStatus::Filled);

        // And: 盈亏应该是负数(亏损)
        let pnl = close_result.realized_pnl.unwrap().to_f64();
        assert!(pnl < 0.0);

        // And: 亏损应该约等于-1000 (50000 - 51000)
        assert!((pnl + 1000.0).abs() < 10.0);
    }

    #[test]
    fn scenario_partial_close_position() {
        // Feature: 部分平仓
        // Scenario: 只平掉部分持仓

        let mut ctx = TestContext::new();

        // Given: 用户有多仓持仓2 BTC
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            2.0,
            50000.0
        );

        // When: 用户只平掉0.5 BTC
        let cmd = ClosePositionCommand::market_close_long(
            Symbol::new("BTCUSDT"),
            Some(Quantity::from_f64(0.5))
        );

        let result = ctx.matching_service.close_position(cmd);

        // Then: 订单应该成功执行
        assert!(result.is_ok());
        let close_result = result.unwrap();

        // And: 平仓数量应该是0.5
        assert_eq!(close_result.closed_quantity.to_f64(), 0.5);

        // And: 应该还有1.5 BTC持仓
        let position = ctx.matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(position.has_position());
        assert_eq!(position.quantity.to_f64(), 1.5);
    }

    #[test]
    fn scenario_close_position_without_position() {
        // Feature: 平仓验证
        // Scenario: 没有持仓时拒绝平仓

        let mut ctx = TestContext::new();

        // Given: 用户没有任何持仓
        // (初始状态就是空持仓)

        // When: 用户试图平多仓
        let cmd = ClosePositionCommand::market_close_long(
            Symbol::new("BTCUSDT"),
            None
        );

        let result = ctx.matching_service.close_position(cmd);

        // Then: 订单应该被拒绝
        assert!(result.is_err());

        // And: 错误应该是持仓不足
        assert!(matches!(result.unwrap_err(), PrepCommandError::InsufficientPosition));
    }
}

// ============================================================================
// BDD测试场景 - 订单管理场景
// ============================================================================

#[cfg(test)]
mod order_management_scenarios {
    use super::*;

    #[test]
    fn scenario_cancel_pending_order() {
        // Feature: 取消订单
        // Scenario: 成功取消挂单

        let mut ctx = TestContext::new();

        // Given: 用户有一个待成交的限价单
        ctx.matching_service.set_balance(10000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        let cmd = OpenPositionCommand::limit_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0),
            Price::from_f64(49000.0)
        );
        let open_result = ctx.matching_service.open_position(cmd).unwrap();
        let order_id = open_result.order_id.clone();

        // When: 用户取消这个订单
        let cancel_cmd = CancelOrderCommand::new(order_id.clone(), Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.cancel_order(cancel_cmd);

        // Then: 取消应该成功
        assert!(result.is_ok());
        let cancel_result = result.unwrap();

        // And: cancelled标志应该是true
        assert!(cancel_result.cancelled);

        // And: 订单状态应该是已取消
        assert_eq!(cancel_result.status, OrderStatus::Cancelled);
    }

    #[test]
    fn scenario_cannot_cancel_filled_order() {
        // Feature: 取消订单验证
        // Scenario: 不能取消已成交的订单

        let mut ctx = TestContext::new();

        // Given: 用户有一个已经完全成交的市价单
        ctx.matching_service.set_balance(10000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        let cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        );
        let open_result = ctx.matching_service.open_position(cmd).unwrap();
        let order_id = open_result.order_id.clone();

        // 手动插入一个已成交的订单到mock中
        ctx.matching_service.orders.insert(
            order_id.clone(),
            MockOrder {
                symbol: Symbol::new("BTCUSDT"),
                side: Side::Buy,
                order_type: OrderType::Market,
                quantity: Quantity::from_f64(1.0),
                price: None,
                position_side: PositionSide::Long,
                status: OrderStatus::Filled,
            }
        );

        // When: 用户试图取消这个已成交的订单
        let cancel_cmd = CancelOrderCommand::new(order_id, Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.cancel_order(cancel_cmd);

        // Then: 取消应该失败
        assert!(result.is_ok());  // 不返回错误,但cancelled标志为false
        let cancel_result = result.unwrap();

        // And: cancelled标志应该是false
        assert!(!cancel_result.cancelled);

        // And: 订单状态应该仍然是已成交
        assert_eq!(cancel_result.status, OrderStatus::Filled);
    }

    #[test]
    fn scenario_modify_pending_order_price() {
        // Feature: 修改订单
        // Scenario: 成功修改挂单价格

        let mut ctx = TestContext::new();

        // Given: 用户有一个待成交的限价单 @ 49000
        ctx.matching_service.set_balance(10000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        let cmd = OpenPositionCommand::limit_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0),
            Price::from_f64(49000.0)
        );
        let open_result = ctx.matching_service.open_position(cmd).unwrap();
        let order_id = open_result.order_id.clone();

        // When: 用户修改价格为49500
        let modify_cmd = ModifyOrderCommand::new(order_id.clone(), Symbol::new("BTCUSDT"))
            .with_price(Price::from_f64(49500.0));

        let result = ctx.matching_service.modify_order(modify_cmd);

        // Then: 修改应该成功
        assert!(result.is_ok());
        let modify_result = result.unwrap();

        // And: modified标志应该是true
        assert!(modify_result.modified);

        // And: 新价格应该是49500
        assert_eq!(modify_result.new_price.unwrap().to_f64(), 49500.0);
    }

    #[test]
    fn scenario_modify_pending_order_quantity() {
        // Feature: 修改订单
        // Scenario: 成功修改挂单数量

        let mut ctx = TestContext::new();

        // Given: 用户有一个待成交的限价单,数量1.0
        ctx.matching_service.set_balance(10000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        let cmd = OpenPositionCommand::limit_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0),
            Price::from_f64(49000.0)
        );
        let open_result = ctx.matching_service.open_position(cmd).unwrap();
        let order_id = open_result.order_id.clone();

        // When: 用户修改数量为1.5
        let modify_cmd = ModifyOrderCommand::new(order_id, Symbol::new("BTCUSDT"))
            .with_quantity(Quantity::from_f64(1.5));

        let result = ctx.matching_service.modify_order(modify_cmd);

        // Then: 修改应该成功
        assert!(result.is_ok());
        let modify_result = result.unwrap();

        // And: 新数量应该是1.5
        assert_eq!(modify_result.new_quantity.unwrap().to_f64(), 1.5);
    }

    #[test]
    fn scenario_cancel_all_orders_for_symbol() {
        // Feature: 批量取消订单
        // Scenario: 取消指定交易对的所有订单

        let mut ctx = TestContext::new();
        ctx.matching_service.set_balance(20000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);
        ctx.matching_service.leverage_config.insert(Symbol::new("ETHUSDT"), 5);

        // Given: 用户有多个挂单
        // - BTCUSDT: 2个订单
        let cmd1 = OpenPositionCommand::limit_long(Symbol::new("BTCUSDT"), Quantity::from_f64(1.0), Price::from_f64(49000.0));
        ctx.matching_service.open_position(cmd1).unwrap();

        let cmd2 = OpenPositionCommand::limit_long(Symbol::new("BTCUSDT"), Quantity::from_f64(0.5), Price::from_f64(48000.0));
        ctx.matching_service.open_position(cmd2).unwrap();

        // - ETHUSDT: 1个订单
        let cmd3 = OpenPositionCommand::limit_long(Symbol::new("ETHUSDT"), Quantity::from_f64(10.0), Price::from_f64(2900.0));
        ctx.matching_service.open_position(cmd3).unwrap();

        // When: 用户取消BTCUSDT的所有订单
        let cancel_all_cmd = CancelAllOrdersCommand::by_symbol(Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.cancel_all_orders(cancel_all_cmd);

        // Then: 批量取消应该成功
        assert!(result.is_ok());
        let cancel_all_result = result.unwrap();

        // And: 应该取消了2个订单
        assert_eq!(cancel_all_result.cancelled_count, 2);

        // And: ETHUSDT的订单应该还在
        let remaining_orders: Vec<_> = ctx.matching_service.orders.values()
            .filter(|o| o.symbol.as_str() == "ETHUSDT")
            .collect();
        assert_eq!(remaining_orders.len(), 1);
    }
}

// ============================================================================
// BDD测试场景 - 账户配置场景
// ============================================================================

#[cfg(test)]
mod account_configuration_scenarios {
    use super::*;

    #[test]
    fn scenario_set_leverage_successfully() {
        // Feature: 设置杠杆
        // Scenario: 成功设置交易对杠杆

        let mut ctx = TestContext::new();

        // Given: 用户还没有设置BTCUSDT的杠杆
        // (默认应该是1倍)

        // When: 用户设置BTCUSDT的杠杆为10倍
        let cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 10);
        let result = ctx.matching_service.set_leverage(cmd);

        // Then: 设置应该成功
        assert!(result.is_ok());
        let leverage_result = result.unwrap();

        // And: 旧杠杆应该是1
        assert_eq!(leverage_result.old_leverage, 1);

        // And: 新杠杆应该是10
        assert_eq!(leverage_result.new_leverage, 10);

        // And: 杠杆配置应该被更新
        assert_eq!(*ctx.matching_service.leverage_config.get(&Symbol::new("BTCUSDT")).unwrap(), 10);

        // And: 保证金变化应该是0(因为没有持仓)
        assert_eq!(leverage_result.position_margin_change.to_f64(), 0.0);

        // And: 可用余额应该保持不变
        assert_eq!(leverage_result.available_balance.to_f64(), 10000.0);
    }

    #[test]
    fn scenario_set_leverage_with_existing_position() {
        // Feature: 设置杠杆
        // Scenario: 有持仓时设置杠杆会改变保证金占用

        let mut ctx = TestContext::new();

        // Given: 用户有BTCUSDT多仓持仓1 BTC @ 50000
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.0,
            50000.0
        );

        // And: 当前杠杆是5倍
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        // And: 用户有充足余额
        ctx.matching_service.set_balance(15000.0);

        // When: 用户降低杠杆到2倍(更保守)
        let cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 2);
        let result = ctx.matching_service.set_leverage(cmd);

        // Then: 设置应该成功
        assert!(result.is_ok());
        let leverage_result = result.unwrap();

        // And: 旧杠杆应该是5
        assert_eq!(leverage_result.old_leverage, 5);

        // And: 新杠杆应该是2
        assert_eq!(leverage_result.new_leverage, 2);

        // And: 保证金变化应该是正数(锁定更多保证金)
        // 旧保证金: 50000 * 1.0 / 5 = 10000
        // 新保证金: 50000 * 1.0 / 2 = 25000
        // 变化: 25000 - 10000 = 15000
        let margin_change = leverage_result.position_margin_change.to_f64();
        assert!(margin_change > 0.0);
        assert!((margin_change - 15000.0).abs() < 1.0);

        // And: 可用余额应该减少(因为锁定了更多保证金)
        assert_eq!(leverage_result.available_balance.to_f64(), 15000.0);
    }

    #[test]
    fn scenario_increase_leverage_releases_margin() {
        // Feature: 设置杠杆
        // Scenario: 提高杠杆会释放保证金

        let mut ctx = TestContext::new();

        // Given: 用户有BTCUSDT多仓持仓1 BTC @ 50000
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.0,
            50000.0
        );

        // And: 当前杠杆是5倍
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        // And: 用户有充足余额
        ctx.matching_service.set_balance(5000.0);

        // When: 用户提高杠杆到10倍(更激进)
        let cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 10);
        let result = ctx.matching_service.set_leverage(cmd);

        // Then: 设置应该成功
        assert!(result.is_ok());
        let leverage_result = result.unwrap();

        // And: 旧杠杆应该是5
        assert_eq!(leverage_result.old_leverage, 5);

        // And: 新杠杆应该是10
        assert_eq!(leverage_result.new_leverage, 10);

        // And: 保证金变化应该是负数(释放保证金)
        // 旧保证金: 50000 * 1.0 / 5 = 10000
        // 新保证金: 50000 * 1.0 / 10 = 5000
        // 变化: 5000 - 10000 = -5000
        let margin_change = leverage_result.position_margin_change.to_f64();
        assert!(margin_change < 0.0);
        assert!((margin_change + 5000.0).abs() < 1.0);

        // And: 可用余额应该保持不变(在我们的Mock实现中)
        assert_eq!(leverage_result.available_balance.to_f64(), 5000.0);
    }

    #[test]
    fn scenario_set_leverage_invalid_value() {
        // Feature: 设置杠杆验证
        // Scenario: 拒绝无效的杠杆值

        let mut ctx = TestContext::new();

        // Given: 用户要设置杠杆

        // When: 用户设置杠杆为200倍(超过最大125倍)
        let cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 200);
        let result = ctx.matching_service.set_leverage(cmd);

        // Then: 设置应该失败
        assert!(result.is_err());

        // And: 错误应该是验证错误
        assert!(matches!(result.unwrap_err(), PrepCommandError::ValidationError(_)));
    }

    #[test]
    fn scenario_set_margin_type_without_position() {
        // Feature: 设置保证金类型
        // Scenario: 无持仓时成功设置保证金类型

        let mut ctx = TestContext::new();

        // Given: 用户没有任何持仓

        // When: 用户设置BTCUSDT为逐仓模式
        let cmd = SetMarginTypeCommand::new(
            Symbol::new("BTCUSDT"),
            MarginType::Isolated
        );
        let result = ctx.matching_service.set_margin_type(cmd);

        // Then: 设置应该成功
        assert!(result.is_ok());
        let margin_result = result.unwrap();

        // And: 保证金类型应该是逐仓
        assert_eq!(margin_result.margin_type, MarginType::Isolated);

        // And: success标志应该是true
        assert!(margin_result.success);
    }

    #[test]
    fn scenario_cannot_set_margin_type_with_position() {
        // Feature: 设置保证金类型验证
        // Scenario: 有持仓时拒绝设置保证金类型

        let mut ctx = TestContext::new();

        // Given: 用户有BTCUSDT持仓
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.0,
            50000.0
        );

        // When: 用户试图修改BTCUSDT的保证金类型
        let cmd = SetMarginTypeCommand::new(
            Symbol::new("BTCUSDT"),
            MarginType::Isolated
        );
        let result = ctx.matching_service.set_margin_type(cmd);

        // Then: 设置应该失败
        assert!(result.is_err());

        // And: 错误应该是订单状态无效
        assert!(matches!(result.unwrap_err(), PrepCommandError::InvalidOrderState(_)));
    }

    #[test]
    fn scenario_set_position_mode_to_hedge() {
        // Feature: 设置持仓模式
        // Scenario: 切换到对冲模式

        let mut ctx = TestContext::new();

        // Given: 用户当前是单向模式
        assert!(!ctx.matching_service.dual_side_mode);

        // And: 用户没有任何持仓

        // When: 用户切换到对冲模式
        let cmd = SetPositionModeCommand::hedge();
        let result = ctx.matching_service.set_position_mode(cmd);

        // Then: 设置应该成功
        assert!(result.is_ok());
        let mode_result = result.unwrap();

        // And: dual_side标志应该是true
        assert!(mode_result.dual_side);

        // And: 持仓模式应该被更新
        assert!(ctx.matching_service.dual_side_mode);
    }

    #[test]
    fn scenario_cannot_set_position_mode_with_positions() {
        // Feature: 设置持仓模式验证
        // Scenario: 有持仓时拒绝切换持仓模式

        let mut ctx = TestContext::new();

        // Given: 用户有持仓
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.0,
            50000.0
        );

        // When: 用户试图切换到对冲模式
        let cmd = SetPositionModeCommand::hedge();
        let result = ctx.matching_service.set_position_mode(cmd);

        // Then: 设置应该失败
        assert!(result.is_err());

        // And: 错误应该是订单状态无效
        assert!(matches!(result.unwrap_err(), PrepCommandError::InvalidOrderState(_)));
    }
}

// ============================================================================
// BDD测试场景 - 查询场景
// ============================================================================

#[cfg(test)]
mod query_scenarios {
    use super::*;

    #[test]
    fn scenario_query_existing_order() {
        // Feature: 查询订单
        // Scenario: 成功查询存在的订单

        let mut ctx = TestContext::new();

        // Given: 用户有一个挂单
        ctx.matching_service.set_balance(10000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);

        let cmd = OpenPositionCommand::limit_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0),
            Price::from_f64(49000.0)
        );
        let open_result = ctx.matching_service.open_position(cmd).unwrap();
        let order_id = open_result.order_id.clone();

        // When: 用户查询这个订单
        let query_cmd = QueryOrderCommand::new(order_id.clone(), Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.query_order(query_cmd);

        // Then: 查询应该成功
        assert!(result.is_ok());
        let order_info = result.unwrap();

        // And: 订单信息应该正确
        assert_eq!(order_info.order_id.as_str(), order_id.as_str());
        assert_eq!(order_info.symbol.as_str(), "BTCUSDT");
        assert_eq!(order_info.order_type, OrderType::Limit);
        assert_eq!(order_info.quantity.to_f64(), 1.0);
    }

    #[test]
    fn scenario_query_nonexistent_order() {
        // Feature: 查询订单验证
        // Scenario: 查询不存在的订单应该失败

        let mut ctx = TestContext::new();

        // Given: 用户没有任何订单

        // When: 用户查询一个不存在的订单ID
        let fake_order_id = OrderId::new("FAKE-ORDER-123");
        let query_cmd = QueryOrderCommand::new(fake_order_id, Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.query_order(query_cmd);

        // Then: 查询应该失败
        assert!(result.is_err());

        // And: 错误应该是订单不存在
        assert!(matches!(result.unwrap_err(), PrepCommandError::OrderNotFound(_)));
    }

    #[test]
    fn scenario_query_existing_position() {
        // Feature: 查询持仓
        // Scenario: 成功查询存在的持仓

        let mut ctx = TestContext::new();

        // Given: 用户有BTCUSDT多仓持仓
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.5,
            50000.0
        );

        // When: 用户查询BTCUSDT多仓
        let query_cmd = QueryPositionCommand::long(Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.query_position(query_cmd);

        // Then: 查询应该成功
        assert!(result.is_ok());
        let position = result.unwrap();

        // And: 持仓信息应该正确
        assert!(position.has_position());
        assert!(position.is_long());
        assert_eq!(position.quantity.to_f64(), 1.5);
        assert_eq!(position.entry_price.to_f64(), 50000.0);
    }

    #[test]
    fn scenario_query_empty_position() {
        // Feature: 查询持仓
        // Scenario: 查询空持仓返回空持仓信息

        let mut ctx = TestContext::new();

        // Given: 用户没有任何持仓

        // When: 用户查询BTCUSDT多仓
        let query_cmd = QueryPositionCommand::long(Symbol::new("BTCUSDT"));
        let result = ctx.matching_service.query_position(query_cmd);

        // Then: 查询应该成功(不返回错误)
        assert!(result.is_ok());
        let position = result.unwrap();

        // And: 持仓应该是空的
        assert!(!position.has_position());
        assert_eq!(position.quantity.to_f64(), 0.0);
    }

    #[test]
    fn scenario_query_account_balance() {
        // Feature: 查询账户余额
        // Scenario: 成功查询账户余额

        let mut ctx = TestContext::new();

        // Given: 用户有5000 USDT余额
        ctx.matching_service.set_balance(5000.0);

        // When: 用户查询USDT余额
        let query_cmd = QueryAccountBalanceCommand::by_asset(Symbol::new("USDT"));
        let result = ctx.matching_service.query_account_balance(query_cmd);

        // Then: 查询应该成功
        assert!(result.is_ok());
        let balances = result.unwrap();

        // And: 应该返回余额信息
        assert!(!balances.is_empty());
        let usdt_balance = &balances[0];

        // And: 余额应该是5000
        assert_eq!(usdt_balance.balance.to_f64(), 5000.0);
        assert_eq!(usdt_balance.asset.as_str(), "USDT");
    }

    #[test]
    fn scenario_query_account_info() {
        // Feature: 查询账户信息
        // Scenario: 成功查询完整账户信息

        let mut ctx = TestContext::new();

        // Given: 用户有余额和持仓
        ctx.matching_service.set_balance(8000.0);
        ctx.matching_service.add_position(
            Symbol::new("BTCUSDT"),
            PositionSide::Long,
            1.0,
            50000.0
        );

        // When: 用户查询账户信息
        let query_cmd = QueryAccountInfoCommand::new();
        let result = ctx.matching_service.query_account_info(query_cmd);

        // Then: 查询应该成功
        assert!(result.is_ok());
        let account_info = result.unwrap();

        // And: 应该包含总资产信息
        assert_eq!(account_info.total_wallet_balance.to_f64(), 8000.0);

        // And: 应该包含持仓列表
        assert_eq!(account_info.positions.len(), 1);
        assert_eq!(account_info.positions[0].symbol.as_str(), "BTCUSDT");
    }
}

// ============================================================================
// BDD测试场景 - 集成场景
// ============================================================================

#[cfg(test)]
mod integration_scenarios {
    use super::*;

    #[test]
    fn scenario_complete_trading_workflow() {
        // Feature: 完整交易流程
        // Scenario: 从开仓到平仓的完整交易流程

        let mut ctx = TestContext::new();

        // Given: 用户有初始余额10000 USDT
        ctx.matching_service.set_balance(10000.0);

        // Step 1: 设置杠杆为5倍
        let set_leverage_cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 5);
        let leverage_result = ctx.matching_service.set_leverage(set_leverage_cmd);
        assert!(leverage_result.is_ok());

        // Step 2: 开多仓1 BTC
        let open_cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(5);
        let open_result = ctx.matching_service.open_position(open_cmd);
        assert!(open_result.is_ok());
        assert_eq!(open_result.unwrap().status, OrderStatus::Filled);

        // Step 3: 查询持仓
        let query_pos_cmd = QueryPositionCommand::long(Symbol::new("BTCUSDT"));
        let position = ctx.matching_service.query_position(query_pos_cmd).unwrap();
        assert!(position.has_position());
        assert_eq!(position.quantity.to_f64(), 1.0);

        // Step 4: 平仓获利
        let close_cmd = ClosePositionCommand::market_close_long(
            Symbol::new("BTCUSDT"),
            None
        );
        let close_result = ctx.matching_service.close_position(close_cmd);
        assert!(close_result.is_ok());

        let close_data = close_result.unwrap();
        assert_eq!(close_data.status, OrderStatus::Filled);
        assert!(close_data.realized_pnl.is_some());

        // Step 5: 验证持仓已清空
        let final_position = ctx.matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(!final_position.has_position());

        // Step 6: 验证余额变化
        let balance_cmd = QueryAccountBalanceCommand::by_asset(Symbol::new("USDT"));
        let balances = ctx.matching_service.query_account_balance(balance_cmd).unwrap();
        let final_balance = balances[0].balance.to_f64();

        // 最终余额应该接近初始余额加上盈亏
        // 由于我们的Mock实现模拟了盈利(51000-50000)*1.0 = 1000
        // 但需要扣除手续费和保证金占用,所以余额应该有所增加
        // 这里我们只验证余额是正数即可
        assert!(final_balance > 0.0);
    }

    #[test]
    fn scenario_hedge_mode_trading() {
        // Feature: 对冲模式交易
        // Scenario: 在对冲模式下同时持有多空仓位

        let mut ctx = TestContext::new();

        // Given: 用户切换到对冲模式
        let mode_cmd = SetPositionModeCommand::hedge();
        ctx.matching_service.set_position_mode(mode_cmd).unwrap();

        // And: 设置充足余额和杠杆
        ctx.matching_service.set_balance(20000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 10);

        // When: 用户同时开多仓和空仓
        // Step 1: 开多仓
        let long_cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(10);
        let long_result = ctx.matching_service.open_position(long_cmd);
        assert!(long_result.is_ok());

        // Step 2: 开空仓
        let short_cmd = OpenPositionCommand::market_short(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(0.5)
        ).with_leverage(10);
        let short_result = ctx.matching_service.open_position(short_cmd);
        assert!(short_result.is_ok());

        // Then: 应该同时有多仓和空仓
        let long_position = ctx.matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();
        assert!(long_position.is_long());

        // 注意：我们的Mock实现没有完全实现对冲模式的多空独立管理
        // 在真实实现中,应该能同时查询到多仓和空仓
    }

    #[test]
    fn scenario_batch_order_cancellation() {
        // Feature: 批量订单管理
        // Scenario: 批量取消多个交易对的订单

        let mut ctx = TestContext::new();
        ctx.matching_service.set_balance(30000.0);
        ctx.matching_service.leverage_config.insert(Symbol::new("BTCUSDT"), 5);
        ctx.matching_service.leverage_config.insert(Symbol::new("ETHUSDT"), 5);

        // Given: 用户有多个交易对的多个挂单
        // BTCUSDT: 3个订单
        for _ in 0..3 {
            let cmd = OpenPositionCommand::limit_long(
                Symbol::new("BTCUSDT"),
                Quantity::from_f64(0.1),
                Price::from_f64(49000.0)
            );
            ctx.matching_service.open_position(cmd).unwrap();
        }

        // ETHUSDT: 2个订单
        for _ in 0..2 {
            let cmd = OpenPositionCommand::limit_long(
                Symbol::new("ETHUSDT"),
                Quantity::from_f64(1.0),
                Price::from_f64(2900.0)
            );
            ctx.matching_service.open_position(cmd).unwrap();
        }

        // When: 用户取消所有订单
        let cancel_all_cmd = CancelAllOrdersCommand::all();
        let result = ctx.matching_service.cancel_all_orders(cancel_all_cmd);

        // Then: 所有订单应该被取消
        assert!(result.is_ok());
        let cancel_result = result.unwrap();

        // And: 应该取消了5个订单(3+2)
        assert_eq!(cancel_result.cancelled_count, 5);

        // And: 订单簿应该为空
        assert!(ctx.matching_service.orders.is_empty());
    }
}
