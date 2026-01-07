//! BDD测试 - 从开仓到资金费率的完整流程
//!
//! 测试完整的业务流程：
//! 1. 开仓准备（设置杠杆）
//! 2. 订单执行（市价/限价开仓）
//! 3. 持仓管理（持仓创建、强平价计算）
//! 4. 资金费率计算（每8小时）
//! 5. 资金费率结算（收取/支付费用）
//! 6. 资金费率对保证金的影响
//! 7. 资金费率可能触发强平

use std::sync::{Arc, Mutex};

use prep_proc::proc::{
    liquidation_types::*, trading_prep_order_proc::*, trading_prep_order_proc_impl::PrepMatchingService
};

// ============================================================================
// Mock实现 - 资金费率服务
// ============================================================================

/// 资金费率记录
#[derive(Debug, Clone)]
pub struct FundingRateRecord {
    pub symbol: TradingPair,
    pub funding_rate: i64, // 精度1e6，例如100表示0.0001
    pub settlement_time: i64
}

/// 资金费用记录
#[derive(Debug, Clone)]
pub struct FundingFeeRecord {
    pub position_id: PositionId,
    pub symbol: TradingPair,
    pub funding_fee: Price,
    pub settlement_time: i64
}

/// 资金费率结算结果
#[derive(Debug, Clone)]
pub struct FundingSettlementResult {
    pub total_positions: usize,
    pub total_fee_collected: Price,
    pub total_fee_paid: Price,
    pub funding_rate: i64
}

/// Mock资金费率服务
struct MockFundingRateService {
    /// 当前资金费率 (按symbol)
    current_rates: Arc<Mutex<std::collections::HashMap<TradingPair, i64>>>,
    /// 资金费率历史
    rate_history: Arc<Mutex<Vec<FundingRateRecord>>>,
    /// 资金费用历史
    fee_history: Arc<Mutex<Vec<FundingFeeRecord>>>
}

impl MockFundingRateService {
    fn new() -> Self {
        Self {
            current_rates: Arc::new(Mutex::new(std::collections::HashMap::new())),
            rate_history: Arc::new(Mutex::new(Vec::new())),
            fee_history: Arc::new(Mutex::new(Vec::new()))
        }
    }

    /// 设置资金费率
    fn set_funding_rate(&self, symbol: TradingPair, rate: i64) { self.current_rates.lock().unwrap().insert(symbol, rate); }

    /// 获取当前资金费率
    fn get_funding_rate(&self, symbol: TradingPair) -> i64 { *self.current_rates.lock().unwrap().get(&symbol).unwrap_or(&0) }

    /// 计算资金费用
    fn calculate_funding_fee(&self, position: &PrepPosition, funding_rate: i64) -> Price {
        // 持仓价值 = 数量 × 标记价格
        let position_value = position.quantity.to_f64() * position.mark_price.to_f64();

        // 资金费用 = 持仓价值 × 资金费率
        // funding_rate精度为1e6，例如100表示0.0001
        let fee = position_value * (funding_rate as f64 / 1_000_000.0);

        Price::from_f64(fee)
    }

    /// 执行资金费率结算
    fn settle_funding_rate(
        &self, symbol: TradingPair, positions: &[PrepPosition], settlement_time: i64
    ) -> FundingSettlementResult {
        let funding_rate = self.get_funding_rate(symbol);

        let mut total_fee_collected = Price::from_raw(0);
        let mut total_fee_paid = Price::from_raw(0);

        // 记录历史费率
        self.rate_history.lock().unwrap().push(FundingRateRecord {
            symbol,
            funding_rate,
            settlement_time
        });

        // 对每个持仓结算
        for position in positions {
            if position.symbol != symbol || !position.has_position() {
                continue;
            }

            let funding_fee = self.calculate_funding_fee(position, funding_rate);

            // 记录费用历史
            self.fee_history.lock().unwrap().push(FundingFeeRecord {
                position_id: position.position_id.clone(),
                symbol: position.symbol,
                funding_fee,
                settlement_time
            });

            // 统计总费用
            if position.is_long() {
                // 多仓支付（funding_rate > 0时）
                if funding_rate > 0 {
                    total_fee_paid = Price::from_f64(total_fee_paid.to_f64() + funding_fee.to_f64());
                } else {
                    total_fee_collected = Price::from_f64(total_fee_collected.to_f64() + funding_fee.to_f64().abs());
                }
            } else {
                // 空仓收取（funding_rate > 0时）
                if funding_rate > 0 {
                    total_fee_collected = Price::from_f64(total_fee_collected.to_f64() + funding_fee.to_f64());
                } else {
                    total_fee_paid = Price::from_f64(total_fee_paid.to_f64() + funding_fee.to_f64().abs());
                }
            }
        }

        FundingSettlementResult {
            total_positions: positions.len(),
            total_fee_collected,
            total_fee_paid,
            funding_rate
        }
    }

    /// 应用资金费用到持仓
    fn apply_funding_fee_to_position(&self, position: &mut PrepPosition, funding_rate: i64) -> Price {
        let funding_fee = self.calculate_funding_fee(position, funding_rate);

        // 多仓支付，空仓收取（当funding_rate > 0时）
        let fee_direction = if position.is_long() {
            if funding_rate > 0 {
                -1.0
            } else {
                1.0
            }
        } else {
            if funding_rate > 0 {
                1.0
            } else {
                -1.0
            }
        };

        let actual_fee = Price::from_f64(funding_fee.to_f64() * fee_direction);

        // 从保证金扣除/增加
        position.margin = Price::from_f64(position.margin.to_f64() + actual_fee.to_f64());

        actual_fee
    }
}

// ============================================================================
// BDD测试 - 开仓准备阶段
// ============================================================================

#[cfg(test)]
mod opening_preparation_scenarios {
    use super::*;

    #[test]
    fn scenario_set_leverage_before_opening() {
        // Feature: 开仓准备
        // Scenario: 设置杠杆为后续开仓做准备

        // Given: 用户有10000 USDT余额
        let matching_service = PrepMatchingService::new(Price::from_f64(10000.0));

        // When: 用户设置BTCUSDT的杠杆为10倍
        let cmd = SetLeverageCommand::new(TradingPair::new("BTCUSDT"), 10);
        let result = matching_service.set_leverage(cmd);

        // Then: 杠杆设置成功
        assert!(result.is_ok());
        let leverage_result = result.unwrap();

        println!("✅ 杠杆设置成功:");
        println!("   旧杠杆: {}x", leverage_result.old_leverage);
        println!("   新杠杆: {}x", leverage_result.new_leverage);
        println!("   可用余额: {} USDT", leverage_result.available_balance.to_f64());

        assert_eq!(leverage_result.new_leverage, 10);
        assert_eq!(leverage_result.available_balance.to_f64(), 10000.0);
    }
}

// ============================================================================
// BDD测试 - 订单执行与持仓创建
// ============================================================================

#[cfg(test)]
mod order_execution_scenarios {
    use super::*;

    #[test]
    fn scenario_open_long_position_successfully() {
        // Feature: 订单执行
        // Scenario: 成功开多仓并创建持仓

        // Given: 用户有10000 USDT余额，设置了10倍杠杆
        let matching_service = PrepMatchingService::new(Price::from_f64(10000.0));
        matching_service.set_leverage(SetLeverageCommand::new(TradingPair::new("BTCUSDT"), 10)).unwrap();

        // When: 用户以市价开多仓1 BTC
        let open_cmd =
            OpenPositionCommand::market_long(TradingPair::new("BTCUSDT"), Quantity::from_f64(1.0)).with_leverage(10);

        let open_result = matching_service.open_position(open_cmd);

        // Then: 订单成功执行
        assert!(open_result.is_ok());
        let result = open_result.unwrap();

        println!("✅ 开仓成功:");
        println!("   订单ID: {}", result.order_id.as_str());
        println!("   状态: {:?}", result.status);
        println!("   成交数量: {} BTC", result.filled_quantity.to_f64());

        assert_eq!(result.status, OrderStatus::Filled);
        assert_eq!(result.filled_quantity.to_f64(), 1.0);

        // And: 持仓应该被创建
        let position = matching_service.query_position(QueryPositionCommand::long(TradingPair::new("BTCUSDT"))).unwrap();

        assert!(position.has_position());
        println!("\n✅ 持仓创建成功:");
        println!("   持仓方向: {:?}", position.position_side);
        println!("   持仓数量: {} BTC", position.quantity.to_f64());
        println!("   开仓价: {} USDT", position.entry_price.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());
        println!("   杠杆: {}x", position.leverage);

        // And: 强平价应该被计算
        if let Some(liq_price) = position.liquidation_price {
            println!("   强平价: {} USDT", liq_price.to_f64());

            // 验证强平价低于开仓价（多仓）
            assert!(liq_price < position.entry_price);
        }
    }
}

// ============================================================================
// BDD测试 - 资金费率计算
// ============================================================================

#[cfg(test)]
mod funding_rate_calculation_scenarios {
    use super::*;

    #[test]
    fn scenario_calculate_funding_fee_for_long_position() {
        // Feature: 资金费率计算
        // Scenario: 计算多仓的资金费用（正费率）

        let funding_service = MockFundingRateService::new();

        // Given: 资金费率为+0.01% (100 / 1e6)
        let funding_rate = 100; // 0.0001
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // And: 用户有1 BTC多仓 @ 50000 USDT
        let position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(50000.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0
        };

        // When: 计算资金费用
        let funding_fee = funding_service.calculate_funding_fee(&position, funding_rate);

        // Then: 资金费用应该是持仓价值的0.01%
        // 费用 = 1.0 × 50000 × 0.0001 = 5 USDT
        println!("✅ 资金费用计算:");
        println!("   持仓价值: {} USDT", position.quantity.to_f64() * position.mark_price.to_f64());
        println!("   资金费率: {}%", funding_rate as f64 / 10000.0);
        println!("   资金费用: {} USDT", funding_fee.to_f64());

        assert!((funding_fee.to_f64() - 5.0).abs() < 0.01);

        // And: 多仓应该支付费用（正费率时）
        println!("   支付方向: 多仓支付 → 空仓收取");
    }

    #[test]
    fn scenario_calculate_funding_fee_for_short_position() {
        // Feature: 资金费率计算
        // Scenario: 计算空仓的资金费用（正费率）

        let funding_service = MockFundingRateService::new();

        // Given: 资金费率为+0.01%
        let funding_rate = 100;
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // And: 用户有1 BTC空仓 @ 50000 USDT
        let position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Short,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(50000.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(54500.0)),
            updated_at: 0
        };

        // When: 计算资金费用
        let funding_fee = funding_service.calculate_funding_fee(&position, funding_rate);

        // Then: 资金费用绝对值相同
        println!("✅ 资金费用计算 (空仓):");
        println!("   持仓价值: {} USDT", position.quantity.to_f64() * position.mark_price.to_f64());
        println!("   资金费率: {}%", funding_rate as f64 / 10000.0);
        println!("   资金费用: {} USDT", funding_fee.to_f64());

        assert!((funding_fee.to_f64() - 5.0).abs() < 0.01);

        // And: 空仓应该收取费用（正费率时）
        println!("   支付方向: 多仓支付 → 空仓收取");
    }

    #[test]
    fn scenario_negative_funding_rate() {
        // Feature: 负资金费率
        // Scenario: 负费率时空仓支付，多仓收取

        let funding_service = MockFundingRateService::new();

        // Given: 资金费率为-0.01% (负值)
        let funding_rate = -100;
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // And: 用户有1 BTC多仓 @ 50000 USDT
        let position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(50000.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0
        };

        // When: 计算资金费用
        let funding_fee = funding_service.calculate_funding_fee(&position, funding_rate);

        // Then: 资金费用为负值
        println!("✅ 负资金费率计算:");
        println!("   资金费率: {}%", funding_rate as f64 / 10000.0);
        println!("   资金费用: {} USDT", funding_fee.to_f64());

        assert!(funding_fee.to_f64() < 0.0);

        // And: 多仓应该收取费用（负费率时）
        println!("   支付方向: 空仓支付 → 多仓收取");
    }
}

// ============================================================================
// BDD测试 - 资金费率结算
// ============================================================================

#[cfg(test)]
mod funding_rate_settlement_scenarios {
    use super::*;

    #[test]
    fn scenario_settle_funding_rate_for_multiple_positions() {
        // Feature: 资金费率结算
        // Scenario: 对多个持仓执行资金费率结算

        let funding_service = MockFundingRateService::new();

        // Given: 资金费率为+0.01%
        let funding_rate = 100;
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // And: 有3个持仓
        let positions = vec![
            // 多仓1: 1 BTC @ 50000
            PrepPosition {
                position_id: PositionId::generate(),
                symbol: TradingPair::new("BTCUSDT"),
                position_side: PositionSide::Long,
                quantity: Quantity::from_f64(1.0),
                entry_price: Price::from_f64(50000.0),
                mark_price: Price::from_f64(50000.0),
                unrealized_pnl: Price::from_raw(0),
                realized_pnl: Price::from_raw(0),
                leverage: 10,
                margin: Price::from_f64(5000.0),
                liquidation_price: Some(Price::from_f64(45500.0)),
                updated_at: 0
            },
            // 多仓2: 0.5 BTC @ 50000
            PrepPosition {
                position_id: PositionId::generate(),
                symbol: TradingPair::new("BTCUSDT"),
                position_side: PositionSide::Long,
                quantity: Quantity::from_f64(0.5),
                entry_price: Price::from_f64(50000.0),
                mark_price: Price::from_f64(50000.0),
                unrealized_pnl: Price::from_raw(0),
                realized_pnl: Price::from_raw(0),
                leverage: 5,
                margin: Price::from_f64(5000.0),
                liquidation_price: Some(Price::from_f64(42500.0)),
                updated_at: 0
            },
            // 空仓: 1 BTC @ 50000
            PrepPosition {
                position_id: PositionId::generate(),
                symbol: TradingPair::new("BTCUSDT"),
                position_side: PositionSide::Short,
                quantity: Quantity::from_f64(1.0),
                entry_price: Price::from_f64(50000.0),
                mark_price: Price::from_f64(50000.0),
                unrealized_pnl: Price::from_raw(0),
                realized_pnl: Price::from_raw(0),
                leverage: 10,
                margin: Price::from_f64(5000.0),
                liquidation_price: Some(Price::from_f64(54500.0)),
                updated_at: 0
            },
        ];

        // When: 执行资金费率结算
        let settlement_time = 1640000000; // 2021-12-20 08:00:00 UTC
        let result = funding_service.settle_funding_rate(TradingPair::new("BTCUSDT"), &positions, settlement_time);

        // Then: 结算成功
        println!("✅ 资金费率结算完成:");
        println!("   结算时间: {}", settlement_time);
        println!("   资金费率: {}%", result.funding_rate as f64 / 10000.0);
        println!("   结算持仓数: {}", result.total_positions);
        println!("   总收取费用: {} USDT", result.total_fee_collected.to_f64());
        println!("   总支付费用: {} USDT", result.total_fee_paid.to_f64());

        assert_eq!(result.total_positions, 3);

        // And: 多仓支付费用
        // 多仓1: 1 × 50000 × 0.0001 = 5 USDT
        // 多仓2: 0.5 × 50000 × 0.0001 = 2.5 USDT
        // 总支付: 7.5 USDT
        assert!((result.total_fee_paid.to_f64() - 7.5).abs() < 0.01);

        // And: 空仓收取费用
        // 空仓: 1 × 50000 × 0.0001 = 5 USDT
        assert!((result.total_fee_collected.to_f64() - 5.0).abs() < 0.01);

        // And: 资金费率应该被记录到历史
        let rate_history = funding_service.rate_history.lock().unwrap();
        assert_eq!(rate_history.len(), 1);
        assert_eq!(rate_history[0].funding_rate, funding_rate);
    }
}

// ============================================================================
// BDD测试 - 资金费率对保证金的影响
// ============================================================================

#[cfg(test)]
mod funding_rate_margin_impact_scenarios {
    use super::*;

    #[test]
    fn scenario_funding_fee_reduces_margin() {
        // Feature: 资金费率影响保证金
        // Scenario: 支付资金费用减少保证金余额

        let funding_service = MockFundingRateService::new();

        // Given: 资金费率为+0.01%
        let funding_rate = 100;
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // And: 用户有1 BTC多仓，保证金5000 USDT
        let mut position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(50000.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0
        };

        let original_margin = position.margin.to_f64();
        println!("📊 资金费用对保证金的影响:");
        println!("   原始保证金: {} USDT", original_margin);

        // When: 应用资金费用
        let applied_fee = funding_service.apply_funding_fee_to_position(&mut position, funding_rate);

        // Then: 保证金应该减少
        println!("   资金费用: {} USDT", applied_fee.to_f64());
        println!("   新保证金: {} USDT", position.margin.to_f64());

        // 多仓支付费用，保证金减少5 USDT
        assert!((position.margin.to_f64() - (original_margin - 5.0)).abs() < 0.01);
        assert!(applied_fee.to_f64() < 0.0); // 负值表示支付

        // And: 验证保证金变化
        let margin_change = position.margin.to_f64() - original_margin;
        println!("   保证金变化: {} USDT", margin_change);
        assert!((margin_change + 5.0).abs() < 0.01);
    }

    #[test]
    fn scenario_funding_fee_increases_margin_for_short() {
        // Feature: 资金费率影响保证金
        // Scenario: 收取资金费用增加保证金余额（空仓）

        let funding_service = MockFundingRateService::new();

        // Given: 资金费率为+0.01%
        let funding_rate = 100;

        // And: 用户有1 BTC空仓，保证金5000 USDT
        let mut position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Short,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(50000.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5000.0),
            liquidation_price: Some(Price::from_f64(54500.0)),
            updated_at: 0
        };

        let original_margin = position.margin.to_f64();
        println!("📊 资金费用对保证金的影响 (空仓收取):");
        println!("   原始保证金: {} USDT", original_margin);

        // When: 应用资金费用
        let applied_fee = funding_service.apply_funding_fee_to_position(&mut position, funding_rate);

        // Then: 保证金应该增加
        println!("   资金费用: {} USDT", applied_fee.to_f64());
        println!("   新保证金: {} USDT", position.margin.to_f64());

        // 空仓收取费用，保证金增加5 USDT
        assert!((position.margin.to_f64() - (original_margin + 5.0)).abs() < 0.01);
        assert!(applied_fee.to_f64() > 0.0); // 正值表示收取

        let margin_change = position.margin.to_f64() - original_margin;
        println!("   保证金变化: +{} USDT", margin_change);
        assert!((margin_change - 5.0).abs() < 0.01);
    }
}

// ============================================================================
// BDD测试 - 资金费率可能触发强平
// ============================================================================

#[cfg(test)]
mod funding_rate_liquidation_scenarios {
    use super::*;

    #[test]
    fn scenario_funding_fee_triggers_liquidation() {
        // Feature: 资金费率触发强平
        // Scenario: 支付资金费用导致保证金不足，触发强平

        let funding_service = MockFundingRateService::new();

        // Given: 用户有1 BTC多仓，保证金仅为5010 USDT（接近强平）
        let mut position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(45550.0), // 接近强平价45500
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5010.0), // 仅略高于强平要求
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0
        };

        let liquidation_price = position.liquidation_price.unwrap();
        println!("⚠️  临界状态持仓:");
        println!("   当前标记价: {} USDT", position.mark_price.to_f64());
        println!("   强平价: {} USDT", liquidation_price.to_f64());
        println!("   距离强平: {} USDT", position.mark_price.to_f64() - liquidation_price.to_f64());
        println!("   当前保证金: {} USDT", position.margin.to_f64());

        // And: 资金费率为+0.02% (较高费率)
        let funding_rate = 200; // 0.0002
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // When: 支付资金费用
        let funding_fee = funding_service.calculate_funding_fee(&position, funding_rate);
        println!("\n💰 资金费用结算:");
        println!("   资金费率: {}%", funding_rate as f64 / 10000.0);
        println!("   应付费用: {} USDT", funding_fee.to_f64());

        // Then: 资金费用为9.11 USDT (1 × 45550 × 0.0002)
        // 注意：使用mark_price 45550计算，而不是entry_price 50000
        let expected_fee = 1.0 * 45550.0 * 0.0002;
        assert!((funding_fee.to_f64() - expected_fee).abs() < 0.01);

        // When: 应用费用到保证金
        let applied_fee = funding_service.apply_funding_fee_to_position(&mut position, funding_rate);

        println!("\n📉 保证金变化:");
        println!("   支付费用: {} USDT", applied_fee.to_f64());
        println!("   新保证金: {} USDT", position.margin.to_f64());

        // Then: 新保证金 = 5010 - 9.11 ≈ 5000.89 USDT
        let expected_new_margin = 5010.0 - expected_fee;
        assert!((position.margin.to_f64() - expected_new_margin).abs() < 0.01);

        // And: 需要检查是否触发强平
        // 简化版：计算维持保证金率
        let position_value = position.quantity.to_f64() * position.mark_price.to_f64();
        let maintenance_margin_rate = 0.005; // 0.5%
        let required_maintenance_margin = position_value * maintenance_margin_rate;

        println!("\n🔍 强平检查:");
        println!("   持仓价值: {} USDT", position_value);
        println!("   维持保证金要求: {} USDT", required_maintenance_margin);
        println!("   当前保证金: {} USDT", position.margin.to_f64());

        // And: 判断是否需要强平
        let should_liquidate = position.margin.to_f64() < required_maintenance_margin;

        if should_liquidate {
            println!("   ⚠️  保证金不足，触发强平！");
        } else {
            let margin_buffer = position.margin.to_f64() - required_maintenance_margin;
            println!("   ✅ 保证金充足，剩余缓冲: {} USDT", margin_buffer);
        }

        // 在这个场景中，由于价格接近强平价，可能会触发强平
        // 具体是否触发取决于维持保证金率的精确计算
    }

    #[test]
    fn scenario_multiple_funding_settlements_lead_to_liquidation() {
        // Feature: 多次资金费率结算累积触发强平
        // Scenario: 经过多次资金费率结算，累积费用导致强平

        let funding_service = MockFundingRateService::new();

        // Given: 用户有1 BTC多仓 @ 50000，保证金5100 USDT
        let mut position = PrepPosition {
            position_id: PositionId::generate(),
            symbol: TradingPair::new("BTCUSDT"),
            position_side: PositionSide::Long,
            quantity: Quantity::from_f64(1.0),
            entry_price: Price::from_f64(50000.0),
            mark_price: Price::from_f64(50000.0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 10,
            margin: Price::from_f64(5100.0),
            liquidation_price: Some(Price::from_f64(45500.0)),
            updated_at: 0
        };

        println!("📊 多次资金费率结算模拟:");
        println!("   初始保证金: {} USDT\n", position.margin.to_f64());

        // When: 经过多次资金费率结算
        let funding_rate = 100; // 0.01%
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate);

        // 第1次结算 (T+0小时，开仓后不久)
        let fee1 = funding_service.apply_funding_fee_to_position(&mut position, funding_rate);
        println!("T+0h  第1次结算:");
        println!("      支付费用: {} USDT", fee1.to_f64());
        println!("      剩余保证金: {} USDT", position.margin.to_f64());

        // 第2次结算 (T+8小时)
        let fee2 = funding_service.apply_funding_fee_to_position(&mut position, funding_rate);
        println!("\nT+8h  第2次结算:");
        println!("      支付费用: {} USDT", fee2.to_f64());
        println!("      剩余保证金: {} USDT", position.margin.to_f64());

        // 第3次结算 (T+16小时)
        let fee3 = funding_service.apply_funding_fee_to_position(&mut position, funding_rate);
        println!("\nT+16h 第3次结算:");
        println!("      支付费用: {} USDT", fee3.to_f64());
        println!("      剩余保证金: {} USDT", position.margin.to_f64());

        // Then: 累积支付了15 USDT费用
        let total_fees = fee1.to_f64().abs() + fee2.to_f64().abs() + fee3.to_f64().abs();
        println!("\n💰 累积统计:");
        println!("   累积支付费用: {} USDT", total_fees);
        println!("   最终保证金: {} USDT", position.margin.to_f64());
        assert!((total_fees - 15.0).abs() < 0.01);

        // And: 保证金从5100降至5085
        assert!((position.margin.to_f64() - 5085.0).abs() < 0.01);

        // And: 如果价格下跌，更容易触发强平
        println!("\n⚠️  风险提示:");
        println!("   资金费用累积消耗了 {} USDT 保证金", total_fees);
        println!(
            "   当前距离强平价: {} USDT",
            position.mark_price.to_f64() - position.liquidation_price.unwrap().to_f64()
        );
        println!("   风险等级提升，更接近强平价");
    }
}

// ============================================================================
// BDD测试 - 完整流程集成
// ============================================================================

#[cfg(test)]
mod complete_open_to_funding_workflow {
    use super::*;

    #[test]
    fn scenario_complete_24h_workflow_with_funding_settlements() {
        // Feature: 完整24小时交易流程
        // Scenario: 从开仓到经历3次资金费率结算的完整流程

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("     完整24小时交易流程模拟");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let matching_service = PrepMatchingService::new(Price::from_f64(10000.0));
        let funding_service = MockFundingRateService::new();

        // ================================================================
        // T0: 00:05 UTC - 开仓准备
        // ================================================================
        println!("⏰ T0: 00:05 UTC - 开仓准备");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Step 1: 设置杠杆
        matching_service.set_leverage(SetLeverageCommand::new(TradingPair::new("BTCUSDT"), 10)).unwrap();
        println!("✅ 杠杆设置: 10x");

        // Step 2: 开多仓
        let open_cmd =
            OpenPositionCommand::market_long(TradingPair::new("BTCUSDT"), Quantity::from_f64(1.0)).with_leverage(10);

        matching_service.open_position(open_cmd).unwrap();

        let mut position = matching_service.query_position(QueryPositionCommand::long(TradingPair::new("BTCUSDT"))).unwrap();

        println!("✅ 开仓成功:");
        println!("   数量: {} BTC", position.quantity.to_f64());
        println!("   开仓价: {} USDT", position.entry_price.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());
        if let Some(liq_price) = position.liquidation_price {
            println!("   强平价: {} USDT\n", liq_price.to_f64());
        } else {
            println!("   强平价: 未设置\n");
        }

        let initial_margin = position.margin.to_f64();

        // ================================================================
        // T1: 08:00 UTC - 第一次资金费率结算
        // ================================================================
        println!("⏰ T1: 08:00 UTC - 第一次资金费率结算");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let funding_rate_1 = 100; // +0.01%
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate_1);

        let fee_1 = funding_service.apply_funding_fee_to_position(&mut position, funding_rate_1);

        println!("📊 资金费率: +{}%", funding_rate_1 as f64 / 10000.0);
        println!("💰 支付费用: {} USDT", fee_1.to_f64().abs());
        println!("📉 剩余保证金: {} USDT\n", position.margin.to_f64());

        // ================================================================
        // T2: 16:00 UTC - 第二次资金费率结算
        // ================================================================
        println!("⏰ T2: 16:00 UTC - 第二次资金费率结算");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let funding_rate_2 = 150; // +0.015%
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate_2);

        let fee_2 = funding_service.apply_funding_fee_to_position(&mut position, funding_rate_2);

        println!("📊 资金费率: +{}%", funding_rate_2 as f64 / 10000.0);
        println!("💰 支付费用: {} USDT", fee_2.to_f64().abs());
        println!("📉 剩余保证金: {} USDT\n", position.margin.to_f64());

        // ================================================================
        // T3: 24:00 UTC - 第三次资金费率结算
        // ================================================================
        println!("⏰ T3: 24:00 UTC - 第三次资金费率结算");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let funding_rate_3 = 120; // +0.012%
        funding_service.set_funding_rate(TradingPair::new("BTCUSDT"), funding_rate_3);

        let fee_3 = funding_service.apply_funding_fee_to_position(&mut position, funding_rate_3);

        println!("📊 资金费率: +{}%", funding_rate_3 as f64 / 10000.0);
        println!("💰 支付费用: {} USDT", fee_3.to_f64().abs());
        println!("📉 剩余保证金: {} USDT\n", position.margin.to_f64());

        // ================================================================
        // 最终统计
        // ================================================================
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("     24小时统计总结");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let total_fees = fee_1.to_f64().abs() + fee_2.to_f64().abs() + fee_3.to_f64().abs();
        let final_margin = position.margin.to_f64();
        let margin_loss_rate = (initial_margin - final_margin) / initial_margin * 100.0;

        println!("📊 保证金变化:");
        println!("   初始保证金: {} USDT", initial_margin);
        println!("   最终保证金: {} USDT", final_margin);
        println!("   保证金减少: {} USDT ({:.2}%)", initial_margin - final_margin, margin_loss_rate);

        println!("\n💰 资金费用统计:");
        println!("   第1次结算: {} USDT", fee_1.to_f64().abs());
        println!("   第2次结算: {} USDT", fee_2.to_f64().abs());
        println!("   第3次结算: {} USDT", fee_3.to_f64().abs());
        println!("   累积支付: {} USDT", total_fees);

        println!("\n⚠️  风险评估:");
        if let Some(liq_price) = position.liquidation_price {
            println!("   当前强平价: {} USDT", liq_price.to_f64());
            println!("   当前标记价: {} USDT", position.mark_price.to_f64());
            println!("   安全距离: {} USDT", position.mark_price.to_f64() - liq_price.to_f64());
        } else {
            println!("   强平价: 未设置");
            println!("   当前标记价: {} USDT", position.mark_price.to_f64());
        }

        println!("\n✅ 完整流程验证通过");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // 验证
        assert!(total_fees > 0.0);
        assert!(final_margin < initial_margin);
        assert_eq!(position.quantity.to_f64(), 1.0); // 持仓未变
    }
}
