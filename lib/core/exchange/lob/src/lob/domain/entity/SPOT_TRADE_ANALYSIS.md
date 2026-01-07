# SpotTrade 字段设计竞品分析

## 当前 SpotTrade 结构
```rust
pub struct SpotTrade {
    pub trade_id: u64,              // 交易唯一标识
    pub price: Price,               // 成交价格
    pub quantity: Quantity,         // 成交数量
    pub taker_trader: TraderId,     // Taker交易员
    pub maker_trader: TraderId,     // Maker交易员
    pub taker_order_id: OrderId,    // Taker订单ID
    pub maker_order_id: OrderId,    // Maker订单ID
    pub taker_side: Side            // Taker方向（Buy/Sell）
}
```

## 竞品交易所对比分析

### 1. 币安 (Binance) - Account Trade List
**关键字段:**
- `id`: 交易ID
- `orderId`: 订单ID
- `price`: 成交价格
- `qty`: 成交数量
- `quoteQty`: 成交金额（qty × price）
- `commission`: 手续费数量
- `commissionAsset`: 手续费资产
- `time`: 成交时间戳 (ms)
- `isBuyer`: 是否买方
- `isMaker`: 是否做市商
- `isBestMatch`: 是否最优匹配

**缺陷:** 只记录当前方（self）的信息，缺少对手方信息

---

### 2. OKX - Get Order History (Fills)
**关键字段:**
- `ordId`: 订单ID
- `tradeId`: 交易ID
- `clOrdId`: 客户订单ID（用于幂等性）
- `instId`: 产品ID/交易对
- `side`: 买卖方向
- `px`: 成交价格
- `sz`: 成交数量
- `fee`: 手续费数量
- `feeRate`: 手续费率（%）
- `feeCcy`: 手续费计价货币
- `execType`: 成交类型 (Taker/Maker)
- `tradeTime`: 交易时间戳
- `state`: 订单状态

**优势:** 包含手续费率、客户订单ID、成交类型

---

### 3. Coinbase - Get Fills
**关键字段:**
- `entry_id`: 条目ID
- `trade_id`: 交易ID
- `order_id`: 订单ID
- `product_id`: 产品ID
- `order_side`: 订单方向 (buy/sell)
- `price`: 成交价格
- `size`: 成交数量
- `fee`: 手续费（绝对值）
- `created_at`: 创建时间 (ISO 8601)
- `liquidity`: 流动性 (M/T) - Maker/Taker

**优势:** 清晰简洁，liquidity 字段简化 isMaker 逻辑

---

## 推荐增强字段

### 核心交易字段（必需）
| 字段 | 类型 | 说明 | 币安 | OKX | Coinbase |
|------|------|------|------|-----|----------|
| trade_id | u64 | 交易唯一ID | ✓ | ✓ | ✓ |
| order_id | u64 | 订单ID | ✓ | ✓ | ✓ |
| price | Price (i64) | 成交价格 | ✓ | ✓ | ✓ |
| quantity | Quantity (i64) | 成交数量 | ✓ | ✓ | ✓ |
| side | Side | 方向 (Buy/Sell) | ✓ | ✓ | ✓ |
| liquidity | Liquidity | Taker/Maker | ✗ | ✓ | ✓ |

### 费用字段（重要）
| 字段 | 类型 | 说明 | 币安 | OKX | Coinbase |
|------|------|------|------|-----|----------|
| commission_qty | Quantity | 手续费数量 | ✓ | ✓ | ✓ |
| commission_asset | AssetId | 手续费资产 | ✓ | ✓ | ✓ |
| commission_rate | i32 | 手续费率 (bp) | ✗ | ✓ | ✗ |

### 成交金额字段（可选）
| 字段 | 类型 | 说明 | 币安 | OKX | Coinbase |
|------|------|------|------|-----|----------|
| quote_qty | Quantity | 成交金额 | ✓ | ✗ | ✗ |

### 对手方信息（本项目特有）
| 字段 | 类型 | 说明 | 币安 | OKX | Coinbase |
|------|------|------|------|-----|----------|
| taker_trader | TraderId | Taker交易员 | ✗ | ✗ | ✗ |
| maker_trader | TraderId | Maker交易员 | ✗ | ✗ | ✗ |
| maker_order_id | u64 | Maker订单ID | ✗ | ✗ | ✗ |

### 时间戳字段（重要）
| 字段 | 类型 | 说明 | 币安 | OKX | Coinbase |
|------|------|------|------|-----|----------|
| timestamp | u64 | 成交时间 (ms) | ✓ | ✓ | ✓ |
| sequence | u64 | 成交序列号 | ✗ | ✗ | ✗ |

---

## 推荐 SpotTrade 增强设计

### 选项1：极简设计（仅核心字段）
```rust
#[derive(Debug, Clone, Copy)]
pub struct SpotTrade {
    // === 交易标识 ===
    pub trade_id: u64,
    pub timestamp: u64,          // 新增：成交时间戳

    // === 价格数量 ===
    pub price: Price,
    pub quantity: Quantity,
    pub quote_qty: Quantity,     // 新增：成交金额 = qty × price

    // === 订单和方向 ===
    pub taker_order_id: OrderId,
    pub maker_order_id: OrderId,
    pub taker_side: Side,

    // === 交易员 ===
    pub taker_trader: TraderId,
    pub maker_trader: TraderId,

    // === 费用 ===
    pub commission_qty: Quantity,    // 新增：手续费数量
    pub commission_asset: AssetId,   // 新增：手续费资产

    // === Taker/Maker标记 ===
    pub is_taker_buy: bool,          // 新增：true=Taker买入, false=Taker卖出
}
```

### 选项2：完整设计（包含手续费率）
```rust
#[derive(Debug, Clone, Copy)]
pub struct SpotTrade {
    // === 交易标识 ===
    pub trade_id: u64,
    pub timestamp: u64,
    pub sequence: u64,               // 新增：成交序列号

    // === 价格数量 ===
    pub price: Price,
    pub quantity: Quantity,
    pub quote_qty: Quantity,

    // === 订单和方向 ===
    pub taker_order_id: OrderId,
    pub maker_order_id: OrderId,
    pub taker_side: Side,

    // === 交易员 ===
    pub taker_trader: TraderId,
    pub maker_trader: TraderId,

    // === 费用 ===
    pub commission_qty: Quantity,
    pub commission_asset: AssetId,
    pub commission_rate: i32,        // 新增：手续费率 (bp, 例: 10 = 0.1%)

    // === Taker/Maker标记 ===
    pub is_taker_buy: bool,
}
```

---

## 优化建议

### 1. 添加 timestamp（必须）
- **原因:** 审计、对账、排序需要
- **币安/OKX/Coinbase:** 都包含
- **推荐:** `timestamp: u64` (ms)

### 2. 添加 quote_qty（推荐）
- **原因:** 直接计算成交金额，提升效率
- **公式:** `quote_qty = quantity × price`
- **币安:** 包含 `quoteQty`
- **Coinbase/OKX:** 不直接提供，但易计算

### 3. 添加 commission 字段（推荐）
- **原因:** 结算、损益计算需要
- **币安/Coinbase:** 都包含
- **OKX:** 包含 `fee`, `feeRate`, `feeCcy`
- **推荐:** `commission_qty` + `commission_asset` + `commission_rate`

### 4. 添加 sequence（可选）
- **原因:** 无序列号交易所不需要；本项目如果涉及数据排序可考虑
- **币安/OKX/Coinbase:** 不直接提供
- **推荐:** 用 `trade_id` 或 `timestamp` 排序

### 5. 优化 Taker/Maker 表示
- **当前:** `taker_side: Side` 表示方向
- **推荐:**
  - 方案A（简洁）: `is_taker_buy: bool` - true=Taker买入
  - 方案B（显式）: `liquidity: Liquidity { Taker, Maker }`

---

## 最终建议

### 推荐最小增强（选项1）
在当前基础上添加这些字段：
1. ✅ `timestamp: u64` - 成交时间
2. ✅ `quote_qty: Quantity` - 成交金额
3. ✅ `commission_qty: Quantity` - 手续费
4. ✅ `commission_asset: AssetId` - 手续费资产

**理由:**
- 这4个字段是行业标准且必需
- 实现简单，零成本
- 满足大多数审计、结算需求

### 进阶增强（选项2）
如果需要更完整的信息，再加入：
5. `commission_rate: i32` - 手续费率 (bp)
6. `sequence: u64` - 成交序列号

---

## 对标结论

| 维度 | 币安 | OKX | Coinbase | 推荐 |
|------|------|-----|----------|------|
| 交易ID | ✓ | ✓ | ✓ | ✓ |
| 价格/数量 | ✓ | ✓ | ✓ | ✓ |
| 成交金额 | ✓ | ✗ | ✗ | ✓ |
| 手续费 | ✓ | ✓ | ✓ | ✓ |
| 时间戳 | ✓ | ✓ | ✓ | ✓ |
| Taker/Maker | ✓ | ✓ | ✓ | ✓ |
| 对手方信息 | ✗ | ✗ | ✗ | ✓ |
| 手续费率 | ✗ | ✓ | ✗ | ✓ |

**本项目优势:** 包含对手方信息（TraderId + Maker订单ID），这是券商/撮合系统的特有需求。
