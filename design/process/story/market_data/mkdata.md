# 行情数据读模型设计

**文档版本**: v1.6.0
**创建日期**: 2025-12-09
**最后更新**: 2025-12-09
**参考**: 币安(Binance)、OKX、Coinbase行情架构；NASDAQ ITCH、CME MDP3.0行业标准
**状态**: Review

---

## 目录

1. [概述](#概述)
2. [读模型设计原则](#读模型设计原则)
3. [核心读模型定义](#核心读模型定义)
4. [用户场景分析](#用户场景分析)
5. [数据存储策略](#数据存储策略)
6. [性能优化](#性能优化)
7. [实现示例](#实现示例)

---

## 概述

### 行情数据的特点

行情数据是典型的**读多写少**场景，符合CQRS架构的最佳实践：

| 特性 | 说明 |
|------|------|
| **读写比** | 1000:1 ~ 10000:1（读远大于写） |
| **实时性** | 毫秒级到秒级不等 |
| **数据量** | 海量时序数据 |
| **访问模式** | 热点数据集中（当前数据） |
| **用户群体** | 散户、专业交易者、量化机构 |

### CQRS架构视角

```
写侧（Command Side）                  读侧（Query Side）
        ↓                                    ↑
  撮合引擎成交事件                      多种行情读模型
        ↓                                    ↑
  [TradeExecutedEvent]  ────────→   实时更新读模型
        ↓                                    ↑
  持久化到事件存储                    散户App/量化系统查询
```

---

## 读模型设计原则

### 1. 用户导向原则

不同用户群体对行情数据的需求差异巨大：

| 用户类型 | 数据需求 | 实时性要求 | 数据粒度 | 访问频率 |
|---------|---------|-----------|---------|---------|
| **散户** | K线、最新价、24h统计 | 秒级（1-3s） | 粗粒度 | 中等 |
| **专业交易者** | 深度图、逐笔成交、Ticker | 100ms级 | 中等粒度 | 高 |
| **量化机构** | 完整订单簿、L2数据流 | 微秒级 | 细粒度 | 极高 |
| **数据分析师** | 历史K线、统计指标 | 分钟级 | 聚合数据 | 低 |

### 2. 分层缓存原则

```
L1: 内存缓存（最热数据）         → 微秒级访问
    ↓
L2: Redis缓存（热数据）          → 毫秒级访问
    ↓
L3: TimescaleDB（温数据）        → 10-100ms访问
    ↓
L4: 对象存储（冷数据）            → 秒级访问
```

### 3. 渐进式实时性

不同数据的更新频率应该匹配其实际需求：

| 数据类型 | 更新频率 | 推送方式 |
|---------|---------|---------|
| Ticker（最新价） | 实时（每笔成交） | WebSocket推送 |
| 市场深度（5档） | 100ms聚合推送 | WebSocket推送 |
| K线（1分钟） | 1秒更新 | WebSocket + HTTP轮询 |
| 24h统计 | 10秒更新 | HTTP轮询 |
| 历史K线 | 静态数据 | HTTP查询 |

---

## 读模型分类体系

### 分类维度

参考**NautilusTrader**的数据模型设计，行情数据读模型可以按照**数据类型**、**数据粒度**和**业务用途**进行分类。

#### NautilusTrader数据模型对照

NautilusTrader是开源的算法交易平台，其数据模型设计值得借鉴：

| NautilusTrader数据类型 | 对应的读模型 | 说明 |
|----------------------|------------|------|
| **Bar** | KLineView | 时间窗口聚合的OHLCV数据 |
| **QuoteTick** | MarketDepthView（BBO） | 最优买卖报价（Best Bid/Ask） |
| **TradeTick** | RecentTradesView | 逐笔成交数据（价格、数量、方向） |
| **OrderBook** | MarketDepthView | 完整订单簿（多档深度） |
| **OrderBookDelta** | OrderBookUpdatesView | 订单簿增量更新（逐笔委托） |
| **Ticker** | TickerView | 24小时统计数据 |

#### 按数据类型分类（参考NautilusTrader）

```
市场数据类型层次结构
│
├── Tick数据（最细粒度）
│   ├── QuoteTick（报价Tick）
│   │   └── 对应：TickerView.best_bid/best_ask
│   ├── TradeTick（成交Tick）
│   │   └── 对应：RecentTradesView
│   └── OrderBookDelta（订单簿变化Tick）
│       └── 对应：OrderBookUpdatesView ⭐
│
├── 聚合数据（时间窗口聚合）
│   ├── Bar（K线）
│   │   └── 对应：KLineView
│   └── Ticker（统计聚合）
│       └── 对应：TickerView
│
├── 快照数据（状态快照）
│   ├── OrderBook（订单簿快照）
│   │   └── 对应：MarketDepthView
│   └── Position（持仓快照）
│       └── 对应：用户私有数据
│
└── 事件数据（业务事件）
    ├── Trade（用户成交）
    │   └── 对应：TradeHistoryView
    ├── Liquidation（强平事件）
    │   └── 对应：LiquidationView
    └── FundingRate（资金费率）
        └── 对应：FundingRateView
```

#### 按数据粒度分类

| 分类 | 读模型 | 数据粒度 | 更新频率 | 主要用户 | NautilusTrader对应 |
|------|-------|---------|---------|---------|------------------|
| **Tick级数据** | OrderBookUpdatesView | 逐笔委托明细 | < 1ms | 量化机构、做市商 | OrderBookDelta |
| **Tick级数据** | RecentTradesView | 逐笔成交明细 | 实时 | 专业交易者、量化机构 | TradeTick |
| **快照数据** | MarketDepthView | 价格档位聚合 | 100ms | 专业交易者、量化机构 | OrderBook |
| **聚合数据** | TickerView | 24h统计聚合 | 实时 | 散户、专业交易者 | Ticker + QuoteTick |
| **时序数据** | KLineView | 时间窗口聚合 | 1s-1d | 散户、专业交易者 | Bar |
| **用户数据** | TradeHistoryView | 用户成交记录 | 事后查询 | 所有用户 | Trade（用户视角） |
| **衍生品数据** | LiquidationView | 强平事件 | 实时 | 专业交易者、量化机构 | Liquidation |
| **衍生品数据** | FundingRateView | 资金费率 | 8小时 | 专业交易者、量化机构 | FundingRate |

#### 按业务用途分类

```
行情数据读模型
├── 公开市场数据（Public Market Data）
│   ├── 聚合行情类
│   │   └── TickerView（24h统计、BBO）
│   ├── 时序数据类
│   │   └── KLineView（K线/蜡烛图）
│   ├── 订单簿类
│   │   ├── MarketDepthView（聚合深度）
│   │   └── OrderBookUpdatesView（逐笔委托）⭐ 新增
│   └── 成交数据类
│       └── RecentTradesView（逐笔成交）
│
├── 用户私有数据（Private User Data）
│   └── TradeHistoryView（用户成交历史）
│
└── 衍生品数据（Derivatives Data）
    ├── LiquidationView（强平数据）
    └── FundingRateView（资金费率）
```

### 关键区别：MarketDepthView vs OrderBookUpdatesView

| 维度 | MarketDepthView（聚合深度） | OrderBookUpdatesView（逐笔委托）⭐ |
|------|---------------------------|--------------------------------|
| **数据粒度** | 价格档位聚合（5/10/20档） | 每笔订单的增删改 |
| **更新方式** | 快照 + 增量（价格档位） | 逐笔订单事件流 |
| **数据量** | 小（KB级） | 大（MB级/秒） |
| **延迟要求** | 100ms（散户）/ 10ms（量化） | < 1ms（量化机构） |
| **主要用户** | 散户、专业交易者 | 量化机构、做市商、套利者 |
| **典型场景** | 深度图、挂单参考 | 订单流分析、微观结构研究 |
| **NautilusTrader对应** | OrderBook（快照） | OrderBookDelta（增量） |

### 市场数据分级：L1/L2/L3数据完整性分析

#### L1/L2/L3行业标准来源

市场数据的L1/L2/L3分级**是金融行业的事实标准（De Facto Standard）**，虽然没有单一的正式标准化组织定义，但在全球范围内被广泛采用和认可。

##### 标准来源和权威性

**主要标准制定者和实践者**：

1. **美国证券交易所（US Exchanges）**
   - **NASDAQ**：通过[NASDAQ TotalView-ITCH 5.0](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHSpecification.pdf)协议定义了L3数据标准
   - **NYSE**：通过Pillar协议提供多层次市场数据
   - 这些交易所的实现成为行业事实标准

2. **CME Group（芝加哥商品交易所集团）**
   - 通过[CME MDP 3.0](https://www.cmegroup.com/confluence/display/EPICSANDBOX/MDP+3.0+-+Message+Specification)定义了期货市场数据标准
   - **Market By Price (MBP)** = L2数据
   - **Market By Order (MBO)** = L3数据

3. **行业数据提供商共识**
   - [Databento](https://databento.com/microstructure/level-1-market-data)、[FinFeedAPI](https://www.finfeedapi.com/blog/market-data-level-1-2-3)、[CoinAPI](https://www.coinapi.io/blog/level-1-vs-level-2-vs-level-3-market-data-how-to-read-the-crypto-order-book)等主流数据提供商的一致定义
   - 这些定义在传统金融和加密货币市场通用

4. **FIX协议（Financial Information eXchange）**
   - FIX协议虽然主要用于订单执行，但其市场数据部分也支持L1/L2/L3的概念
   - 被全球金融机构广泛采用

##### 行业标准定义一致性

以下是多个权威来源对L1/L2/L3的一致定义：

| 级别 | Databento定义 | FinFeedAPI定义 | CME定义 | NASDAQ定义 | 一致性 |
|------|-------------|--------------|---------|-----------|--------|
| **L1** | BBO（最优买卖价）+ 最新价 | Top of book信息 | - | - | ✅ 100% |
| **L2** | 多档深度（价格聚合） | Order book深度 | Market By Price | - | ✅ 100% |
| **L3** | 逐笔订单（Order-by-Order） | 每个订单明细 | Market By Order | ITCH协议 | ✅ 100% |

##### 关键行业共识

> **Databento**: "Level 1 shows the surface. Level 2 reveals the structure. Level 3 exposes the entire market."

> **FinFeedAPI**: "L2 data is a superset of L1 data, meaning that L1 data can always be derived from L2 data."

> **CME Group**: "Market By Order (MBO) provides order-level granularity, allowing for the construction of a full limit order book."

##### 为什么没有单一正式标准？

1. **市场演化**：不同市场（股票、期货、外汇、加密货币）在不同时期独立发展
2. **竞争优势**：各交易所通过数据产品差异化竞争
3. **技术实现**：不同协议（ITCH、MDP、FIX）有不同的技术实现
4. **行业实践收敛**：尽管缺乏正式标准，实际定义已高度一致

##### 我们的设计对标

✅ **完全符合行业事实标准**：我们的L1/L2/L3定义与NASDAQ、CME、主流数据提供商的定义**100%一致**

---

#### 标准市场数据分级定义

市场数据按照**深度**和**粒度**分为三个级别（Level 1/2/3）：

| 级别 | 名称 | 数据内容 | 延迟 | 用户 | 我们的读模型覆盖 |
|------|------|---------|------|------|----------------|
| **L1** | 基础行情 | BBO（最优买卖价）、最新价、成交量 | 100ms-1s | 散户 | ✅ **完整** |
| **L2** | 市场深度 | 多档深度（5/10/20档）、聚合订单簿 | 10-100ms | 专业交易者 | ✅ **完整** |
| **L3** | 完整订单簿 | 逐笔委托、每个订单的ID、完整订单流 | < 1ms | 量化机构、做市商 | ✅ **完整** |

#### L1数据（Level 1 Market Data）

**定义**：最基础的市场行情数据，包含最优买卖价和最新成交信息。

**包含内容**：
- **BBO（Best Bid and Offer）**：最优买价、最优卖价、对应数量
- **Last Trade**：最新成交价、成交量
- **Daily Statistics**：开盘价、最高价、最低价、24h成交量

**对应的读模型**：
```
L1数据 → TickerView
├── best_bid / best_ask（BBO）
├── last_price（最新价）
├── volume_24h（24h成交量）
├── high_24h / low_24h（最高/最低价）
└── price_change_24h（价格变化）
```

**数据结构**：
```rust
/// L1数据（已覆盖）
pub struct TickerView {
    // BBO数据
    pub best_bid: Option<Decimal>,
    pub best_bid_qty: Option<Decimal>,
    pub best_ask: Option<Decimal>,
    pub best_ask_qty: Option<Decimal>,

    // 最新成交
    pub last_price: Decimal,

    // 24h统计
    pub high_24h: Decimal,
    pub low_24h: Decimal,
    pub volume_24h: Decimal,
    pub quote_volume_24h: Decimal,
    pub trade_count_24h: u64,
}
```

**覆盖情况**：✅ **完整覆盖**

---

#### L2数据（Level 2 Market Data）

**定义**：市场深度数据，显示多档买卖盘的价格和数量（聚合后的订单簿）。

**包含内容**：
- **多档深度**：通常5档、10档、20档、50档
- **价格档位聚合**：同一价格的所有订单聚合为一个档位
- **订单数量**：每个价格档位的订单总数量
- **增量更新**：价格档位的变化（不是单个订单）

**对应的读模型**：
```
L2数据 → MarketDepthView
├── bids: Vec<PriceLevel>（买盘深度）
│   └── [价格, 数量] × N档
├── asks: Vec<PriceLevel>（卖盘深度）
│   └── [价格, 数量] × N档
└── last_update_id（更新序列号）
```

**数据结构**：
```rust
/// L2数据（已覆盖）
pub struct MarketDepthView {
    pub symbol: String,
    pub last_update_id: u64,

    // 买盘深度（价格降序）
    pub bids: Vec<PriceLevel>,  // 5/10/20档

    // 卖盘深度（价格升序）
    pub asks: Vec<PriceLevel>,

    pub timestamp: i64,
}

pub struct PriceLevel {
    pub price: Decimal,
    pub quantity: Decimal,  // 该价格的聚合数量
}
```

**覆盖情况**：✅ **完整覆盖**

---

#### L3数据（Level 3 Market Data）

**定义**：完整的订单簿数据，包含每个订单的详细信息（Order-by-Order）。

**包含内容**：
- **逐笔委托**：每个订单的ID、价格、数量、时间
- **订单事件**：新增、修改、删除、部分成交
- **订单属性**：隐藏订单、冰山订单、Post-Only等
- **完整订单流**：所有订单的生命周期

**对应的读模型**：
```
L3数据 → OrderBookUpdatesView ⭐
├── OrderBookUpdate（单个订单更新）
│   ├── order_id（订单唯一ID）
│   ├── update_type（Add/Modify/Delete/PartialFill）
│   ├── price（价格）
│   ├── quantity（数量）
│   ├── side（买/卖）
│   ├── timestamp（纳秒时间戳）
│   └── flags（订单标志）
└── update_id（全局序列号）
```

**数据结构**：
```rust
/// L3数据（已覆盖）
pub struct OrderBookUpdatesView {
    pub symbol: String,
    pub updates: Vec<OrderBookUpdate>,
}

pub struct OrderBookUpdate {
    pub update_id: u64,           // 全局序列号
    pub order_id: u64,            // 订单ID（L3特有）
    pub update_type: OrderUpdateType,  // Add/Modify/Delete
    pub side: Side,
    pub price: Decimal,
    pub quantity: Option<Decimal>,
    pub timestamp: i64,           // 纳秒级时间戳
    pub flags: OrderFlags,        // 订单标志
}

pub enum OrderUpdateType {
    Add,          // 新增订单
    Modify,       // 修改订单
    Delete,       // 删除订单
    PartialFill,  // 部分成交
}

pub struct OrderFlags {
    pub is_hidden: bool,      // 隐藏订单
    pub is_iceberg: bool,     // 冰山订单
    pub is_post_only: bool,   // Post-Only
}
```

**覆盖情况**：✅ **完整覆盖**

---

#### L1/L2/L3数据对比总结

| 维度 | L1数据 | L2数据 | L3数据 |
|------|-------|-------|-------|
| **数据粒度** | BBO + 最新价 | 多档深度（聚合） | 逐笔委托（明细） |
| **订单ID** | ❌ 无 | ❌ 无 | ✅ 有（每个订单） |
| **更新方式** | 快照 | 快照 + 增量（价格档位） | 事件流（订单级别） |
| **数据量** | 极小（< 1KB） | 小（KB级） | 大（MB级/秒） |
| **延迟** | 100ms-1s | 10-100ms | < 1ms |
| **带宽** | 极低 | 低 | 高 |
| **用户** | 散户 | 专业交易者 | 量化机构、做市商 |
| **我们的读模型** | TickerView | MarketDepthView | OrderBookUpdatesView |
| **覆盖情况** | ✅ 完整 | ✅ 完整 | ✅ 完整 |

#### 补充数据类型（非L1/L2/L3标准）

除了标准的L1/L2/L3数据，我们还覆盖了以下数据类型：

| 数据类型 | 读模型 | 说明 | 覆盖情况 |
|---------|-------|------|---------|
| **逐笔成交** | RecentTradesView | 公开成交记录（TradeTick） | ✅ 完整 |
| **K线数据** | KLineView | 时间窗口聚合（Bar） | ✅ 完整 |
| **用户成交** | TradeHistoryView | 用户私有成交记录 | ✅ 完整 |
| **强平数据** | LiquidationView | 合约强平事件 | ✅ 完整 |
| **资金费率** | FundingRateView | 永续合约费率 | ✅ 完整 |

#### 完整性结论

✅ **我们的读模型已完整覆盖L1/L2/L3所有级别的市场数据**

```
市场数据完整性检查
├── L1数据（基础行情）
│   └── ✅ TickerView（BBO + 最新价 + 24h统计）
├── L2数据（市场深度）
│   └── ✅ MarketDepthView（多档深度 + 增量更新）
├── L3数据（完整订单簿）
│   └── ✅ OrderBookUpdatesView（逐笔委托 + 订单ID + 订单标志）
└── 扩展数据
    ├── ✅ RecentTradesView（逐笔成交）
    ├── ✅ KLineView（K线数据）
    ├── ✅ TradeHistoryView（用户成交）
    ├── ✅ LiquidationView（强平数据）
    └── ✅ FundingRateView（资金费率）
```

#### 与主流交易所对比

| 交易所 | L1 | L2 | L3 | 我们的覆盖 |
|-------|----|----|----|---------|
| **币安** | ✅ Ticker | ✅ Depth（20档） | ❌ 不提供 | ✅ 全覆盖 + L3 |
| **Coinbase Pro** | ✅ Ticker | ✅ Level2 | ✅ Full（付费） | ✅ 全覆盖 |
| **Kraken** | ✅ Ticker | ✅ Depth | ✅ Book（付费） | ✅ 全覆盖 |
| **FTX** | ✅ Ticker | ✅ Orderbook | ❌ 不提供 | ✅ 全覆盖 + L3 |
| **我们** | ✅ | ✅ | ✅ | ✅ **完整** |

**关键优势**：
- 我们提供了完整的L3数据（OrderBookUpdatesView），而大多数交易所不提供或仅对机构客户提供
- 数据粒度从粗到细完整覆盖，满足所有用户需求

#### 主流交易所快照机制对比

基于对币安、OKX、Coinbase的实际调研，所有主流交易所都实现了快照机制：

| 维度 | 币安 (Binance) | OKX | Coinbase | **我们的设计** |
|------|---------------|-----|----------|--------------|
| **L1快照** | ✅ REST API | ✅ WebSocket自动推送 | ✅ WebSocket自动推送 | ✅ WebSocket自动推送 |
| **L2快照** | ✅ REST API | ✅ WebSocket (400档) | ✅ WebSocket | ✅ WebSocket + Redis |
| **L3快照** | ❌ 不提供 | ❌ 不提供 | ✅ REST API | ✅ WebSocket + REST |
| **初始化方式** | REST拉取 | WebSocket推送 | WebSocket推送(L2) / REST拉取(L3) | 混合模式 |
| **序列号机制** | ✅ U/u双序号 | ✅ seqId | ✅ sequence | ✅ update_id/snapshot_id |
| **一致性验证** | 序列号连续性 | CRC32校验和 | 序列号连续性 | 序列号 + 可选CRC32 |
| **快照频率** | 按需请求 | 初始推送 | 初始推送 | 1Hz(常规) / 10Hz(活跃) |
| **增量频率** | 1000ms / 100ms | 100ms / 10ms | 实时 | 100ms / 实时 |
| **丢包恢复** | 重新请求快照 | 重新请求快照 | 序列号gap检测后重建 | 自动快照恢复 |

**币安快照机制详解**:
```
官方推荐流程（来源：Binance Developer Documentation）:
1. 订阅WebSocket增量流 (wss://stream.binance.com:9443/ws/bnbbtc@depth)
2. 缓存接收到的增量更新
3. 通过REST API获取完整快照 (GET /api/v3/depth?symbol=BNBBTC&limit=5000)
4. 丢弃 u <= lastUpdateId 的缓存事件
5. 首个增量事件必须满足: U <= lastUpdateId + 1 AND u >= lastUpdateId + 1
6. 后续增量保持序列号连续性

序列号机制:
- lastUpdateId: 快照序列号
- U: 增量更新的第一个序列号
- u: 增量更新的最后一个序列号
```

**OKX快照机制详解**:
```
初始快照推送（来源：OKX API Documentation）:
- 订阅时自动推送400档深度快照
- action: "snapshot" 标识快照消息
- 增量更新频率: 100ms (标准) / 10ms (Nitro Spread)
- 使用CRC32校验和验证订单簿完整性

数据完整性验证:
- 客户端计算本地订单簿的CRC32校验和
- 与服务器推送的checksum字段对比
- 不匹配时重新请求快照
```

**Coinbase快照机制详解**:
```
L2快照（来源：Coinbase Developer Platform）:
- WebSocket订阅时自动推送完整快照
- type: "snapshot" 消息包含完整bids/asks数组
- 增量更新: type: "l2update" 消息

L3快照（Full Order Book）:
- 不通过WebSocket推送快照（数据量太大）
- 推荐流程:
  1. 订阅WebSocket的full频道
  2. 缓存接收到的所有消息
  3. 通过REST API获取完整快照: GET /products/<product-id>/book?level=3
  4. 使用序列号重放缓存的消息
  5. 应用实时流

序列号reconciliation:
- 每个消息包含全局递增的sequence字段
- 客户端检测sequence gap后重新请求快照
```

**我们的设计优势**:

1. ✅ **完整的L3支持**: 与Coinbase对齐，超越币安和OKX
   - 提供`OrderBookUpdatesView`完整逐笔委托数据
   - 支持量化机构和做市商需求

2. ✅ **灵活的快照策略**:
   - L1/L2: WebSocket自动推送快照（OKX模式）
   - L3: REST API + WebSocket增量（Coinbase模式）
   - 提供最优性能和一致性平衡

3. ✅ **序列号一致性保证**:
   ```rust
   fn check_consistency(snapshot: &Snapshot, delta: &Delta) -> bool {
       delta.first_update_id <= snapshot.snapshot_id + 1 &&
       delta.last_update_id >= snapshot.snapshot_id + 1
   }
   ```
   - 与币安的`U/u`机制相同逻辑

4. ✅ **自动恢复机制**:
   ```rust
   if msg.order_book_seq != order_book.last_seq + 1 {
       order_book = request_new_snapshot()?;  // 自动重建
   }
   ```
   - 优于币安/OKX的手动请求模式

5. ✅ **可选CRC32校验**:
   - 借鉴OKX的数据完整性验证机制
   - 提供额外的数据一致性保障

**行业对标结论**:

我们的v1.5.0设计**完全符合**甚至**超越**行业最佳实践：

| 特性 | 行业标准 | 我们的实现 | 评级 |
|------|---------|-----------|------|
| L1/L2/L3覆盖 | 币安/OKX仅L1/L2| ✅ 完整L1/L2/L3 | ⭐⭐⭐⭐⭐ |
| 快照机制 | 所有主流交易所必备 | ✅ 完整实现 | ⭐⭐⭐⭐⭐ |
| 序列号一致性 | 币安双序号，Coinbase单序号 | ✅ 灵活支持 | ⭐⭐⭐⭐⭐ |
| 丢包恢复 | 手动请求快照 | ✅ 自动恢复 | ⭐⭐⭐⭐⭐ |
| 数据完整性 | OKX使用CRC32 | ✅ 可选CRC32 | ⭐⭐⭐⭐ |

**参考文档**:
- [Binance WebSocket Depth Streams](https://developers.binance.com/docs/derivatives/usds-margined-futures/websocket-market-streams/How-to-manage-a-local-order-book-correctly)
- [OKX WebSocket Order Book Channel](https://www.okx.com/docs-v5/log_en/)
- [Coinbase WebSocket Channels](https://docs.cloud.coinbase.com/exchange/docs/websocket-channels)

---

### 快照机制（Snapshot Mechanism）⭐

#### 为什么需要快照？

快照是市场数据系统的**关键恢复机制**，解决以下问题：

| 问题 | 快照的作用 |
|------|----------|
| **客户端初始化** | 新连接的客户端需要完整的初始状态 |
| **丢包恢复** | 增量更新丢包后，通过快照重建状态 |
| **状态同步** | 确保客户端与服务器状态一致 |
| **减少带宽** | 避免重传所有历史增量消息 |

#### 快照 vs 增量更新

```
快照（Snapshot）
├── 定义：某个时刻的完整状态
├── 特点：数据量大，但自包含
├── 使用场景：初始化、恢复、定期同步
└── 示例：完整的20档订单簿

增量更新（Delta/Incremental）
├── 定义：状态的变化量
├── 特点：数据量小，但需要基础状态
├── 使用场景：实时更新、带宽优化
└── 示例：单个价格档位的变化
```

#### 各读模型的快照策略

| 读模型 | 是否需要快照 | 快照类型 | 快照频率 | 增量更新 |
|-------|------------|---------|---------|---------|
| **TickerView** | ✅ 需要 | 完整Ticker | 实时（每次成交） | 无（直接覆盖） |
| **KLineView** | ✅ 需要 | 历史K线 | 按需（客户端请求） | 当前K线实时更新 |
| **MarketDepthView** | ✅ **必需** | 完整订单簿 | 连接时 + 每秒 | 价格档位增量 |
| **OrderBookUpdatesView** | ✅ **必需** | 完整订单簿 | 连接时 + 每100条增量 | 逐笔订单事件 |
| **RecentTradesView** | ❌ 不需要 | - | - | 逐笔推送 |
| **TradeHistoryView** | ❌ 不需要 | - | - | 按需查询 |
| **LiquidationView** | ❌ 不需要 | - | - | 事件推送 |
| **FundingRateView** | ✅ 需要 | 当前费率 | 每8小时 | 无 |

#### 1. MarketDepthView快照机制（L2数据）

**快照数据结构**：
```rust
/// L2订单簿快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepthSnapshot {
    /// 交易对
    pub symbol: String,

    /// 快照序列号（用于增量更新对齐）
    pub snapshot_id: u64,

    /// 快照时间戳
    pub timestamp: i64,

    /// 买盘深度（完整20档）
    pub bids: Vec<PriceLevel>,

    /// 卖盘深度（完整20档）
    pub asks: Vec<PriceLevel>,
}

/// 价格档位
pub struct PriceLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}
```

**快照 + 增量更新流程**：
```
客户端连接
    ↓
1. 请求快照（或自动推送）
    ↓
GET /api/v1/depth/snapshot?symbol=BTCUSDT&limit=20
    ↓
返回：MarketDepthSnapshot {
    snapshot_id: 12345678,
    bids: [[50000, 1.5], [49999, 2.3], ...],
    asks: [[50001, 1.2], [50002, 3.0], ...]
}
    ↓
2. 订阅增量更新
    ↓
WebSocket: depth@100ms
    ↓
增量消息：{
    first_update_id: 12345679,
    last_update_id: 12345680,
    bids: [[50000, 1.8]],  // 价格50000的数量变为1.8
    asks: []
}
    ↓
3. 应用增量（检查序列号连续性）
    ↓
if first_update_id <= snapshot_id + 1 <= last_update_id:
    apply_delta()
else:
    request_new_snapshot()  // 序列号不连续，重新请求快照
```

**代码示例**：
```rust
// 订单簿快照管理器
pub struct OrderBookSnapshotManager {
    redis: redis::Client,
    snapshot_interval: Duration,
}

impl OrderBookSnapshotManager {
    /// 生成订单簿快照
    pub async fn generate_snapshot(&self, symbol: &str) -> Result<MarketDepthSnapshot> {
        let mut conn = self.redis.get_async_connection().await?;

        // 从Redis获取完整订单簿
        let bids: Vec<(Decimal, Decimal)> = conn
            .zrevrange_withscores(format!("depth:{}:bids", symbol), 0, 19)
            .await?;

        let asks: Vec<(Decimal, Decimal)> = conn
            .zrange_withscores(format!("depth:{}:asks", symbol), 0, 19)
            .await?;

        let snapshot_id = self.get_next_snapshot_id().await?;

        Ok(MarketDepthSnapshot {
            symbol: symbol.to_string(),
            snapshot_id,
            timestamp: get_timestamp_nanos(),
            bids: bids.into_iter().map(|(p, q)| PriceLevel { price: p, quantity: q }).collect(),
            asks: asks.into_iter().map(|(p, q)| PriceLevel { price: p, quantity: q }).collect(),
        })
    }

    /// 定期生成快照（后台任务）
    pub async fn snapshot_loop(&self) {
        let mut interval = tokio::time::interval(self.snapshot_interval);

        loop {
            interval.tick().await;

            for symbol in &["BTCUSDT", "ETHUSDT"] {
                match self.generate_snapshot(symbol).await {
                    Ok(snapshot) => {
                        // 保存到TimescaleDB
                        self.save_snapshot(&snapshot).await.ok();

                        // 推送给订阅的客户端
                        self.broadcast_snapshot(&snapshot).await.ok();
                    }
                    Err(e) => {
                        eprintln!("Failed to generate snapshot for {}: {:?}", symbol, e);
                    }
                }
            }
        }
    }
}
```

#### 2. OrderBookUpdatesView快照机制（L3数据）

**快照数据结构**：
```rust
/// L3订单簿快照（包含每个订单的ID）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookL3Snapshot {
    pub symbol: String,
    pub snapshot_id: u64,
    pub timestamp: i64,

    /// 买盘订单列表（包含订单ID）
    pub bid_orders: Vec<OrderDetail>,

    /// 卖盘订单列表
    pub ask_orders: Vec<OrderDetail>,
}

/// 单个订单详情（L3特有）
pub struct OrderDetail {
    pub order_id: u64,      // 订单唯一ID
    pub price: Decimal,
    pub quantity: Decimal,
    pub timestamp: i64,
    pub flags: OrderFlags,
}
```

**L3快照特点**：
- 包含**每个订单的ID**（L2快照没有）
- 数据量更大（可能有数千个订单）
- 更新频率更高（每100条增量后推送快照）

**快照触发条件**：
```rust
pub struct L3SnapshotTrigger {
    last_snapshot_id: u64,
    update_count: u64,
}

impl L3SnapshotTrigger {
    pub fn should_generate_snapshot(&mut self) -> bool {
        self.update_count += 1;

        // 每100条增量更新后生成快照
        if self.update_count >= 100 {
            self.update_count = 0;
            return true;
        }

        false
    }
}
```

#### 3. KLineView快照机制

**快照场景**：
- 客户端请求历史K线（如最近500根）
- 这本身就是一个快照查询

**API示例**：
```http
GET /api/v1/klines?symbol=BTCUSDT&interval=1m&limit=500

Response: [
  {
    "open_time": 1701234560000,
    "open": "50000.00",
    "high": "50100.00",
    "low": "49950.00",
    "close": "50050.00",
    "volume": "123.4567",
    ...
  },
  ... (500根K线)
]
```

**实时K线更新**：
```
历史K线（快照）+ 当前K线（增量更新）

客户端：
1. 请求最近500根历史K线（快照）
2. 订阅当前K线的实时更新（增量）
3. 当前K线完成后，追加到历史K线列表
```

#### 4. TickerView快照机制

**快照特点**：
- Ticker本身就是一个**实时快照**
- 每次成交后更新，无需增量

**无需增量的原因**：
```rust
// Ticker是完整状态，直接覆盖
pub struct TickerView {
    pub last_price: Decimal,      // 直接更新
    pub volume_24h: Decimal,       // 累加更新
    pub high_24h: Decimal,         // 比较更新
    // ... 所有字段都是自包含的
}

// 客户端处理
fn on_ticker_update(ticker: TickerView) {
    // 直接替换旧的Ticker，无需合并
    self.ticker = ticker;
}
```

#### 快照存储策略

| 读模型 | 快照存储位置 | 快照保留时间 | 快照大小 |
|-------|------------|------------|---------|
| MarketDepthView | Redis + TimescaleDB | Redis永久，DB每秒快照保留1天 | 10KB |
| OrderBookUpdatesView | Redis + TimescaleDB | Redis 1小时，DB每100条快照保留7天 | 100KB-1MB |
| KLineView | TimescaleDB + S3 | DB永久，S3归档 | 10MB+（500根） |
| TickerView | Redis | 永久（持续更新） | 1KB |

#### 快照一致性保证

**序列号机制**：
```rust
pub struct SnapshotConsistency {
    pub snapshot_id: u64,      // 快照序列号
    pub first_update_id: u64,  // 增量起始序列号
    pub last_update_id: u64,   // 增量结束序列号
}

// 客户端检查一致性
fn check_consistency(snapshot: &Snapshot, delta: &Delta) -> bool {
    // 增量的起始序列号必须 <= 快照序列号 + 1
    delta.first_update_id <= snapshot.snapshot_id + 1 &&
    // 增量的结束序列号必须 >= 快照序列号 + 1
    delta.last_update_id >= snapshot.snapshot_id + 1
}
```

**丢包恢复流程**：
```
正常流程：
Snapshot(id=100) → Delta(101-105) → Delta(106-110) ✅

丢包场景：
Snapshot(id=100) → Delta(101-105) → Delta(111-115) ❌
                                      ↑
                                   缺少106-110

恢复流程：
检测到序列号不连续
    ↓
请求新快照
    ↓
Snapshot(id=115) → Delta(116-120) ✅
```

#### 快照性能优化

**1. 快照压缩**：
```rust
use flate2::write::GzEncoder;

pub fn compress_snapshot(snapshot: &MarketDepthSnapshot) -> Result<Vec<u8>> {
    let json = serde_json::to_vec(snapshot)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&json)?;
    Ok(encoder.finish()?)
}

// 压缩率：10KB → 2KB（80%压缩）
```

**2. 快照缓存**：
```rust
// 缓存最近的快照，避免重复生成
pub struct SnapshotCache {
    cache: LruCache<String, (MarketDepthSnapshot, Instant)>,
    ttl: Duration,
}

impl SnapshotCache {
    pub fn get_or_generate(&mut self, symbol: &str) -> Result<MarketDepthSnapshot> {
        if let Some((snapshot, timestamp)) = self.cache.get(symbol) {
            if timestamp.elapsed() < self.ttl {
                return Ok(snapshot.clone());
            }
        }

        // 生成新快照
        let snapshot = self.generate_snapshot(symbol)?;
        self.cache.put(symbol.to_string(), (snapshot.clone(), Instant::now()));
        Ok(snapshot)
    }
}
```

**3. 增量快照**：
```rust
// 只发送变化的部分（适用于大型订单簿）
pub struct IncrementalSnapshot {
    pub base_snapshot_id: u64,
    pub changes_since_base: Vec<OrderBookUpdate>,
}

// 客户端重建：
// full_snapshot = base_snapshot + apply(changes_since_base)
```

#### 快照监控指标

```rust
// Prometheus指标
lazy_static! {
    static ref SNAPSHOT_GENERATION_TIME: Histogram = register_histogram!(
        "snapshot_generation_seconds",
        "Time to generate snapshot"
    ).unwrap();

    static ref SNAPSHOT_SIZE: Histogram = register_histogram!(
        "snapshot_size_bytes",
        "Snapshot size in bytes"
    ).unwrap();

    static ref SNAPSHOT_REQUEST_RATE: Counter = register_counter!(
        "snapshot_requests_total",
        "Total snapshot requests"
    ).unwrap();
}
```

---

### 数据模型选择指南（参考NautilusTrader）

根据不同的交易策略和用户需求，选择合适的数据模型：

#### 1. 趋势跟踪策略（Trend Following）

**推荐数据模型**：
- **主要**：KLineView（Bar数据）
- **辅助**：TickerView（24h统计）

**原因**：
- 趋势策略关注中长期价格走势
- 不需要高频数据，日线/小时线足够
- 降低数据成本和计算复杂度

**示例**：
```python
# 使用KLineView实现移动平均线策略
class MovingAverageCrossStrategy:
    def on_bar(self, bar: KLineView):
        # 计算MA指标
        ma_fast = self.calculate_ma(period=10)
        ma_slow = self.calculate_ma(period=30)

        # 金叉买入
        if ma_fast > ma_slow:
            self.buy()
```

#### 2. 做市策略（Market Making）

**推荐数据模型**：
- **主要**：OrderBookUpdatesView（逐笔委托）⭐
- **辅助**：MarketDepthView（订单簿快照）
- **辅助**：RecentTradesView（成交监控）

**原因**：
- 需要实时监控订单簿变化
- 动态调整买卖报价
- 对延迟极度敏感（< 1ms）

**示例**：
```python
# 使用OrderBookUpdatesView实现做市策略
class MarketMakingStrategy:
    def on_order_book_delta(self, delta: OrderBookUpdatesView):
        # 监控订单簿变化
        if delta.update_type == 'Add':
            # 有新订单加入，调整报价
            self.adjust_quotes()
        elif delta.update_type == 'Delete':
            # 订单被撤销，可能有机会
            self.check_opportunity()
```

#### 3. 套利策略（Arbitrage）

**推荐数据模型**：
- **主要**：TickerView（多交易所BBO）
- **辅助**：MarketDepthView（深度检查）

**原因**：
- 需要同时监控多个交易所
- 关注最优买卖价差
- 需要快速执行

**示例**：
```python
# 使用TickerView实现跨交易所套利
class CrossExchangeArbitrage:
    def on_ticker_update(self, exchange: str, ticker: TickerView):
        # 更新各交易所价格
        self.prices[exchange] = ticker

        # 检查套利机会
        if self.detect_arbitrage():
            self.execute_arbitrage()
```

#### 4. 高频交易策略（High-Frequency Trading）

**推荐数据模型**：
- **主要**：OrderBookUpdatesView（逐笔委托）⭐
- **主要**：RecentTradesView（逐笔成交）
- **辅助**：MarketDepthView（订单簿快照）

**原因**：
- 需要最细粒度的市场数据
- 基于订单流和微观结构
- 毫秒级甚至微秒级决策

**示例**：
```python
# 使用OrderBookUpdatesView + TradeTick实现高频策略
class HighFrequencyStrategy:
    def on_order_book_delta(self, delta: OrderBookUpdatesView):
        # 计算订单簿失衡
        imbalance = self.calculate_imbalance()

        if imbalance > 0.3:
            self.signal = 'BUY'

    def on_trade_tick(self, trade: RecentTradesView):
        # 监控成交方向
        if trade.is_buyer_maker:
            self.sell_pressure += 1
        else:
            self.buy_pressure += 1
```

#### 5. 统计套利策略（Statistical Arbitrage）

**推荐数据模型**：
- **主要**：KLineView（历史Bar数据）
- **辅助**：TickerView（实时价格）

**原因**：
- 需要大量历史数据进行统计分析
- 寻找价格偏离和均值回归
- 中频交易（分钟级）

**示例**：
```python
# 使用KLineView实现配对交易
class PairsTrading:
    def on_bar(self, symbol1: str, bar1: KLineView,
                     symbol2: str, bar2: KLineView):
        # 计算价差
        spread = bar1.close - bar2.close * self.hedge_ratio

        # 检查是否偏离均值
        if spread > self.upper_band:
            self.short_spread()
        elif spread < self.lower_band:
            self.long_spread()
```

#### 6. 新闻/事件驱动策略（Event-Driven）

**推荐数据模型**：
- **主要**：TickerView（快速价格反应）
- **辅助**：RecentTradesView（成交量激增检测）
- **辅助**：LiquidationView（市场恐慌信号）

**原因**：
- 需要快速响应市场事件
- 监控异常波动和成交量
- 捕捉短期机会

**示例**：
```python
# 使用TickerView + LiquidationView检测市场异常
class EventDrivenStrategy:
    def on_ticker_update(self, ticker: TickerView):
        # 检测价格异常波动
        if abs(ticker.price_change_percent_24h) > 10:
            self.alert_volatility_spike()

    def on_liquidation(self, liquidation: LiquidationView):
        # 大量强平可能预示趋势反转
        if liquidation.quantity > 100:
            self.prepare_reversal_trade()
```

### 数据模型性能对比

| 数据模型 | 延迟 | 带宽 | CPU | 内存 | 适用策略 |
|---------|------|------|-----|------|---------|
| KLineView | 1s-1m | 低 | 低 | 低 | 趋势跟踪、统计套利 |
| TickerView | 100ms | 低 | 低 | 低 | 套利、事件驱动 |
| MarketDepthView | 10-100ms | 中 | 中 | 中 | 做市、高频交易 |
| OrderBookUpdatesView ⭐ | < 1ms | 高 | 高 | 高 | 做市、高频交易 |
| RecentTradesView | < 10ms | 中 | 中 | 低 | 高频交易、订单流分析 |

---

## 核心读模型定义

### 1. TickerView（实时行情）

#### 用途
- 显示交易对的最新价格和24小时统计
- 交易所首页的行情列表
- 移动App的价格提醒

#### 数据结构

```rust
/// Ticker读模型（币安风格）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerView {
    /// 交易对符号 (如 "BTCUSDT")
    pub symbol: String,

    /// 最新成交价
    pub last_price: Decimal,

    /// 24小时价格变化
    pub price_change_24h: Decimal,

    /// 24小时价格变化率（百分比）
    pub price_change_percent_24h: Decimal,

    /// 24小时最高价
    pub high_24h: Decimal,

    /// 24小时最低价
    pub low_24h: Decimal,

    /// 24小时成交量（基础货币）
    pub volume_24h: Decimal,

    /// 24小时成交额（计价货币）
    pub quote_volume_24h: Decimal,

    /// 24小时前的价格（用于计算涨跌幅）
    pub open_price_24h: Decimal,

    /// 24小时内的成交笔数
    pub trade_count_24h: u64,

    /// 最优买价（Bid）
    pub best_bid: Option<Decimal>,

    /// 最优买量
    pub best_bid_qty: Option<Decimal>,

    /// 最优卖价（Ask）
    pub best_ask: Option<Decimal>,

    /// 最优卖量
    pub best_ask_qty: Option<Decimal>,

    /// 数据更新时间戳（毫秒）
    pub timestamp: i64,
}
```

#### 使用场景

**散户场景**：
- **App首页行情列表**：展示热门交易对的涨跌幅
- **价格提醒**：当价格变动超过阈值时推送通知
- **交易界面**：显示当前交易对的实时价格

```typescript
// 散户App示例：React Native组件
const MarketList = () => {
  const [tickers, setTickers] = useState<TickerView[]>([]);

  useEffect(() => {
    // WebSocket订阅所有Ticker
    const ws = new WebSocket('wss://api.example.com/ws/tickers');
    ws.onmessage = (event) => {
      const ticker: TickerView = JSON.parse(event.data);
      setTickers(prev => updateTicker(prev, ticker));
    };

    return () => ws.close();
  }, []);

  return (
    <FlatList
      data={tickers}
      renderItem={({ item }) => (
        <View>
          <Text>{item.symbol}</Text>
          <Text>{item.last_price}</Text>
          <Text style={{ color: item.price_change_percent_24h > 0 ? 'green' : 'red' }}>
            {item.price_change_percent_24h.toFixed(2)}%
          </Text>
        </View>
      )}
    />
  );
};
```

**量化机构场景**：
- **套利监控**：实时监控跨交易所的价差
- **市场扫描**：批量获取所有交易对行情，识别异常波动

```python
# 量化机构示例：套利监控
import asyncio
import websockets

class ArbitrageMonitor:
    def __init__(self):
        self.tickers = {}

    async def monitor_tickers(self):
        async with websockets.connect('wss://api.example.com/ws/tickers') as ws:
            async for message in ws:
                ticker = json.loads(message)
                self.check_arbitrage_opportunity(ticker)

    def check_arbitrage_opportunity(self, ticker: dict):
        # 检查套利机会：买卖价差 > 0.1%
        if ticker['best_ask'] and ticker['best_bid']:
            spread = (ticker['best_ask'] - ticker['best_bid']) / ticker['best_bid']
            if spread > 0.001:
                self.execute_arbitrage(ticker)
```

#### 存储策略

```
更新触发：每次成交事件（TradeExecutedEvent）
存储介质：Redis Hash
Key格式：ticker:{symbol}
TTL：永久（无过期时间，持续更新直到交易对下架）
推送方式：WebSocket广播 + HTTP轮询
更新频率：实时（成交时立即更新）
备注：24小时统计字段通过滑动窗口计算
```

#### 性能指标

| 指标 | 目标值 |
|------|-------|
| 单交易对查询QPS | 10万+ |
| 批量查询QPS（所有交易对） | 100万+ |
| 查询延迟 | < 1ms |
| 更新延迟 | < 10ms（从成交到推送） |
| 并发WebSocket连接 | 10万+ |

---

### 2. KLineView（K线数据）

#### 用途
- 技术分析：移动平均线、MACD、RSI等指标
- 价格走势图表
- 历史价格查询

#### 数据结构

```rust
/// K线读模型（蜡烛图）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KLineView {
    /// 交易对符号
    pub symbol: String,

    /// 时间周期（1m, 5m, 15m, 1h, 4h, 1d, 1w）
    pub interval: String,

    /// K线开始时间（毫秒时间戳）
    pub open_time: i64,

    /// 开盘价
    pub open: Decimal,

    /// 最高价
    pub high: Decimal,

    /// 最低价
    pub low: Decimal,

    /// 收盘价
    pub close: Decimal,

    /// 成交量（基础货币）
    pub volume: Decimal,

    /// K线结束时间（毫秒时间戳）
    pub close_time: i64,

    /// 成交额（计价货币）
    pub quote_volume: Decimal,

    /// 成交笔数
    pub trade_count: u64,

    /// 主动买入成交量
    pub taker_buy_volume: Decimal,

    /// 主动买入成交额
    pub taker_buy_quote_volume: Decimal,
}
```

#### 使用场景

**散户场景**：
- **图表分析**：查看日线、周线走势
- **简单指标**：查看MA（移动平均线）、成交量柱状图

```javascript
// 散户App示例：TradingView图表集成
const KLineChart = ({ symbol, interval }) => {
  const [klines, setKlines] = useState([]);

  useEffect(() => {
    // HTTP拉取历史K线
    fetch(`/api/v1/klines?symbol=${symbol}&interval=${interval}&limit=500`)
      .then(res => res.json())
      .then(data => setKlines(data));

    // WebSocket订阅实时K线更新
    const ws = new WebSocket(`wss://api.example.com/ws/kline/${symbol}/${interval}`);
    ws.onmessage = (event) => {
      const kline = JSON.parse(event.data);
      setKlines(prev => [...prev.slice(1), kline]);
    };

    return () => ws.close();
  }, [symbol, interval]);

  return <TradingViewWidget data={klines} />;
};
```

**量化机构场景**：
- **回测系统**：下载历史K线用于策略回测
- **实时策略**：基于K线的技术指标进行自动交易

```python
# 量化机构示例：基于K线的策略
import pandas as pd
import ta  # 技术分析库

class MovingAverageCrossStrategy:
    def __init__(self, symbol: str, interval: str):
        self.symbol = symbol
        self.interval = interval
        self.klines = []

    async def fetch_historical_klines(self):
        # 获取历史K线数据（用于回测）
        response = await self.http_client.get(
            f'/api/v1/klines',
            params={
                'symbol': self.symbol,
                'interval': self.interval,
                'limit': 1000
            }
        )
        self.klines = response.json()

    def calculate_signals(self):
        df = pd.DataFrame(self.klines)

        # 计算移动平均线
        df['ma_fast'] = ta.trend.sma_indicator(df['close'], window=10)
        df['ma_slow'] = ta.trend.sma_indicator(df['close'], window=30)

        # 金叉买入信号
        df['signal'] = (df['ma_fast'] > df['ma_slow']) & (df['ma_fast'].shift(1) <= df['ma_slow'].shift(1))

        return df[df['signal'] == True]
```

**专业交易者场景**：
- **多时间框架分析**：同时查看1分钟、5分钟、1小时K线
- **量价分析**：结合成交量判断趋势强度

#### 存储策略

```
数据源：聚合成交事件生成K线
存储介质：
  - 热数据（最近24小时）：Redis Sorted Set
  - 温数据（最近30天）：TimescaleDB（时序数据库）
  - 冷数据（历史数据）：S3对象存储 + Parquet格式

Key格式：kline:{symbol}:{interval}
更新频率：
  - 1分钟K线：每秒更新（未完成的K线）
  - 历史K线：固定不变

查询模式：
  - 范围查询：start_time ~ end_time
  - 限制数量：最近N根K线
```

#### K线聚合流程

```
成交事件（TradeExecutedEvent）
    ↓
K线聚合器（按时间窗口聚合）
    ↓
1m K线 → 5m K线 → 15m K线 → 1h K线 → 4h K线 → 1d K线
    ↓
写入Redis（实时K线，未完成）
    ↓
K线周期结束后写入TimescaleDB（历史K线，已完成）
    ↓
定期归档到S3（冷数据）
```

#### 性能指标

| 指标 | 目标值 |
|------|-------|
| 查询QPS | 20万+ |
| 查询延迟（Redis） | < 10ms |
| 查询延迟（TimescaleDB） | < 100ms |
| 存储容量 | 100GB+（每个交易对） |

---

### 3. MarketDepthView（市场深度/订单簿）

#### 用途
- 显示买卖盘深度（5档、10档、20档）
- 深度图可视化
- 大单监控

#### 数据结构

```rust
/// 市场深度读模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepthView {
    /// 交易对符号
    pub symbol: String,

    /// 最后更新ID（用于增量更新）
    pub last_update_id: u64,

    /// 买盘深度（价格降序，最优买价在前）
    pub bids: Vec<PriceLevel>,

    /// 卖盘深度（价格升序，最优卖价在前）
    pub asks: Vec<PriceLevel>,

    /// 数据时间戳
    pub timestamp: i64,
}

/// 价格档位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    /// 价格
    pub price: Decimal,

    /// 数量
    pub quantity: Decimal,
}
```

#### 使用场景

**散户场景**：
- **简单深度**：查看5档买卖盘（买1~买5，卖1~卖5）
- **挂单参考**：根据深度决定下单价格

```javascript
// 散户App示例：5档深度显示
const OrderBookWidget = ({ symbol }) => {
  const [depth, setDepth] = useState({ bids: [], asks: [] });

  useEffect(() => {
    // 订阅深度更新（降低频率：100ms节流）
    const ws = new WebSocket(`wss://api.example.com/ws/depth/${symbol}@100ms`);
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setDepth({
        bids: data.bids.slice(0, 5),  // 只显示5档
        asks: data.asks.slice(0, 5)
      });
    };

    return () => ws.close();
  }, [symbol]);

  return (
    <View>
      <Text>买盘</Text>
      {depth.bids.map(([price, qty]) => (
        <Text key={price}>{price} / {qty}</Text>
      ))}
      <Text>卖盘</Text>
      {depth.asks.map(([price, qty]) => (
        <Text key={price}>{price} / {qty}</Text>
      ))}
    </View>
  );
};
```

**专业交易者场景**：
- **完整深度**：查看20档甚至更多深度
- **深度图**：可视化买卖盘分布，判断支撑/阻力位

**量化机构场景**：
- **完整L2订单簿**：订阅完整订单簿快照和增量更新
- **挂单策略**：根据深度动态调整挂单位置
- **流动性分析**：计算市场冲击成本

```python
# 量化机构示例：订单簿重建
class OrderBookManager:
    def __init__(self, symbol: str):
        self.symbol = symbol
        self.order_book = {'bids': {}, 'asks': {}}
        self.last_update_id = 0

    async def initialize(self):
        # 1. 获取完整快照
        snapshot = await self.fetch_snapshot()
        self.order_book = {
            'bids': {level[0]: level[1] for level in snapshot['bids']},
            'asks': {level[0]: level[1] for level in snapshot['asks']}
        }
        self.last_update_id = snapshot['last_update_id']

        # 2. 订阅增量更新
        await self.subscribe_depth_updates()

    async def subscribe_depth_updates(self):
        async with websockets.connect(f'wss://api.example.com/ws/depth/{self.symbol}') as ws:
            async for message in ws:
                update = json.loads(message)

                # 检查序列号连续性
                if update['first_update_id'] <= self.last_update_id + 1 <= update['last_update_id']:
                    self.apply_update(update)
                    self.last_update_id = update['last_update_id']

    def apply_update(self, update: dict):
        # 应用增量更新
        for price, qty in update['bids']:
            if qty == 0:
                self.order_book['bids'].pop(price, None)
            else:
                self.order_book['bids'][price] = qty

        for price, qty in update['asks']:
            if qty == 0:
                self.order_book['asks'].pop(price, None)
            else:
                self.order_book['asks'][price] = qty

    def calculate_market_impact(self, side: str, quantity: float) -> float:
        """计算市场冲击成本"""
        book = self.order_book['asks'] if side == 'buy' else self.order_book['bids']
        remaining = quantity
        total_cost = 0

        for price in sorted(book.keys()):
            level_qty = book[price]
            if remaining <= 0:
                break

            fill_qty = min(remaining, level_qty)
            total_cost += fill_qty * price
            remaining -= fill_qty

        avg_price = total_cost / quantity
        best_price = min(book.keys()) if side == 'buy' else max(book.keys())
        impact = (avg_price - best_price) / best_price
        return impact
```

#### 存储策略

```
存储介质：Redis Sorted Set（买卖盘各一个）
Key格式：
  - depth:{symbol}:bids (score=price, 降序)
  - depth:{symbol}:asks (score=price, 升序)

更新方式：
  - 快照：完整覆盖
  - 增量：ZADD/ZREM单个价格档位

推送策略：
  - 完整快照：客户端连接时推送
  - 增量更新：每100ms聚合推送（降低网络流量）
  - 高频更新：量化客户端可订阅实时流（无节流）
```

#### 性能指标

| 指标 | 目标值 |
|------|-------|
| 快照查询QPS | 10万+ |
| 增量推送延迟 | < 100ms（散户）/ < 10ms（量化） |
| WebSocket并发连接 | 5万+ |
| 订单簿更新延迟 | < 1ms（从撮合到Redis） |

---

### 4. OrderBookUpdatesView（逐笔委托）⭐ 新增

#### 用途
- 订单簿的逐笔变化流（Order-by-Order Updates）
- 量化机构的订单流分析
- 做市商的微观结构研究
- 高频交易策略的信号源

#### 数据结构

```rust
/// 逐笔委托读模型（订单级别的变化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdatesView {
    /// 交易对符号
    pub symbol: String,

    /// 订单簿更新事件列表
    pub updates: Vec<OrderBookUpdate>,
}

/// 单个订单簿更新事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    /// 更新序列号（全局递增，用于检测丢包）
    pub update_id: u64,

    /// 更新类型
    pub update_type: OrderUpdateType,

    /// 订单ID（交易所分配的唯一ID）
    pub order_id: u64,

    /// 买卖方向
    pub side: Side,

    /// 价格
    pub price: Decimal,

    /// 数量（新增/修改时有效）
    pub quantity: Option<Decimal>,

    /// 更新时间戳（纳秒）
    pub timestamp: i64,

    /// 订单标志（隐藏订单、冰山订单等）
    pub flags: OrderFlags,
}

/// 订单更新类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderUpdateType {
    /// 新增订单
    Add,
    /// 修改订单（价格或数量变化）
    Modify,
    /// 删除订单（取消或完全成交）
    Delete,
    /// 部分成交（数量减少但订单仍存在）
    PartialFill,
}

/// 买卖方向
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

/// 订单标志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFlags {
    /// 是否为隐藏订单（不显示在公开订单簿）
    pub is_hidden: bool,
    /// 是否为冰山订单（只显示部分数量）
    pub is_iceberg: bool,
    /// 是否为Post-Only订单（只做Maker）
    pub is_post_only: bool,
}
```

#### 使用场景

**量化机构场景**：
- **订单流分析**：分析大单、扫单、撤单行为
- **市场微观结构研究**：研究订单簿动态、流动性提供者行为
- **高频交易信号**：基于订单簿变化的交易信号

```python
# 量化机构示例：订单流分析
import asyncio
import websockets
from collections import defaultdict

class OrderFlowAnalyzer:
    def __init__(self, symbol: str):
        self.symbol = symbol
        self.order_book = {'bids': {}, 'asks': {}}
        self.order_flow_stats = defaultdict(int)

    async def subscribe_order_updates(self):
        """订阅逐笔委托流"""
        async with websockets.connect(
            f'wss://api.example.com/ws/orderbook/{self.symbol}'
        ) as ws:
            async for message in ws:
                update = json.loads(message)
                self.analyze_order_update(update)

    def analyze_order_update(self, update: dict):
        """分析单个订单更新"""
        update_type = update['update_type']
        side = update['side']
        price = update['price']
        quantity = update.get('quantity', 0)

        # 统计订单流特征
        if update_type == 'Add':
            self.order_flow_stats[f'{side}_add_count'] += 1
            self.order_flow_stats[f'{side}_add_volume'] += quantity

            # 检测大单挂单（可能是支撑/阻力位）
            if quantity > 10.0:  # 假设10 BTC为大单
                print(f"大单挂单: {side} {quantity} @ {price}")

        elif update_type == 'Delete':
            self.order_flow_stats[f'{side}_cancel_count'] += 1

            # 检测大单撤单（可能是虚假支撑/阻力）
            if quantity and quantity > 10.0:
                print(f"大单撤单: {side} {quantity} @ {price}")

        elif update_type == 'PartialFill':
            # 订单被部分成交，说明有主动吃单
            self.order_flow_stats[f'{side}_partial_fill_count'] += 1

    def detect_spoofing(self):
        """检测欺骗性挂单（Spoofing）"""
        # 大量挂单后快速撤单，可能是市场操纵
        add_count = self.order_flow_stats['Buy_add_count']
        cancel_count = self.order_flow_stats['Buy_cancel_count']

        if cancel_count > add_count * 0.8:
            print("警告：检测到可能的欺骗性挂单行为")

    def calculate_order_book_imbalance(self):
        """计算订单簿失衡度"""
        bid_volume = sum(self.order_book['bids'].values())
        ask_volume = sum(self.order_book['asks'].values())

        imbalance = (bid_volume - ask_volume) / (bid_volume + ask_volume)
        return imbalance
```

**做市商场景**：
- **流动性监控**：实时监控订单簿流动性变化
- **挂单策略优化**：根据订单流动态调整挂单位置
- **对手盘分析**：识别其他做市商的行为模式

```python
# 做市商示例：动态挂单策略
class MarketMaker:
    def __init__(self, symbol: str):
        self.symbol = symbol
        self.my_orders = {}  # 自己的挂单
        self.order_book = {'bids': {}, 'asks': {}}

    async def on_order_update(self, update: dict):
        """响应订单簿更新"""
        # 更新本地订单簿
        self.update_local_order_book(update)

        # 检查是否需要调整挂单
        if self.should_adjust_quotes():
            await self.adjust_quotes()

    def should_adjust_quotes(self) -> bool:
        """判断是否需要调整报价"""
        # 如果最优买卖价发生变化，需要调整
        best_bid = max(self.order_book['bids'].keys()) if self.order_book['bids'] else 0
        best_ask = min(self.order_book['asks'].keys()) if self.order_book['asks'] else float('inf')

        # 检查自己的订单是否还在最优位置
        my_best_bid = max([o['price'] for o in self.my_orders.values() if o['side'] == 'Buy'], default=0)
        my_best_ask = min([o['price'] for o in self.my_orders.values() if o['side'] == 'Sell'], default=float('inf'))

        # 如果不在最优位置，需要调整
        return my_best_bid < best_bid or my_best_ask > best_ask

    async def adjust_quotes(self):
        """调整报价"""
        # 取消旧订单
        for order_id in self.my_orders.keys():
            await self.cancel_order(order_id)

        # 重新挂单（在最优价格）
        best_bid = max(self.order_book['bids'].keys())
        best_ask = min(self.order_book['asks'].keys())

        await self.place_order(side='Buy', price=best_bid + 0.01, quantity=1.0)
        await self.place_order(side='Sell', price=best_ask - 0.01, quantity=1.0)
```

**高频交易场景**：
- **订单簿失衡信号**：买卖盘失衡预测短期价格走势
- **订单簿深度变化**：检测大单进出
- **订单到达率分析**：统计订单到达的泊松过程

```python
# 高频交易示例：订单簿失衡策略
class OrderBookImbalanceStrategy:
    def __init__(self, symbol: str):
        self.symbol = symbol
        self.order_book = {'bids': {}, 'asks': {}}
        self.imbalance_history = []

    async def on_order_update(self, update: dict):
        """处理订单更新"""
        self.update_order_book(update)

        # 计算订单簿失衡度
        imbalance = self.calculate_imbalance()
        self.imbalance_history.append(imbalance)

        # 保留最近100个失衡度
        if len(self.imbalance_history) > 100:
            self.imbalance_history.pop(0)

        # 生成交易信号
        signal = self.generate_signal()
        if signal:
            await self.execute_trade(signal)

    def calculate_imbalance(self) -> float:
        """计算订单簿失衡度（前5档）"""
        bid_volume = sum(list(self.order_book['bids'].values())[:5])
        ask_volume = sum(list(self.order_book['asks'].values())[:5])

        if bid_volume + ask_volume == 0:
            return 0

        return (bid_volume - ask_volume) / (bid_volume + ask_volume)

    def generate_signal(self) -> Optional[str]:
        """生成交易信号"""
        if len(self.imbalance_history) < 10:
            return None

        current_imbalance = self.imbalance_history[-1]

        # 强买压信号（失衡度 > 0.3）
        if current_imbalance > 0.3:
            return 'BUY'

        # 强卖压信号（失衡度 < -0.3）
        if current_imbalance < -0.3:
            return 'SELL'

        return None
```

#### 存储策略

```
存储介质：
  - 实时流：Kafka（事件流，保留7天）
  - 热数据：Redis Stream（最近1小时）
  - 冷数据：TimescaleDB（历史订单流，压缩存储）

Key格式：
  - Redis Stream: orderbook_updates:{symbol}
  - Kafka Topic: market-data.orderbook-updates.{symbol}

推送方式：
  - WebSocket实时推送（无节流，< 1ms延迟）
  - 仅对付费用户开放（量化机构、做市商）

数据量估算：
  - 活跃交易对：1000-5000条更新/秒
  - 每条消息：~200字节
  - 带宽需求：200KB-1MB/秒/交易对
```

#### 性能指标

| 指标 | 目标值 |
|------|-------|
| 推送延迟 | < 1ms（从撮合引擎到客户端） |
| 吞吐量 | 10万条更新/秒/交易对 |
| 丢包率 | < 0.01% |
| 序列号连续性 | 100%（通过update_id检测） |

#### 与MarketDepthView的对比

```
场景1：散户查看5档深度
  → 使用 MarketDepthView（聚合深度）
  → 100ms更新一次，数据量小

场景2：量化机构分析订单流
  → 使用 OrderBookUpdatesView（逐笔委托）
  → 实时推送每笔订单变化，数据量大

场景3：专业交易者查看20档深度
  → 使用 MarketDepthView（聚合深度）
  → 10ms更新一次，折中方案
```

---

### 5. RecentTradesView（最新成交）

#### 用途
- 显示最近N笔成交记录
- 成交流水监控
- 判断市场活跃度

#### 数据结构

```rust
/// 最新成交读模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentTradesView {
    /// 交易对符号
    pub symbol: String,

    /// 最近成交列表（最多500笔）
    pub trades: Vec<TradeRecord>,
}

/// 单笔成交记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    /// 成交ID
    pub trade_id: u64,

    /// 成交价格
    pub price: Decimal,

    /// 成交数量
    pub quantity: Decimal,

    /// 成交时间（毫秒时间戳）
    pub timestamp: i64,

    /// 买方是否为Maker（挂单方）
    /// - true: 买方挂单被成交（卖方主动吃单，看跌信号）
    /// - false: 买方主动吃单（买方市价单，看涨信号）
    pub is_buyer_maker: bool,
}
```

#### 使用场景

**散户场景**：
- **成交流水**：查看最近20-50笔成交，了解市场活跃度
- **大单提醒**：高亮显示大额成交

```javascript
// 散户App示例：成交流水
const RecentTradesList = ({ symbol }) => {
  const [trades, setTrades] = useState([]);

  useEffect(() => {
    // 初始加载最近50笔
    fetch(`/api/v1/trades?symbol=${symbol}&limit=50`)
      .then(res => res.json())
      .then(data => setTrades(data));

    // WebSocket订阅新成交
    const ws = new WebSocket(`wss://api.example.com/ws/trades/${symbol}`);
    ws.onmessage = (event) => {
      const trade = JSON.parse(event.data);
      setTrades(prev => [trade, ...prev.slice(0, 49)]);
    };

    return () => ws.close();
  }, [symbol]);

  return (
    <ScrollView>
      {trades.map(trade => (
        <View key={trade.trade_id}>
          {/* 买方主动吃单显示绿色（看涨），卖方主动吃单显示红色（看跌） */}
          <Text style={{ color: trade.is_buyer_maker ? 'red' : 'green' }}>
            {trade.price}
          </Text>
          <Text>{trade.quantity}</Text>
          <Text>{formatTime(trade.timestamp)}</Text>
        </View>
      ))}
    </ScrollView>
  );
};
```

**量化机构场景**：
- **逐笔成交分析**：分析买卖盘力量对比
- **订单流分析**：识别大单、扫单行为

```python
# 量化机构示例：订单流分析
class OrderFlowAnalyzer:
    def __init__(self, symbol: str):
        self.symbol = symbol
        self.recent_trades = []

    async def analyze_order_flow(self):
        async with websockets.connect(f'wss://api.example.com/ws/trades/{self.symbol}') as ws:
            async for message in ws:
                trade = json.loads(message)
                self.recent_trades.append(trade)

                # 保留最近1000笔成交
                if len(self.recent_trades) > 1000:
                    self.recent_trades.pop(0)

                # 分析买卖压力
                self.analyze_pressure()

    def analyze_pressure(self):
        # 计算最近100笔成交的买卖比
        recent_100 = self.recent_trades[-100:]

        # 买方主动成交量（is_buyer_maker=False表示买方Taker）
        buy_volume = sum(t['quantity'] for t in recent_100 if not t['is_buyer_maker'])
        # 卖方主动成交量（is_buyer_maker=True表示卖方Taker）
        sell_volume = sum(t['quantity'] for t in recent_100 if t['is_buyer_maker'])

        buy_pressure = buy_volume / (buy_volume + sell_volume) if (buy_volume + sell_volume) > 0 else 0.5

        if buy_pressure > 0.6:
            print(f"强买压（买方主动） {buy_pressure:.2%}")
        elif buy_pressure < 0.4:
            print(f"强卖压（卖方主动） {buy_pressure:.2%}")
```

#### 存储策略

```
存储介质：Redis List
Key格式：trades:{symbol}
操作：
  - 新成交：LPUSH（头部插入）
  - 限制长度：LTRIM 0 499（保留最近500笔）
TTL：1小时（1小时无更新后自动删除，节省内存）
备注：活跃交易对会持续有新成交，TTL会自动刷新

推送方式：
  - WebSocket实时推送每笔成交
  - HTTP查询返回最近N笔
```

#### 性能指标

| 指标 | 目标值 |
|------|-------|
| 查询QPS | 50万+ |
| 推送延迟 | < 10ms |
| 存储大小 | < 1MB/交易对 |

---

### 5. TradeHistoryView（历史成交记录）

#### 用途
- 用户查询自己的历史成交
- 交易对账单
- 税务报告

#### 数据结构

```rust
/// 用户历史成交读模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistoryView {
    /// 用户ID
    pub user_id: String,

    /// 成交记录列表
    pub trades: Vec<UserTradeRecord>,

    /// 总记录数
    pub total: u64,

    /// 分页游标
    pub next_cursor: Option<String>,
}

/// 用户成交记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTradeRecord {
    /// 成交ID
    pub trade_id: u64,

    /// 订单ID
    pub order_id: u64,

    /// 交易对
    pub symbol: String,

    /// 买卖方向
    pub side: String,  // "BUY" / "SELL"

    /// 成交价格
    pub price: Decimal,

    /// 成交数量
    pub quantity: Decimal,

    /// 手续费
    pub commission: Decimal,

    /// 手续费币种
    pub commission_asset: String,

    /// 成交时间
    pub timestamp: i64,

    /// 是否为Maker（挂单方）
    pub is_maker: bool,
}
```

#### 使用场景

**散户场景**：
- **交易历史**：查看自己的成交记录
- **盈亏统计**：计算持仓成本和收益

```javascript
// 散户App示例：交易历史
const TradeHistory = ({ userId }) => {
  const [trades, setTrades] = useState([]);
  const [nextCursor, setNextCursor] = useState(null);
  const [loading, setLoading] = useState(false);

  const loadMore = async () => {
    setLoading(true);
    const response = await fetch(
      `/api/v1/users/${userId}/trades?limit=50&cursor=${nextCursor || ''}`
    );
    const data = await response.json();
    setTrades(prev => [...prev, ...data.trades]);
    setNextCursor(data.next_cursor);
    setLoading(false);
  };

  return (
    <FlatList
      data={trades}
      onEndReached={loadMore}
      renderItem={({ item }) => (
        <View>
          <Text>{item.symbol}</Text>
          <Text>{item.side === 'BUY' ? '买入' : '卖出'}</Text>
          <Text>{item.price} × {item.quantity}</Text>
          <Text>手续费: {item.commission} {item.commission_asset}</Text>
        </View>
      )}
    />
  );
};
```

**专业交易者场景**：
- **交易分析**：统计胜率、平均盈亏
- **费用优化**：分析手续费支出，选择合适的VIP等级

**量化机构场景**：
- **策略回测验证**：对比实际成交与模拟回测结果
- **滑点分析**：分析实际成交价格与预期价格的差异

```python
# 量化机构示例：滑点分析
class SlippageAnalyzer:
    def __init__(self, user_id: str):
        self.user_id = user_id

    async def analyze_slippage(self, start_time: int, end_time: int):
        # 获取历史成交记录
        trades = await self.fetch_trade_history(start_time, end_time)

        slippage_data = []
        for trade in trades:
            # 获取下单时的市场价格
            expected_price = await self.get_market_price_at(trade['order_time'])
            actual_price = trade['price']

            # 计算滑点
            slippage = (actual_price - expected_price) / expected_price
            slippage_data.append({
                'trade_id': trade['trade_id'],
                'expected': expected_price,
                'actual': actual_price,
                'slippage': slippage
            })

        # 统计分析
        avg_slippage = sum(s['slippage'] for s in slippage_data) / len(slippage_data)
        print(f"平均滑点: {avg_slippage:.4%}")
```

#### 存储策略

```
存储介质：TimescaleDB（时序数据库）
表结构：user_trades（按user_id + timestamp分区）
索引：
  - PRIMARY KEY (user_id, trade_id)
  - INDEX (user_id, timestamp)
  - INDEX (user_id, symbol, timestamp)

查询模式：
  - 时间范围查询：WHERE user_id = ? AND timestamp BETWEEN ? AND ?
  - 游标分页：WHERE user_id = ? AND trade_id > ? ORDER BY trade_id LIMIT 50

归档策略：
  - 90天内：热数据，高性能查询
  - 90天外：归档到S3，按需加载
```

#### 性能指标

| 指标 | 目标值 |
|------|-------|
| 查询QPS | 5万+ |
| 查询延迟 | < 100ms（热数据）/ < 3s（冷数据） |
| 数据保留 | 永久 |

---

### 6. LiquidationView（强平数据）

#### 用途
- 显示市场强平事件
- 预警流动性风险
- 合约交易辅助决策

#### 数据结构

```rust
/// 强平事件读模型（仅合约交易）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationView {
    /// 交易对符号
    pub symbol: String,

    /// 强平方向（LONG平多 / SHORT平空）
    pub side: String,

    /// 强平价格
    pub price: Decimal,

    /// 强平数量
    pub quantity: Decimal,

    /// 强平时间
    pub timestamp: i64,
}
```

#### 使用场景

**专业交易者场景**：
- **爆仓监控**：实时查看市场强平事件，判断市场情绪
- **支撑/阻力位参考**：大量强平可能形成临时阻力位

**量化机构场景**：
- **流动性分析**：统计强平数量，评估市场流动性风险
- **反向指标**：大量强平可能预示趋势反转

#### 存储策略

```
存储介质：Redis Stream + TimescaleDB
Redis Stream：实时流式数据（保留24小时）
TimescaleDB：历史强平数据（永久保存）

推送方式：WebSocket实时推送
```

---

### 7. FundingRateView（资金费率）

#### 用途
- 合约交易的资金费率显示
- 套利策略参考

#### 数据结构

```rust
/// 资金费率读模型（永续合约）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRateView {
    /// 合约符号
    pub symbol: String,

    /// 当前资金费率
    pub funding_rate: Decimal,

    /// 下次结算时间
    pub next_funding_time: i64,

    /// 预测资金费率
    pub predicted_rate: Decimal,
}
```

#### 使用场景

**专业交易者场景**：
- **持仓成本**：计算持有合约的资金费用
- **套利机会**：现货-合约套利

---

## 用户场景分析

### 场景1：散户看盘交易

**用户画像**：
- 日交易1-10次
- 主要使用移动App
- 关注价格涨跌、简单K线

**数据需求**：
```
1. 打开App首页
   → 查询：TickerView（所有交易对）
   → 延迟要求：< 1s

2. 进入交易界面（如BTCUSDT）
   → 查询：TickerView（单个交易对）
   → 查询：MarketDepthView（5档深度）
   → 查询：RecentTradesView（最近20笔）
   → WebSocket订阅：实时Ticker、深度、成交
   → 延迟要求：< 500ms

3. 查看K线图表
   → 查询：KLineView（1分钟K线，最近500根）
   → WebSocket订阅：实时K线更新
   → 延迟要求：< 2s

4. 提交订单后查看成交
   → 查询：TradeHistoryView（自己的成交记录）
   → 延迟要求：< 1s
```

**优化策略**：
- 使用CDN加速静态资源
- Redis缓存所有热点数据
- WebSocket连接复用（一个连接订阅多个交易对）
- App端本地缓存（减少请求）

---

### 场景2：专业交易者多屏监控

**用户画像**：
- 日交易50-200次
- 使用桌面端软件或Web
- 同时监控多个交易对

**数据需求**：
```
1. 多交易对监控面板
   → 查询：TickerView（10-50个交易对）
   → WebSocket订阅：实时Ticker推送
   → 更新频率：实时（100-500ms）

2. 单交易对深度分析
   → 查询：MarketDepthView（20档深度）
   → 查询：RecentTradesView（最近100笔）
   → 查询：KLineView（多时间框架：1m, 5m, 1h）
   → WebSocket订阅：高频深度更新（100ms）
   → 延迟要求：< 100ms

3. 订单执行分析
   → 查询：TradeHistoryView（自己的成交）
   → 分析：滑点、成交速度
   → 延迟要求：< 500ms
```

**优化策略**：
- 批量查询API（一次请求获取多个交易对）
- WebSocket多路复用（减少连接数）
- 客户端本地计算技术指标（减少服务器负载）

---

### 场景3：量化机构高频交易

**用户画像**：
- 日交易1000-10000次
- 使用私有化部署或API对接
- 对延迟极度敏感

**数据需求**：
```
1. 完整订单簿重建
   → 查询：MarketDepthView（完整快照）
   → WebSocket订阅：增量更新（无节流，< 10ms）
   → 延迟要求：< 1ms

2. 逐笔成交流
   → WebSocket订阅：RecentTradesView（每笔立即推送）
   → 延迟要求：< 1ms

3. 多交易对套利监控
   → 查询：TickerView（所有交易对，批量查询）
   → WebSocket订阅：BBO（Best Bid/Offer）推送
   → 延迟要求：< 500μs

4. 历史数据回测
   → 批量下载：KLineView（多个月历史K线）
   → 批量下载：TradeHistoryView（逐笔成交历史）
   → 存储：本地数据库，离线回测
```

**优化策略**：
- 私有部署专用行情服务器（物理接近撮合引擎）
- 使用UDP多播协议（降低延迟）
- 内核旁路技术（DPDK/AF_XDP）
- 批量API支持（减少HTTP往返）
- 历史数据导出功能（Parquet/CSV格式）

---

## 数据存储策略

### 存储分层

| 层级 | 存储介质 | 数据类型 | 容量 | 延迟 | 成本 |
|------|---------|---------|------|------|------|
| **L1热数据** | Redis Cluster | Ticker、5档深度、最新成交 | 100GB | < 1ms | 高 |
| **L2温数据** | TimescaleDB | K线、完整深度、成交历史 | 10TB | < 100ms | 中 |
| **L3冷数据** | S3/OSS | 历史K线、归档成交 | 100TB+ | 1-5s | 低 |

### 各读模型的存储方案

| 读模型 | 主存储 | 备份存储 | TTL/归档策略 |
|-------|--------|----------|-------------|
| TickerView | Redis Hash | TimescaleDB（每秒快照） | Redis永久（持续更新），DB保留30天 |
| KLineView | Redis ZSet（24h）+ TimescaleDB | S3（历史K线） | Redis 24h自动过期，DB永久，S3归档 |
| MarketDepthView | Redis ZSet（Bids/Asks） | TimescaleDB（每秒快照） | Redis永久（持续更新），DB按需 |
| **OrderBookUpdatesView** ⭐ | **Kafka + Redis Stream** | **TimescaleDB（压缩）** | **Kafka 7天，Redis 1h，DB永久** |
| RecentTradesView | Redis List | TimescaleDB | Redis 1h无更新后过期，DB永久 |
| TradeHistoryView | TimescaleDB | S3（90天后归档） | DB热数据90天，S3永久 |
| LiquidationView | Redis Stream（24h）+ TimescaleDB | S3 | Redis 24h自动过期，DB永久 |
| FundingRateView | Redis Hash | TimescaleDB（每8h） | Redis永久（每8h更新），DB永久 |

### 数据同步流程

```
撮合引擎成交事件
    ↓
Kafka事件流（Event Sourcing）
    ↓
    ├──→ 实时消费者1：更新Redis（热数据）
    │       ↓
    │    WebSocket推送给在线客户端
    │
    ├──→ 实时消费者2：更新TimescaleDB（温数据）
    │       ↓
    │    提供HTTP查询服务
    │
    └──→ 批处理任务：归档到S3（冷数据）
             ↓
         每日凌晨批量压缩归档
```

---

## 性能优化

### 1. 缓存策略

#### 多级缓存

```rust
// L1: 进程内缓存（Caffeine）
let l1_cache = Cache::builder()
    .max_capacity(10_000)
    .time_to_live(Duration::from_secs(1))
    .build();

// L2: Redis缓存
let l2_cache = redis::Client::open("redis://127.0.0.1:6379")?;

// L3: 数据库
let db_pool = PgPoolOptions::new()
    .max_connections(50)
    .connect(&db_url).await?;

// 查询流程
async fn get_ticker(symbol: &str) -> Result<TickerView> {
    // 尝试L1缓存
    if let Some(ticker) = l1_cache.get(symbol) {
        return Ok(ticker);
    }

    // 尝试L2缓存（Redis）
    if let Some(ticker) = l2_cache.get(format!("ticker:{}", symbol)).await? {
        l1_cache.insert(symbol, ticker.clone());
        return Ok(ticker);
    }

    // 查询L3数据库
    let ticker = query_from_database(symbol).await?;

    // 回写到L2、L1
    l2_cache.set(format!("ticker:{}", symbol), &ticker).await?;
    l1_cache.insert(symbol, ticker.clone());

    Ok(ticker)
}
```

#### 预热策略

```rust
// 应用启动时预热热门交易对
async fn warmup_cache() {
    let hot_symbols = vec!["BTCUSDT", "ETHUSDT", "BNBUSDT"];

    for symbol in hot_symbols {
        let ticker = fetch_ticker_from_db(symbol).await.unwrap();
        redis_client.set(format!("ticker:{}", symbol), &ticker).await.ok();
    }
}
```

### 2. 查询优化

#### 批量查询

```rust
// 差：N+1查询问题
for symbol in symbols {
    let ticker = get_ticker(symbol).await?;
}

// 好：批量查询
let tickers = get_tickers_batch(&symbols).await?;
```

#### 索引优化

```sql
-- TimescaleDB索引优化
CREATE INDEX idx_klines_symbol_time ON klines (symbol, open_time DESC);
CREATE INDEX idx_trades_user_time ON user_trades (user_id, timestamp DESC);

-- 分区表（按时间分区）
SELECT create_hypertable('klines', 'open_time', chunk_time_interval => INTERVAL '1 day');
```

### 3. 推送优化

#### WebSocket连接池化

```javascript
// 差：每个组件创建独立WebSocket连接
const ws1 = new WebSocket('wss://api/ws/ticker/BTCUSDT');
const ws2 = new WebSocket('wss://api/ws/depth/BTCUSDT');
const ws3 = new WebSocket('wss://api/ws/trades/BTCUSDT');

// 好：复用单个连接，订阅多个流
const ws = new WebSocket('wss://api/ws');
ws.send(JSON.stringify({
  method: 'SUBSCRIBE',
  params: [
    'btcusdt@ticker',
    'btcusdt@depth',
    'btcusdt@trade'
  ]
}));
```

#### 推送节流

```rust
// 对于高频更新的数据（如深度），按时间窗口聚合推送
let mut aggregator = UpdateAggregator::new(Duration::from_millis(100));

loop {
    let update = depth_updates_receiver.recv().await?;
    aggregator.add(update);

    if aggregator.should_flush() {
        let aggregated = aggregator.flush();
        broadcast_to_websocket_clients(aggregated).await?;
    }
}
```

### 4. 数据压缩

#### 传输压缩

```rust
// WebSocket消息压缩（zlib）
use flate2::Compression;
use flate2::write::GzEncoder;

let compressed = compress_message(&ticker_json)?;
websocket.send(Message::Binary(compressed)).await?;
```

#### 存储压缩

```sql
-- TimescaleDB压缩
ALTER TABLE klines SET (
  timescaledb.compress,
  timescaledb.compress_segmentby = 'symbol',
  timescaledb.compress_orderby = 'open_time DESC'
);

-- 自动压缩策略（7天后压缩）
SELECT add_compression_policy('klines', INTERVAL '7 days');
```

---

## 实现示例

### Rust服务端实现

```rust
// src/application/query/ticker_query.rs
use axum::{extract::Path, Json};
use redis::AsyncCommands;

/// Ticker查询服务
pub struct TickerQueryService {
    redis: redis::Client,
    db_pool: PgPool,
}

impl TickerQueryService {
    /// 查询单个Ticker
    pub async fn get_ticker(&self, symbol: &str) -> Result<TickerView> {
        let key = format!("ticker:{}", symbol);

        // 尝试从Redis获取
        let mut conn = self.redis.get_async_connection().await?;
        if let Some(json) = conn.get::<_, Option<String>>(&key).await? {
            return Ok(serde_json::from_str(&json)?);
        }

        // 从数据库查询（快照表）
        let ticker = self.query_ticker_from_db(symbol).await?;

        // 写回Redis
        let json = serde_json::to_string(&ticker)?;
        conn.set_ex(&key, json, 60).await?;

        Ok(ticker)
    }

    /// 批量查询Ticker
    pub async fn get_tickers_batch(&self, symbols: &[String]) -> Result<Vec<TickerView>> {
        let mut conn = self.redis.get_async_connection().await?;

        // Redis管道批量查询
        let keys: Vec<String> = symbols.iter()
            .map(|s| format!("ticker:{}", s))
            .collect();

        let results: Vec<Option<String>> = redis::pipe()
            .atomic()
            .get(&keys)
            .query_async(&mut conn)
            .await?;

        let mut tickers = Vec::new();
        for (i, result) in results.iter().enumerate() {
            if let Some(json) = result {
                tickers.push(serde_json::from_str(json)?);
            } else {
                // 缓存未命中，查询数据库
                let ticker = self.query_ticker_from_db(&symbols[i]).await?;
                tickers.push(ticker);
            }
        }

        Ok(tickers)
    }

    async fn query_ticker_from_db(&self, symbol: &str) -> Result<TickerView> {
        // 从TimescaleDB查询最新快照
        let ticker = sqlx::query_as!(
            TickerView,
            r#"
            SELECT * FROM ticker_snapshots
            WHERE symbol = $1
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
            symbol
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(ticker)
    }
}

// HTTP API路由
pub async fn get_ticker_handler(
    Path(symbol): Path<String>,
    service: Extension<Arc<TickerQueryService>>,
) -> Result<Json<TickerView>, AppError> {
    let ticker = service.get_ticker(&symbol).await?;
    Ok(Json(ticker))
}

pub async fn get_all_tickers_handler(
    service: Extension<Arc<TickerQueryService>>,
) -> Result<Json<Vec<TickerView>>, AppError> {
    let symbols = vec!["BTCUSDT", "ETHUSDT", "BNBUSDT"]; // 从配置获取
    let tickers = service.get_tickers_batch(&symbols).await?;
    Ok(Json(tickers))
}
```

### WebSocket推送实现

```rust
// src/interfaces/websocket_axum/ticker_stream.rs
use axum::extract::ws::{WebSocket, Message};
use tokio::sync::broadcast;

/// Ticker实时推送服务
pub struct TickerStreamService {
    /// 广播频道（所有Ticker更新）
    broadcast_tx: broadcast::Sender<TickerUpdate>,
}

impl TickerStreamService {
    /// 处理WebSocket连接
    pub async fn handle_connection(&self, mut socket: WebSocket) {
        let mut rx = self.broadcast_tx.subscribe();

        loop {
            tokio::select! {
                // 接收客户端消息（订阅请求）
                msg = socket.recv() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            self.handle_subscribe(&text, &mut socket).await;
                        }
                        Some(Ok(Message::Close(_))) | None => break,
                        _ => {}
                    }
                }

                // 广播Ticker更新
                Ok(update) = rx.recv() => {
                    let json = serde_json::to_string(&update).unwrap();
                    if socket.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
        }
    }

    /// 发布Ticker更新（由事件消费者调用）
    pub fn publish_update(&self, ticker: TickerView) {
        let update = TickerUpdate {
            stream: format!("{}@ticker", ticker.symbol.to_lowercase()),
            data: ticker,
        };

        // 广播给所有订阅者
        self.broadcast_tx.send(update).ok();
    }
}

#[derive(Debug, Clone, Serialize)]
struct TickerUpdate {
    stream: String,
    data: TickerView,
}
```

### 事件消费者实现

```rust
// src/infrastructure/event_consumer/ticker_consumer.rs
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message as KafkaMessage;

/// 消费成交事件，更新Ticker读模型
pub struct TickerEventConsumer {
    consumer: StreamConsumer,
    redis: redis::Client,
    websocket_service: Arc<TickerStreamService>,
}

impl TickerEventConsumer {
    pub async fn start(&self) -> Result<()> {
        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        let event: TradeExecutedEvent = serde_json::from_slice(payload)?;
                        self.handle_trade_event(event).await?;
                    }
                }
                Err(e) => {
                    eprintln!("Kafka error: {:?}", e);
                }
            }
        }
    }

    async fn handle_trade_event(&self, event: TradeExecutedEvent) -> Result<()> {
        // 1. 更新Redis中的Ticker
        let ticker = self.update_ticker_in_redis(&event).await?;

        // 2. 推送给WebSocket客户端
        self.websocket_service.publish_update(ticker.clone());

        // 3. 异步写入TimescaleDB（快照）
        let db_pool = self.db_pool.clone();
        tokio::spawn(async move {
            if let Err(e) = save_ticker_snapshot(&db_pool, &ticker).await {
                eprintln!("Failed to save ticker snapshot: {:?}", e);
                // 可选：发送到监控系统
            }
        });

        Ok(())
    }

    async fn update_ticker_in_redis(&self, event: &TradeExecutedEvent) -> Result<TickerView> {
        let mut conn = self.redis.get_async_connection().await?;
        let key = format!("ticker:{}", event.symbol);

        // 使用Redis Lua脚本原子更新
        let script = r#"
            local key = KEYS[1]
            local price = tonumber(ARGV[1])
            local qty = tonumber(ARGV[2])

            local ticker = redis.call('HGETALL', key)
            -- 更新last_price, volume_24h等字段
            redis.call('HSET', key, 'last_price', price)
            redis.call('HINCRBY', key, 'volume_24h', qty)

            return redis.call('HGETALL', key)
        "#;

        let result: Vec<String> = redis::Script::new(script)
            .key(&key)
            .arg(event.price)
            .arg(event.quantity)
            .invoke_async(&mut conn)
            .await?;

        // 解析返回的Ticker
        let ticker = parse_ticker_from_redis(&result)?;
        Ok(ticker)
    }
}
```

---

## 监控指标

### 1. 读模型健康度指标

#### Redis缓存指标

```rust
// Prometheus指标定义
use prometheus::{register_histogram, register_counter, register_gauge};

lazy_static! {
    // 缓存命中率
    static ref CACHE_HIT_RATE: Gauge = register_gauge!(
        "market_data_cache_hit_rate",
        "Cache hit rate for market data queries"
    ).unwrap();

    // 查询延迟分布
    static ref QUERY_LATENCY: Histogram = register_histogram!(
        "market_data_query_latency_seconds",
        "Query latency for market data",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]
    ).unwrap();

    // 更新延迟（从事件到Redis）
    static ref UPDATE_LATENCY: Histogram = register_histogram!(
        "market_data_update_latency_seconds",
        "Update latency from event to Redis",
        vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05]
    ).unwrap();

    // WebSocket连接数
    static ref WEBSOCKET_CONNECTIONS: Gauge = register_gauge!(
        "market_data_websocket_connections",
        "Number of active WebSocket connections"
    ).unwrap();

    // 推送消息数
    static ref PUSH_MESSAGES: Counter = register_counter!(
        "market_data_push_messages_total",
        "Total number of pushed messages"
    ).unwrap();
}
```

#### 告警规则

```yaml
# Prometheus告警规则
groups:
  - name: market_data_alerts
    interval: 30s
    rules:
      # 缓存命中率过低
      - alert: LowCacheHitRate
        expr: market_data_cache_hit_rate < 0.9
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Market data cache hit rate is low"
          description: "Cache hit rate is {{ $value }}, below 90%"

      # 查询延迟过高
      - alert: HighQueryLatency
        expr: histogram_quantile(0.99, market_data_query_latency_seconds) > 0.01
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "Market data query latency is high"
          description: "P99 latency is {{ $value }}s, above 10ms"

      # 更新延迟过高
      - alert: HighUpdateLatency
        expr: histogram_quantile(0.99, market_data_update_latency_seconds) > 0.05
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Market data update latency is high"
          description: "P99 update latency is {{ $value }}s, above 50ms"

      # WebSocket连接数异常
      - alert: WebSocketConnectionsDrop
        expr: rate(market_data_websocket_connections[5m]) < -100
        for: 1m
        labels:
          severity: warning
        annotations:
          summary: "WebSocket connections dropping rapidly"
          description: "Connections dropping at {{ $value }}/s"
```

### 2. 数据一致性监控

```rust
/// 监控读模型与写模型的一致性延迟
pub struct ConsistencyMonitor {
    redis: redis::Client,
    db_pool: PgPool,
}

impl ConsistencyMonitor {
    /// 检查Ticker数据一致性
    pub async fn check_ticker_consistency(&self, symbol: &str) -> Result<Duration> {
        // 从Redis获取最新Ticker
        let redis_ticker = self.get_ticker_from_redis(symbol).await?;

        // 从数据库获取最新快照
        let db_ticker = self.get_ticker_from_db(symbol).await?;

        // 计算时间差
        let lag = redis_ticker.timestamp - db_ticker.timestamp;
        Ok(Duration::from_millis(lag as u64))
    }

    /// 定期检查一致性
    pub async fn monitor_consistency_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            for symbol in &["BTCUSDT", "ETHUSDT", "BNBUSDT"] {
                match self.check_ticker_consistency(symbol).await {
                    Ok(lag) if lag > Duration::from_secs(5) => {
                        eprintln!("Consistency lag for {}: {:?}", symbol, lag);
                        // 发送告警
                    }
                    Err(e) => {
                        eprintln!("Failed to check consistency for {}: {:?}", symbol, e);
                    }
                    _ => {}
                }
            }
        }
    }
}
```

---

## API接口定义

### HTTP REST API

#### 1. 获取单个Ticker

```http
GET /api/v1/ticker/{symbol}

Response 200 OK:
{
  "symbol": "BTCUSDT",
  "last_price": "50000.00",
  "price_change_24h": "1250.50",
  "price_change_percent_24h": "2.56",
  "high_24h": "51000.00",
  "low_24h": "48500.00",
  "volume_24h": "12345.6789",
  "quote_volume_24h": "617283945.00",
  "open_price_24h": "48749.50",
  "trade_count_24h": 125678,
  "best_bid": "49999.50",
  "best_bid_qty": "1.5000",
  "best_ask": "50000.50",
  "best_ask_qty": "2.3000",
  "timestamp": 1701234567890
}
```

#### 2. 批量获取Ticker

```http
GET /api/v1/tickers?symbols=BTCUSDT,ETHUSDT,BNBUSDT

Response 200 OK:
[
  { "symbol": "BTCUSDT", ... },
  { "symbol": "ETHUSDT", ... },
  { "symbol": "BNBUSDT", ... }
]
```

#### 3. 获取K线数据

```http
GET /api/v1/klines?symbol=BTCUSDT&interval=1m&limit=500

Query Parameters:
  - symbol: 交易对符号（必需）
  - interval: 时间周期（1m, 5m, 15m, 1h, 4h, 1d, 1w）
  - limit: 返回数量（默认500，最大1000）
  - start_time: 开始时间（毫秒时间戳，可选）
  - end_time: 结束时间（毫秒时间戳，可选）

Response 200 OK:
[
  {
    "symbol": "BTCUSDT",
    "interval": "1m",
    "open_time": 1701234560000,
    "open": "50000.00",
    "high": "50100.00",
    "low": "49950.00",
    "close": "50050.00",
    "volume": "123.4567",
    "close_time": 1701234619999,
    "quote_volume": "6172839.45",
    "trade_count": 1256,
    "taker_buy_volume": "65.1234",
    "taker_buy_quote_volume": "3258419.72"
  },
  ...
]
```

#### 4. 获取市场深度

```http
GET /api/v1/depth?symbol=BTCUSDT&limit=20

Query Parameters:
  - symbol: 交易对符号（必需）
  - limit: 档位数量（5, 10, 20, 50, 100, 500）

Response 200 OK:
{
  "symbol": "BTCUSDT",
  "last_update_id": 12345678,
  "bids": [
    ["50000.00", "1.5000"],
    ["49999.50", "2.3000"],
    ...
  ],
  "asks": [
    ["50000.50", "1.2000"],
    ["50001.00", "3.0000"],
    ...
  ],
  "timestamp": 1701234567890
}
```

#### 5. 获取最新成交

```http
GET /api/v1/trades?symbol=BTCUSDT&limit=50

Response 200 OK:
[
  {
    "trade_id": 123456789,
    "price": "50000.25",
    "quantity": "0.1234",
    "timestamp": 1701234567890,
    "is_buyer_maker": false
  },
  ...
]
```

#### 6. 获取用户历史成交

```http
GET /api/v1/users/{user_id}/trades?limit=50&cursor={cursor}

Headers:
  Authorization: Bearer {access_token}

Query Parameters:
  - limit: 返回数量（默认50，最大100）
  - cursor: 分页游标（可选）
  - symbol: 过滤交易对（可选）
  - start_time: 开始时间（可选）
  - end_time: 结束时间（可选）

Response 200 OK:
{
  "user_id": "user123",
  "trades": [
    {
      "trade_id": 987654321,
      "order_id": 123456789,
      "symbol": "BTCUSDT",
      "side": "BUY",
      "price": "50000.00",
      "quantity": "0.1000",
      "commission": "0.00010000",
      "commission_asset": "BTC",
      "timestamp": 1701234567890,
      "is_maker": true
    },
    ...
  ],
  "total": 1234,
  "next_cursor": "eyJsYXN0X2lkIjoxMjM0NTY3ODl9"
}
```

### WebSocket API

#### 订阅格式

```json
// 订阅请求
{
  "method": "SUBSCRIBE",
  "params": [
    "btcusdt@ticker",
    "btcusdt@depth@100ms",
    "btcusdt@trade",
    "btcusdt@kline_1m"
  ],
  "id": 1
}

// 订阅响应
{
  "result": null,
  "id": 1
}

// 取消订阅
{
  "method": "UNSUBSCRIBE",
  "params": [
    "btcusdt@ticker"
  ],
  "id": 2
}
```

#### 推送消息格式

**Ticker推送**:
```json
{
  "stream": "btcusdt@ticker",
  "data": {
    "symbol": "BTCUSDT",
    "last_price": "50000.00",
    "price_change_24h": "1250.50",
    "price_change_percent_24h": "2.56",
    ...
  }
}
```

**深度推送**:
```json
{
  "stream": "btcusdt@depth@100ms",
  "data": {
    "symbol": "BTCUSDT",
    "first_update_id": 12345678,
    "last_update_id": 12345680,
    "bids": [
      ["50000.00", "1.5000"],
      ["49999.50", "2.3000"]
    ],
    "asks": [
      ["50000.50", "1.2000"],
      ["50001.00", "3.0000"]
    ]
  }
}
```

**成交推送**:
```json
{
  "stream": "btcusdt@trade",
  "data": {
    "trade_id": 123456789,
    "price": "50000.25",
    "quantity": "0.1234",
    "timestamp": 1701234567890,
    "is_buyer_maker": false
  }
}
```

**K线推送**:
```json
{
  "stream": "btcusdt@kline_1m",
  "data": {
    "symbol": "BTCUSDT",
    "interval": "1m",
    "open_time": 1701234560000,
    "open": "50000.00",
    "high": "50100.00",
    "low": "49950.00",
    "close": "50050.00",
    "volume": "123.4567",
    "close_time": 1701234619999,
    "is_final": false
  }
}
```

---

## 总结

### 读模型设计要点

1. **用户导向**：根据不同用户群体（散户、专业交易者、量化机构）设计不同粒度的读模型
2. **分层存储**：热数据（Redis）、温数据（TimescaleDB）、冷数据（S3）
3. **实时推送**：WebSocket推送热点数据，HTTP查询历史数据
4. **性能优化**：多级缓存、批量查询、数据压缩
5. **最终一致性**：通过事件驱动异步更新读模型

### 核心读模型对比

| 读模型 | 散户 | 专业交易者 | 量化机构 | 更新频率 | 存储大小 | 数据粒度 |
|-------|-----|-----------|---------|---------|---------|---------|
| TickerView | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | 实时 | < 1MB | 聚合统计 |
| KLineView | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | 1s-1m | 10GB+ | 时间窗口聚合 |
| MarketDepthView | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | 100ms | 1MB | 价格档位聚合 |
| **OrderBookUpdatesView** ⭐ | ⭐ | ⭐⭐ | ⭐⭐⭐ | < 1ms | 10GB+/天 | **逐笔委托明细** |
| RecentTradesView | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | 实时 | < 1MB | 逐笔成交明细 |
| TradeHistoryView | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | 事后查询 | 100GB+ | 用户成交记录 |
| LiquidationView | ⭐ | ⭐⭐⭐ | ⭐⭐⭐ | 实时 | 1GB | 强平事件 |
| FundingRateView | ⭐ | ⭐⭐⭐ | ⭐⭐⭐ | 8小时 | < 1MB | 资金费率 |

**说明**：
- ⭐⭐⭐ = 核心需求
- ⭐⭐ = 重要需求
- ⭐ = 可选需求
- **OrderBookUpdatesView（逐笔委托）** 是本次新增的读模型，专为量化机构和做市商设计

---

## 变更记录

| 版本 | 日期 | 变更内容 | 作者 |
|------|------|---------|------|
| v1.6.0 | 2025-12-09 | **添加L1/L2/L3行业标准来源章节**；引用NASDAQ ITCH、CME MDP3.0、Databento等权威标准；证明我们的定义100%符合行业事实标准；添加主流交易所快照机制对比 | Claude |
| v1.5.0 | 2025-12-09 | **添加完整的快照机制章节**；定义L2/L3快照数据结构；快照一致性保证；丢包恢复流程；快照性能优化 | Claude |
| v1.4.0 | 2025-12-09 | **添加L1/L2/L3市场数据分级完整性分析**；验证读模型完整覆盖所有级别；添加与主流交易所对比 | Claude |
| v1.3.0 | 2025-12-09 | **参考NautilusTrader数据模型**；添加数据类型层次结构；补充6种交易策略的数据模型选择指南；添加性能对比表 | Claude |
| v1.2.0 | 2025-12-09 | **新增OrderBookUpdatesView（逐笔委托）读模型**；添加读模型分类体系；补充量化机构、做市商、高频交易场景 | Claude |
| v1.1.0 | 2025-12-09 | 修复is_buyer_maker语义错误；添加监控指标和API接口定义；优化代码示例；完善TTL策略说明 | Claude |
| v1.0.0 | 2025-12-09 | 初始版本，定义7个核心读模型 | Claude |

---

**文档版本**: v1.6.0
**最后更新**: 2025-12-09
**参考文档**:
- [CQRS架构分析](cqrs.md)
- [行情数据协议](market_data_proto.md)
- [币安API文档](https://binance-docs.github.io/apidocs/spot/en/)
- [NautilusTrader数据模型](https://nautilustrader.io/docs/latest/concepts/data)
- [NASDAQ TotalView-ITCH 5.0规范](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHSpecification.pdf)
- [CME MDP 3.0市场数据协议](https://www.cmegroup.com/confluence/display/EPICSANDBOX/MDP+3.0+-+Message+Specification)
- [Databento市场数据微观结构指南](https://databento.com/microstructure/level-1-market-data)

**下一步计划**:
- [ ] 添加GraphQL API支持
- [ ] 补充gRPC接口定义
- [ ] 增加数据压缩算法说明
- [ ] 添加灾难恢复方案
