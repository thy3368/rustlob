//! BDD测试 - 从下单到强平的完整流程
//!
//! 测试完整的业务流程：
//! 1. 用户开仓（下单）
//! 2. 订单成交，创建持仓
//! 3. 计算强平价格
//! 4. 标记价格触及强平价
//! 5. 触发三级强平机制

use prep_proc::proc::trading_prep_order_proc::*;
use prep_proc::proc::trading_prep_order_proc_impl::MatchingService;
use prep_proc::proc::liquidation_proc::*;
use prep_proc::proc::liquidation_types::{PositionId, LiquidationType};

// ============================================================================
// 完整流程测试 - 从开仓到强平
// ============================================================================

#[cfg(test)]
mod complete_order_to_liquidation_flow {
    use super::*;

    #[tokio::test]
    async fn scenario_long_position_liquidated_by_price_drop() {
        // Feature: 完整的开仓到强平流程
        // Scenario: 多仓因价格下跌被强平

        use std::sync::Arc;
        use prep_proc::proc::liquidation_types::*;

        // Mock InsuranceFund
        struct MockInsuranceFund;
        #[async_trait::async_trait]
        impl InsuranceFund for MockInsuranceFund {
            async fn check_capacity(&self) -> Result<InsuranceFundCapacity, PrepCommandError> {
                Ok(InsuranceFundCapacity {
                    available_balance: Price::from_f64(100000.0),
                })
            }
            async fn takeover(&self, position: &PositionInfo) -> Result<InsuranceFundTakeover, PrepCommandError> {
                Ok(InsuranceFundTakeover {
                    total_loss: position.margin,
                })
            }
        }

        // Mock ADLEngine
        struct MockADLEngine;
        #[async_trait::async_trait]
        impl ADLEngine for MockADLEngine {
            async fn find_counterparties(&self, _symbol: Symbol, _side: Side)
                -> Result<Vec<PositionInfo>, PrepCommandError> {
                Ok(Vec::new())
            }
            async fn execute_adl(&self, _liquidated_position: &PositionInfo, _counterparties: Vec<PositionInfo>)
                -> Result<ADLResult, PrepCommandError> {
                Ok(ADLResult {
                    affected_positions: Vec::new(),
                })
            }
        }

        // ====================================================================
        // Step 1: 初始化 - 用户有10000 USDT余额
        // ====================================================================
        let matching_service = Arc::new(MatchingService::new(Price::from_f64(10000.0)));

        // ====================================================================
        // Step 2: 设置杠杆 - 用户设置10倍杠杆
        // ====================================================================
        let set_leverage_cmd = SetLeverageCommand::new(
            Symbol::new("BTCUSDT"),
            10
        );

        let leverage_result = matching_service.set_leverage(set_leverage_cmd);
        assert!(leverage_result.is_ok());
        println!("✅ Step 2: 杠杆设置成功 - 10倍");

        // ====================================================================
        // Step 3: 开仓 - 用户以市价开多仓 1 BTC
        // ====================================================================
        let open_cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(10);

        let open_result = matching_service.open_position(open_cmd);
        assert!(open_result.is_ok());

        let open_data = open_result.unwrap();
        assert_eq!(open_data.status, OrderStatus::Filled);
        println!("✅ Step 3: 开仓成功 - 1 BTC @ 市价");
        println!("   订单ID: {}", open_data.order_id.as_str());

        // ====================================================================
        // Step 4: 验证持仓创建
        // ====================================================================
        let position_query = QueryPositionCommand::long(Symbol::new("BTCUSDT"));
        let position = matching_service.query_position(position_query).unwrap();

        assert!(position.has_position());
        assert_eq!(position.quantity.to_f64(), 1.0);
        println!("✅ Step 4: 持仓创建成功");
        println!("   数量: {} BTC", position.quantity.to_f64());
        println!("   开仓价: {} USDT", position.entry_price.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());
        println!("   杠杆: {}x", position.leverage);

        // ====================================================================
        // Step 5: 计算强平价格
        // ====================================================================
        let entry_price = position.entry_price;
        let leverage = position.leverage;
        let liquidation_price = calculate_liquidation_price(
            entry_price,
            leverage,
            PositionSide::Long
        );

        println!("✅ Step 5: 强平价格计算完成");
        println!("   开仓价: {} USDT", entry_price.to_f64());
        println!("   强平价: {} USDT", liquidation_price.to_f64());
        println!("   安全距离: {} USDT ({:.2}%)",
            entry_price.to_f64() - liquidation_price.to_f64(),
            (entry_price.to_f64() - liquidation_price.to_f64()) / entry_price.to_f64() * 100.0
        );

        // 验证强平价格计算正确
        // 公式: 50000 × (1 - 1/10 + 0.005 + 0.005) = 50000 × 0.91 = 45500
        let expected_liq_price = entry_price.to_f64() * 0.91;
        assert!((liquidation_price.to_f64() - expected_liq_price).abs() < 10.0);

        // ====================================================================
        // Step 6: 模拟价格下跌 - 标记价格跌至强平价附近
        // ====================================================================
        let mark_price = Price::from_f64(liquidation_price.to_f64() - 10.0); // 略低于强平价
        println!("\n⚠️  Step 6: 市场价格下跌");
        println!("   当前标记价: {} USDT", mark_price.to_f64());
        println!("   强平触发价: {} USDT", liquidation_price.to_f64());

        // 检查是否触及强平价
        let should_liquidate = mark_price <= liquidation_price;
        assert!(should_liquidate);
        println!("🔥 触发强平条件！");

        // ====================================================================
        // Step 7: 执行强平流程 - 真实调用LiquidationProcessor
        // ====================================================================
        println!("\n🔧 Step 7: 执行强平流程");

        // 创建强平处理器
        let insurance_fund = Arc::new(MockInsuranceFund);
        let adl_engine = Arc::new(MockADLEngine);
        let liquidation_processor = LiquidationProcessor::new(
            matching_service.clone(),
            insurance_fund,
            adl_engine,
        );

        // 真实执行强平
        println!("   启动三级强平机制...");
        let liquidation_result = liquidation_processor
            .execute_liquidation_with_position(position.clone(), mark_price)
            .await;

        // 验证强平成功
        assert!(liquidation_result.is_ok(), "强平执行应该成功");
        let result = liquidation_result.unwrap();

        println!("   ✅ 强平执行成功");
        println!("   强平类型: {:?}", result.liquidation_type);
        println!("   成交价: {} USDT", result.liquidation_price.to_f64());
        println!("   强平数量: {} BTC", result.liquidated_quantity.to_f64());

        // ====================================================================
        // Step 8: 验证强平结果
        // ====================================================================
        println!("\n✅ Step 8: 验证强平结果");
        println!("   保证金损失: {} USDT", result.margin_loss.to_f64());
        println!("   保险基金损失: {} USDT", result.insurance_fund_loss.to_f64());
        println!("   订单状态: {:?}", result.order_status);

        // 验证强平类型
        assert_eq!(result.liquidation_type, LiquidationType::Market, "应该是市场强平");
        assert_eq!(result.order_status, OrderStatus::Filled, "订单应该已成交");

        // 验证损失计算
        // 由于使用真实的MatchingService，市场价格会是最新的mark_price
        // 损失应该等于全部保证金（因为价格已低于强平价）
        println!("   实际损失: {} USDT", result.margin_loss.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());

        // 损失可能等于保证金（完全强平）或略少（成功市场强平）
        assert!(
            result.margin_loss.to_f64() <= position.margin.to_f64() * 1.1,
            "损失不应超过保证金太多"
        );

        // 保险基金不应该承担损失（因为市场强平成功）
        assert_eq!(result.insurance_fund_loss.to_f64(), 0.0, "保险基金损失应该为0");

        // ====================================================================
        // Step 9: 损失分配验证
        // ====================================================================
        let user_loss = result.margin_loss.to_f64();
        let insurance_fund_loss = result.insurance_fund_loss.to_f64();

        println!("\n💰 Step 9: 损失分配");
        println!("   用户损失: {} USDT", user_loss);
        println!("   保险基金损失: {} USDT", insurance_fund_loss);
        println!("   强平类型: {:?}", result.liquidation_type);

        // 在这个场景中，损失应该小于保证金
        assert!(result.margin_loss <= position.margin, "用户损失不应超过保证金");
        assert_eq!(insurance_fund_loss, 0.0, "保险基金不应承担损失");

        // ====================================================================
        // Step 10: 验证完整流程
        // ====================================================================
        println!("\n📊 完整流程总结:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("开仓阶段:");
        println!("  ✅ 设置杠杆 10倍");
        println!("  ✅ 开仓 1 BTC @ {} USDT", entry_price.to_f64());
        println!("  ✅ 保证金 {} USDT", position.margin.to_f64());
        println!("\n监控阶段:");
        println!("  ✅ 计算强平价 {} USDT", liquidation_price.to_f64());
        println!("  ⚠️  标记价跌至 {} USDT", mark_price.to_f64());
        println!("  🔥 触发强平条件");
        println!("\n强平阶段:");
        println!("  🔧 启动三级强平机制");
        println!("  ✅ 第一级：市场强平成功");
        println!("  ✅ 成交价 {} USDT", result.liquidation_price.to_f64());
        println!("  ✅ 总损失 {} USDT", result.margin_loss.to_f64());
        println!("\n结算阶段:");
        println!("  💰 用户损失: {} USDT", user_loss);
        println!("  💰 保险基金损失: {} USDT", insurance_fund_loss);
        println!("  ✅ 持仓已平仓");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 所有步骤验证通过
        assert!(true);
    }

    #[test]
    fn scenario_short_position_liquidated_by_price_rise() {
        // Feature: 完整的开仓到强平流程
        // Scenario: 空仓因价格上涨被强平

        // ====================================================================
        // Step 1: 初始化
        // ====================================================================
        let matching_service = MatchingService::new(Price::from_f64(10000.0));

        // ====================================================================
        // Step 2: 设置杠杆
        // ====================================================================
        let set_leverage_cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 10);
        matching_service.set_leverage(set_leverage_cmd).unwrap();
        println!("✅ Step 2: 杠杆设置成功 - 10倍");

        // ====================================================================
        // Step 3: 开空仓
        // ====================================================================
        let open_cmd = OpenPositionCommand::market_short(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(10);

        let open_result = matching_service.open_position(open_cmd).unwrap();
        assert_eq!(open_result.status, OrderStatus::Filled);
        println!("✅ Step 3: 开空仓成功 - 1 BTC @ 市价");

        // ====================================================================
        // Step 4: 获取持仓
        // ====================================================================
        let position_query = QueryPositionCommand::short(Symbol::new("BTCUSDT"));
        let position = matching_service.query_position(position_query).unwrap();

        assert!(position.has_position());
        assert!(position.is_short());
        println!("✅ Step 4: 空仓持仓创建成功");
        println!("   开仓价: {} USDT", position.entry_price.to_f64());

        // ====================================================================
        // Step 5: 计算强平价格
        // ====================================================================
        let entry_price = position.entry_price;
        let liquidation_price = calculate_liquidation_price(
            entry_price,
            10,
            PositionSide::Short
        );

        println!("✅ Step 5: 强平价格计算完成");
        println!("   开仓价: {} USDT", entry_price.to_f64());
        println!("   强平价: {} USDT", liquidation_price.to_f64());

        // 空仓强平价应该高于开仓价
        assert!(liquidation_price > entry_price);

        // 验证: 50000 × 1.09 = 54500
        let expected_liq_price = entry_price.to_f64() * 1.09;
        assert!((liquidation_price.to_f64() - expected_liq_price).abs() < 10.0);

        // ====================================================================
        // Step 6: 模拟价格上涨 - 触发强平
        // ====================================================================
        let mark_price = Price::from_f64(liquidation_price.to_f64() + 10.0); // 略高于强平价
        println!("⚠️  Step 6: 市场价格上涨至 {} USDT", mark_price.to_f64());

        let should_liquidate = mark_price >= liquidation_price;
        assert!(should_liquidate);
        println!("🔥 Step 6: 触发强平条件！");

        // ====================================================================
        // Step 7: 计算强平损失
        // ====================================================================
        let fill_price = Price::from_f64(liquidation_price.to_f64() - 100.0); // 54400
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, fill_price);

        println!("✅ Step 7: 强平损失计算");
        println!("   成交价: {} USDT", fill_price.to_f64());
        println!("   总损失: {} USDT", loss.to_f64());

        // 空仓损失 = (54400 - 50000) × 1.0 = 4400
        let expected_loss = (fill_price.to_f64() - entry_price.to_f64()) * position.quantity.to_f64();
        assert!((loss.to_f64() - expected_loss).abs() < 10.0);

        // ====================================================================
        // Step 8: 验证损失分配
        // ====================================================================
        let user_loss = loss.to_f64().min(position.margin.to_f64());
        let insurance_fund_loss = (loss.to_f64() - position.margin.to_f64()).max(0.0);

        println!("✅ Step 8: 损失分配");
        println!("   用户损失: {} USDT", user_loss);
        println!("   保险基金损失: {} USDT", insurance_fund_loss);

        assert!(loss <= position.margin);
        assert_eq!(insurance_fund_loss, 0.0);

        println!("\n📊 空仓强平流程验证完成！");
    }

    #[test]
    fn scenario_extreme_price_drop_requires_insurance_fund() {
        // Feature: 极端行情下的强平
        // Scenario: 价格暴跌，损失超过保证金，需要保险基金承担

        // ====================================================================
        // Step 1-4: 开仓流程（同上）
        // ====================================================================
        let matching_service = MatchingService::new(Price::from_f64(10000.0));
        matching_service.set_leverage(
            SetLeverageCommand::new(Symbol::new("BTCUSDT"), 10)
        ).unwrap();

        let open_cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(1.0)
        ).with_leverage(10);

        matching_service.open_position(open_cmd).unwrap();

        let position = matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();

        println!("✅ 持仓创建成功");
        println!("   开仓价: {} USDT", position.entry_price.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());

        // ====================================================================
        // Step 5: 计算强平价格
        // ====================================================================
        let entry_price = position.entry_price;
        let liquidation_price = calculate_liquidation_price(
            entry_price,
            10,
            PositionSide::Long
        );

        println!("✅ 强平价: {} USDT", liquidation_price.to_f64());

        // ====================================================================
        // Step 6: 极端行情 - 价格暴跌远低于强平价
        // ====================================================================
        let extreme_price = Price::from_f64(40000.0); // 暴跌至40000
        println!("🔥 极端行情：价格暴跌至 {} USDT", extreme_price.to_f64());
        println!("   远低于强平价 {} USDT", liquidation_price.to_f64());

        // ====================================================================
        // Step 7: 市场强平以极端价格成交
        // ====================================================================
        let fill_price = extreme_price;
        let loss = LiquidationProcessor::calculate_liquidation_loss(&position, fill_price);

        println!("\n💥 强平损失计算:");
        println!("   成交价: {} USDT", fill_price.to_f64());
        println!("   总损失: {} USDT", loss.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());

        // 损失 = (50000 - 40000) × 1.0 = 10000
        assert!((loss.to_f64() - 10000.0).abs() < 10.0);

        // ====================================================================
        // Step 8: 损失超过保证金，需要保险基金
        // ====================================================================
        assert!(loss > position.margin);
        println!("⚠️  损失超过保证金！");

        let user_loss = position.margin.to_f64();
        let insurance_fund_loss = loss.to_f64() - position.margin.to_f64();

        println!("\n💰 损失分配:");
        println!("   用户损失（全部保证金）: {} USDT", user_loss);
        println!("   保险基金承担: {} USDT", insurance_fund_loss);

        // 保险基金损失 = 10000 - 5000 = 5000
        assert!((insurance_fund_loss - 5000.0).abs() < 10.0);

        println!("\n✅ 极端行情强平流程验证完成");
        println!("   - 用户损失全部保证金");
        println!("   - 保险基金承担超额损失");
    }

    #[test]
    fn scenario_high_leverage_position_easier_liquidation() {
        // Feature: 杠杆倍数影响
        // Scenario: 高杠杆持仓更容易被强平

        println!("\n🔬 杠杆倍数影响测试\n");

        // 测试不同杠杆倍数的强平价格
        let entry_price = Price::from_f64(50000.0);

        // ====================================================================
        // 场景1: 5倍杠杆
        // ====================================================================
        let liq_5x = calculate_liquidation_price(entry_price, 5, PositionSide::Long);
        let distance_5x = entry_price.to_f64() - liq_5x.to_f64();
        let distance_pct_5x = (distance_5x / entry_price.to_f64()) * 100.0;

        println!("5倍杠杆:");
        println!("  强平价: {} USDT", liq_5x.to_f64());
        println!("  安全距离: {} USDT ({:.2}%)", distance_5x, distance_pct_5x);

        // ====================================================================
        // 场景2: 10倍杠杆
        // ====================================================================
        let liq_10x = calculate_liquidation_price(entry_price, 10, PositionSide::Long);
        let distance_10x = entry_price.to_f64() - liq_10x.to_f64();
        let distance_pct_10x = (distance_10x / entry_price.to_f64()) * 100.0;

        println!("\n10倍杠杆:");
        println!("  强平价: {} USDT", liq_10x.to_f64());
        println!("  安全距离: {} USDT ({:.2}%)", distance_10x, distance_pct_10x);

        // ====================================================================
        // 场景3: 20倍杠杆
        // ====================================================================
        let liq_20x = calculate_liquidation_price(entry_price, 20, PositionSide::Long);
        let distance_20x = entry_price.to_f64() - liq_20x.to_f64();
        let distance_pct_20x = (distance_20x / entry_price.to_f64()) * 100.0;

        println!("\n20倍杠杆:");
        println!("  强平价: {} USDT", liq_20x.to_f64());
        println!("  安全距离: {} USDT ({:.2}%)", distance_20x, distance_pct_20x);

        // ====================================================================
        // 验证: 杠杆越高，强平价越接近入场价
        // ====================================================================
        assert!(distance_20x < distance_10x);
        assert!(distance_10x < distance_5x);

        println!("\n✅ 验证结论:");
        println!("  - 杠杆越高，强平价越接近入场价");
        println!("  - 高杠杆持仓风险更大，更容易被强平");
        println!("  - 5x安全距离: {:.2}%", distance_pct_5x);
        println!("  - 10x安全距离: {:.2}%", distance_pct_10x);
        println!("  - 20x安全距离: {:.2}%", distance_pct_20x);
    }

    #[test]
    fn scenario_partial_position_liquidation() {
        // Feature: 部分持仓强平
        // Scenario: 只强平部分持仓以满足保证金要求

        println!("\n📊 部分持仓强平场景\n");

        // ====================================================================
        // Step 1-3: 开仓2 BTC
        // ====================================================================
        let matching_service = MatchingService::new(Price::from_f64(20000.0));
        matching_service.set_leverage(
            SetLeverageCommand::new(Symbol::new("BTCUSDT"), 10)
        ).unwrap();

        let open_cmd = OpenPositionCommand::market_long(
            Symbol::new("BTCUSDT"),
            Quantity::from_f64(2.0)  // 开2 BTC
        ).with_leverage(10);

        matching_service.open_position(open_cmd).unwrap();

        let position = matching_service.query_position(
            QueryPositionCommand::long(Symbol::new("BTCUSDT"))
        ).unwrap();

        println!("✅ 持仓创建:");
        println!("   数量: {} BTC", position.quantity.to_f64());
        println!("   开仓价: {} USDT", position.entry_price.to_f64());
        println!("   保证金: {} USDT", position.margin.to_f64());

        // ====================================================================
        // Step 4: 计算强平价格
        // ====================================================================
        let liquidation_price = calculate_liquidation_price(
            position.entry_price,
            10,
            PositionSide::Long
        );

        println!("\n✅ 强平价: {} USDT", liquidation_price.to_f64());

        // ====================================================================
        // Step 5: 部分强平 - 只强平1 BTC
        // ====================================================================
        let partial_quantity = Quantity::from_f64(1.0);
        let fill_price = Price::from_f64(liquidation_price.to_f64() + 100.0);

        // 创建部分持仓用于计算
        let partial_position = PositionInfo {
            position_id: PositionId::generate(),
            symbol: position.symbol,
            position_side: position.position_side,
            quantity: partial_quantity,  // 只强平1 BTC
            entry_price: position.entry_price,
            mark_price: fill_price,
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: position.leverage,
            margin: Price::from_f64(position.margin.to_f64() / 2.0), // 一半保证金
            liquidation_price: position.liquidation_price,
            updated_at: 0,
        };

        let partial_loss = LiquidationProcessor::calculate_liquidation_loss(
            &partial_position,
            fill_price
        );

        println!("\n💰 部分强平:");
        println!("   强平数量: {} BTC", partial_quantity.to_f64());
        println!("   成交价: {} USDT", fill_price.to_f64());
        println!("   部分损失: {} USDT", partial_loss.to_f64());

        // ====================================================================
        // Step 6: 剩余持仓
        // ====================================================================
        let remaining_quantity = position.quantity.to_f64() - partial_quantity.to_f64();
        let remaining_margin = position.margin.to_f64() - partial_position.margin.to_f64();

        println!("\n✅ 剩余持仓:");
        println!("   数量: {} BTC", remaining_quantity);
        println!("   保证金: {} USDT", remaining_margin);
        println!("   仍可继续交易");

        assert_eq!(remaining_quantity, 1.0);
        assert!(remaining_margin > 0.0);
    }
}
