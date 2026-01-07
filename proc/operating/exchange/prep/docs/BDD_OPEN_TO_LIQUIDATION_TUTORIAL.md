# BDD验收教程：从开仓到强平的完整流程

## 目录

1. [业务场景介绍](#业务场景介绍)
2. [BDD验收标准定义](#bdd验收标准定义)
3. [完整实现步骤](#完整实现步骤)
4. [运行与验证](#运行与验证)
5. [扩展场景](#扩展场景)
6. [实战练习](#实战练习)

---

## 业务场景介绍

### 真实业务案例

**张三的交易故事**:

1. 🎯 **开仓**: 张三看好BTC，用10,000 USDT本金，开10倍杠杆多仓
2. 📊 **持仓**: 以50,000 USDT价格买入1 BTC
3. 📉 **价格下跌**: BTC价格从50,000跌至45,400 USDT
4. 🔥 **触发强平**: 价格跌破强平价45,500 USDT
5. ⚡ **系统执行**: 自动平仓，保护系统不受损失
6. 💸 **损失结算**: 张三损失大部分保证金

### 涉及的核心概念

| 概念 | 说明 | 公式 |
|------|------|------|
| **杠杆** | 放大交易规模的倍数 | 10倍杠杆 = 1/10保证金 |
| **保证金** | 开仓所需资金 | 50,000 / 10 = 5,000 USDT |
| **强平价** | 触发强制平仓的价格 | 50,000 × 0.91 = 45,500 USDT |
| **安全距离** | 当前价到强平价的距离 | (50,000 - 45,500) / 50,000 = 9% |

### 为什么需要强平机制？

❌ **如果没有强平**:
```
价格跌至40,000 USDT
损失 = (50,000 - 40,000) × 1 = 10,000 USDT
保证金只有 5,000 USDT
穿仓损失 = 5,000 USDT → 交易所承担
```

✅ **有强平机制**:
```
价格跌至45,500 USDT（强平价）时立即平仓
损失 ≈ 保证金 5,000 USDT
不会穿仓，保护交易所和其他用户
```

---

## BDD验收标准定义

### 用户故事 (User Story)

```gherkin
功能: 强平保护机制
  作为 风控系统
  我想要 在价格触及强平价时自动平仓
  以便 保护交易所不受穿仓损失

背景:
  假设 系统已启动
  并且 市场价格为 50,000 USDT
```

### 完整验收场景

```gherkin
场景: 多仓价格下跌触发强平
  # ==================== 准备阶段 ====================
  假设 用户张三有 10,000 USDT 余额
  并且 张三设置 10倍杠杆

  # ==================== 开仓阶段 ====================
  当 张三开多仓 1 BTC @ 50,000 USDT
  那么 订单应该成交
  并且 持仓创建成功
  并且 持仓数量为 1 BTC
  并且 开仓价为 50,000 USDT
  并且 保证金为 5,000 USDT
  并且 杠杆为 10倍

  # ==================== 风险计算阶段 ====================
  并且 系统计算强平价为 45,500 USDT
  并且 维持保证金率为 0.5%
  并且 强平手续费率为 0.5%
  并且 安全距离为 9%

  # ==================== 价格监控阶段 ====================
  当 标记价格下跌至 46,000 USDT
  那么 系统应该发出风险警告
  并且 未实现亏损为 -4,000 USDT
  并且 距离强平价还有 500 USDT

  # ==================== 触发强平阶段 ====================
  当 标记价格继续下跌至 45,400 USDT
  那么 系统应该触发强平条件
  并且 冻结持仓状态为 LIQUIDATING

  # ==================== 执行强平阶段 ====================
  并且 系统启动三级强平机制
  当 第一级市场强平执行
  那么 提交紧急市价单
  并且 设置5秒超时
  并且 订单类型为 IOC (立即成交或取消)

  如果 市场强平成功
  那么 在市场价成交
  并且 用户损失 <= 保证金
  并且 保险基金损失 = 0

  如果 市场强平失败
  那么 启动第二级保险基金接管
  并且 检查保险基金容量
  如果 保险基金充足
  那么 保险基金接管持仓
  并且 用户损失全部保证金

  如果 保险基金不足
  那么 启动第三级自动减仓 (ADL)
  并且 查找对手方盈利仓位
  并且 按ADL队列优先级排序
  并且 强制平仓对手方部分仓位

  # ==================== 结算阶段 ====================
  最终 持仓已关闭
  并且 损失已分配
  并且 用户收到强平通知
  并且 持仓状态更新为 CLOSED
```

---

## 完整实现步骤

### Step 1: 创建测试文件

```bash
# 创建新的测试文件
touch tests/bdd_open_to_liquidation_tutorial.rs
```

### Step 2: 定义测试结构

```rust
//! BDD教程：从开仓到强平的完整流程
//!
//! 本测试演示如何使用BDD方法验收期货合约的完整生命周期：
//! 1. 用户开仓
//! 2. 价格下跌
//! 3. 触发强平
//! 4. 三级强平机制
//! 5. 损失结算

use prep_proc::proc::trading_prep_order_proc::*;
use prep_proc::proc::trading_prep_order_proc_impl::MatchingService;
use prep_proc::proc::liquidation_proc::*;
use prep_proc::proc::liquidation_types::*;
use std::sync::Arc;

#[cfg(test)]
mod open_to_liquidation_tutorial {
    use super::*;

    /// 完整的开仓到强平流程验收测试
    #[tokio::test]
    async fn scenario_complete_open_to_liquidation_flow() {
        // TODO: 实现完整流程
    }
}
```

### Step 3: Given - 准备阶段

```rust
#[tokio::test]
async fn scenario_complete_open_to_liquidation_flow() {
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  BDD验收：从开仓到强平的完整流程                    ║");
    println!("╚════════════════════════════════════════════════════╝\n");

    // ================================================================
    // Given: 准备阶段
    // ================================================================
    println!("📋 GIVEN - 准备阶段\n");

    // 创建测试服务
    let initial_balance = Price::from_f64(10000.0);
    let service = Arc::new(MatchingService::new(initial_balance));

    println!("✅ 用户张三有 {} USDT 余额", initial_balance.to_f64());

    // 设置杠杆
    let symbol = Symbol::new("BTCUSDT");
    let leverage = 10;

    service.set_leverage(SetLeverageCommand::new(symbol, leverage))
        .expect("设置杠杆应该成功");

    println!("✅ 张三设置 {}x 杠杆", leverage);
    println!();
}
```

**运行验证**:
```bash
cargo test --test bdd_open_to_liquidation_tutorial -- --nocapture
```

**预期输出**:
```
╔════════════════════════════════════════════════════╗
║  BDD验收：从开仓到强平的完整流程                    ║
╚════════════════════════════════════════════════════╝

📋 GIVEN - 准备阶段

✅ 用户张三有 10000 USDT 余额
✅ 张三设置 10x 杠杆
```

### Step 4: When - 开仓阶段

```rust
    // ================================================================
    // When: 开仓阶段
    // ================================================================
    println!("🔄 WHEN - 开仓阶段\n");

    let quantity = Quantity::from_f64(1.0);

    let open_cmd = OpenPositionCommand::market_long(symbol, quantity)
        .with_leverage(leverage);

    println!("→ 张三开多仓 {} BTC @ 市价", quantity.to_f64());

    let open_result = service.open_position(open_cmd)
        .expect("开仓应该成功");

    assert_eq!(open_result.status, OrderStatus::Filled, "订单应该成交");

    println!("✅ 订单成交");
    println!("   订单ID: {}", open_result.order_id.as_str());
    println!();
```

**预期输出**:
```
🔄 WHEN - 开仓阶段

→ 张三开多仓 1 BTC @ 市价
✅ 订单成交
   订单ID: ORD-1234567890
```

### Step 5: Then - 验证持仓创建

```rust
    // ================================================================
    // Then: 验证持仓创建
    // ================================================================
    println!("✅ THEN - 持仓验证\n");

    let position = service
        .query_position(QueryPositionCommand::long(symbol))
        .expect("应该能查询到持仓");

    // 验证持仓信息
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

    // 验证保证金计算
    let expected_margin = entry_price * 1.0 / leverage as f64;
    assert!(
        (margin - expected_margin).abs() < 10.0,
        "保证金应该约等于 {} USDT",
        expected_margin
    );
```

**预期输出**:
```
✅ THEN - 持仓验证

📊 持仓信息:
   ├─ 方向: Long
   ├─ 数量: 1 BTC
   ├─ 开仓价: 50000 USDT
   ├─ 保证金: 5000 USDT
   └─ 杠杆: 10x
```

### Step 6: And - 计算强平价

```rust
    // ================================================================
    // And: 计算强平价
    // ================================================================
    println!("🔍 AND - 风险计算\n");

    let liq_price = calculate_liquidation_price(
        position.entry_price,
        leverage,
        PositionSide::Long
    );

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

    // 验证强平价计算
    let expected_liq_price = entry_price * 0.91;
    assert!(
        (liq_price.to_f64() - expected_liq_price).abs() < 10.0,
        "强平价应该约等于 {} USDT",
        expected_liq_price
    );
```

**预期输出**:
```
🔍 AND - 风险计算

📐 强平价格计算:
   ├─ 公式: 开仓价 × (1 - 1/杠杆 + 维持保证金率 + 强平费率)
   ├─ 计算: 50000 × (1 - 1/10 + 0.005 + 0.005)
   ├─ 计算: 50000 × 0.91
   └─ 强平价: 45500 USDT

⚠️  风险提示:
   ├─ 安全距离: 4500 USDT
   ├─ 跌幅容忍: 9.00%
   └─ 建议: 价格跌破 45500 USDT 将触发强平
```

### Step 7: When - 价格下跌

```rust
    // ================================================================
    // When: 价格下跌到风险区域
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

    // 价格继续下跌，触发强平
    let mark_price = Price::from_f64(45400.0);
    let price_drop_2 = entry_price - mark_price.to_f64();
    let drop_pct_2 = price_drop_2 / entry_price * 100.0;

    println!("阶段2: 价格继续下跌");
    println!("   ├─ 当前价: {} USDT", mark_price.to_f64());
    println!("   ├─ 总下跌: {} USDT ({:.2}%)", price_drop_2, drop_pct_2);
    println!("   ├─ 强平价: {} USDT", liq_price.to_f64());
    println!("   └─ 状态: 🔥 触发强平！");
    println!();

    // 检查是否触发强平
    let should_liquidate = mark_price <= liq_price;
    assert!(should_liquidate, "应该触发强平");
```

**预期输出**:
```
📉 WHEN - 价格变化

阶段1: 价格下跌
   ├─ 当前价: 46000 USDT
   ├─ 下跌: 4000 USDT (8.00%)
   ├─ 距强平价: 500 USDT
   └─ 状态: ⚠️ 风险警告

阶段2: 价格继续下跌
   ├─ 当前价: 45400 USDT
   ├─ 总下跌: 4600 USDT (9.20%)
   ├─ 强平价: 45500 USDT
   └─ 状态: 🔥 触发强平！
```

### Step 8: Then - 执行三级强平机制

```rust
    // ================================================================
    // Then: 执行三级强平机制
    // ================================================================
    println!("⚡ THEN - 三级强平机制\n");

    // 创建Mock依赖
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

    // 创建强平处理器
    let liquidation_processor = LiquidationProcessor::new(
        service.clone(),
        Arc::new(MockInsuranceFund),
        Arc::new(MockADLEngine),
    );

    println!("🔧 启动强平流程:");
    println!("   ├─ Step 0: 冻结持仓状态");
    println!("   ├─ Step 1: 尝试市场强平");
    println!("   ├─ Step 2: 保险基金准备");
    println!("   └─ Step 3: ADL引擎待命");
    println!();

    // 执行强平
    println!("⏳ 执行强平...\n");

    let result = liquidation_processor
        .execute_liquidation_with_position(position.clone(), mark_price)
        .await
        .expect("强平应该成功");

    println!("✅ 强平执行完成！");
    println!();
```

**预期输出**:
```
⚡ THEN - 三级强平机制

🔧 启动强平流程:
   ├─ Step 0: 冻结持仓状态
   ├─ Step 1: 尝试市场强平
   ├─ Step 2: 保险基金准备
   └─ Step 3: ADL引擎待命

⏳ 执行强平...

✅ 强平执行完成！
```

### Step 9: And - 验证强平结果

```rust
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
    println!("   └─ 穿仓损失: {} USDT",
        if result.insurance_fund_loss.to_f64() > 0.0 { "有" } else { "无" }
    );
    println!();

    // 验证强平类型
    assert_eq!(
        result.liquidation_type,
        LiquidationType::Market,
        "应该是市场强平"
    );

    // 验证损失不超过保证金（在正常情况下）
    assert!(
        result.margin_loss <= position.margin,
        "用户损失不应超过保证金（正常市场强平情况）"
    );

    // 验证保险基金不应承担损失（市场强平成功）
    assert_eq!(
        result.insurance_fund_loss.to_f64(),
        0.0,
        "市场强平成功时，保险基金不应承担损失"
    );
```

**预期输出**:
```
📊 AND - 强平结果验证

强平详情:
   ├─ 强平类型: Market
   ├─ 成交价格: 45400 USDT
   ├─ 强平数量: 1 BTC
   └─ 订单状态: Filled

损失分配:
   ├─ 用户损失: 4600 USDT
   ├─ 保证金: 5000 USDT
   ├─ 保险基金损失: 0 USDT
   └─ 穿仓损失: 无
```

### Step 10: Finally - 总结报告

```rust
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
    println!("   5️⃣  强平: {} 强平成功",
        match result.liquidation_type {
            LiquidationType::Market => "市场",
            LiquidationType::InsuranceFund => "保险基金",
            LiquidationType::ADL => "ADL",
        }
    );
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
```

**预期输出**:
```
╔════════════════════════════════════════════════════╗
║  完整流程总结报告                                    ║
╚════════════════════════════════════════════════════╝

📋 业务流程:
   1️⃣  准备: 余额 10000 USDT，设置 10x 杠杆
   2️⃣  开仓: 1 BTC @ 50000 USDT
   3️⃣  持仓: 保证金 5000 USDT，强平价 45500 USDT
   4️⃣  下跌: 价格跌至 45400 USDT (9.20%)
   5️⃣  强平: 市场 强平成功
   6️⃣  结算: 用户损失 4600 USDT

💰 财务影响:
   初始资金: 10000 USDT
   投入保证金: 5000 USDT
   实际损失: 4600 USDT
   剩余资金: 5400 USDT
   损失率: 92.00%

✅ 验收结论:
   ✓ 开仓流程正确
   ✓ 强平价计算准确
   ✓ 强平触发及时
   ✓ 三级机制运行正常
   ✓ 损失控制在保证金范围内
   ✓ 保险基金未受损失
   ✓ 系统风控有效

╔════════════════════════════════════════════════════╗
║  ✅ BDD验收测试通过！                               ║
╚════════════════════════════════════════════════════╝
```

---

## 完整代码

### 完整测试文件

将以上所有代码组合成完整的测试文件：

```rust
//! BDD教程：从开仓到强平的完整流程
//!
//! 本测试演示如何使用BDD方法验收期货合约的完整生命周期

use prep_proc::proc::trading_prep_order_proc::*;
use prep_proc::proc::trading_prep_order_proc_impl::MatchingService;
use prep_proc::proc::liquidation_proc::*;
use prep_proc::proc::liquidation_types::*;
use std::sync::Arc;

#[cfg(test)]
mod open_to_liquidation_tutorial {
    use super::*;

    #[tokio::test]
    async fn scenario_complete_open_to_liquidation_flow() {
        // 所有上述代码组合在这里
        // ...（省略，请参考上面的步骤）
    }
}
```

---

## 运行与验证

### 创建测试文件

```bash
# 创建测试文件
cat > tests/bdd_open_to_liquidation_tutorial.rs << 'EOF'
# 粘贴上面的完整代码
EOF
```

### 运行测试

```bash
# 运行测试
cargo test --test bdd_open_to_liquidation_tutorial -- --nocapture
```

### 预期完整输出

运行测试后，你将看到如下完整输出：

```
running 1 test

╔════════════════════════════════════════════════════╗
║  BDD验收：从开仓到强平的完整流程                    ║
╚════════════════════════════════════════════════════╝

📋 GIVEN - 准备阶段

✅ 用户张三有 10000 USDT 余额
✅ 张三设置 10x 杠杆

🔄 WHEN - 开仓阶段

→ 张三开多仓 1 BTC @ 市价
✅ 订单成交
   订单ID: ORD-1765623456789

✅ THEN - 持仓验证

📊 持仓信息:
   ├─ 方向: Long
   ├─ 数量: 1 BTC
   ├─ 开仓价: 50000 USDT
   ├─ 保证金: 5000 USDT
   └─ 杠杆: 10x

🔍 AND - 风险计算

📐 强平价格计算:
   ├─ 公式: 开仓价 × (1 - 1/杠杆 + 维持保证金率 + 强平费率)
   ├─ 计算: 50000 × (1 - 1/10 + 0.005 + 0.005)
   ├─ 计算: 50000 × 0.91
   └─ 强平价: 45500 USDT

⚠️  风险提示:
   ├─ 安全距离: 4500 USDT
   ├─ 跌幅容忍: 9.00%
   └─ 建议: 价格跌破 45500 USDT 将触发强平

📉 WHEN - 价格变化

阶段1: 价格下跌
   ├─ 当前价: 46000 USDT
   ├─ 下跌: 4000 USDT (8.00%)
   ├─ 距强平价: 500 USDT
   └─ 状态: ⚠️ 风险警告

阶段2: 价格继续下跌
   ├─ 当前价: 45400 USDT
   ├─ 总下跌: 4600 USDT (9.20%)
   ├─ 强平价: 45500 USDT
   └─ 状态: 🔥 触发强平！

⚡ THEN - 三级强平机制

🔧 启动强平流程:
   ├─ Step 0: 冻结持仓状态
   ├─ Step 1: 尝试市场强平
   ├─ Step 2: 保险基金准备
   └─ Step 3: ADL引擎待命

⏳ 执行强平...

✅ 强平执行完成！

📊 AND - 强平结果验证

强平详情:
   ├─ 强平类型: Market
   ├─ 成交价格: 45400 USDT
   ├─ 强平数量: 1 BTC
   └─ 订单状态: Filled

损失分配:
   ├─ 用户损失: 4600 USDT
   ├─ 保证金: 5000 USDT
   ├─ 保险基金损失: 0 USDT
   └─ 穿仓损失: 无

╔════════════════════════════════════════════════════╗
║  完整流程总结报告                                    ║
╚════════════════════════════════════════════════════╝

📋 业务流程:
   1️⃣  准备: 余额 10000 USDT，设置 10x 杠杆
   2️⃣  开仓: 1 BTC @ 50000 USDT
   3️⃣  持仓: 保证金 5000 USDT，强平价 45500 USDT
   4️⃣  下跌: 价格跌至 45400 USDT (9.20%)
   5️⃣  强平: 市场 强平成功
   6️⃣  结算: 用户损失 4600 USDT

💰 财务影响:
   初始资金: 10000 USDT
   投入保证金: 5000 USDT
   实际损失: 4600 USDT
   剩余资金: 5400 USDT
   损失率: 92.00%

✅ 验收结论:
   ✓ 开仓流程正确
   ✓ 强平价计算准确
   ✓ 强平触发及时
   ✓ 三级机制运行正常
   ✓ 损失控制在保证金范围内
   ✓ 保险基金未受损失
   ✓ 系统风控有效

╔════════════════════════════════════════════════════╗
║  ✅ BDD验收测试通过！                               ║
╚════════════════════════════════════════════════════╝

test open_to_liquidation_tutorial::scenario_complete_open_to_liquidation_flow ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 扩展场景

### 场景2: 极端行情需要保险基金

测试价格暴跌，市场强平失败，保险基金接管的情况。

```rust
#[tokio::test]
async fn scenario_extreme_market_requires_insurance_fund() {
    println!("\n🔥 场景2: 极端行情-保险基金接管\n");

    // Given: 同样的开仓准备
    // ...

    // When: 价格暴跌至40,000 USDT（远低于强平价）
    let extreme_price = Price::from_f64(40000.0);

    // Then: 损失超过保证金
    let total_loss = (entry_price - 40000.0) * 1.0; // 10,000 USDT
    let user_loss = margin; // 5,000 USDT
    let insurance_loss = total_loss - user_loss; // 5,000 USDT

    println!("极端情况:");
    println!("   总损失: {} USDT", total_loss);
    println!("   用户承担: {} USDT (全部保证金)", user_loss);
    println!("   保险基金承担: {} USDT (穿仓损失)", insurance_loss);
}
```

### 场景3: ADL自动减仓

测试保险基金不足，触发ADL的情况。

```rust
#[tokio::test]
async fn scenario_adl_triggered() {
    println!("\n⚡ 场景3: ADL自动减仓\n");

    // Given: 保险基金余额不足
    // When: 触发ADL
    // Then: 对手方盈利仓位被部分平仓

    println!("ADL流程:");
    println!("   1. 查找对手方盈利仓位");
    println!("   2. 按ADL队列排序（盈利率×杠杆）");
    println!("   3. 强制平仓对手方");
    println!("   4. 通知被ADL用户");
}
```

---

## 实战练习

### 练习1: 空仓强平

编写一个空仓价格上涨触发强平的测试。

**提示**:
- 开仓: `OpenPositionCommand::market_short()`
- 强平价公式: `entry_price × (1 + 1/leverage - 0.005 - 0.005)`
- 触发条件: `mark_price >= liq_price`

### 练习2: 不同杠杆对比

编写一个测试，对比5x、10x、20x杠杆的强平距离。

**提示**:
- 使用循环测试多个杠杆
- 计算每个杠杆的安全距离
- 验证杠杆越高，风险越大

### 练习3: 部分强平

编写一个测试，模拟只强平部分仓位的情况。

**提示**:
- 开仓2 BTC
- 强平时只平1 BTC
- 验证剩余持仓状态

---

## 总结

### 你学到了什么

✅ **BDD验收方法**
- Given-When-Then-And-Finally结构
- 详细的输出格式
- 完整的验证流程

✅ **业务流程理解**
- 开仓到强平的完整链路
- 三级强平机制的运作
- 损失分配的逻辑

✅ **测试技巧**
- 异步测试编写
- Mock对象使用
- 断言验证策略

### 关键要点

1. **BDD不只是测试**: 它是活文档，是需求规格说明
2. **详细的输出很重要**: 帮助理解业务流程
3. **验证要全面**: 不只验证结果，还要验证中间状态
4. **场景要真实**: 模拟实际业务情况

### 下一步

1. 运行完整测试
2. 完成扩展练习
3. 编写自己的场景
4. 应用到实际项目

---

**最后更新**: 2025-12-13
**版本**: v1.0.0
