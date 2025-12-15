# 期货交易业务流程文档

## 概述

本文档描述永续合约交易的三个核心业务流程：
1. **正常交易流程**: 设置杠杆 → 开仓 → 主动平仓
2. **强平流程**: 开仓 → 价格触发强平 → 三级强平机制
3. **风险管理**: 监控持仓风险，防止穿仓

---

## 流程 1: 正常交易流程

这是最常见的交易场景，用户主动开平仓获利或止损。

### 业务流程图

```
┌─────────────────────────────────────────────────────────────────────┐
│                         正常交易完整流程                               │
└─────────────────────────────────────────────────────────────────────┘

Step 1: 初始化账户                    Step 2: 设置杠杆
┌──────────────────┐                ┌──────────────────┐
│  用户存入资金     │                │ SetLeverageCommand│
│  Balance: 10000  │ ──────────────▶│  Leverage: 10x   │
│  USDT            │                │  Symbol: BTCUSDT │
└──────────────────┘                └──────────────────┘
                                              │
                                              │
Step 3: 开仓                                  ▼
┌──────────────────┐                ┌──────────────────┐
│OpenPositionCommand│               │  杠杆配置已更新   │
│  Type: Market    │◀──────────────│  Available: 10000│
│  Side: Long      │                │  USDT            │
│  Quantity: 1 BTC │                └──────────────────┘
│  Leverage: 10x   │
└──────────────────┘
         │
         │ 撮合成交
         ▼
┌──────────────────────────────────────────┐
│         持仓创建成功                      │
│  ┌────────────────────────────────────┐  │
│  │ Position Information               │  │
│  │  - Direction: Long                 │  │
│  │  - Quantity: 1 BTC                 │  │
│  │  - Entry Price: 50,000 USDT        │  │
│  │  - Margin: 5,000 USDT              │  │
│  │  - Leverage: 10x                   │  │
│  │  - Liquidation Price: 45,500 USDT  │  │
│  └────────────────────────────────────┘  │
└──────────────────────────────────────────┘
         │
         │
Step 4: 价格变化                      Step 5: 主动平仓
         │                           ┌──────────────────┐
         │  价格上涨至 55,000 USDT   │ClosePositionCommand│
         ▼                           │  Type: Market    │
┌──────────────────┐                │  Quantity: All   │
│  未实现盈亏计算   │                │  Side: Sell      │
│  PnL = +5,000    │ ──────────────▶│                  │
│  USDT (+10%)     │                └──────────────────┘
└──────────────────┘                         │
                                             │ 撮合成交
                                             ▼
                                    ┌──────────────────┐
                                    │   平仓成功       │
                                    │  Realized PnL:   │
                                    │  +5,000 USDT     │
                                    │  Return: 100%    │
                                    └──────────────────┘
```

### 关键步骤说明

#### Step 1: 初始化账户
```rust
let initial_balance = Price::from_f64(10000.0);
let matching_service = MatchingService::new(initial_balance);
```
- 用户存入 10,000 USDT 作为初始保证金
- 创建交易服务实例

#### Step 2: 设置杠杆
```rust
let set_leverage_cmd = SetLeverageCommand::new(
    Symbol::new("BTCUSDT"),
    10  // 10倍杠杆
);
matching_service.set_leverage(set_leverage_cmd)?;
```
- 用户设置 10 倍杠杆
- 影响开仓保证金占用和强平价计算

#### Step 3: 开仓
```rust
let open_cmd = OpenPositionCommand::market_long(
    Symbol::new("BTCUSDT"),
    Quantity::from_f64(1.0)
).with_leverage(10);

let result = matching_service.open_position(open_cmd)?;
```

**开仓计算**:
- 持仓价值 = 1 BTC × 50,000 USDT = 50,000 USDT
- 所需保证金 = 50,000 / 10 = 5,000 USDT
- 强平价格 = 50,000 × 0.91 = 45,500 USDT (多仓)

**强平价格公式** (多仓):
```
强平价 = 开仓价 × (1 - 1/杠杆 + 维持保证金率 + 强平手续费率)
      = 50000 × (1 - 1/10 + 0.005 + 0.005)
      = 50000 × 0.91 = 45,500 USDT
```

#### Step 4: 持仓监控
```rust
let position = matching_service.query_position(
    QueryPositionCommand::long(symbol)
)?;

// 计算未实现盈亏
let unrealized_pnl = (current_price - entry_price) × quantity
                   = (55,000 - 50,000) × 1.0 = 5,000 USDT
```

#### Step 5: 主动平仓
```rust
// 全仓平仓
let close_cmd = ClosePositionCommand::market_close_long(symbol, None);

// 部分平仓
let close_cmd = ClosePositionCommand::market_close_long(
    symbol,
    Some(Quantity::from_f64(0.5))
);

let result = matching_service.close_position(close_cmd)?;
```

**实现盈亏计算**:
- 平仓价格: 55,000 USDT
- 实现盈亏 = (55,000 - 50,000) × 1.0 = 5,000 USDT
- 收益率 = 5,000 / 5,000 = 100% (基于保证金)

---

## 流程 2: 强平流程

当持仓价格触及强平价格时，系统自动启动三级强平机制。

### 强平流程图

```
┌─────────────────────────────────────────────────────────────────────┐
│                     三级强平机制流程                                   │
└─────────────────────────────────────────────────────────────────────┘

                        触发条件检测
                        ┌─────────┐
                        │标记价格  │
                        │跌破强平价│
                        └────┬────┘
                             │
                ┌────────────▼──────────────┐
                │  mark_price ≤ liq_price   │
                │  (多仓触发强平)            │
                └────────────┬──────────────┘
                             │
                ┌────────────▼──────────────┐
                │   0️⃣  冻结持仓             │
                │   Status: LIQUIDATING     │
                └────────────┬──────────────┘
                             │
        ┌────────────────────┴───────────────────┐
        │                                        │
        ▼                                        │
┌──────────────────────────────┐                │
│  1️⃣  第一级: 市场强平          │                │
│  (Market Liquidation)        │                │
├──────────────────────────────┤                │
│  - 提交紧急市价单              │                │
│  - 5秒超时等待                │                │
│  - Type: IOC (立即成交或取消) │                │
└────────┬─────────────────────┘                │
         │                                      │
    成交? │                                      │
         ├─ YES ──▶ 结算 ─────┐                  │
         │                   │                  │
         └─ NO ───▶ 流动性不足 │                  │
                             │                  │
        ┌────────────────────┘                  │
        │                                        │
        ▼                                        │
┌──────────────────────────────┐                │
│  2️⃣  第二级: 保险基金接管      │                │
│  (Insurance Fund)            │                │
├──────────────────────────────┤                │
│  - 检查保险基金容量            │                │
│  - 评估接管成本               │                │
│  - 执行接管操作               │                │
└────────┬─────────────────────┘                │
         │                                      │
    足够? │                                      │
         ├─ YES ──▶ 结算 ─────┐                  │
         │                   │                  │
         └─ NO ───▶ 容量不足   │                  │
                             │                  │
        ┌────────────────────┘                  │
        │                                        │
        ▼                                        │
┌──────────────────────────────┐                │
│  3️⃣  第三级: 自动减仓 (ADL)    │                │
│  (Auto-Deleveraging)         │                │
├──────────────────────────────┤                │
│  - 查找对手方盈利仓位          │                │
│  - 按ADL队列优先级排序         │                │
│  - 强制平仓对手方部分仓位      │                │
│  - 通知被ADL用户              │                │
└────────┬─────────────────────┘                │
         │                                      │
         └──────────▶ 强制结算                   │
                                                │
        ┌───────────────────────────────────────┘
        │
        ▼
┌──────────────────────────────┐
│      强平完成                 │
│  - 持仓已关闭                 │
│  - 损失已分配                 │
│  - 用户通知已发送              │
└──────────────────────────────┘
```

### 强平价格计算

#### 多仓强平价
```rust
pub fn calculate_liquidation_price(
    entry_price: Price,
    leverage: u8,
    position_side: PositionSide,
) -> Price {
    const MAINTENANCE_MARGIN_RATE: f64 = 0.005; // 0.5%
    const LIQUIDATION_FEE_RATE: f64 = 0.005;    // 0.5%

    match position_side {
        PositionSide::Long => {
            // 多仓强平价 = 开仓价 × (1 - 1/杠杆 + 0.005 + 0.005)
            entry_price.to_f64() * (1.0 - 1.0/leverage as f64 + 0.01)
        }
        PositionSide::Short => {
            // 空仓强平价 = 开仓价 × (1 + 1/杠杆 - 0.005 - 0.005)
            entry_price.to_f64() * (1.0 + 1.0/leverage as f64 - 0.01)
        }
    }
}
```

#### 示例计算

**场景**: 多仓 1 BTC @ 50,000 USDT, 10倍杠杆

```
开仓价: 50,000 USDT
杠杆: 10x
保证金: 50,000 / 10 = 5,000 USDT

强平价 = 50,000 × (1 - 1/10 + 0.005 + 0.005)
      = 50,000 × (1 - 0.1 + 0.01)
      = 50,000 × 0.91
      = 45,500 USDT

安全距离 = 50,000 - 45,500 = 4,500 USDT
跌幅容忍 = 4,500 / 50,000 = 9%
```

### 三级强平机制详解

#### Level 1: 市场强平 (Market Liquidation)

```rust
async fn try_market_liquidation(
    &self,
    position: &PositionInfo,
    side: Side,
) -> Result<LiquidationResult, PrepCommandError> {
    // 提交紧急市价单
    let order_cmd = OpenPositionCommand {
        symbol: position.symbol,
        side,
        order_type: OrderType::Market,
        quantity: position.quantity,
        time_in_force: TimeInForce::IOC,  // 立即成交或取消
        ..
    };

    // 5秒超时等待成交
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        self.matching_service.open_position(order_cmd)
    ).await;

    match result {
        Ok(Ok(filled)) => {
            // 成交成功，计算损失
            let loss = calculate_liquidation_loss(position, filled.avg_price);
            Ok(settle_market_liquidation(position, loss))
        }
        _ => Err(PrepCommandError::market_liquidity_insufficient())
    }
}
```

**优势**:
- 最快速度平仓
- 市场承担损失
- 用户损失可能小于保证金

**失败原因**:
- 市场流动性不足
- 价格剧烈波动
- 订单簿深度不够

#### Level 2: 保险基金接管 (Insurance Fund)

```rust
async fn try_insurance_fund_takeover(
    &self,
    position: &PositionInfo,
) -> Result<LiquidationResult, PrepCommandError> {
    // 检查保险基金容量
    let capacity = self.insurance_fund.check_capacity().await?;

    if !capacity.can_takeover(position) {
        return Err(PrepCommandError::insurance_fund_insufficient());
    }

    // 执行接管
    let takeover = self.insurance_fund.takeover(position).await?;

    Ok(settle_insurance_fund_liquidation(position, takeover))
}
```

**保险基金作用**:
- 保护市场不受穿仓影响
- 承担市场强平失败的损失
- 由盈利交易者手续费补充

**接管条件**:
- 市场强平失败
- 保险基金余额充足
- 损失小于基金容量

#### Level 3: 自动减仓 (ADL)

```rust
async fn trigger_auto_deleveraging(
    &self,
    position: &PositionInfo,
    side: Side,
) -> Result<LiquidationResult, PrepCommandError> {
    // 查找对手方盈利仓位（按ADL队列优先级）
    let counterparties = self.adl_engine
        .find_counterparties(position.symbol, side)
        .await?;

    // 执行ADL
    let adl_result = self.adl_engine
        .execute_adl(position, counterparties)
        .await?;

    // 通知被ADL的用户
    for counterparty_id in &adl_result.affected_positions {
        self.notify_adl_counterparty(counterparty_id).await?;
    }

    Ok(settle_adl_liquidation(position, adl_result))
}
```

**ADL队列优先级**:
```
优先级 = 盈利率 × 杠杆倍数

排序规则:
1. 盈利率最高的仓位
2. 杠杆倍数最高的仓位
3. 组合评分最高优先被ADL
```

**被ADL影响**:
- 盈利仓位被强制部分平仓
- 按破产价格成交（不利价格）
- 收到系统通知

### 损失分配示例

#### 场景 1: 市场强平成功

```
开仓: 1 BTC @ 50,000 USDT (10x杠杆)
保证金: 5,000 USDT
强平价: 45,500 USDT

实际成交价: 46,000 USDT (高于强平价)

损失计算:
  总损失 = (50,000 - 46,000) × 1.0 = 4,000 USDT
  用户损失 = 4,000 USDT
  保险基金损失 = 0 USDT

剩余保证金: 5,000 - 4,000 = 1,000 USDT (返还用户)
```

#### 场景 2: 极端行情，需要保险基金

```
开仓: 1 BTC @ 50,000 USDT (10x杠杆)
保证金: 5,000 USDT
强平价: 45,500 USDT

极端价格: 40,000 USDT (远低于强平价)

损失计算:
  总损失 = (50,000 - 40,000) × 1.0 = 10,000 USDT
  用户损失 = 5,000 USDT (全部保证金)
  保险基金损失 = 10,000 - 5,000 = 5,000 USDT

用户穿仓损失: 5,000 USDT (由保险基金承担)
```

---

## 流程 3: 杠杆影响分析

不同杠杆倍数对强平价格和风险的影响。

### 杠杆倍数对比表

| 杠杆 | 保证金占用 | 强平价 (多仓) | 安全距离 | 跌幅容忍 | 风险等级 |
|------|-----------|--------------|---------|---------|---------|
| 5x   | 20%       | 48,000 USDT  | 2,000   | 4%      | 低      |
| 10x  | 10%       | 45,500 USDT  | 4,500   | 9%      | 中      |
| 20x  | 5%        | 47,500 USDT  | 2,500   | 5%      | 高      |
| 50x  | 2%        | 49,000 USDT  | 1,000   | 2%      | 极高    |

**开仓价**: 50,000 USDT

### 杠杆选择建议

#### 低杠杆 (5x-10x)
- **适用场景**: 长期持仓，稳健交易
- **优势**: 安全距离大，不易强平
- **劣势**: 资金利用率低，收益倍数小

#### 中杠杆 (10x-20x)
- **适用场景**: 中短期交易，有一定风险承受能力
- **优势**: 平衡收益和风险
- **劣势**: 需要密切监控仓位

#### 高杠杆 (20x-50x)
- **适用场景**: 短线交易，专业交易者
- **优势**: 资金利用率高，收益倍数大
- **劣势**: 极易强平，风险极高

---

## 风险管理建议

### 1. 合理使用杠杆
```
建议杠杆倍数:
  - 新手: 2x-5x
  - 中级: 5x-10x
  - 专业: 10x-20x
  - 超短线专家: 20x+ (谨慎使用)
```

### 2. 设置止损止盈
```rust
// 设置止损单
let stop_loss_price = entry_price * 0.95;  // 5%止损
let stop_loss_cmd = ClosePositionCommand::limit_close_long(
    symbol,
    quantity,
    stop_loss_price
);

// 设置止盈单
let take_profit_price = entry_price * 1.10;  // 10%止盈
let take_profit_cmd = ClosePositionCommand::limit_close_long(
    symbol,
    quantity,
    take_profit_price
);
```

### 3. 仓位管理原则
```
单笔仓位建议:
  - 保守: 总资金的 10-20%
  - 适中: 总资金的 20-30%
  - 激进: 总资金的 30-50%

警告: 不要超过总资金的 50%
```

### 4. 监控强平价
```rust
// 计算当前价格距离强平价的百分比
fn calculate_liquidation_distance(
    current_price: Price,
    liquidation_price: Price,
    position_side: PositionSide
) -> f64 {
    match position_side {
        PositionSide::Long => {
            (current_price.to_f64() - liquidation_price.to_f64())
                / current_price.to_f64() * 100.0
        }
        PositionSide::Short => {
            (liquidation_price.to_f64() - current_price.to_f64())
                / current_price.to_f64() * 100.0
        }
    }
}

// 示例
let distance = calculate_liquidation_distance(
    Price::from_f64(50000.0),
    Price::from_f64(45500.0),
    PositionSide::Long
);
// distance = 9.0%

// 警告阈值
if distance < 5.0 {
    println!("⚠️ 警告: 接近强平价，建议增加保证金或减仓");
}
```

---

## 测试用例参考

### 正常交易流程测试
```bash
# 运行正常交易流程BDD测试
cd proc/operating/exchange/prep
cargo test --test bdd_normal_trading_flow -- --nocapture

# 测试场景:
# ✅ scenario_full_long_position_lifecycle - 多仓完整生命周期
# ✅ scenario_short_position_take_profit - 空仓止盈
# ✅ scenario_partial_close_position - 部分平仓
# ✅ scenario_limit_order_close - 限价平仓
```

### 强平流程测试
```bash
# 运行强平流程BDD测试
cargo test --test bdd_order_to_liquidation -- --nocapture

# 测试场景:
# ✅ scenario_long_position_liquidated_by_price_drop - 多仓价格下跌强平
# ✅ scenario_short_position_liquidated_by_price_rise - 空仓价格上涨强平
# ✅ scenario_extreme_price_drop_requires_insurance_fund - 极端行情保险基金
# ✅ scenario_high_leverage_position_easier_liquidation - 高杠杆更易强平
```

---

## 相关代码文件

### 核心实现
- `src/proc/trading_prep_order_proc.rs` - 交易命令定义
- `src/proc/trading_prep_order_proc_impl.rs` - 交易逻辑实现
- `src/proc/liquidation_proc.rs` - 强平流程实现
- `src/proc/liquidation_types.rs` - 强平类型定义

### 测试文件
- `tests/bdd_normal_trading_flow.rs` - 正常交易流程BDD测试
- `tests/bdd_order_to_liquidation.rs` - 强平流程BDD测试
- `tests/bdd_perp_order_exch_proc.rs` - 订单处理单元测试

---

## 总结

### 三个核心流程对比

| 流程 | 触发方式 | 执行速度 | 用户控制 | 损失范围 |
|------|---------|---------|---------|---------|
| 正常交易 | 用户主动 | 取决于订单类型 | 完全控制 | 由用户决定 |
| 市场强平 | 系统自动 | 5秒内 | 无法控制 | ≤ 保证金 |
| 保险基金 | 系统自动 | 立即 | 无法控制 | 全部保证金 |
| ADL | 系统自动 | 立即 | 无法控制 | 被ADL损失 |

### 关键要点

1. **杠杆是双刃剑**: 放大收益的同时也放大风险
2. **强平价是生命线**: 时刻关注当前价格与强平价的距离
3. **保证金管理**: 合理分配资金，避免过度杠杆
4. **止损止盈**: 设置自动平仓保护，避免情绪化交易
5. **理解强平机制**: 知道什么情况下会被强平，如何避免

### 最佳实践

✅ **DO**:
- 使用合理的杠杆倍数
- 设置止损止盈订单
- 监控持仓强平风险
- 分散投资，控制单笔仓位
- 保持足够的账户余额

❌ **DON'T**:
- 使用过高杠杆（>20x）
- 重仓单一交易对
- 忽视强平警告
- 情绪化交易
- 满仓操作
