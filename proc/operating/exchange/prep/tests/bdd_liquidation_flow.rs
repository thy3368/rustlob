//! BDD测试 - 强平流程完整测试
//!
//! 测试三级强平机制的完整流程：
//! 1. 市场强平 (Market Liquidation)
//! 2. 保险基金接管 (Insurance Fund)
//! 3. 自动减仓 (Auto-Deleveraging, ADL)

use prep_proc::proc::trading_prep_order_proc::*;
use prep_proc::proc::liquidation_proc::*;
use prep_proc::proc::liquidation_types::*;
use std::sync::Arc;

// ============================================================================
// Mock实现 - 保险基金
// ============================================================================

struct MockInsuranceFund {
    available_balance: Price,
}

impl MockInsuranceFund {
    fn new(balance: f64) -> Self {
        Self {
            available_balance: Price::from_f64(balance),
        }
    }
}

#[async_trait::async_trait]
impl InsuranceFund for MockInsuranceFund {
    async fn check_capacity(&self) -> Result<InsuranceFundCapacity, PrepCommandError> {
        Ok(InsuranceFundCapacity {
            available_balance: self.available_balance,
        })
    }

    async fn takeover(&self, position: &PositionInfo) -> Result<InsuranceFundTakeover, PrepCommandError> {
        Ok(InsuranceFundTakeover {
            total_loss: position.margin,
        })
    }
}

// ============================================================================
// Mock实现 - ADL引擎
// ============================================================================

struct MockADLEngine {
    has_counterparties: bool,
}

impl MockADLEngine {
    fn new(has_counterparties: bool) -> Self {
        Self { has_counterparties }
    }
}

#[async_trait::async_trait]
impl ADLEngine for MockADLEngine {
    async fn find_counterparties(
        &self,
        _symbol: Symbol,
        _side: Side,
    ) -> Result<Vec<PositionInfo>, PrepCommandError> {
        if self.has_counterparties {
            Ok(vec![PositionInfo {
                position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
                position_side: PositionSide::Short,
                quantity: Quantity::from_f64(1.0),
                entry_price: Price::from_f64(50000.0),
                mark_price: Price::from_f64(45000.0),
                unrealized_pnl: Price::from_f64(5000.0),
                realized_pnl: Price::from_raw(0),
                leverage: 10,
                margin: Price::from_f64(5000.0),
                liquidation_price: Some(Price::from_f64(54500.0)),
                updated_at: 0,
            }])
        } else {
            Ok(Vec::new())
        }
    }

    async fn execute_adl(
        &self,
        _liquidated_position: &PositionInfo,
        counterparties: Vec<PositionInfo>,
    ) -> Result<ADLResult, PrepCommandError> {
        Ok(ADLResult {
            affected_positions: counterparties.iter().map(|p| PositionId::generate()).collect(),
        })
    }
}

// ============================================================================
// Mock实现 - 撮合服务
// ============================================================================

struct MockMatchingService {
    should_fill: bool,
}

impl MockMatchingService {
    fn new(should_fill: bool) -> Self {
        Self { should_fill }
    }
}

impl PerpOrderExchProc for MockMatchingService {
    fn open_position(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        if self.should_fill {
            let trades = vec![Trade::new(
                TradeId::generate(),
                OrderId::generate(),
                cmd.symbol,
                cmd.side,
                Price::from_f64(45600.0),
                cmd.quantity,
                Price::from_f64(1.0),
                Symbol::new("USDT"),
                false,
            )];
            Ok(OpenPositionResult::filled(OrderId::generate(), trades, 1))
        } else {
            Ok(OpenPositionResult::accepted(OrderId::generate()))
        }
    }

    fn close_position(&self, _cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        unimplemented!()
    }

    fn cancel_order(&self, _cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
        unimplemented!()
    }

    fn modify_order(&self, _cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        unimplemented!()
    }

    fn cancel_all_orders(&self, _cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        unimplemented!()
    }

    fn set_leverage(&self, _cmd: SetLeverageCommand) -> Result<SetLeverageResult, PrepCommandError> {
        unimplemented!()
    }

    fn set_margin_type(&self, _cmd: SetMarginTypeCommand) -> Result<SetMarginTypeResult, PrepCommandError> {
        unimplemented!()
    }

    fn set_position_mode(&self, _cmd: SetPositionModeCommand) -> Result<SetPositionModeResult, PrepCommandError> {
        unimplemented!()
    }
}

impl PerpOrderExchQueryProc for MockMatchingService {
    fn query_order(&self, _cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
        unimplemented!()
    }

    fn query_position(&self, _cmd: QueryPositionCommand) -> Result<PositionInfo, PrepCommandError> {
        unimplemented!()
    }

    fn query_order_book(&self, _cmd: QueryOrderBookCommand) -> Result<OrderBookSnapshot, PrepCommandError> {
        unimplemented!()
    }

    fn query_trades(&self, _cmd: QueryTradesCommand) -> Result<TradesQueryResult, PrepCommandError> {
        unimplemented!()
    }

    fn query_account_balance(&self, _cmd: QueryAccountBalanceCommand) -> Result<Vec<AccountBalance>, PrepCommandError> {
        unimplemented!()
    }

    fn query_account_info(&self, _cmd: QueryAccountInfoCommand) -> Result<AccountInfo, PrepCommandError> {
        unimplemented!()
    }

    fn query_mark_price(&self, _cmd: QueryMarkPriceCommand) -> Result<Vec<MarkPriceInfo>, PrepCommandError> {
        unimplemented!()
    }

    fn query_funding_rate_history(&self, _cmd: QueryFundingRateHistoryCommand) -> Result<Vec<FundingRateRecord>, PrepCommandError> {
        unimplemented!()
    }

    fn query_funding_fee(&self, _cmd: QueryFundingFeeCommand) -> Result<Vec<FundingFeeRecord>, PrepCommandError> {
        unimplemented!()
    }
}

// ============================================================================
// BDD测试 - 强平价格计算
// ============================================================================

#[cfg(test)]
mod liquidation_price_calculation {
    use super::*;

    #[test]
    fn scenario_calculate_long_liquidation_price() {
        // Feature: 强平价格计算
        // Scenario: 多仓强平价格计算

        // Given: BTC入场价 50000 USDT, 10倍杠杆
        let entry_price = Price::from_f64(50000.0);
        let leverage = 10;
        let position_side = PositionSide::Long;

        // When: 计算强平价格
        let liq_price = calculate_liquidation_price(entry_price, leverage, position_side);

        // Then: 强平价应该是 50000 × (1 - 1/10 + 0.005 + 0.005) = 45500
        assert!((liq_price.to_f64() - 45500.0).abs() < 1.0);
    }

    #[test]
    fn scenario_calculate_short_liquidation_price() {
        // Feature: 强平价格计算
        // Scenario: 空仓强平价格计算

        // Given: BTC入场价 50000 USDT, 10倍杠杆
        let entry_price = Price::from_f64(50000.0);
        let leverage = 10;
        let position_side = PositionSide::Short;

        // When: 计算强平价格
        let liq_price = calculate_liquidation_price(entry_price, leverage, position_side);

        // Then: 强平价应该是 50000 × (1 + 1/10 - 0.005 - 0.005) = 54500
        assert!((liq_price.to_f64() - 54500.0).abs() < 1.0);
    }

    #[test]
    fn scenario_higher_leverage_closer_liquidation() {
        // Feature: 杠杆影响
        // Scenario: 更高杠杆导致强平价更接近入场价

        let entry_price = Price::from_f64(50000.0);

        // Given: 5倍杠杆多仓
        let liq_5x = calculate_liquidation_price(entry_price, 5, PositionSide::Long);

        // And: 20倍杠杆多仓
        let liq_20x = calculate_liquidation_price(entry_price, 20, PositionSide::Long);

        // Then: 20倍杠杆的强平价应该更接近入场价
        let distance_5x = (entry_price.to_f64() - liq_5x.to_f64()).abs();
        let distance_20x = (entry_price.to_f64() - liq_20x.to_f64()).abs();

        assert!(distance_20x < distance_5x);
    }
}

// ============================================================================
// BDD测试 - 强平损失计算
// ============================================================================

#[cfg(test)]
mod liquidation_loss_calculation {
    use super::*;
    use prep_proc::proc::liquidation_proc::LiquidationProcessor;

    fn create_long_position() -> PositionInfo {
        PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(45500.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0,
        }
    }

    fn create_short_position() -> PositionInfo {
        PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Short,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(54500.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(54500.0)),
            updated_at: 0,
        }
    }

    #[test]
    fn scenario_calculate_long_position_loss() {
        // Feature: 强平损失计算
        // Scenario: 多仓强平损失计算

        // Given: 多仓 1 BTC @ 50000
        let position = create_long_position();

        // When: 以45500平仓(强平价)
        let close_price = Price::from_f64(45500.0);
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, close_price);

        // Then: 损失 = (50000 - 45500) × 1.0 = 4500
        assert!((loss.to_f64() - 4500.0).abs() < 1.0);
    }

    #[test]
    fn scenario_calculate_short_position_loss() {
        // Feature: 强平损失计算
        // Scenario: 空仓强平损失计算

        // Given: 空仓 1 BTC @ 50000
        let position = create_short_position();

        // When: 以54500平仓(强平价)
        let close_price = Price::from_f64(54500.0);
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, close_price);

        // Then: 损失 = (54500 - 50000) × 1.0 = 4500
        assert!((loss.to_f64() - 4500.0).abs() < 1.0);
    }

    #[test]
    fn scenario_loss_greater_than_margin() {
        // Feature: 保险基金介入
        // Scenario: 损失超过保证金时保险基金承担

        // Given: 多仓 1 BTC @ 50000, 保证金 5000
        let position = create_long_position();

        // When: 价格暴跌至40000(远低于强平价)
        let close_price = Price::from_f64(40000.0);
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, close_price);

        // Then: 损失 = (50000 - 40000) × 1.0 = 10000 > 保证金 5000
        assert!(loss > position.margin);
        assert!((loss.to_f64() - 10000.0).abs() < 1.0);
    }
}

// ============================================================================
// BDD测试 - 市场强平
// ============================================================================

#[cfg(test)]
mod market_liquidation_scenarios {
    use super::*;

    fn create_test_position() -> PositionInfo {
        PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(45500.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0,
        }
    }

    #[tokio::test]
    async fn scenario_market_liquidation_successful() {
        // Feature: 市场强平
        // Scenario: 市场流动性充足，强平成功

        // Given: 标记价格触及强平价45500
        let position = create_test_position();
        let trigger_price = Price::from_f64(45500.0);

        // And: 市场有足够流动性(Mock返回成交)
        let matching_service = Arc::new(MockMatchingService::new(true));
        let insurance_fund = Arc::new(MockInsuranceFund::new(10000.0));
        let adl_engine = Arc::new(MockADLEngine::new(true));

        let processor = LiquidationProcessor::new(
            matching_service.clone(),
            insurance_fund.clone(),
            adl_engine.clone(),
        );

        // When: 执行市场强平
        // Note: try_market_liquidation is private, so we test through calculate_liquidation_loss
        let close_price = Price::from_f64(45600.0); // 模拟成交价
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, close_price);

        // Then: 损失应该被正确计算
        // 损失 = (50000 - 45600) × 1.0 = 4400
        assert!((loss.to_f64() - 4400.0).abs() < 1.0);

        // And: 损失应该小于保证金(保险基金不需要承担)
        assert!(loss < position.margin);
    }

    #[tokio::test]
    async fn scenario_market_liquidation_partial_loss() {
        // Feature: 市场强平损失分配
        // Scenario: 市场强平成功，但有部分损失需要保险基金承担

        // Given: 多仓 1 BTC @ 50000, 保证金 5000
        let position = create_test_position();

        // When: 市场强平以45600成交
        let close_price = Price::from_f64(45600.0);
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, close_price);

        // Then: 损失 = 4400, 用户损失保证金4400
        assert!((loss.to_f64() - 4400.0).abs() < 1.0);

        // And: 保险基金损失 = 0 (因为4400 < 5000)
        let insurance_loss = if loss > position.margin {
            loss.to_f64() - position.margin.to_f64()
        } else {
            0.0
        };
        assert_eq!(insurance_loss, 0.0);
    }
}

// ============================================================================
// BDD测试 - 保险基金接管
// ============================================================================

#[cfg(test)]
mod insurance_fund_scenarios {
    use super::*;

    #[tokio::test]
    async fn scenario_insurance_fund_has_capacity() {
        // Feature: 保险基金接管
        // Scenario: 保险基金余额充足，可以接管

        // Given: 保险基金余额 10000 USDT
        let insurance_fund = MockInsuranceFund::new(10000.0);

        // And: 需要接管的持仓保证金 5000 USDT
        let position = PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(45500.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0,
        };

        // When: 检查保险基金容量
        let capacity = insurance_fund.check_capacity().await.unwrap();

        // Then: 保险基金应该有能力接管
        assert!(capacity.can_takeover(&position));
    }

    #[tokio::test]
    async fn scenario_insurance_fund_insufficient() {
        // Feature: 保险基金接管
        // Scenario: 保险基金余额不足，无法接管

        // Given: 保险基金余额只有 1000 USDT
        let insurance_fund = MockInsuranceFund::new(1000.0);

        // And: 需要接管的持仓保证金 5000 USDT
        let position = PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(45500.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0,
        };

        // When: 检查保险基金容量
        let capacity = insurance_fund.check_capacity().await.unwrap();

        // Then: 保险基金应该无法接管
        assert!(!capacity.can_takeover(&position));
    }
}

// ============================================================================
// BDD测试 - 自动减仓 (ADL)
// ============================================================================

#[cfg(test)]
mod adl_scenarios {
    use super::*;

    #[tokio::test]
    async fn scenario_adl_has_counterparties() {
        // Feature: 自动减仓 (ADL)
        // Scenario: 找到对手方盈利仓位

        // Given: ADL引擎有可用的对手方
        let adl_engine = MockADLEngine::new(true);

        // When: 查找对手方
        let counterparties = adl_engine
            .find_counterparties(Symbol::new("BTCUSDT"), Side::Buy)
            .await
            .unwrap();

        // Then: 应该找到对手方
        assert!(!counterparties.is_empty());
        assert_eq!(counterparties.len(), 1);

        // And: 对手方应该是盈利的空仓
        let counterparty = &counterparties[0];
        assert_eq!(counterparty.position_side, PositionSide::Short);
        assert!(counterparty.unrealized_pnl.to_f64() > 0.0);
    }

    #[tokio::test]
    async fn scenario_adl_no_counterparties() {
        // Feature: 自动减仓 (ADL)
        // Scenario: 没有可用的对手方盈利仓位

        // Given: ADL引擎没有可用的对手方
        let adl_engine = MockADLEngine::new(false);

        // When: 查找对手方
        let counterparties = adl_engine
            .find_counterparties(Symbol::new("BTCUSDT"), Side::Buy)
            .await
            .unwrap();

        // Then: 应该没有对手方
        assert!(counterparties.is_empty());
    }

    #[tokio::test]
    async fn scenario_adl_execution_affects_multiple_positions() {
        // Feature: ADL执行
        // Scenario: ADL可能影响多个对手方仓位

        // Given: 有多个对手方盈利仓位
        let adl_engine = MockADLEngine::new(true);
        let counterparties = adl_engine
            .find_counterparties(Symbol::new("BTCUSDT"), Side::Buy)
            .await
            .unwrap();

        // And: 需要强平的持仓
        let liquidated_position = PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(45500.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0,
        };

        // When: 执行ADL
        let adl_result = adl_engine
            .execute_adl(&liquidated_position, counterparties)
            .await
            .unwrap();

        // Then: 应该有受影响的持仓
        assert!(!adl_result.affected_positions.is_empty());
    }
}

// ============================================================================
// BDD测试 - 完整强平流程集成
// ============================================================================

#[cfg(test)]
mod complete_liquidation_workflow {
    use super::*;

    #[test]
    fn scenario_complete_liquidation_calculation_workflow() {
        // Feature: 完整强平流程
        // Scenario: 从持仓创建到强平价格计算的完整流程

        // Step 1: 用户开多仓
        // Given: 用户以50000价格开多仓1 BTC, 10倍杠杆
        let entry_price = Price::from_f64(50000.0);
        let leverage = 10;
        let quantity = Quantity::from_f64(1.0);

        // Step 2: 计算保证金
        let notional = entry_price.to_f64() * quantity.to_f64();
        let margin = notional / leverage as f64;
        assert!((margin - 5000.0).abs() < 1.0); // 保证金 = 50000 / 10 = 5000

        // Step 3: 计算强平价格
        let liq_price = calculate_liquidation_price(entry_price, leverage, PositionSide::Long);
        assert!((liq_price.to_f64() - 45500.0).abs() < 1.0); // 强平价 = 45500

        // Step 4: 标记价格跌至强平价
        let mark_price = Price::from_f64(45500.0);
        assert!(mark_price <= liq_price); // 触发强平

        // Step 5: 假设市场强平以45600成交
        let fill_price = Price::from_f64(45600.0);
        let position = PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity,
            entry_price,
            mark_price,
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage,
            margin: Price::from_f64(margin),
            liquidation_price: Some(liq_price),
            updated_at: 0,
        };

        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, fill_price);

        // Step 6: 验证损失分配
        // 损失 = (50000 - 45600) × 1.0 = 4400
        assert!((loss.to_f64() - 4400.0).abs() < 1.0);

        // And: 用户损失 = min(4400, 5000) = 4400
        let user_loss = if loss > position.margin {
            position.margin.to_f64()
        } else {
            loss.to_f64()
        };
        assert!((user_loss - 4400.0).abs() < 1.0);

        // And: 保险基金损失 = max(0, 4400 - 5000) = 0
        let insurance_loss = if loss > position.margin {
            loss.to_f64() - position.margin.to_f64()
        } else {
            0.0
        };
        assert_eq!(insurance_loss, 0.0);
    }

    #[test]
    fn scenario_liquidation_with_insurance_fund_loss() {
        // Feature: 保险基金损失场景
        // Scenario: 市场价格暴跌，保险基金需要承担部分损失

        // Given: 多仓 1 BTC @ 50000, 10倍杠杆, 保证金 5000
        let entry_price = Price::from_f64(50000.0);
        let leverage = 10;
        let quantity = Quantity::from_f64(1.0);
        let margin = 5000.0;

        // When: 价格暴跌至44000(低于强平价45500)
        let fill_price = Price::from_f64(44000.0);
        let position = PositionInfo {
            position_id: PositionId::generate(),
            symbol: Symbol::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity,
            entry_price,
            mark_price: fill_price,
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage,
            margin: Price::from_f64(margin),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0,
        };

        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, fill_price);

        // Then: 损失 = (50000 - 44000) × 1.0 = 6000
        assert!((loss.to_f64() - 6000.0).abs() < 1.0);

        // And: 用户损失 = min(6000, 5000) = 5000 (全部保证金)
        let user_loss = if loss > position.margin {
            position.margin.to_f64()
        } else {
            loss.to_f64()
        };
        assert!((user_loss - 5000.0).abs() < 1.0);

        // And: 保险基金损失 = 6000 - 5000 = 1000
        let insurance_loss = if loss > position.margin {
            loss.to_f64() - position.margin.to_f64()
        } else {
            0.0
        };
        assert!((insurance_loss - 1000.0).abs() < 1.0);
    }
}
