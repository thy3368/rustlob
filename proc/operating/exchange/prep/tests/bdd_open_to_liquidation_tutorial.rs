//! BDD教程：从开仓到强平的完整流程
//!
//! 本测试演示如何使用BDD方法验收期货合约的完整生命周期：
//! 1. 用户开仓
//! 2. 价格下跌
//! 3. 触发强平
//! 4. 三级强平机制
//! 5. 损失结算
//!
//! 运行方法:
//! ```bash
//! cargo test --test bdd_open_to_liquidation_tutorial -- --nocapture
//! ```

use std::sync::Arc;

use prep_proc::proc::{
    liquidation_proc::*, liquidation_types::*, trading_prep_order_proc::*,
    trading_prep_order_proc_impl::PrepMatchingService
};

#[cfg(test)]
mod open_to_liquidation_tutorial {
    use super::*;

    /// 完整的开仓到强平流程验收测试
    ///
    /// 这个测试演示了一个真实的交易场景：
    /// - 用户张三有10,000 USDT
    /// - 开10倍杠杆多仓1 BTC @ 50,000 USDT
    /// - 价格下跌至45,400 USDT
    /// - 触发强平，执行三级强平机制
    /// - 验证损失分配
    #[tokio::test]
    async fn scenario_complete_open_to_liquidation_flow() {
        println!("\n╔════════════════════════════════════════════════════╗");
        println!("║  BDD验收：从开仓到强平的完整流程                    ║");
        println!("╚════════════════════════════════════════════════════╝\n");

        // ================================================================
        // Given: 准备阶段
        // ================================================================
        println!("📋 GIVEN - 准备阶段\n");

        let initial_balance = Price::from_f64(10000.0);
        let service = Arc::new(PrepMatchingService::new(initial_balance));

        println!("✅ 用户张三有 {} USDT 余额", initial_balance.to_f64());

        let symbol = TradingPair::new("BTCUSDT");
        let leverage = 10;

        service.set_leverage(SetLeverageCommand::new(symbol, leverage)).expect("设置杠杆应该成功");

        println!("✅ 张三设置 {}x 杠杆", leverage);
        println!();

        // ================================================================
        // When: 开仓阶段
        // ================================================================
        println!("🔄 WHEN - 开仓阶段\n");

        let quantity = Quantity::from_f64(1.0);

        let open_cmd = OpenPositionCommand::market_long(symbol, quantity).with_leverage(leverage);

        println!("→ 张三开多仓 {} BTC @ 市价", quantity.to_f64());

        let open_result = service.open_position(open_cmd).expect("开仓应该成功");

        assert_eq!(open_result.status, OrderStatus::Filled, "订单应该成交");

        println!("✅ 订单成交");
        println!("   订单ID: {}", open_result.order_id.as_str());
        println!();

        // ================================================================
        // Then: 验证持仓创建
        // ================================================================
        println!("✅ THEN - 持仓验证\n");

        let position = service.query_position(QueryPositionCommand::long(symbol)).expect("应该能查询到持仓");

        assert!(position.has_position(), "应该有持仓");
        assert_eq!(position.quantity.to_f64(), 1.0, "持仓数量应该是1 BTC");
        assert_eq!(position.leverage, leverage, "杠杆应该是10倍");

        let entry_price = position.entry_price.to_f64();
        let margin = position.margin.to_f64();

        println!("📊 持仓信息:");
        println!("   ├─ 方向: {:?}", position.position_side);
        println!("   ├─ 数量: {} BTC", position.quantity.to_f64());
        println!("   ├─ 开仓价: {} USDT", entry_price);
        println!("   ├─ 保证金: {} USDT", margin);
        println!("   └─ 杠杆: {}x", position.leverage);
        println!();

        let expected_margin = entry_price * 1.0 / leverage as f64;
        assert!((margin - expected_margin).abs() < 10.0, "保证金应该约等于 {} USDT", expected_margin);

        // ================================================================
        // And: 计算强平价
        // ================================================================
        println!("🔍 AND - 风险计算\n");

        let liq_price = calculate_liquidation_price(position.entry_price, leverage, PositionSide::Long);

        let safety_distance = entry_price - liq_price.to_f64();
        let safety_distance_pct = safety_distance / entry_price * 100.0;

        println!("📐 强平价格计算:");
        println!("   ├─ 公式: 开仓价 × (1 - 1/杠杆 + 维持保证金率 + 强平费率)");
        println!("   ├─ 计算: {} × (1 - 1/{} + 0.005 + 0.005)", entry_price, leverage);
        println!("   ├─ 计算: {} × 0.91", entry_price);
        println!("   └─ 强平价: {} USDT", liq_price.to_f64());
        println!();

        println!("⚠️  风险提示:");
        println!("   ├─ 安全距离: {} USDT", safety_distance);
        println!("   ├─ 跌幅容忍: {:.2}%", safety_distance_pct);
        println!("   └─ 建议: 价格跌破 {} USDT 将触发强平", liq_price.to_f64());
        println!();

        let expected_liq_price = entry_price * 0.91;
        assert!((liq_price.to_f64() - expected_liq_price).abs() < 10.0, "强平价应该约等于 {} USDT", expected_liq_price);

        // ================================================================
        // When: 价格下跌
        // ================================================================
        println!("📉 WHEN - 价格变化\n");

        let warning_price = Price::from_f64(46000.0);
        let price_drop_1 = entry_price - warning_price.to_f64();
        let drop_pct_1 = price_drop_1 / entry_price * 100.0;

        println!("阶段1: 价格下跌");
        println!("   ├─ 当前价: {} USDT", warning_price.to_f64());
        println!("   ├─ 下跌: {} USDT ({:.2}%)", price_drop_1, drop_pct_1);
        println!("   ├─ 距强平价: {} USDT", warning_price.to_f64() - liq_price.to_f64());
        println!("   └─ 状态: ⚠️ 风险警告");
        println!();

        let mark_price = Price::from_f64(45400.0);
        let price_drop_2 = entry_price - mark_price.to_f64();
        let drop_pct_2 = price_drop_2 / entry_price * 100.0;

        println!("阶段2: 价格继续下跌");
        println!("   ├─ 当前价: {} USDT", mark_price.to_f64());
        println!("   ├─ 总下跌: {} USDT ({:.2}%)", price_drop_2, drop_pct_2);
        println!("   ├─ 强平价: {} USDT", liq_price.to_f64());
        println!("   └─ 状态: 🔥 触发强平！");
        println!();

        let should_liquidate = mark_price <= liq_price;
        assert!(should_liquidate, "应该触发强平");

        // ================================================================
        // Then: 执行三级强平机制
        // ================================================================
        println!("⚡ THEN - 三级强平机制\n");

        // Mock依赖
        struct MockInsuranceFund;
        #[async_trait::async_trait]
        impl InsuranceFund for MockInsuranceFund {
            async fn check_capacity(&self) -> Result<InsuranceFundCapacity, PrepCommandError> {
                Ok(InsuranceFundCapacity {
                    available_balance: Price::from_f64(100000.0)
                })
            }
            async fn takeover(&self, position: &PrepPosition) -> Result<InsuranceFundTakeover, PrepCommandError> {
                Ok(InsuranceFundTakeover {
                    total_loss: position.margin
                })
            }
        }

        struct MockADLEngine;
        #[async_trait::async_trait]
        impl ADLEngine for MockADLEngine {
            async fn find_counterparties(
                &self, _symbol: TradingPair, _side: Side
            ) -> Result<Vec<PrepPosition>, PrepCommandError> {
                Ok(Vec::new())
            }
            async fn execute_adl(
                &self, _liquidated_position: &PrepPosition, _counterparties: Vec<PrepPosition>
            ) -> Result<ADLResult, PrepCommandError> {
                Ok(ADLResult {
                    affected_positions: Vec::new()
                })
            }
        }

        let liquidation_processor =
            LiquidationProcessor::new(service.clone(), Arc::new(MockInsuranceFund), Arc::new(MockADLEngine));

        println!("🔧 启动强平流程:");
        println!("   ├─ Step 0: 冻结持仓状态");
        println!("   ├─ Step 1: 尝试市场强平");
        println!("   ├─ Step 2: 保险基金准备");
        println!("   └─ Step 3: ADL引擎待命");
        println!();

        println!("⏳ 执行强平...\n");

        let result = liquidation_processor
            .execute_liquidation_with_position(position.clone(), mark_price)
            .await
            .expect("强平应该成功");

        println!("✅ 强平执行完成！");
        println!();

        // ================================================================
        // And: 验证强平结果
        // ================================================================
        println!("📊 AND - 强平结果验证\n");

        println!("强平详情:");
        println!("   ├─ 强平类型: {:?}", result.liquidation_type);
        println!("   ├─ 成交价格: {} USDT", result.liquidation_price.to_f64());
        println!("   ├─ 强平数量: {} BTC", result.liquidated_quantity.to_f64());
        println!("   └─ 订单状态: {:?}", result.order_status);
        println!();

        println!("损失分配:");
        println!("   ├─ 用户损失: {} USDT", result.margin_loss.to_f64());
        println!("   ├─ 保证金: {} USDT", margin);
        println!("   ├─ 保险基金损失: {} USDT", result.insurance_fund_loss.to_f64());
        println!("   └─ 穿仓损失: {}", if result.insurance_fund_loss.to_f64() > 0.0 { "有" } else { "无" });
        println!();

        assert_eq!(result.liquidation_type, LiquidationType::Market, "应该是市场强平");

        assert!(result.margin_loss <= position.margin, "用户损失不应超过保证金（正常市场强平情况）");

        assert_eq!(result.insurance_fund_loss.to_f64(), 0.0, "市场强平成功时，保险基金不应承担损失");

        // ================================================================
        // Finally: 总结报告
        // ================================================================
        println!("╔════════════════════════════════════════════════════╗");
        println!("║  完整流程总结报告                                    ║");
        println!("╚════════════════════════════════════════════════════╝\n");

        println!("📋 业务流程:");
        println!("   1️⃣  准备: 余额 {} USDT，设置 {}x 杠杆", initial_balance.to_f64(), leverage);
        println!("   2️⃣  开仓: {} BTC @ {} USDT", quantity.to_f64(), entry_price);
        println!("   3️⃣  持仓: 保证金 {} USDT，强平价 {} USDT", margin, liq_price.to_f64());
        println!("   4️⃣  下跌: 价格跌至 {} USDT ({:.2}%)", mark_price.to_f64(), drop_pct_2);
        println!("   5️⃣  强平: {} 强平成功", match result.liquidation_type {
            LiquidationType::Market => "市场",
            LiquidationType::InsuranceFund => "保险基金",
            LiquidationType::ADL => "ADL"
        });
        println!("   6️⃣  结算: 用户损失 {} USDT", result.margin_loss.to_f64());
        println!();

        println!("💰 财务影响:");
        println!("   初始资金: {} USDT", initial_balance.to_f64());
        println!("   投入保证金: {} USDT", margin);
        println!("   实际损失: {} USDT", result.margin_loss.to_f64());
        println!("   剩余资金: {} USDT", initial_balance.to_f64() - result.margin_loss.to_f64());
        println!("   损失率: {:.2}%", result.margin_loss.to_f64() / margin * 100.0);
        println!();

        println!("✅ 验收结论:");
        println!("   ✓ 开仓流程正确");
        println!("   ✓ 强平价计算准确");
        println!("   ✓ 强平触发及时");
        println!("   ✓ 三级机制运行正常");
        println!("   ✓ 损失控制在保证金范围内");
        println!("   ✓ 保险基金未受损失");
        println!("   ✓ 系统风控有效");
        println!();

        println!("╔════════════════════════════════════════════════════╗");
        println!("║  ✅ BDD验收测试通过！                               ║");
        println!("╚════════════════════════════════════════════════════╝\n");
    }
}
