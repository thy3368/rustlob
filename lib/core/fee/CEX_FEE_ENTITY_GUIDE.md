# Rust之从0-1低时延CEX：CEX 手续费实体模型设计与应用指南

## 目录
- [概述](#概述)
- [核心概念](#核心概念)
- [实体模型架构](#实体模型架构)
- [功能特性](#功能特性)
- [现货成交中的应用](#现货成交中的应用)
- [使用示例](#使用示例)
- [集成指南](#集成指南)

---

## 概述

### 问题背景
在加密货币交易所（CEX）中，手续费计算涉及多个维度：
- **Maker/Taker 区分**：做市商（Maker）和吃单者（Taker）享有不同费率
- **VIP 等级体系**：高净值用户享受费率折扣
- **分层费率结构**：基于用户交易量的阶梯式费率
- **特定资产配置**：不同数字资产可能有不同的手续费规则
- **临时促销活动**：限时性的折扣或优惠

### 解决方案
`CexFeeEntity` 是一个通用的、可配置的手续费元数据实体，用于：
1. **统一管理**费率配置
2. **灵活计算**各种场景下的手续费
3. **独立于订单处理**，作为元数据存储和查询
4. **支持多维度**的费率组合和优化

### 核心价值
- ✅ **解耦**：手续费配置与订单处理逻辑分离
- ✅ **复用**：一份配置支持整个交易系统
- ✅ **灵活**：支持复杂的多维度费率组合
- ✅ **可维护**：集中化管理，易于更新和变更

---

## 核心概念

### 费率计算维度

#### 1. 费率类型（FeeType）
```rust
pub enum FeeType {
    Taker,          // 吃单手续费 - 立即成交的一方支付
    Maker,          // 挂单手续费 - 挂单被执行时支付（通常更低）
    Withdrawal,     // 提现手续费 - 资金提出交易所时支付
    Deposit,        // 充值手续费 - 资金充入交易所时支付
    Transfer,       // 转账手续费 - 账户间转账时支付
}
```

#### 2. 计算模式（FeeCalculationMode）
```rust
pub enum FeeCalculationMode {
    Fixed,          // 固定费率 - 所有用户统一费率
    Tiered,         // 分层费率 - 基于交易量的阶梯式费率
    MarketMaker,    // 做市商优惠 - 特殊用户享受折扣
    VIP,            // VIP 等级费率 - 基于用户等级的折扣
}
```

#### 3. 资产类型（AssetType）
```rust
pub enum AssetType {
    Crypto(String),     // 加密货币（如 BTC、ETH）
    Fiat(String),       // 法币（如 USD、CNY）
    Stablecoin,         // 稳定币（如 USDT、USDC）
}
```

### 费率基点（Basis Points）
手续费通常以**基点（bp）**表示，1 bp = 0.01%：
```
10 bp = 0.1%
50 bp = 0.5%
100 bp = 1.0%
```

---

## 实体模型架构

### CexFeeEntity - 核心实体

```rust
pub struct CexFeeEntity {
    // ===== 基础标识 =====
    pub fee_schedule_id: String,           // 费率表唯一标识
    pub exchange_id: String,               // 交易所标识
    pub schedule_name: String,             // 费率表名称（如"标准费率"）
    pub valid_from: DateTime<Utc>,         // 生效时间
    pub valid_until: Option<DateTime<Utc>>,// 失效时间（可选）
    pub is_active: bool,                   // 是否激活

    // ===== 费率配置 =====
    pub fee_tiers: Vec<FeeTier>,          // 分层费率配置
    pub vip_levels: Vec<VIPLevel>,        // VIP 等级配置
    pub asset_specific_fees: HashMap<String, AssetSpecificFee>, // 特定资产费率

    // ===== 计算参数 =====
    pub default_maker_fee: f64,           // 默认 Maker 费率
    pub default_taker_fee: f64,           // 默认 Taker 费率
    pub precision_config: FeePrecision,   // 精度配置

    // ===== 特殊规则 =====
    pub promotion_campaigns: Vec<Promotion>,    // 促销活动
    pub blacklist_rules: Vec<BlacklistRule>,    // 黑名单规则
}
```

### 关键子实体

#### FeeTier - 分层费率配置
```rust
pub struct FeeTier {
    pub tier_id: u32,               // 层级 ID（1=Bronze, 2=Silver, 3=Gold）
    pub tier_name: String,          // 层级名称
    pub min_volume_30d: f64,        // 30天最小交易量（触发条件）
    pub min_balance: f64,           // 最小持仓余额
    pub maker_fee: f64,             // 该层级 Maker 费率
    pub taker_fee: f64,             // 该层级 Taker 费率
    pub withdrawal_fee_fixed: f64,  // 固定提现费
    pub withdrawal_fee_percent: f64,// 提现费率百分比
    pub is_active: bool,            // 是否激活
}
```

**示例**：
```
Bronze 级（新手）: Maker 0.10%, Taker 0.15% | 最小交易量: 无限制
Silver 级（中级）: Maker 0.08%, Taker 0.12% | 最小交易量: 100 BTC/月
Gold 级（高级）  : Maker 0.05%, Taker 0.10% | 最小交易量: 1000 BTC/月
```

#### VIPLevel - VIP 等级配置
```rust
pub struct VIPLevel {
    pub level_id: u32,                  // VIP 等级 ID
    pub level_name: String,             // 等级名称（VIP1, VIP2, VIP3）
    pub requirements: String,           // 升级要求（文本描述）
    pub fee_discount: f64,              // 费率折扣（如 0.2 = 20% 折扣）
    pub special_benefits: Vec<String>,  // 特殊权益（优先客服、市场数据等）
}
```

**示例**：
```
VIP1: 20% 费率折扣 | 要求：30天交易量 > 100 BTC
VIP2: 40% 费率折扣 | 要求：30天交易量 > 500 BTC
VIP3: 60% 费率折扣 | 要求：30天交易量 > 2000 BTC
```

#### AssetSpecificFee - 特定资产费率
```rust
pub struct AssetSpecificFee {
    pub asset_id: String,              // 资产标识
    pub asset_type: AssetType,         // 资产类型
    pub base_withdrawal_fee: f64,      // 基础提现费
    pub min_withdrawal_amount: f64,    // 最小提现金额
    pub max_withdrawal_amount: f64,    // 最大提现金额
    pub network_fee: f64,              // 网络费用
    pub special_conditions: Vec<String>,// 特殊条件
}
```

#### Promotion - 促销活动
```rust
pub struct Promotion {
    pub promotion_id: String,          // 活动 ID
    pub description: String,           // 活动描述
    pub discount_rate: f64,            // 折扣率（如 0.1 = 10% 折扣）
    pub start_time: DateTime<Utc>,     // 开始时间
    pub end_time: DateTime<Utc>,       // 结束时间
    pub target_users: Vec<String>,     // 目标用户（空 = 所有用户）
    pub conditions: Vec<String>,       // 参与条件
}
```

---

## 功能特性

### 1. 多维度费率计算

费率计算是**叠加式**而非**替代式**：

```
最终费率 = 基础费率
         × (1 - VIP折扣)        // 应用 VIP 折扣
         × (1 - 促销折扣)        // 应用促销活动

示例：Taker 基础费率 0.1%
      + VIP2 折扣 40%  → 0.06%
      + 促销折扣 10%  → 0.054%
```

### 2. 做市商优惠

做市商获得特殊的 **50% Maker 费率优惠**：

```
普通 Maker: 0.05%
做市商:     0.05% × 0.5 = 0.025%
```

### 3. 分层费率系统

基于用户 **30天交易量** 自动匹配费率档位：

```
交易量 0    ~ 100 BTC  → Bronze  (0.10%)
交易量 100  ~ 500 BTC  → Silver  (0.08%)
交易量 500+ BTC        → Gold    (0.05%)
```

### 4. VIP 等级折扣

用户可以同时享受**分层费率**和**VIP 折扣**：

```
普通 Bronze 用户: 0.10%
VIP1 Bronze 用户: 0.10% × (1 - 0.2) = 0.08%
VIP2 Bronze 用户: 0.10% × (1 - 0.4) = 0.06%
```

### 5. 最小手续费保护

小额交易应用最小手续费，防止费用过低：

```
基础资产最小费用: 0.001 BTC
报价资产费用: max(计算费用, 最小费用)
```

### 6. 促销活动支持

支持临时性的折扣活动，可指定目标用户和有效期：

```
活动名称: "新年促销"
折扣率: 10%
有效期: 2025-01-01 ~ 2025-01-31
目标用户: 所有用户（或特定 VIP 等级）
```

---

## 现货成交中的应用

### 应用场景

#### 场景 1: Taker 订单成交
```
订单: 买 1.0 BTC @ 50,000 USDT (市价单)
身份: Taker（立即成交）
用户等级: 普通用户（无 VIP）

手续费计算:
  交易金额 = 1.0 × 50,000 = 50,000 USDT
  Taker 费率 = 0.1%
  手续费 = 50,000 × 0.1% = 50 USDT

成交记录:
  commission_rate: 10 bp (0.1%)
  commission_qty: 50 USDT
  commission_asset: USDT
```

#### 场景 2: Maker 订单成交
```
订单: 卖 1.0 BTC @ 50,000 USDT (挂单)
身份: Maker（被 Taker 执行）
用户等级: 普通用户

手续费计算:
  交易金额 = 1.0 × 50,000 = 50,000 USDT
  Maker 费率 = 0.05%
  手续费 = 50,000 × 0.05% = 25 USDT

成交记录:
  commission_rate: 5 bp (0.05%)
  commission_qty: 25 USDT
  commission_asset: USDT
```

#### 场景 3: VIP 用户成交
```
订单: 买 10.0 ETH @ 3,000 USDT (市价单)
身份: Taker
用户等级: VIP2（40% 折扣）

手续费计算:
  交易金额 = 10.0 × 3,000 = 30,000 USDT
  基础 Taker 费率 = 0.1%
  VIP2 折扣 = 40%
  最终费率 = 0.1% × (1 - 0.4) = 0.06%
  手续费 = 30,000 × 0.06% = 18 USDT

成交记录:
  commission_rate: 6 bp (0.06%)
  commission_qty: 18 USDT
  commission_asset: USDT

用户节省: 30,000 × 0.04% = 12 USDT
```

#### 场景 4: 做市商成交
```
订单: 卖 5.0 BTC @ 50,100 USDT (挂单)
身份: Maker
用户等级: 做市商

手续费计算:
  交易金额 = 5.0 × 50,100 = 250,500 USDT
  基础 Maker 费率 = 0.05%
  做市商优惠 = 50%
  最终费率 = 0.05% × 0.5 = 0.025%
  手续费 = 250,500 × 0.025% = 62.625 USDT

成交记录:
  commission_rate: 2.5 bp (0.025%)
  commission_qty: 62.625 USDT
  commission_asset: USDT

用户节省: 250,500 × (0.05% - 0.025%) = 62.625 USDT
```

### 应用集成点

#### 1. 订单提交时
```
订单在 pending 状态时，预计算费率（供用户确认）
```

#### 2. 订单成交时（核心应用）
```rust
// 在 SpotOrder::trade() 方法中
let (commission_rate, commission_qty) = self.calculate_fee_with_amount(
    &fee_entity,          // CEX 手续费配置元数据
    is_taker,            // 区分 Maker/Taker
    is_market_maker,     // 特殊身份标识
    user_vip_level,      // 用户 VIP 等级
    user_tier,           // 用户分层等级
    filled,              // 成交数量
    price,               // 成交价格
);

// 创建成交记录
let trade = SpotTrade::new(
    trade_id,
    self.order_id,
    timestamp,
    price,
    filled,
    taker_side,
    commission_qty,      // ← 计算出的手续费数量
    commission_asset,
    commission_rate,     // ← 计算出的手续费率
);
```

#### 3. 账户结算时
```
收取手续费: account.balance -= commission_qty
手续费归集账户: fee_account.balance += commission_qty
```

#### 4. 用户查询时
```
显示成交记录中的:
  - commission_rate: 10 bp
  - commission_qty: 50 USDT
  - 费率说明: "Taker 0.1% 基础费率"
```

---

## 使用示例

### 示例 1: 创建基础费率表

```rust
use fee::core::fee_types::CexFeeEntity;

// 创建默认配置的费率表
let mut fee_config = CexFeeEntity::new(
    "binance".to_string(),
    "standard_fees".to_string(),
    0.0005,  // Maker: 0.05%
    0.001,   // Taker: 0.1%
);

println!("费率表已创建: {}", fee_config.fee_schedule_id);
```

### 示例 2: 添加 VIP 等级

```rust
use fee::core::fee_types::VIPLevel;

fee_config.vip_levels.push(VIPLevel {
    level_id: 1,
    level_name: "VIP1".to_string(),
    requirements: "30天交易量 > 100 BTC".to_string(),
    fee_discount: 0.2,  // 20% 折扣
    special_benefits: vec!["Priority support".to_string()],
});

fee_config.vip_levels.push(VIPLevel {
    level_id: 2,
    level_name: "VIP2".to_string(),
    requirements: "30天交易量 > 500 BTC".to_string(),
    fee_discount: 0.4,  // 40% 折扣
    special_benefits: vec![
        "Priority support".to_string(),
        "Fast withdrawal".to_string(),
    ],
});
```

### 示例 3: 添加分层费率

```rust
use fee::core::fee_types::FeeTier;

fee_config.fee_tiers.push(FeeTier {
    tier_id: 1,
    tier_name: "Bronze".to_string(),
    min_volume_30d: 0.0,
    min_balance: 0.0,
    maker_fee: 0.001,      // 0.1%
    taker_fee: 0.0015,     // 0.15%
    withdrawal_fee_fixed: 0.0,
    withdrawal_fee_percent: 0.0,
    is_active: true,
});

fee_config.fee_tiers.push(FeeTier {
    tier_id: 2,
    tier_name: "Silver".to_string(),
    min_volume_30d: 100.0,  // 100 BTC
    min_balance: 10000.0,
    maker_fee: 0.0008,      // 0.08%
    taker_fee: 0.0012,      // 0.12%
    withdrawal_fee_fixed: 0.0,
    withdrawal_fee_percent: 0.0,
    is_active: true,
});

fee_config.fee_tiers.push(FeeTier {
    tier_id: 3,
    tier_name: "Gold".to_string(),
    min_volume_30d: 1000.0,  // 1000 BTC
    min_balance: 100000.0,
    maker_fee: 0.0005,       // 0.05%
    taker_fee: 0.001,        // 0.1%
    withdrawal_fee_fixed: 0.0,
    withdrawal_fee_percent: 0.0,
    is_active: true,
});
```

### 示例 4: 在订单成交中应用

```rust
use fee::core::fee_types::FeeType;

// 计算 Taker 费率（立即成交）
let result = fee_config.calculate_trading_fee(
    FeeType::Taker,
    "BTC",        // 基础资产
    "USDT",       // 报价资产
    1.0,          // 交易量
    50000.0,      // 价格
    Some(2),      // Silver 分层
    Some(1),      // VIP1 等级
    false,        // 非做市商
)?;

println!("Taker 费率: {:.6} ({:.4}%)",
         result.final_rate,
         result.final_rate * 100.0);

// 计算手续费金额: 50000 USDT * 0.0008 = 40 USDT
// Silver (0.12%) × VIP1折扣 (1-0.2) = 0.096% → ~0.08%
```

### 示例 5: 在 SpotOrder 中应用

```rust
// 在 trade() 方法中自动计算费率
let (commission_rate, commission_qty) = order.calculate_fee_with_amount(
    &fee_config,
    true,           // is_taker
    false,          // is_market_maker
    Some(1),        // VIP1
    Some(2),        // Silver tier
    Quantity::from_f64(1.0),
    Price::from_f64(50000.0),
);

println!("手续费率: {} bp", commission_rate);
println!("手续费数量: {} USDT", commission_qty.to_f64());

// 创建成交记录，包含计算出的费率信息
let trade = SpotTrade::new(
    trade_id,
    order_id,
    timestamp,
    price,
    filled,
    taker_side,
    commission_qty,      // 自动计算的手续费
    commission_asset,
    commission_rate,     // 自动计算的费率
);
```

---

## 集成指南

### 集成步骤

#### 1. 初始化费率配置

```rust
// 应用启动时，从数据库或配置文件加载 CexFeeEntity
let fee_config = load_fee_config_from_db("binance_standard");

// 或创建新的默认配置
let mut fee_config = CexFeeEntity::new(
    "exchange_id".to_string(),
    "schedule_name".to_string(),
    0.0005,
    0.001,
);
```

#### 2. 注册到全局上下文

```rust
// 在应用全局状态中存储 CexFeeEntity
pub struct AppState {
    pub fee_entity: Arc<CexFeeEntity>,
    // ... 其他字段
}

let app_state = AppState {
    fee_entity: Arc::new(fee_config),
    // ...
};
```

#### 3. 在订单成交时应用

```rust
// 在匹配引擎的 make_trade() 中
pub fn make_trade(
    &mut self,
    buyer: &mut SpotOrder,
    seller: &mut SpotOrder,
    fee_entity: &CexFeeEntity,  // 传入费率配置
) {
    // ...
    let buyer_trade = buyer.trade(
        filled,
        price,
        true,  // buyer is taker
        &mut buyer_balance,
        &mut base_balance,
    );

    let seller_trade = seller.trade(
        filled,
        price,
        false,  // seller is maker
        &mut seller_balance,
        &mut base_balance,
    );
    // ...
}
```

#### 4. 持久化成交记录

```rust
// 保存包含费率信息的成交记录到数据库
let trade_record = TradeRecord {
    trade_id: trade.trade_id,
    order_id: trade.order_id,
    price: trade.price,
    quantity: trade.quantity,
    commission_rate: trade.commission_rate,  // 基点数
    commission_qty: trade.commission_qty,    // 手续费数量
    commission_asset: trade.commission_asset,
    timestamp: trade.timestamp,
};

db.insert_trade(trade_record)?;
```

### 查询和报告

#### 用户手续费查询

```rust
// 查询用户在某个时间段的手续费统计
pub fn query_user_fees(
    user_id: u64,
    start_time: u64,
    end_time: u64,
) -> FeeStatistics {
    let trades = db.query_trades(user_id, start_time, end_time);

    let total_fees: f64 = trades
        .iter()
        .map(|t| t.commission_qty)
        .sum();

    let by_type = trades
        .iter()
        .group_by(|t| t.fee_type)
        .into_iter()
        .map(|(fee_type, trades)| {
            (fee_type, trades.map(|t| t.commission_qty).sum())
        })
        .collect();

    FeeStatistics {
        total_fees,
        by_type,
        avg_fee_rate: calculate_average_rate(&trades),
    }
}
```

#### 手续费收入报告

```rust
// 交易所收取的总手续费统计
pub fn generate_fee_report(date: NaiveDate) -> ExchangeFeeReport {
    let trades = db.query_trades_by_date(date);

    let total_collected = trades
        .iter()
        .map(|t| t.commission_qty)
        .sum();

    let by_asset = trades
        .iter()
        .group_by(|t| &t.commission_asset)
        .into_iter()
        .map(|(asset, trades)| {
            (asset.clone(), trades.map(|t| t.commission_qty).sum())
        })
        .collect();

    let by_rate = categorize_by_rate(&trades);

    ExchangeFeeReport {
        date,
        total_collected,
        by_asset,
        by_rate,
        trade_count: trades.len(),
    }
}
```

---

## 架构图

### 数据流

```
┌─────────────────────────────────────┐
│   用户提交订单                        │
│   (Buyer/Seller)                    │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   订单进入订单簿                      │
│   Pending 状态                        │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   订单配对成交                        │
│   make_trade()                       │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   查询 CexFeeEntity 配置              │
│   获取费率元数据                      │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   calculate_fee_with_amount()        │
│   - 判断 Maker/Taker                 │
│   - 查询 VIP 等级折扣                 │
│   - 查询分层费率                      │
│   - 查询促销活动                      │
│   → 返回 (rate_bp, fee_qty)          │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   创建 SpotTrade 成交记录             │
│   - commission_rate: 10 bp           │
│   - commission_qty: 50 USDT          │
└────────────┬────────────────────────┘
             │
             ▼
┌─────────────────────────────────────┐
│   账户结算                            │
│   - 冻结手续费                        │
│   - 更新余额                          │
│   - 持久化成交记录                    │
└─────────────────────────────────────┘
```

### 模块依赖

```
┌──────────────────┐
│   fee crate      │
│                  │
│ ┌──────────────┐ │
│ │CexFeeEntity  │ │
│ │CalculateRate│ │
│ └──────────────┘ │
└────────┬─────────┘
         │
         │ 依赖
         ▼
┌──────────────────┐
│   lob crate      │
│                  │
│ ┌──────────────┐ │
│ │ SpotOrder    │ │
│ │ .trade()     │ │
│ │ .make_trade()│ │
│ └──────────────┘ │
│                  │
│ ┌──────────────┐ │
│ │ SpotTrade    │ │
│ │ (成交记录)    │ │
│ └──────────────┘ │
└──────────────────┘
```

---

## 扩展性和维护

### 添加新的费率维度

如需支持新的费率影响因素：

1. **在 CexFeeEntity 中添加新字段**
   ```rust
   pub struct CexFeeEntity {
       // ... 现有字段
       pub region_specific_fees: HashMap<String, f64>,  // 按地区费率
   }
   ```

2. **在 calculate_fee_with_amount() 中添加逻辑**
   ```rust
   let region_adjustment = self.get_region_adjustment(user_region);
   final_rate *= region_adjustment;
   ```

3. **编写相应的测试**
   ```rust
   #[test]
   fn test_region_specific_fee() {
       // 测试按地区的费率
   }
   ```

### 更新费率表

```rust
// 方案1: 创建新的费率表配置，旧配置设置失效时间
fee_entity.valid_until = Some(Utc::now() + Duration::days(30));

// 方案2: 直接修改配置（需要考虑并发问题）
fee_entity.default_taker_fee = 0.0009;  // 新费率

// 方案3: 使用数据库版本控制
db.create_fee_schedule_version(fee_config_v2);
```

### 性能考虑

1. **缓存 CexFeeEntity**
   - 将活跃的费率表缓存在内存中
   - 使用 Arc<RwLock<CexFeeEntity>> 支持并发读

2. **费率计算优化**
   - 缓存用户的 VIP 等级和分层信息
   - 预计算常用的费率组合

3. **大批量查询**
   ```rust
   // 批量查询时，复用同一个 fee_entity 实例
   for trade in trades.iter() {
       let rate = order.calculate_fee_with_amount(
           &shared_fee_entity,  // 复用实例
           // ...
       );
   }
   ```

---

## 总结

### 关键优势

| 优势 | 说明 |
|------|------|
| **解耦** | 手续费逻辑独立，易于维护和更新 |
| **灵活** | 支持多维度的费率组合和优化 |
| **可配置** | 无需代码修改即可调整费率 |
| **可审计** | 成交记录包含完整的费率信息 |
| **可扩展** | 易于添加新的费率维度 |

### 应用场景

- ✅ 现货交易成交计算
- ✅ 衍生品交易手续费
- ✅ 用户费率等级查询
- ✅ 交易所费收统计
- ✅ 用户成本分析
- ✅ 费率政策变更实施

### 最佳实践

1. **集中管理**：所有费率配置存储在 CexFeeEntity 中
2. **版本控制**：费率变更时创建新版本而非直接修改
3. **缓存策略**：将活跃配置缓存在内存，定期更新
4. **监控告警**：监控手续费计算异常
5. **用户透明**：在确认界面向用户展示实际费率
6. **审计日志**：记录所有费率变更

---

## 附录

### 常见问题 (FAQ)

**Q: 如何处理费率实时变更？**
A: 创建新的 CexFeeEntity 版本，设置 valid_from 和 valid_until 时间，系统会自动使用有效的配置。

**Q: 多个费率规则冲突时如何处理？**
A: 优先级为：促销活动 > VIP 折扣 > 分层费率 > 基础费率。取最优（最低）费率。

**Q: 如何支持动态的交易量计算？**
A: 在 trade() 方法调用前，实时查询用户 30 天交易量，根据结果判断应该使用的 FeeTier。

**Q: 手续费精度如何控制？**
A: 通过 FeePrecision 配置，或在 SpotTrade 中使用 Quantity 类型（已包含精度处理）。

---

**文档版本**: v1.0
**最后更新**: 2025-12-27
