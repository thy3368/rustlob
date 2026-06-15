# 统一账户 (Unified Account) 概念解释

> 解释加密货币交易所的"统一账户"模式
> 对比分账模式与统一账户模式的区别
> 更新时间: 2026-04-12

---

## 1. 什么是统一账户？

### 1.1 定义

**统一账户 (Unified Account)** 是指一个账户可以同时交易多个产品线（现货、合约、期权、杠杆），共享同一个保证金池，无需分开管理多个子账户。

### 1.2 对比分账模式

| 模式 | 描述 | 代表交易所 |
|------|------|-----------|
| **分账模式** | 每个产品线独立账户，保证金隔离 | Binance |
| **统一账户模式** | 一个账户全产品线，共享保证金池 | OKX, Bybit |

### 1.3 核心区别示意

```
分账模式 (Binance):
┌─────────────────────────────────────────────┐
│  用户ID: 12345                              │
├─────────────────────────────────────────────┤
│  Spot Account     → BTC: 1.0, USDT: 5000   │
│  Futures Account → (独立账户)               │
│  Margin Account  → (独立账户)               │
│  Funding Account → (独立账户)              │
└─────────────────────────────────────────────┘
  × 每个产品线单独的风控/保证金计算

统一账户模式 (OKX):
┌─────────────────────────────────────────────┐
│  用户ID: 12345                              │
├─────────────────────────────────────────────┤
│  Unified Account                          │
│  ├── Spot Holdings    → BTC: 1.0           │
│  ├── Futures Positions → BTC-PERP: 0.5    │
│  ├── Options        → BTC-30JUN-50000C      │
│  └── Margin Balance  → (共享)              │
└─────────────────────────────────────────────┘
  ✓ 全产品线共享保证金，统一风控
```

---

## 2. 统一账户的核心字段

### 2.1 OKX 统一账户字段

```json
{
  "totalEq": "10000.0",        // 总权益 (折算 USDT)
  "availEq": "9500.0",        // 可用保证金
  "availBal": "9500.0",      // 可用余额
  "frozenBal": "500.0",       // 冻结余额
  "locked": "0.0",           // 挂单锁定
  "equity": "10000.0",       // 账户权益
  "notionalLever": "0.0",     // 杠杆倍数
  "orderFrozen": "500.0",      // 订单冻结
  "isoEq": "0.0"             // 逐仓保证金
}
```

### 2.2 字段解释

| 字段 | 中文名 | 说明 |
|------|--------|------|
| totalEq | 总权益 | 所有资产折算 USDT 的总价值 |
| availEq | 可用保证金 | 可以用于开仓的保证金 |
| frozenBal | 冻结余额 | 风控冻结（不可用） |
| locked | 锁定余额 | 挂单锁定（订单成交后释放） |
| equity | 账户权益 | cashBal + frozenBal + locked + 未实现盈亏 |
| isoEq | 逐仓保证金 | 逐仓模式下的独立保证金 |

### 2.3 计算公式

```
availEq = totalEq - margin_used - frozenBal - orderFrozen
equity = cashBal + frozenBal + locked + unrealized_pnl
margin_used = Σ(position_value / leverage) + Σ(futures_margin)
```

---

## 3. 使用场景举例

### 3.1 场景1：现货+合约混合交易

**用户操作**：
1. 持有 1 BTC 现货
2. 用 0.5 BTC 作为保证金开 5x 杠杆的 BTC 永续合约

**分账模式**：
- Spot 账户：1 BTC
- Futures 账户：需单独转入 0.5 BTC 保证金

**统一账户模式**：
- 统一账户：1 BTC 现货 + 0.5 BTC 保证金（合约）
- 总权益自动计算：1 BTC + 0.5 BTC = 1.5 BTC
- 可用保证金：1.5 BTC - 所需保证金 = 剩余可用

**优势**：
- ✓ 无需手动转账保证金
- ✓ 现货盈利可自动补充合约保证金
- ✓ 风控更灵活

### 3.2 场景2：跨产品线止损

**用户操作**：
1. 持有 10000 USDT
2. 5000 USDT 买现货 BTC
3. 3000 USDT 开合约仓位（剩余 2000 USDT 作为保证金）

**统一账户计算**：
```
总权益: 10000 USDT
仓位占用:
  - 现货: 5000 USDT
  - 合约保证金: 3000 USDT
可用: 10000 - 5000 - 3000 = 2000 USDT

如果合约亏损至需要追加保证金:
- 自动从现货释放资金补充
- 无需手动操作
```

**分账模式的问题**：
- 现货账户剩余 5000 USDT（无法使用）
- 合约账户亏损，需要额外转入保证金
- 需要手动转账操作

### 3.3 场景3：期权对冲

**用户操作**：
1. 持有 1 BTC 现货
2. 买入看跌期权作为保险（支付权利金）

**统一账户模式**：
```
账户结构:
  - 现货: 1 BTC
  - 期权:
      - 付出权利金: 500 USDT (锁定)
      - 潜在赔付: up to 1 BTC (期权价值)
  
风控计算:
  - totalEq = 1 BTC + 期权价值 - 权利金
  - 可用保证金 = max(现货价值 - 仓位 - 期权风险)
```

**优势**：
- ✓ 期权费计入风控，可与现货对冲
- ✓ 保证金计算更准确

---

## 4. 统一账户的优缺点

### 4.1 优点

| 优点 | 说明 |
|------|------|
| **保证金效率高** | 跨产品线共享保证金，减少闲置资金 |
| **操作便捷** | 无需手动转账，一个账户全产品线 |
| **风控统一** | 全仓位统一计算风险暴露 |
| **用户体验好** | 更灵活的仓位管理 |

### 4. 缺点

| 缺点 | 说明 |
|------|------|
| **复杂度高** | 保证金计算逻辑复杂 |
| **风险集中** | 一个仓位爆仓可能影响全部 |
| **技术难度** | 需要精确的风控系统 |

---

## 5. 各交易所的实现

### 5.1 OKX - 统一账户

```
架构: Unified Account
产品线: 现货 + 合约 + 期权 + 杠杆
保证金: Cross/ Isolated 可选
API: /api/v5/account/balance
```

### 5.2 Bybit - 统一账户

```
架构: Unified Trading Account
产品线: 现货 + 合约 + 期权 + 杠杆
保证金: Only Cross (全仓)
API: /v5/account/wallet-balance
```

### 5.3 Binance - 分账模式

```
架构: Multi-Account (分开)
产品线:
  - SPOT: /api/v3/account
  - MARGIN: /sapi/v1/margin/account
  - FUTURES: /v1/futures/balance
  - POWR: /sapi/v1/pool/balance
保证金: Cross/ Isolated (合约)
```

### 5.4 选择建议

| 场景 | 推荐模式 |
|------|---------|
| 简单现货交易 | 分账模式 |
| 多产品线交易 | 统一账户 |
| 高频合约交易 | 统一账户（但需风控经验） |
| 合规要求严格 | 分账模式（风险隔离） |

---

## 6. RustLOB 设计建议

### 6.1 短期：分账模式（简单）

```
AccountType:
  - Spot     (现货账户)
  - Futures  (合约账户)
  - Funding (资金账户)
```

### 6.2 长期：统一账户（推荐）

```
UnifiedAccount (统一账户):
  - spot_holdings: HashMap<AssetId, Decimal>
  - margin_summary: MarginSummary
  - positions: Vec<Position>
  
MarginSummary:
  - total_eq: Decimal      // 总权益
  - available_margin: Decimal // 可用保证金
  - margin_used: Decimal    // 已用保证金
```

### 6.3 迁移路���

```
Phase 1: 分账模式 (当前)
  └── Spot Account + Futures Account 分离

Phase 2: 账户聚合 (可选)
  └── 账户聚合视图，统一展示

Phase 3: 统一账户 (最终)
  └── 全产品线共享保证金
```

---

## 7. Rust 实体设计

### 7.1 核心实体（基于现有 account2.rs 扩展）

基于 `lib/common/base_types/src/account/account2.rs` 的现有结构，设计统一账户实体：

```rust
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::{AssetId, Timestamp, UserId, TradingPair, OrderSide};

/// 统一账户 (Unified Account)
///
/// 一个账户可以同时交易现货、合约、期权、杠杆
/// 共享保证金池，统一风控
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAccount {
    /// 用户ID
    pub user_id: UserId,
    /// 账户权限
    pub permissions: AccountPermissions,
    /// 现货持仓
    pub spot_holdings: Vec<SpotHolding>,
    /// 合约持仓
    pub futures_positions: Vec<FuturesPosition>,
    /// 期权持仓
    pub options_positions: Vec<OptionsPosition>,
    /// 保证金摘要
    pub margin_summary: MarginSummary,
    /// 保证金模式
    pub margin_mode: MarginMode,
    /// 创建时间
    pub created_at: Timestamp,
    /// 更新时间
    pub updated_at: Timestamp,
}

impl UnifiedAccount {
    /// 创建新统一账户
    pub fn new(user_id: UserId) -> Self {
        let now = Timestamp::now();
        Self {
            user_id,
            permissions: AccountPermissions::default(),
            spot_holdings: Vec::new(),
            futures_positions: Vec::new(),
            options_positions: Vec::new(),
            margin_summary: MarginSummary::default(),
            margin_mode: MarginMode::Cross,
            created_at: now,
            updated_at: now,
        }
    }

    /// 总权益（折算 USDT）
    pub fn total_equity(&self) -> Decimal {
        self.margin_summary.total_eq
    }

    /// 可用保证金
    pub fn available_margin(&self) -> Decimal {
        self.margin_summary.available_margin
    }

    /// 是否可以开仓
    pub fn can_open_position(&self, required_margin: Decimal) -> bool {
        self.margin_summary.available_margin >= required_margin
    }
}

/// 现货持仓
///
/// 对应 OKX API: spotState.balances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotHolding {
    pub asset: AssetId,
    /// 可用数量
    pub free: Decimal,
    /// 锁定（挂单）
    pub locked: Decimal,
    /// 冻结（风控）
    pub freeze: Decimal,
}

impl SpotHolding {
    pub fn new(asset: AssetId, amount: Decimal) -> Self {
        Self { asset, free: amount, locked: Decimal::ZERO, freeze: Decimal::ZERO }
    }

    /// 总持仓
    pub fn total(&self) -> Decimal {
        self.free + self.locked + self.freeze
    }

    /// 可用数量
    pub fn available(&self) -> Decimal {
        self.free
    }
}

/// 合约持仓
///
/// 对应 OKX / Bybit 永续合约持仓
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPosition {
    pub trading_pair: TradingPair,
    /// 持仓方向
    pub side: OrderSide,
    /// 持仓数量（正值=多头，负值=空头）
    pub size: Decimal,
    /// 开仓均价
    pub entry_price: Decimal,
    /// 杠杆倍数
    pub leverage: u32,
    /// 未实现盈亏
    pub unrealized_pnl: Decimal,
    /// 持仓价值（折算 USDT）
    pub position_value: Decimal,
    /// 所需保证金
    pub margin_required: Decimal,
}

impl FuturesPosition {
    /// 创建新持仓
    pub fn new(trading_pair: TradingPair, side: OrderSide, size: Decimal, entry_price: Decimal, leverage: u32) -> Self {
        let margin_required = entry_price * size / Decimal::from(leverage as i64);
        Self {
            trading_pair,
            side,
            size,
            entry_price,
            leverage,
            unrealized_pnl: Decimal::ZERO,
            position_value: entry_price * size,
            margin_required,
        }
    }

    /// 更新未实现盈亏
    pub fn update_pnl(&mut self, current_price: Decimal) {
        let pnl = match self.side {
            OrderSide::Buy => (current_price - self.entry_price) * self.size,
            OrderSide::Sell => (self.entry_price - current_price) * self.size,
        };
        self.unrealized_pnl = pnl;
        self.position_value = current_price * self.size;
    }
}

/// 期权持仓
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPosition {
    pub underlying: AssetId,
    /// 期权类型
    pub option_type: OptionType,
    /// 执行价
    pub strike_price: Decimal,
    /// 数量
    pub size: Decimal,
    /// 已支付权利金
    pub premium_paid: Decimal,
    /// 到期时间
    pub expiry: Timestamp,
    /// 当前价值
    pub current_value: Decimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionType {
    /// 看涨期权
    Call,
    /// 看跌期权
    Put,
}

/// 保证金摘要
///
/// 对应 OKX API: marginSummary
/// 对应 Hyperliquid API: marginSummary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSummary {
    /// 总权益（折算 USDT）
    pub total_eq: Decimal,
    /// 可用保证金
    pub available_margin: Decimal,
    /// 已用保证金
    pub margin_used: Decimal,
    /// 冻结余额（风控冻结）
    pub frozen_balance: Decimal,
    /// 订单冻结
    pub order_frozen: Decimal,
    /// 可提现余额
    pub withdrawable: Decimal,
}

impl Default for MarginSummary {
    fn default() -> Self {
        Self {
            total_eq: Decimal::ZERO,
            available_margin: Decimal::ZERO,
            margin_used: Decimal::ZERO,
            frozen_balance: Decimal::ZERO,
            order_frozen: Decimal::ZERO,
            withdrawable: Decimal::ZERO,
        }
    }
}

impl MarginSummary {
    /// 计算并更新保证金摘要
    pub fn recalculate(
        &mut self,
        spot_holdings: &[SpotHolding],
        futures_positions: &[FuturesPosition],
    ) {
        // 现货总价值（简化计算，实际需要价格喂价）
        let spot_total: Decimal = spot_holdings.iter().map(|h| h.total()).sum();

        // 合约已用保证金
        let futures_margin: Decimal = futures_positions.iter().map(|p| p.margin_required).sum();

        self.margin_used = futures_margin;
        self.total_eq = spot_total;
        self.available_margin = spot_total - futures_margin - self.frozen_balance - self.order_frozen;
        self.withdrawable = self.available_margin; // 简化：预留风控缓冲
    }
}

/// 保证金模式（复用 account.rs 的 MarginMode）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginMode {
    /// 全仓
    Cross,
    /// 逐仓
    Isolated,
}

/// 账户权限（复用 account.rs 的 AccountPermissions）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPermissions {
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub is_trading_enabled: bool,
    pub is_withdrawal_enabled: bool,
}

impl Default for AccountPermissions {
    fn default() -> Self {
        Self {
            can_trade: true,
            can_deposit: true,
            can_withdraw: true,
            is_trading_enabled: true,
            is_withdrawal_enabled: true,
        }
    }
}

/// 账户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnifiedAccountStatus {
    Active,
    Frozen,
    Closed,
}
```

### 7.2 使用示例

#### 示例1：现货+合约混合交易

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_account_spot_futures() {
        let mut account = UnifiedAccount::new(UserId(12345));

        // 持有 1 BTC 现货
        account.spot_holdings.push(SpotHolding::new(AssetId::Btc, Decimal::from(1)));

        // 用 0.5 BTC 作为保证金开 5x 杠杆合约
        let futures_position = FuturesPosition::new(
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            Decimal::from(1),       // 1 BTC
            Decimal::from(50000),   // 50000 USDT
            5,                     // 5x 杠杆
        );
        account.futures_positions.push(futures_position);

        // 更新保证金摘要
        let mut summary = MarginSummary::default();
        summary.recalculate(&account.spot_holdings, &account.futures_positions);
        account.margin_summary = summary;

        // 验证
        // 现货价值: 1 BTC * 50000 = 50000 USDT
        // 合约保证金: 50000 / 5 = 10000 USDT
        // 可用保证金: 50000 - 10000 = 40000 USDT
        assert!(account.can_open_position(Decimal::from(10000)));
    }
}
```

#### 示例2：跨产品线风控

```rust
/// 检查账户是否可以追加保证金
pub fn check_margin_call(account: &UnifiedAccount, maintenance_margin_ratio: Decimal) -> bool {
    let margin_ratio = account.margin_summary.total_eq / account.margin_summary.margin_used;
    margin_ratio < maintenance_margin_ratio
}

/// 计算账户健康度
pub fn account_health(account: &UnifiedAccount) -> Decimal {
    if account.margin_summary.margin_used == Decimal::ZERO {
        return Decimal::from(100);
    }
    // 健康度 = 可用保证金 / 总权益 * 100
    (account.margin_summary.available_margin / account.margin_summary.total_eq)
        * Decimal::from(100)
}
```

#### 示例3：期权和现货对冲

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_hedge() {
        let mut account = UnifiedAccount::new(UserId(12345));

        // 持有 1 BTC 现货
        account.spot_holdings.push(SpotHolding::new(AssetId::Btc, Decimal::from(1)));

        // 买入看跌期权（保险）
        account.options_positions.push(OptionsPosition {
            underlying: AssetId::Btc,
            option_type: OptionType::Put,
            strike_price: Decimal::from(45000),
            size: Decimal::from(1),
            premium_paid: Decimal::from(500),  // 500 USDT 权利金
            expiry: Timestamp(1767225600000000000),  // 2026-01-01
            current_value: Decimal::ZERO,
        });

        // 风控计算：期权价值计入
        // totalEq = 1 BTC + 期权价值 - 权利金
        // 简化计算：50000 + 0 - 500 = 49500 USDT
    }
}
```

### 7.3 与现有 account2.rs 的关系

```
现有 account2.rs          → 扩展为 UnifiedAccount
─────────────────────────────────────────────────────
Account                  → UnifiedAccount
Balance (4状态)           → SpotHolding (现货)
FuturesAccount           → FuturesPosition (合约)
新增                    → OptionsPosition (期权)
新增                    → MarginSummary (保证金摘要)
AccountPermissions     → 复用
MarginMode             → 复用
```

### 7.4 设计对齐

| 文档字段 | Rust 字段 | 说明 |
|----------|-----------|------|
| totalEq | margin_summary.total_eq | 总权益 |
| availEq | margin_summary.available_margin | 可用保证金 |
| frozenBal | margin_summary.frozen_balance | 风控冻结 |
| locked | spot_holdings[].locked | 挂单锁定 |
| equity | total_eq + unrealized_pnl | 账户权益 |
| isoEq | margin_summary.isolated_eq | 逐仓保证金 |

---

## 8. 总结

### 8.1 统一账户核心概念

1. **一个账户**：可交易所有产品线
2. **共享保证金**：跨产品线计算风控
3. **总权益计算**：totalEq = Σ(资产) - Σ(负债)

### 8.2 适用场景

- ✓ 多产品线交易（现货+合约+期权）
- ✓ 跨产品线对冲
- ✓ 提高资金利用率
- ✗ 简单现货交易（建议分账）

### 8.3 关键指标

| 指标 | 统一账户 |
|------|---------|
| 保证金利用率 | 高 |
| 风控复杂度 | 高 |
| 用户体验 | 好 |
| 技术实现 | 复杂 |

---

## 参考资料

- OKX Unified Account: `/api/v5/account/balance`
- Bybit Unified Trading Account: `/v5/account/wallet-balance`
- Binance Multi-Account: `/api/v3/account`, `/sapi/v1/margin/account`