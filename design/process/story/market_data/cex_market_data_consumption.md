# CEX行情数据消费模型设计

**文档版本**: v1.0.0  
**创建日期**: 2025-12-09  
**项目**: RustLOB - 低延迟订单簿系统  
**状态**: Draft

---

## 目录

1. [概述](#概述)
2. [设计原则](#设计原则)
3. [Level 1 数据模型](#level-1-数据模型)
4. [Level 2 数据模型](#level-2-数据模型)
5. [Level 3 数据模型](#level-3-数据模型)
6. [数据消费接口](#数据消费接口)
7. [订单簿重建机制](#订单簿重建机制)
8. [性能优化](#性能优化)
9. [使用示例](#使用示例)

---

## 概述

### 设计目标

本文档从**CEX（中心化交易所）**的角度定义L1/L2/L3市场数据消费模型，旨在：

1. **高效分发**: 支持大量订阅者同时消费行情数据
2. **分级推送**: 根据用户需求提供不同粒度的数据
3. **低延迟**: 满足从零售到高频交易的各类延迟需求
4. **可重建**: 订阅者能够从任意时刻重建完整订单簿状态

### 架构概览

\`\`\`
┌─────────────────────────────────────────────────────────┐
│              Matching Engine (撮合引擎)                  │
│  - 订单簿维护                                             │
│  - 订单匹配                                               │
│  - 事件生成                                               │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│         Market Data Publisher (行情发布器)               │
│  - L1: BBO推送 (高频,小数据)                             │
│  - L2: 深度聚合 (中频,中数据)                            │
│  - L3: 完整订单流 (超高频,大数据)                        │
└────┬──────────┬──────────┬──────────────────────────────┘
     │          │          │
     ▼          ▼          ▼
  L1频道     L2频道     L3频道
(UDP多播)  (UDP多播)  (UDP多播/TCP)
     │          │          │
     └──────────┴──────────┴─────────────────┐
                                              ▼
                                    ┌──────────────────┐
                                    │  订阅者客户端     │
                                    │  - 零售交易者     │
                                    │  - 量化策略       │
                                    │  - 做市商         │
                                    │  - 高频交易       │
                                    └──────────────────┘
\`\`\`

### 数据级别对比

| 级别 | 目标用户 | 更新频率 | 数据大小 | 传输方式 | 延迟要求 |
|------|---------|---------|---------|---------|---------|
| L1 | 零售交易者 | 100-1000 Hz | 128 bytes | UDP多播 | < 500ns |
| L2 | 量化交易者 | 10-100 Hz | 576 bytes | UDP多播 | < 1μs |
| L3 | 高频交易 | 1000-10000 Hz | 128 bytes/event | TCP/UDP | < 10μs |

---

## 设计原则

### 1. 分级推送策略

不同用户对数据粒度需求不同：

- **L1**: 只需要最优买卖价，适合价格监控和简单交易
- **L2**: 需要市场深度，适合评估流动性和优化订单执行
- **L3**: 需要完整订单流，适合订单流分析和高频策略

### 2. 推送 vs 拉取

采用**推送模式**而非拉取：
- 减少往返延迟（RTT）
- 服务器主动推送，客户端被动接收
- 使用UDP多播高效分发

### 3. 快照 + 增量

- **快照（Snapshot）**: 定期发送完整状态，用于初始化和恢复
- **增量（Delta）**: 实时发送变化，节省带宽
- 客户端可随时通过快照重新同步

### 4. 序列号保证

- 每条消息携带全局递增序列号
- 客户端检测序列号缺失，触发重同步
- 保证消息顺序和完整性

---

## Level 1 数据模型

### 定义

**Level 1 (BBO - Best Bid/Offer)**: 最优买卖价数据，包含订单簿顶部的最佳报价信息。

### 目标用户

- 零售交易者
- 价格监控系统
- 简单交易策略
- 移动端应用

### 推送策略

- **触发条件**: 每次BBO变化时推送
- **更新频率**: 100-1000 Hz（取决于市场活跃度）
- **传输方式**: UDP多播
- **多播地址**: `239.255.0.x:9001`

### Rust数据结构

```rust
use crate::lob::domain::entity::lob_types::{Price, Quantity};

/// Level 1 市场数据 - 最优买卖价
/// 
/// CEX推送策略:
/// - 仅在BBO变化时推送
/// - 包含价差和中间价计算
/// - 适合显示报价和简单交易决策
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Level1MarketData {
    /// 消息头
    pub header: MessageHeader,           // 40 bytes
    
    /// 交易对ID
    pub symbol_id: u32,                  // 4 bytes
    
    /// 序列号(用于检测丢包)
    pub sequence: u64,                   // 8 bytes
    
    /// 最优买价
    pub best_bid_price: Price,           // 8 bytes (scaled integer)
    
    /// 最优买量
    pub best_bid_qty: Quantity,          // 8 bytes
    
    /// 最优卖价
    pub best_ask_price: Price,           // 8 bytes
    
    /// 最优卖量
    pub best_ask_qty: Quantity,          // 8 bytes
    
    /// 买卖价差
    pub spread: Price,                   // 8 bytes
    
    /// 中间价
    pub mid_price: Price,                // 8 bytes
    
    /// 最新成交价
    pub last_trade_price: Price,         // 8 bytes
    
    /// 最新成交量
    pub last_trade_qty: Quantity,        // 8 bytes
    
    /// 24小时成交量
    pub volume_24h: Quantity,            // 8 bytes
    
    /// 预留字段
    _reserved: [u8; 8],                  // 8 bytes
}
// 总大小: 128 bytes (2个缓存行)

impl Level1MarketData {
    /// 从订单簿生成L1数据
    pub fn from_orderbook(
        symbol_id: u32,
        sequence: u64,
        best_bid: Option<(Price, Quantity)>,
        best_ask: Option<(Price, Quantity)>,
        last_trade: Option<(Price, Quantity)>,
        volume_24h: Quantity,
    ) -> Self {
        let (bid_price, bid_qty) = best_bid.unwrap_or((0, 0));
        let (ask_price, ask_qty) = best_ask.unwrap_or((0, 0));
        let (trade_price, trade_qty) = last_trade.unwrap_or((0, 0));
        
        let spread = if bid_price > 0 && ask_price > 0 {
            ask_price - bid_price
        } else {
            0
        };
        
        let mid_price = if bid_price > 0 && ask_price > 0 {
            (bid_price + ask_price) / 2
        } else {
            0
        };
        
        Self {
            header: MessageHeader::new(0x71, sequence),
            symbol_id,
            sequence,
            best_bid_price: bid_price,
            best_bid_qty: bid_qty,
            best_ask_price: ask_price,
            best_ask_qty: ask_qty,
            spread,
            mid_price,
            last_trade_price: trade_price,
            last_trade_qty: trade_qty,
            volume_24h,
            _reserved: [0; 8],
        }
    }
    
    /// 检查是否有有效市场
    #[inline]
    pub fn has_valid_market(&self) -> bool {
        self.best_bid_price > 0 && self.best_ask_price > 0
    }
    
    /// 计算价差百分比
    #[inline]
    pub fn spread_bps(&self) -> u32 {
        if self.mid_price == 0 {
            return 0;
        }
        ((self.spread as f64 / self.mid_price as f64) * 10000.0) as u32
    }
}
```

### 数据示例

```json
{
  "symbol_id": 1,
  "sequence": 1234567890,
  "best_bid_price": 5000000,     // $50000.00
  "best_bid_qty": 150000000,     // 1.5 BTC
  "best_ask_price": 5000100,     // $50001.00
  "best_ask_qty": 230000000,     // 2.3 BTC
  "spread": 100,                 // $1.00
  "mid_price": 5000050,          // $50000.50
  "last_trade_price": 5000050,
  "last_trade_qty": 50000000,    // 0.5 BTC
  "volume_24h": 1234567890000    // 12345.67890 BTC
}
```

### 性能特性

- **缓存行对齐**: 64字节对齐，优化CPU缓存访问
- **零拷贝**: 可直接从网络缓冲区转换
- **固定大小**: 128字节，便于批量处理
- **延迟目标**: < 500ns 端到端

---

## Level 2 数据模型

### 定义

**Level 2 (Market Depth)**: 市场深度数据，显示多档价格的聚合订单数量，不包含单个订单详情。

### 目标用户

- 量化交易者
- 做市商
- 专业交易者
- 大单执行优化

### 推送策略

#### 快照模式
- **触发条件**: 定期发送完整订单簿
- **更新频率**: 1 Hz (每秒1次)
- **用途**: 初始化和定期同步

#### 增量模式
- **触发条件**: 每次价格档位变化
- **更新频率**: 10-100 Hz
- **用途**: 实时维护订单簿状态

#### 混合模式（推荐）
- 快照: 每秒1次
- 增量: 实时推送变化
- 客户端: 快照初始化 + 增量更新

### Rust数据结构

```rust
/// Level 2 价格档位
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct L2PriceLevel {
    /// 价格
    pub price: Price,                    // 8 bytes
    
    /// 该价格的总数量
    pub quantity: Quantity,              // 8 bytes
    
    /// 该价格的订单数量
    pub order_count: u32,                // 4 bytes
    
    /// 预留
    _padding: u32,                       // 4 bytes
}
// 单个档位: 24 bytes

/// Level 2 市场数据 - 订单簿深度快照
/// 
/// CEX推送策略:
/// - 快照模式: 每秒1次完整快照
/// - 增量模式: 每次价格档位变化时推送增量
/// - 支持10档/20档/50档可选
#[repr(C, align(64))]
#[derive(Debug, Clone)]
pub struct Level2Snapshot {
    /// 消息头
    pub header: MessageHeader,           // 40 bytes
    
    /// 交易对ID
    pub symbol_id: u32,                  // 4 bytes
    
    /// 快照序列号
    pub snapshot_seq: u64,               // 8 bytes
    
    /// 档位数量
    pub num_levels: u8,                  // 1 byte
    
    /// 预留
    _padding: [u8; 7],                   // 7 bytes
    
    /// 买盘深度 (从高到低)
    pub bids: [L2PriceLevel; 10],        // 240 bytes
    
    /// 卖盘深度 (从低到高)
    pub asks: [L2PriceLevel; 10],        // 240 bytes
    
    /// 最新成交价
    pub last_trade_price: Price,         // 8 bytes
    
    /// 24小时成交量
    pub volume_24h: Quantity,            // 8 bytes
    
    /// 预留字段
    _reserved: [u8; 16],                 // 16 bytes
}
// 总大小: 576 bytes (9个缓存行)

impl Level2Snapshot {
    /// 从订单簿生成L2快照
    pub fn from_orderbook<R: OrderRepository>(
        symbol_id: u32,
        snapshot_seq: u64,
        repository: &R,
        depth: usize,
    ) -> Self {
        let mut snapshot = Self {
            header: MessageHeader::new(0x70, snapshot_seq),
            symbol_id,
            snapshot_seq,
            num_levels: depth.min(10) as u8,
            _padding: [0; 7],
            bids: [L2PriceLevel::default(); 10],
            asks: [L2PriceLevel::default(); 10],
            last_trade_price: 0,
            volume_24h: 0,
            _reserved: [0; 16],
        };
        
        // 填充买盘深度
        let bid_levels = repository.get_bid_levels(depth);
        for (i, level) in bid_levels.iter().enumerate().take(10) {
            snapshot.bids[i] = *level;
        }
        
        // 填充卖盘深度
        let ask_levels = repository.get_ask_levels(depth);
        for (i, level) in ask_levels.iter().enumerate().take(10) {
            snapshot.asks[i] = *level;
        }
        
        snapshot
    }
    
    /// 计算订单簿不平衡度
    pub fn calculate_imbalance(&self) -> f64 {
        let bid_volume: Quantity = self.bids.iter()
            .take(self.num_levels as usize)
            .map(|l| l.quantity)
            .sum();
            
        let ask_volume: Quantity = self.asks.iter()
            .take(self.num_levels as usize)
            .map(|l| l.quantity)
            .sum();
            
        let total = bid_volume + ask_volume;
        if total == 0 {
            0.0
        } else {
            (bid_volume as f64 - ask_volume as f64) / total as f64
        }
    }
    
    /// 计算价格冲击
    pub fn calculate_price_impact(&self, side: Side, quantity: Quantity) -> Option<f64> {
        let levels = match side {
            Side::Buy => &self.asks,
            Side::Sell => &self.bids,
        };
        
        let best_price = levels[0].price as f64;
        let mut remaining = quantity;
        let mut total_cost = 0.0;
        
        for level in levels.iter().take(self.num_levels as usize) {
            if level.quantity == 0 {
                break;
            }
            
            let fill_qty = remaining.min(level.quantity);
            total_cost += fill_qty as f64 * level.price as f64;
            remaining -= fill_qty;
            
            if remaining == 0 {
                break;
            }
        }
        
        if remaining > 0 {
            return None; // 流动性不足
        }
        
        let avg_price = total_cost / quantity as f64;
        Some((avg_price - best_price) / best_price)
    }
}

/// Level 2 增量更新
/// 
/// 用于高效传输订单簿变化
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Level2Delta {
    /// 消息头
    pub header: MessageHeader,           // 40 bytes
    
    /// 交易对ID
    pub symbol_id: u32,                  // 4 bytes
    
    /// 增量序列号
    pub delta_seq: u64,                  // 8 bytes
    
    /// 更新类型
    /// 1 = Add (新增价格档位)
    /// 2 = Update (更新数量)
    /// 3 = Delete (删除价格档位)
    pub update_type: u8,                 // 1 byte
    
    /// 买卖方向 (1=Bid, 2=Ask)
    pub side: u8,                        // 1 byte
    
    _padding: [u8; 6],                   // 6 bytes
    
    /// 价格
    pub price: Price,                    // 8 bytes
    
    /// 新数量 (0表示删除)
    pub quantity: Quantity,              // 8 bytes
    
    /// 订单数量
    pub order_count: u32,                // 4 bytes
    
    /// 预留
    _reserved: [u8; 44],                 // 44 bytes
}
// 总大小: 128 bytes
```

### 数据示例

**快照示例**:
```
Symbol: BTCUSDT (ID=1)
Snapshot Seq: 1234567890
Timestamp: 2025-12-09 15:30:00.123456789

Bids (买盘):
  50000.00 | 1.5000 BTC | 3 orders
  49999.50 | 2.3000 BTC | 5 orders
  49999.00 | 3.1000 BTC | 7 orders
  49998.50 | 1.8000 BTC | 4 orders
  49998.00 | 4.2000 BTC | 9 orders

Asks (卖盘):
  50001.00 | 2.3000 BTC | 4 orders
  50001.50 | 1.9000 BTC | 3 orders
  50002.00 | 3.5000 BTC | 6 orders
  50002.50 | 2.1000 BTC | 5 orders
  50003.00 | 1.7000 BTC | 2 orders

Imbalance: +5.7% (买盘优势)
```

**增量示例**:
```json
{
  "update_type": 2,        // Update
  "side": 1,               // Bid
  "price": 5000000,        // $50000.00
  "quantity": 200000000,   // 2.0 BTC (从1.5更新到2.0)
  "order_count": 4         // 订单数从3增加到4
}
```

### 性能特性

- **快照大小**: 576 bytes (10档)
- **增量大小**: 128 bytes
- **延迟目标**: < 1μs 端到端
- **带宽估算**: ~150 KB/s/交易对 (活跃市场)

---

## Level 3 数据模型

### 定义

**Level 3 (Full Order Book)**: 完整订单簿数据，包含每个独立订单的完整信息和生命周期事件。

### 目标用户

- 高频交易（HFT）
- 做市商
- 订单流分析
- 市场微观结构研究

### 推送策略

- **触发条件**: 每个订单事件（新增、修改、取消、成交）
- **更新频率**: 1000-10000 Hz
- **传输方式**: TCP可靠传输 或 UDP多播+重传
- **多播地址**: `239.255.3.x:9004` (UDP) 或 TCP端口9005

### Rust数据结构

```rust
/// Level 3 订单事件类型
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum L3EventType {
    /// 新增订单
    Add = 1,
    /// 修改订单
    Modify = 2,
    /// 取消订单
    Cancel = 3,
    /// 订单成交
    Execute = 4,
    /// 订单完全成交
    Fill = 5,
}

/// Level 3 订单事件
/// 
/// CEX推送策略:
/// - 每个订单生命周期事件都推送
/// - 包含订单ID用于追踪
/// - 支持订单簿完整重建
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Level3OrderEvent {
    /// 消息头
    pub header: MessageHeader,           // 40 bytes
    
    /// 事件序列号 (全局递增)
    pub event_seq: u64,                  // 8 bytes
    
    /// 订单ID
    pub order_id: OrderId,               // 8 bytes
    
    /// 交易对ID
    pub symbol_id: u32,                  // 4 bytes
    
    /// 事件类型
    pub event_type: u8,                  // 1 byte
    
    /// 买卖方向 (1=Buy, 2=Sell)
    pub side: u8,                        // 1 byte
    
    _padding: [u8; 2],                   // 2 bytes
    
    /// 价格
    pub price: Price,                    // 8 bytes
    
    /// 数量
    pub quantity: Quantity,              // 8 bytes
    
    /// 剩余未成交数量
    pub remaining_qty: Quantity,         // 8 bytes
    
    /// 对手订单ID (仅成交事件)
    pub counterparty_order_id: OrderId,  // 8 bytes
    
    /// 订单标志
    /// Bit 0: Hidden (隐藏订单)
    /// Bit 1: PostOnly (只做Maker)
    /// Bit 2: IOC (立即成交或取消)
    /// Bit 3: FOK (全部成交或取消)
    pub flags: u8,                       // 1 byte
    
    /// 预留
    _reserved: [u8; 23],                 // 23 bytes
}
// 总大小: 128 bytes

impl Level3OrderEvent {
    /// 创建新增订单事件
    pub fn new_add_event(
        event_seq: u64,
        order_id: OrderId,
        symbol_id: u32,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> Self {
        Self {
            header: MessageHeader::new(0x78, event_seq),
            event_seq,
            order_id,
            symbol_id,
            event_type: L3EventType::Add as u8,
            side: side as u8,
            _padding: [0; 2],
            price,
            quantity,
            remaining_qty: quantity,
            counterparty_order_id: 0,
            flags: 0,
            _reserved: [0; 23],
        }
    }
    
    /// 创建成交事件
    pub fn new_execute_event(
        event_seq: u64,
        order_id: OrderId,
        symbol_id: u32,
        price: Price,
        executed_qty: Quantity,
        remaining_qty: Quantity,
        counterparty_id: OrderId,
    ) -> Self {
        Self {
            header: MessageHeader::new(0x7B, event_seq),
            event_seq,
            order_id,
            symbol_id,
            event_type: L3EventType::Execute as u8,
            side: 0,
            _padding: [0; 2],
            price,
            quantity: executed_qty,
            remaining_qty,
            counterparty_order_id: counterparty_id,
            flags: 0,
            _reserved: [0; 23],
        }
    }
    
    /// 创建取消事件
    pub fn new_cancel_event(
        event_seq: u64,
        order_id: OrderId,
        symbol_id: u32,
    ) -> Self {
        Self {
            header: MessageHeader::new(0x7A, event_seq),
            event_seq,
            order_id,
            symbol_id,
            event_type: L3EventType::Cancel as u8,
            side: 0,
            _padding: [0; 2],
            price: 0,
            quantity: 0,
            remaining_qty: 0,
            counterparty_order_id: 0,
            flags: 0,
            _reserved: [0; 23],
        }
    }
}
```

### 事件流示例

```
时间轴: 订单生命周期

T0: Add Event
    order_id: 12345678
    side: Buy
    price: $50000.00
    quantity: 1.0 BTC
    
T1: Execute Event (部分成交)
    order_id: 12345678
    executed_qty: 0.3 BTC
    remaining_qty: 0.7 BTC
    counterparty: 87654321
    
T2: Execute Event (再次成交)
    order_id: 12345678
    executed_qty: 0.5 BTC
    remaining_qty: 0.2 BTC
    counterparty: 11223344
    
T3: Cancel Event (取消剩余)
    order_id: 12345678
    remaining_qty: 0.2 BTC
```

### 性能特性

- **事件大小**: 128 bytes
- **延迟目标**: < 10μs 端到端
- **吞吐量**: 支持100万事件/秒
- **带宽估算**: ~128 MB/s (1M events/s)

---

## 数据消费接口

### 订阅者Trait

```rust
use async_trait::async_trait;

/// 市场数据订阅者trait
/// 
/// CEX客户端实现此trait来消费行情数据
#[async_trait]
pub trait MarketDataSubscriber: Send + Sync {
    /// 处理L1数据更新
    async fn on_level1_update(&mut self, data: &Level1MarketData) -> Result<()>;
    
    /// 处理L2快照
    async fn on_level2_snapshot(&mut self, snapshot: &Level2Snapshot) -> Result<()>;
    
    /// 处理L2增量更新
    async fn on_level2_delta(&mut self, delta: &Level2Delta) -> Result<()>;
    
    /// 处理L3订单事件
    async fn on_level3_event(&mut self, event: &Level3OrderEvent) -> Result<()>;
    
    /// 处理连接断开
    async fn on_disconnect(&mut self) -> Result<()>;
    
    /// 处理序列号缺失(需要重新同步)
    async fn on_sequence_gap(&mut self, expected: u64, received: u64) -> Result<()>;
}
```

### 发布器实现

```rust
use tokio::net::UdpSocket;
use std::sync::atomic::{AtomicU64, Ordering};

/// 市场数据发布器
/// 
/// CEX服务端使用此结构发布行情数据
pub struct MarketDataPublisher {
    /// L1多播socket
    l1_socket: UdpSocket,
    
    /// L2快照多播socket
    l2_snapshot_socket: UdpSocket,
    
    /// L2增量多播socket
    l2_delta_socket: UdpSocket,
    
    /// L3事件socket (TCP或UDP)
    l3_socket: UdpSocket,
    
    /// 序列号生成器
    sequence_gen: AtomicU64,
}

impl MarketDataPublisher {
    /// 创建新的发布器
    pub async fn new() -> Result<Self> {
        let l1_socket = UdpSocket::bind("0.0.0.0:0").await?;
        l1_socket.set_multicast_ttl_v4(32)?;
        
        let l2_snapshot_socket = UdpSocket::bind("0.0.0.0:0").await?;
        l2_snapshot_socket.set_multicast_ttl_v4(32)?;
        
        let l2_delta_socket = UdpSocket::bind("0.0.0.0:0").await?;
        l2_delta_socket.set_multicast_ttl_v4(32)?;
        
        let l3_socket = UdpSocket::bind("0.0.0.0:0").await?;
        l3_socket.set_multicast_ttl_v4(32)?;
        
        Ok(Self {
            l1_socket,
            l2_snapshot_socket,
            l2_delta_socket,
            l3_socket,
            sequence_gen: AtomicU64::new(0),
        })
    }
    
    /// 发布L1数据
    pub async fn publish_l1(&self, data: &Level1MarketData) -> Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                data as *const _ as *const u8,
                std::mem::size_of::<Level1MarketData>(),
            )
        };
        
        self.l1_socket.send_to(bytes, "239.255.0.1:9001").await?;
        Ok(())
    }
    
    /// 发布L2快照
    pub async fn publish_l2_snapshot(&self, snapshot: &Level2Snapshot) -> Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                snapshot as *const _ as *const u8,
                std::mem::size_of::<Level2Snapshot>(),
            )
        };
        
        self.l2_snapshot_socket.send_to(bytes, "239.255.1.1:9002").await?;
        Ok(())
    }
    
    /// 发布L2增量
    pub async fn publish_l2_delta(&self, delta: &Level2Delta) -> Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                delta as *const _ as *const u8,
                std::mem::size_of::<Level2Delta>(),
            )
        };
        
        self.l2_delta_socket.send_to(bytes, "239.255.2.1:9003").await?;
        Ok(())
    }
    
    /// 发布L3事件
    pub async fn publish_l3_event(&self, event: &Level3OrderEvent) -> Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                event as *const _ as *const u8,
                std::mem::size_of::<Level3OrderEvent>(),
            )
        };
        
        self.l3_socket.send_to(bytes, "239.255.3.1:9004").await?;
        Ok(())
    }
    
    /// 生成下一个序列号
    fn next_sequence(&self) -> u64 {
        self.sequence_gen.fetch_add(1, Ordering::SeqCst)
    }
}
```

---

## 订单簿重建机制

### 重建流程

```rust
use std::collections::BTreeMap;

/// 订单簿重建器
/// 
/// 从L2/L3数据流重建完整订单簿
pub struct OrderBookRebuilder {
    /// 交易对ID
    symbol_id: u32,
    
    /// 当前订单簿状态
    bids: BTreeMap<Price, Quantity>,
    asks: BTreeMap<Price, Quantity>,
    
    /// 最后处理的序列号
    last_sequence: u64,
    
    /// 是否已同步
    is_synced: bool,
}

impl OrderBookRebuilder {
    /// 创建新的重建器
    pub fn new(symbol_id: u32) -> Self {
        Self {
            symbol_id,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_sequence: 0,
            is_synced: false,
        }
    }
    
    /// 从L2快照初始化
    pub fn from_l2_snapshot(&mut self, snapshot: &Level2Snapshot) {
        self.bids.clear();
        self.asks.clear();
        self.last_sequence = snapshot.snapshot_seq;
        
        // 重建买盘
        for level in &snapshot.bids {
            if level.quantity > 0 {
                self.bids.insert(level.price, level.quantity);
            }
        }
        
        // 重建卖盘
        for level in &snapshot.asks {
            if level.quantity > 0 {
                self.asks.insert(level.price, level.quantity);
            }
        }
        
        self.is_synced = true;
        println!("OrderBook synced from snapshot seq={}", snapshot.snapshot_seq);
    }
    
    /// 应用L2增量更新
    pub fn apply_l2_delta(&mut self, delta: &Level2Delta) -> Result<()> {
        // 检查序列号连续性
        if delta.delta_seq != self.last_sequence + 1 {
            self.is_synced = false;
            return Err(Error::SequenceGap {
                expected: self.last_sequence + 1,
                received: delta.delta_seq,
            });
        }
        
        // 应用更新
        let book = if delta.side == 1 { &mut self.bids } else { &mut self.asks };
        
        match delta.update_type {
            1 | 2 => { // Add or Update
                if delta.quantity > 0 {
                    book.insert(delta.price, delta.quantity);
                } else {
                    book.remove(&delta.price);
                }
            }
            3 => { // Delete
                book.remove(&delta.price);
            }
            _ => {}
        }
        
        self.last_sequence = delta.delta_seq;
        Ok(())
    }
    
    /// 应用L3订单事件
    pub fn apply_l3_event(&mut self, event: &Level3OrderEvent) -> Result<()> {
        // 检查序列号
        if event.event_seq != self.last_sequence + 1 {
            self.is_synced = false;
            return Err(Error::SequenceGap {
                expected: self.last_sequence + 1,
                received: event.event_seq,
            });
        }
        
        match event.event_type {
            1 => { // Add
                let book = if event.side == 1 { &mut self.bids } else { &mut self.asks };
                book.entry(event.price)
                    .and_modify(|q| *q += event.quantity)
                    .or_insert(event.quantity);
            }
            3 => { // Cancel
                // 需要维护订单ID到价格的映射才能精确删除
                // 简化版本: 忽略取消事件,依赖快照同步
            }
            4 => { // Execute
                let book = if event.side == 1 { &mut self.bids } else { &mut self.asks };
                if let Some(qty) = book.get_mut(&event.price) {
                    *qty = qty.saturating_sub(event.quantity);
                    if *qty == 0 {
                        book.remove(&event.price);
                    }
                }
            }
            _ => {}
        }
        
        self.last_sequence = event.event_seq;
        Ok(())
    }
    
    /// 获取最优买价
    pub fn best_bid(&self) -> Option<(Price, Quantity)> {
        self.bids.iter().next_back().map(|(p, q)| (*p, *q))
    }
    
    /// 获取最优卖价
    pub fn best_ask(&self) -> Option<(Price, Quantity)> {
        self.asks.iter().next().map(|(p, q)| (*p, *q))
    }
}
```

### 同步策略

```rust
/// 订阅客户端
pub struct MarketDataClient {
    rebuilder: OrderBookRebuilder,
    snapshot_interval: Duration,
    last_snapshot_time: Instant,
}

impl MarketDataClient {
    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                // 接收增量更新
                Ok(delta) = self.recv_delta() => {
                    if let Err(e) = self.rebuilder.apply_l2_delta(&delta) {
                        eprintln!("Delta apply failed: {:?}", e);
                        // 序列号缺失,请求快照
                        self.request_snapshot().await?;
                    }
                }
                
                // 定期快照同步
                _ = tokio::time::sleep(self.snapshot_interval) => {
                    let snapshot = self.recv_snapshot().await?;
                    self.rebuilder.from_l2_snapshot(&snapshot);
                    self.last_snapshot_time = Instant::now();
                }
            }
        }
    }
}
```

---

## 性能优化

### 1. 批量接收

```rust
use libc::{recvmmsg, mmsghdr};

/// 批量接收UDP消息
pub fn recv_batch(socket: &UdpSocket, batch_size: usize) -> Result<Vec<Vec<u8>>> {
    const MAX_BATCH: usize = 32;
    let batch_size = batch_size.min(MAX_BATCH);
    
    // 使用recvmmsg系统调用批量接收
    // 减少系统调用开销
    // 实现细节省略...
    
    Ok(vec![])
}
```

### 2. 零拷贝解析

```rust
/// 零拷贝消息解析
pub fn parse_message_zerocopy(buf: &[u8]) -> Result<&Level1MarketData> {
    if buf.len() < std::mem::size_of::<Level1MarketData>() {
        return Err(Error::InvalidMessageSize);
    }
    
    let msg = unsafe {
        &*(buf.as_ptr() as *const Level1MarketData)
    };
    
    // 验证校验和
    if !msg.header.verify_checksum() {
        return Err(Error::ChecksumMismatch);
    }
    
    Ok(msg)
}
```

### 3. SIMD优化

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// 使用SIMD查找价格档位
#[target_feature(enable = "avx2")]
unsafe fn find_price_level_simd(levels: &[L2PriceLevel; 10], target_price: Price) -> Option<usize> {
    // 使用AVX2指令并行比较8个价格
    // 实现细节省略...
    None
}
```

---

## 使用示例

### 服务端发布示例

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // 1. 创建市场数据发布器
    let publisher = MarketDataPublisher::new().await?;
    
    // 2. 从撮合引擎获取订单簿状态
    let orderbook = get_orderbook_from_matching_engine()?;
    
    // 3. 生成并发布L1数据
    let l1_data = Level1MarketData::from_orderbook(
        1,  // BTCUSDT
        publisher.next_sequence(),
        orderbook.best_bid(),
        orderbook.best_ask(),
        orderbook.last_trade(),
        orderbook.volume_24h(),
    );
    publisher.publish_l1(&l1_data).await?;
    
    // 4. 生成并发布L2快照
    let l2_snapshot = Level2Snapshot::from_orderbook(
        1,
        publisher.next_sequence(),
        &orderbook,
        10,  // 10档深度
    );
    publisher.publish_l2_snapshot(&l2_snapshot).await?;
    
    // 5. 订单事件触发L3推送
    let l3_event = Level3OrderEvent::new_add_event(
        publisher.next_sequence(),
        order_id,
        1,
        Side::Buy,
        50000_00,  // $50000.00
        1_00000000,  // 1.0 BTC
    );
    publisher.publish_l3_event(&l3_event).await?;
    
    Ok(())
}
```

### 客户端订阅示例

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // 创建订阅者
    let mut client = MarketDataClient::new();
    
    // 订阅L1数据
    client.subscribe_l1(vec![1, 2, 3]).await?;  // BTCUSDT, ETHUSDT, BNBUSDT
    
    // 订阅L2数据
    client.subscribe_l2(vec![1], L2Mode::Incremental).await?;
    
    // 订阅L3数据
    client.subscribe_l3(vec![1]).await?;
    
    // 处理数据流
    loop {
        match client.recv().await? {
            MarketDataMessage::L1(data) => {
                println!("L1: BBO = {}/{}", data.best_bid_price, data.best_ask_price);
                
                // 检查价差
                if data.spread_bps() < 10 {
                    println!("Tight spread: {} bps", data.spread_bps());
                }
            }
            
            MarketDataMessage::L2Snapshot(snapshot) => {
                println!("L2 Snapshot: {} levels", snapshot.num_levels);
                client.rebuilder.from_l2_snapshot(&snapshot);
                
                // 计算订单簿不平衡
                let imbalance = snapshot.calculate_imbalance();
                println!("Imbalance: {:.2}%", imbalance * 100.0);
            }
            
            MarketDataMessage::L2Delta(delta) => {
                if let Err(e) = client.rebuilder.apply_l2_delta(&delta) {
                    eprintln!("Failed to apply delta: {:?}", e);
                    client.request_snapshot().await?;
                }
            }
            
            MarketDataMessage::L3Event(event) => {
                println!("L3 Event: {:?} order {}", event.event_type, event.order_id);
                
                // 订单流分析
                if event.event_type == L3EventType::Add as u8 {
                    analyze_order_flow(&event);
                }
            }
        }
    }
}

fn analyze_order_flow(event: &Level3OrderEvent) {
    // 检测大单
    if event.quantity > 10_00000000 {  // > 10 BTC
        println!("Large order detected: {} BTC @ ${}", 
                 event.quantity / 100000000, 
                 event.price / 100);
    }
}
```

---

## 总结

### 关键设计要点

1. **分级推送**:
   - L1: 128 bytes, 100-1000 Hz, 适合零售交易者
   - L2: 576 bytes, 10-100 Hz, 适合量化交易
   - L3: 128 bytes/event, 1000-10000 Hz, 适合高频交易

2. **数据一致性**:
   - 全局递增序列号
   - 定期快照 + 增量更新
   - 序列号缺失检测和重同步机制

3. **性能优化**:
   - 缓存行对齐 (64字节)
   - 零拷贝传输
   - UDP多播降低服务器负载
   - 固定大小消息便于解析

4. **可扩展性**:
   - 支持多交易对独立频道
   - 支持不同深度档位 (10/20/50档)
   - 支持多种推送策略配置

### 与现有代码集成

本设计完全兼容现有代码:
- `lib/core/exchange/lob/src/lob/domain/entity/level_types.rs` - 数据结构定义
- `app/design/process/story/market_data_proto.md` - 传输协议规范
- `CLAUDE.md` - 低延迟性能标准

### 下一步实施

1. **实现数据结构**: 在 `level_types.rs` 中实现完整的L1/L2/L3结构
2. **实现发布器**: 在 `adaptor/outbound/` 中实现市场数据发布器
3. **实现订阅器**: 创建客户端SDK用于消费行情数据
4. **性能测试**: 验证延迟指标 (L1 < 500ns, L2 < 1μs, L3 < 10μs)
5. **集成测试**: 端到端测试订单簿重建和数据一致性

---

**文档版本**: v1.0.0  
**最后更新**: 2025-12-09  
**作者**: RustLOB Team  
**状态**: Draft - 待审核

