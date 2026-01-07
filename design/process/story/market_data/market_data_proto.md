# RLTOP行情数据协议规范 (Market Data Protocol)

**文档版本**: v1.0.0
**创建日期**: 2025-12-06
**协议族**: RLTOP (RustLob Low-Latency Trading Order Protocol)
**状态**: Draft

---

## 目录

1. [协议概述](#协议概述)
2. [设计原则](#设计原则)
3. [消息格式规范](#消息格式规范)
4. [市场数据消息](#市场数据消息)
5. [订单簿重建](#订单簿重建)
6. [多播传输](#多播传输)
7. [数据压缩](#数据压缩)
8. [实现指南](#实现指南)
9. [性能优化](#性能优化)

---

## 协议概述

### 协议定位

RLTOP行情数据协议专注于**市场数据分发**，对标NASDAQ ITCH协议，旨在为交易者提供超低时延的市场行情和订单簿深度数据。

**核心特性**:
- **超低时延**: < 500ns市场数据分发时延
- **单向推送**: 无请求-响应开销
- **UDP多播**: 高效分发给多个订阅者
- **完整订单簿**: 支持完整订单簿重建
- **增量更新**: 带宽优化的增量消息

### 协议分层

```
┌─────────────────────────────────┐
│   Trading Application Layer     │  行情展示、策略分析
├─────────────────────────────────┤
│  RLTOP Market Data Protocol     │  本协议（行情数据）
├─────────────────────────────────┤
│   Transport Layer (UDP)         │  无连接传输
├─────────────────────────────────┤
│   Multicast (IGMP/MLD)          │  多播组管理
├─────────────────────────────────┤
│   Kernel Bypass (DPDK/AF_XDP)   │  内核旁路
├─────────────────────────────────┤
│   Physical Layer (Ethernet)     │  10/25/40GbE
└─────────────────────────────────┘
```

### 与交易指令协议的关系

| 协议 | 职责 | 方向 | 传输 | 时延要求 |
|------|------|------|------|----------|
| Trading Command | 订单输入、成交回报 | 双向 | TCP | < 1μs |
| **Market Data** | 市场行情、订单簿 | 单向（推送） | UDP多播 | < 500ns |

**数据流向**:
```
                    ┌──────────────┐
                    │ Matching Eng │
                    └──────┬───────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
           ▼               ▼               ▼
    ┌──────────┐    ┌──────────┐    ┌──────────┐
    │ Client A │    │ Client B │    │ Client C │
    └──────────┘    └──────────┘    └──────────┘
           ▲               ▲               ▲
           └───────────────┴───────────────┘
                  UDP Multicast
              (239.255.x.x:port)
```

---

## 设计原则

### 1. 单向推送

市场数据采用单向推送模式，无需订阅者发送请求：
- 匹配引擎主动推送所有市场事件
- 订阅者被动接收
- 减少往返时延（RTT）

### 2. UDP多播

使用UDP多播（Multicast）高效分发：
- 一次发送，多个订阅者接收
- 节省网络带宽
- 降低服务器负载

**多播地址分配**:
- `239.255.0.x`: 订单簿快照
- `239.255.1.x`: 订单簿增量更新
- `239.255.2.x`: 成交数据
- `239.255.3.x`: 统计数据

### 3. 幂等性

所有消息设计为幂等，支持重复处理：
- 每个消息携带唯一序列号
- 订阅者可重建完整状态
- 丢包可通过重传或快照恢复

### 4. 增量与快照

提供两种数据模式：
- **快照（Snapshot）**: 完整订单簿状态
- **增量（Incremental）**: 订单簿变化事件

订阅者根据需要选择：
- 启动时订阅快照，建立初始状态
- 运行时订阅增量，维护最新状态

---

## 消息格式规范

### 通用消息头

行情数据消息与交易指令消息共享相同的消息头（兼容性）：

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MessageHeader {
    pub magic: u32,              // 0x524C544F ('RLTO')
    pub length: u16,             // 消息长度
    pub msg_type: u8,            // 消息类型 (0x70-0x8F)
    pub version: u8,             // 协议版本
    pub session_id: u64,         // 会话ID（多播组ID）
    pub seq_num: u64,            // 序列号
    pub timestamp: u64,          // 时间戳（纳秒）
    pub checksum: u32,           // CRC32校验和
}

// 总大小: 40 bytes
```

**行情数据特殊字段**:
- `session_id`: 用作多播组标识（交易对分组）
- `seq_num`: 全局递增序列号（检测丢包）
- `checksum`: UDP强制校验

### 消息类型枚举

```rust
#[repr(u8)]
pub enum MarketDataMessageType {
    // 订单簿快照 (0x70-0x77)
    OrderBookSnapshot       = 0x70,  // 完整订单簿
    TopOfBookSnapshot       = 0x71,  // 最优买卖价
    TradeSnapshot           = 0x72,  // 最近成交快照

    // 订单簿增量 (0x78-0x7F)
    AddOrder                = 0x78,  // 新增订单
    ModifyOrder             = 0x79,  // 修改订单
    DeleteOrder             = 0x7A,  // 删除订单
    ExecuteOrder            = 0x7B,  // 订单成交
    Trade                   = 0x7C,  // 成交事件

    // 统计数据 (0x80-0x8F)
    DailyStatistics         = 0x80,  // 24小时统计
    KlineUpdate             = 0x81,  // K线更新
    TickerUpdate            = 0x82,  // Ticker更新
    LiquidationEvent        = 0x83,  // 强平事件
}
```

---

## 市场数据消息

### 1. OrderBookSnapshot（订单簿快照）

**消息类型**: `0x70`
**传输**: UDP多播（快照频道）
**大小**: 576字节（9个缓存行）

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PriceLevel {
    /// 价格（scaled integer）
    pub price: i64,              // 8 bytes

    /// 数量（scaled integer）
    pub quantity: u64,           // 8 bytes

    /// 订单数量（该价格档位的订单数）
    pub order_count: u32,        // 4 bytes

    /// 预留
    pub _padding: u32,           // 4 bytes
}
// PriceLevel: 24 bytes

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct OrderBookSnapshotMessage {
    pub header: MessageHeader,   // 40 bytes

    /// 交易对ID
    pub symbol_id: u32,          // 4 bytes

    /// 档位数量（实际有效档位）
    pub num_levels: u8,          // 1 byte

    pub _padding: [u8; 3],       // 3 bytes

    /// 快照序列号（订单簿版本号）
    pub snapshot_seq: u64,       // 8 bytes

    /// 买盘深度（最多10档）
    pub bids: [PriceLevel; 10],  // 240 bytes

    /// 卖盘深度（最多10档）
    pub asks: [PriceLevel; 10],  // 240 bytes

    /// 最新成交价
    pub last_trade_price: i64,   // 8 bytes

    /// 24小时成交量
    pub volume_24h: u64,         // 8 bytes

    /// 预留字段
    pub _reserved: [u8; 16],     // 16 bytes
}

// 总大小: 40 + 536 = 576 bytes (9 * 64字节缓存行)
```

**数据示例**（BTCUSDT）:
```
Snapshot #12345678
Symbol: 1 (BTCUSDT)
Timestamp: 1701234567890123000

Bids (买盘):
  50000.00 | 1.5000 BTC | 3 orders
  49999.50 | 2.3000 BTC | 5 orders
  49999.00 | 0.8500 BTC | 2 orders
  ...

Asks (卖盘):
  50000.50 | 1.2000 BTC | 2 orders
  50001.00 | 3.0000 BTC | 4 orders
  50001.50 | 1.5000 BTC | 3 orders
  ...

Last Trade: 50000.25
24h Volume: 12345.6789 BTC
```

**发送频率**:
- 每秒1次（常规市场）
- 每100ms 1次（活跃市场）
- 或每100次增量更新后发送

### 2. TopOfBookSnapshot（最优买卖价）

**消息类型**: `0x71`
**传输**: UDP多播（高频频道）
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TopOfBookSnapshotMessage {
    pub header: MessageHeader,   // 40 bytes

    /// 交易对ID
    pub symbol_id: u32,          // 4 bytes

    pub _padding: [u8; 4],       // 4 bytes

    /// 最优买价
    pub best_bid_price: i64,     // 8 bytes

    /// 最优买量
    pub best_bid_qty: u64,       // 8 bytes

    /// 最优卖价
    pub best_ask_price: i64,     // 8 bytes

    /// 最优卖量
    pub best_ask_qty: u64,       // 8 bytes

    /// 买卖价差
    pub spread: i64,             // 8 bytes

    /// 中间价
    pub mid_price: i64,          // 8 bytes

    /// 预留字段
    pub _reserved: [u8; 32],     // 32 bytes
}

// 总大小: 128 bytes (2 * 64字节缓存行)
```

**使用场景**:
- 超高频策略（只需BBO数据）
- 降低带宽消耗
- 发送频率：每次BBO变化时

### 3. AddOrder（新增订单）

**消息类型**: `0x78`
**传输**: UDP多播（增量频道）
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct AddOrderMessage {
    pub header: MessageHeader,   // 40 bytes

    /// 订单簿序列号（递增）
    pub order_book_seq: u64,     // 8 bytes

    /// 订单ID（交易所分配）
    pub order_id: u64,           // 8 bytes

    /// 交易对ID
    pub symbol_id: u32,          // 4 bytes

    /// 买卖方向 (1=Buy, 2=Sell)
    pub side: u8,                // 1 byte

    pub _padding: [u8; 3],       // 3 bytes

    /// 价格
    pub price: i64,              // 8 bytes

    /// 数量
    pub quantity: u64,           // 8 bytes

    /// 订单标志
    /// Bit 0: Hidden（隐藏订单）
    /// Bit 1-7: Reserved
    pub flags: u8,               // 1 byte

    pub _padding2: [u8; 7],      // 7 bytes

    /// 预留字段
    pub _reserved: [u8; 32],     // 32 bytes
}

// 总大小: 128 bytes
```

**处理逻辑**（订阅者端）:
```rust
fn handle_add_order(&mut self, msg: &AddOrderMessage) {
    let order_book = self.order_books.get_mut(&msg.symbol_id).unwrap();

    // 检查序列号连续性
    if msg.order_book_seq != order_book.last_seq + 1 {
        // 序列号缺失，请求快照
        self.request_snapshot(msg.symbol_id);
        return;
    }

    // 添加订单到订单簿
    let order = Order {
        order_id: msg.order_id,
        price: msg.price,
        quantity: msg.quantity,
        side: msg.side,
    };

    if msg.side == 1 {
        order_book.bids.insert(msg.price, order);
    } else {
        order_book.asks.insert(msg.price, order);
    }

    order_book.last_seq = msg.order_book_seq;
}
```

### 4. ModifyOrder（修改订单）

**消息类型**: `0x79`
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ModifyOrderMessage {
    pub header: MessageHeader,   // 40 bytes

    pub order_book_seq: u64,     // 8 bytes
    pub order_id: u64,           // 8 bytes
    pub symbol_id: u32,          // 4 bytes

    /// 修改类型
    /// 1 = PriceModify（价格修改，失去队列优先级）
    /// 2 = QuantityDecrease（数量减少，保持优先级）
    /// 3 = QuantityIncrease（数量增加，失去优先级）
    pub modify_type: u8,         // 1 byte

    pub _padding: [u8; 3],       // 3 bytes

    pub new_price: i64,          // 8 bytes（仅PriceModify）
    pub new_quantity: u64,       // 8 bytes

    pub _reserved: [u8; 48],     // 48 bytes
}

// 总大小: 128 bytes
```

**修改语义**:
- `PriceModify`: 等同于Delete + Add
- `QuantityDecrease`: 保持原队列位置
- `QuantityIncrease`: 失去优先级，移到队列末尾

### 5. DeleteOrder（删除订单）

**消息类型**: `0x7A`
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DeleteOrderMessage {
    pub header: MessageHeader,   // 40 bytes

    pub order_book_seq: u64,     // 8 bytes
    pub order_id: u64,           // 8 bytes
    pub symbol_id: u32,          // 4 bytes

    /// 删除原因
    /// 1 = Canceled（用户取消）
    /// 2 = Expired（过期）
    /// 3 = Filled（完全成交）
    /// 4 = Rejected（风控拒绝）
    pub delete_reason: u8,       // 1 byte

    pub _padding: [u8; 3],       // 3 bytes

    pub _reserved: [u8; 64],     // 64 bytes
}

// 总大小: 128 bytes
```

### 6. Trade（成交事件）

**消息类型**: `0x7C`
**传输**: UDP多播（成交频道）
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TradeMessage {
    pub header: MessageHeader,   // 40 bytes

    /// 成交ID（全局唯一）
    pub trade_id: u64,           // 8 bytes

    /// 交易对ID
    pub symbol_id: u32,          // 4 bytes

    /// Taker方向 (1=Buy, 2=Sell)
    pub taker_side: u8,          // 1 byte

    pub _padding: [u8; 3],       // 3 bytes

    /// 成交价格
    pub price: i64,              // 8 bytes

    /// 成交数量
    pub quantity: u64,           // 8 bytes

    /// Maker订单ID
    pub maker_order_id: u64,     // 8 bytes

    /// Taker订单ID
    pub taker_order_id: u64,     // 8 bytes

    /// 预留字段
    pub _reserved: [u8; 32],     // 32 bytes
}

// 总大小: 128 bytes
```

**公开成交数据**:
- 发布到公开多播组
- 不包含用户身份信息
- 用于K线生成、成交量统计

### 7. DailyStatistics（24小时统计）

**消息类型**: `0x80`
**传输**: UDP多播（统计频道）
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DailyStatisticsMessage {
    pub header: MessageHeader,   // 40 bytes

    /// 交易对ID
    pub symbol_id: u32,          // 4 bytes

    pub _padding: [u8; 4],       // 4 bytes

    /// 开盘价（24h前）
    pub open_price: i64,         // 8 bytes

    /// 最高价
    pub high_price: i64,         // 8 bytes

    /// 最低价
    pub low_price: i64,          // 8 bytes

    /// 最新价
    pub last_price: i64,         // 8 bytes

    /// 24小时成交量
    pub volume_24h: u64,         // 8 bytes

    /// 24小时成交额
    pub turnover_24h: u64,       // 8 bytes

    /// 价格变化（绝对值）
    pub price_change: i64,       // 8 bytes

    /// 价格变化率（百分比，scaled by 10000）
    /// 例如：+5.23% = 523
    pub price_change_pct: i32,   // 4 bytes

    /// 成交笔数
    pub trade_count: u32,        // 4 bytes

    /// 预留字段
    pub _reserved: [u8; 8],      // 8 bytes
}

// 总大小: 128 bytes
```

**发送频率**: 每秒1次

---

## 订单簿重建

### 完整订单簿重建流程

**步骤1: 订阅快照频道**
```rust
// 加入快照多播组
let socket = UdpSocket::bind("0.0.0.0:9001")?;
let multicast_addr = Ipv4Addr::new(239, 255, 0, 1);
socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::new(0, 0, 0, 0))?;

// 接收快照
let snapshot = receive_snapshot(&socket)?;
let mut order_book = OrderBook::from_snapshot(&snapshot);
```

**步骤2: 订阅增量频道**
```rust
// 加入增量多播组
let incremental_socket = UdpSocket::bind("0.0.0.0:9002")?;
let incremental_addr = Ipv4Addr::new(239, 255, 1, 1);
incremental_socket.join_multicast_v4(&incremental_addr, &Ipv4Addr::new(0, 0, 0, 0))?;
```

**步骤3: 应用增量更新**
```rust
loop {
    let msg = receive_incremental_message(&incremental_socket)?;

    // 跳过快照之前的消息
    if msg.order_book_seq <= order_book.snapshot_seq {
        continue;
    }

    // 检查序列号连续性
    if msg.order_book_seq != order_book.last_seq + 1 {
        eprintln!("Gap detected: expected {}, got {}",
                  order_book.last_seq + 1, msg.order_book_seq);
        // 重新请求快照
        order_book = request_new_snapshot()?;
        continue;
    }

    // 应用增量更新
    match msg.msg_type {
        0x78 => order_book.apply_add_order(&msg),
        0x79 => order_book.apply_modify_order(&msg),
        0x7A => order_book.apply_delete_order(&msg),
        _ => {}
    }

    order_book.last_seq = msg.order_book_seq;
}
```

### 订单簿数据结构

```rust
use std::collections::BTreeMap;

pub struct OrderBook {
    pub symbol_id: u32,
    pub snapshot_seq: u64,      // 快照序列号
    pub last_seq: u64,          // 最新序列号

    // 买盘：价格降序（最优买价在前）
    pub bids: BTreeMap<i64, Vec<Order>>,

    // 卖盘：价格升序（最优卖价在前）
    pub asks: BTreeMap<i64, Vec<Order>>,
}

pub struct Order {
    pub order_id: u64,
    pub price: i64,
    pub quantity: u64,
    pub timestamp: u64,
}

impl OrderBook {
    pub fn from_snapshot(snapshot: &OrderBookSnapshotMessage) -> Self {
        let mut book = OrderBook {
            symbol_id: snapshot.symbol_id,
            snapshot_seq: snapshot.snapshot_seq,
            last_seq: snapshot.snapshot_seq,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        };

        // 初始化买盘
        for level in &snapshot.bids {
            if level.quantity > 0 {
                book.bids.insert(level.price, vec![]);
            }
        }

        // 初始化卖盘
        for level in &snapshot.asks {
            if level.quantity > 0 {
                book.asks.insert(level.price, vec![]);
            }
        }

        book
    }

    pub fn apply_add_order(&mut self, msg: &AddOrderMessage) {
        let order = Order {
            order_id: msg.order_id,
            price: msg.price,
            quantity: msg.quantity,
            timestamp: msg.header.timestamp,
        };

        if msg.side == 1 {
            self.bids.entry(msg.price).or_insert(vec![]).push(order);
        } else {
            self.asks.entry(msg.price).or_insert(vec![]).push(order);
        }
    }

    pub fn get_best_bid(&self) -> Option<i64> {
        self.bids.keys().next_back().copied()
    }

    pub fn get_best_ask(&self) -> Option<i64> {
        self.asks.keys().next().copied()
    }
}
```

---

## 多播传输

### UDP多播配置

**服务器端（发送者）**:
```rust
use std::net::{UdpSocket, Ipv4Addr};

pub fn setup_multicast_sender(multicast_addr: &str, port: u16) -> Result<UdpSocket> {
    let socket = UdpSocket::bind(("0.0.0.0", 0))?;

    // 设置多播TTL（跳数）
    socket.set_multicast_ttl_v4(32)?;

    // 禁用多播回环（不接收自己发送的数据）
    socket.set_multicast_loop_v4(false)?;

    // 设置多播接口（可选）
    let interface = Ipv4Addr::new(192, 168, 1, 100);
    socket.set_multicast_if_v4(&interface)?;

    Ok(socket)
}

// 发送市场数据
pub fn publish_market_data(socket: &UdpSocket, msg: &OrderBookSnapshotMessage) -> Result<()> {
    let multicast_addr = "239.255.0.1:9001";
    socket.send_to(msg.as_bytes(), multicast_addr)?;
    Ok(())
}
```

**客户端（接收者）**:
```rust
pub fn setup_multicast_receiver(multicast_addr: &str, port: u16) -> Result<UdpSocket> {
    // 绑定到所有接口
    let socket = UdpSocket::bind(("0.0.0.0", port))?;

    // 加入多播组
    let multicast_ip: Ipv4Addr = multicast_addr.parse()?;
    let interface = Ipv4Addr::new(0, 0, 0, 0);  // 所有接口
    socket.join_multicast_v4(&multicast_ip, &interface)?;

    // 设置接收缓冲区大小（防止丢包）
    socket.set_recv_buffer_size(8 * 1024 * 1024)?; // 8MB

    Ok(socket)
}

// 接收市场数据
pub fn receive_market_data(socket: &UdpSocket) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; 1500]; // MTU大小
    let (len, _src) = socket.recv_from(&mut buf)?;
    buf.truncate(len);
    Ok(buf)
}
```

### 多播组规划

| 多播地址 | 端口 | 内容 | 更新频率 | 带宽估算 |
|---------|------|------|----------|----------|
| 239.255.0.1 | 9001 | 订单簿快照 | 1 Hz | 576 bytes/s |
| 239.255.0.2 | 9001 | BBO快照 | 10 Hz | 1.28 KB/s |
| 239.255.1.1 | 9002 | 订单簿增量 | 1000 Hz | 128 KB/s |
| 239.255.2.1 | 9003 | 成交数据 | 100 Hz | 12.8 KB/s |
| 239.255.3.1 | 9004 | 统计数据 | 1 Hz | 128 bytes/s |

**总带宽**: ~150 KB/s/交易对（活跃市场）

### 丢包处理

**检测丢包**:
```rust
impl MarketDataClient {
    pub fn check_sequence_gap(&self, msg: &MessageHeader) -> bool {
        let expected = self.last_seq + 1;
        if msg.seq_num != expected {
            eprintln!("Packet loss detected: expected {}, got {}", expected, msg.seq_num);
            return true;
        }
        false
    }
}
```

**恢复策略**:
1. **重传请求**（如果支持）: 请求缺失的消息序列号
2. **快照重建**: 订阅最新快照，丢弃旧增量消息
3. **容忍丢包**: 对于统计数据，可容忍少量丢包

---

## 数据压缩

### 增量编码优化

**价格增量编码**（参考FAST协议）:
```rust
#[repr(C, packed)]
pub struct CompressedAddOrderMessage {
    pub header: MessageHeader,   // 40 bytes

    pub order_book_seq: u64,     // 8 bytes
    pub order_id: u64,           // 8 bytes
    pub symbol_id: u32,          // 4 bytes
    pub side: u8,                // 1 byte

    /// 价格增量（相对于上一个价格）
    /// 可以使用更小的数据类型
    pub price_delta: i32,        // 4 bytes（而不是i64）

    pub quantity: u64,           // 8 bytes
    pub _reserved: [u8; 27],     // 27 bytes
}

// 节省4字节
```

**量化压缩**:
```rust
// 对于价格变化较小的市场，使用tick数代替绝对价格
pub struct TickBasedOrder {
    pub tick_offset: i16,        // 相对于基准价格的tick数
    pub quantity_lots: u32,      // 使用手数而不是绝对数量
}

// 例如：
// 基准价格 = 50000.00
// tick_size = 0.01
// tick_offset = 100 表示价格 = 50000.00 + 100 * 0.01 = 50001.00
```

---

## 实现指南

### 订阅者实现

**基本订阅者**:
```rust
use std::net::{UdpSocket, Ipv4Addr};
use std::collections::HashMap;

pub struct MarketDataSubscriber {
    sockets: HashMap<String, UdpSocket>,
    order_books: HashMap<u32, OrderBook>,
}

impl MarketDataSubscriber {
    pub fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            order_books: HashMap::new(),
        }
    }

    pub fn subscribe_snapshot(&mut self, symbol_id: u32) -> Result<()> {
        let socket = setup_multicast_receiver("239.255.0.1", 9001)?;
        self.sockets.insert("snapshot".to_string(), socket);
        Ok(())
    }

    pub fn subscribe_incremental(&mut self, symbol_id: u32) -> Result<()> {
        let socket = setup_multicast_receiver("239.255.1.1", 9002)?;
        self.sockets.insert("incremental".to_string(), socket);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            // 使用select/poll同时监听多个socket
            for (name, socket) in &self.sockets {
                let mut buf = vec![0u8; 1500];
                match socket.recv_from(&mut buf) {
                    Ok((len, _)) => {
                        buf.truncate(len);
                        self.handle_message(&buf)?;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
    }

    fn handle_message(&mut self, buf: &[u8]) -> Result<()> {
        let header = unsafe { MessageHeader::from_bytes(buf) };

        // 验证校验和
        if !header.verify_checksum(&buf[40..]) {
            eprintln!("Checksum mismatch");
            return Ok(());
        }

        // 分发消息
        match header.msg_type {
            0x70 => self.handle_snapshot(buf),
            0x78 => self.handle_add_order(buf),
            0x79 => self.handle_modify_order(buf),
            0x7A => self.handle_delete_order(buf),
            0x7C => self.handle_trade(buf),
            _ => Ok(()),
        }
    }

    fn handle_snapshot(&mut self, buf: &[u8]) -> Result<()> {
        let snapshot = unsafe { OrderBookSnapshotMessage::from_bytes(buf) };
        let order_book = OrderBook::from_snapshot(&snapshot);
        self.order_books.insert(snapshot.symbol_id, order_book);
        println!("Snapshot received for symbol {}", snapshot.symbol_id);
        Ok(())
    }
}
```

### 发布者实现

**市场数据发布器**:
```rust
pub struct MarketDataPublisher {
    snapshot_socket: UdpSocket,
    incremental_socket: UdpSocket,
    trade_socket: UdpSocket,
    seq_num: AtomicU64,
}

impl MarketDataPublisher {
    pub fn new() -> Result<Self> {
        Ok(Self {
            snapshot_socket: setup_multicast_sender("239.255.0.1", 9001)?,
            incremental_socket: setup_multicast_sender("239.255.1.1", 9002)?,
            trade_socket: setup_multicast_sender("239.255.2.1", 9003)?,
            seq_num: AtomicU64::new(0),
        })
    }

    pub fn publish_snapshot(&self, snapshot: &OrderBookSnapshotMessage) -> Result<()> {
        self.snapshot_socket.send_to(
            snapshot.as_bytes(),
            "239.255.0.1:9001"
        )?;
        Ok(())
    }

    pub fn publish_add_order(&self, msg: &AddOrderMessage) -> Result<()> {
        let mut msg = *msg;
        msg.header.seq_num = self.seq_num.fetch_add(1, Ordering::SeqCst);
        msg.header.timestamp = get_monotonic_nanos();
        msg.header.calculate_checksum(&msg.as_bytes()[40..]);

        self.incremental_socket.send_to(
            msg.as_bytes(),
            "239.255.1.1:9002"
        )?;
        Ok(())
    }

    pub fn publish_trade(&self, trade: &TradeMessage) -> Result<()> {
        self.trade_socket.send_to(
            trade.as_bytes(),
            "239.255.2.1:9003"
        )?;
        Ok(())
    }
}
```

---

## 性能优化

### 1. 批量接收

**使用recvmmsg批量接收**:
```rust
use libc::{recvmmsg, mmsghdr, iovec};

pub fn recv_batch(socket: &UdpSocket) -> Result<Vec<Vec<u8>>> {
    const BATCH_SIZE: usize = 32;

    let fd = socket.as_raw_fd();
    let mut msgs = vec![Vec::with_capacity(1500); BATCH_SIZE];
    let mut iovecs: Vec<iovec> = Vec::with_capacity(BATCH_SIZE);
    let mut mmsgs: Vec<mmsghdr> = Vec::with_capacity(BATCH_SIZE);

    // 配置iovec和mmsghdr
    for msg in &mut msgs {
        iovecs.push(iovec {
            iov_base: msg.as_mut_ptr() as *mut _,
            iov_len: msg.capacity(),
        });
    }

    for iov in &iovecs {
        mmsgs.push(unsafe { std::mem::zeroed() });
        let last = mmsgs.last_mut().unwrap();
        last.msg_hdr.msg_iov = iov as *const _ as *mut _;
        last.msg_hdr.msg_iovlen = 1;
    }

    // 批量接收
    let count = unsafe {
        recvmmsg(
            fd,
            mmsgs.as_mut_ptr(),
            BATCH_SIZE as u32,
            0,
            std::ptr::null_mut(),
        )
    };

    if count < 0 {
        return Err("recvmmsg failed".into());
    }

    // 处理接收到的消息
    let mut results = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        let len = mmsgs[i].msg_len as usize;
        msgs[i].truncate(len);
        results.push(msgs[i].clone());
    }

    Ok(results)
}
```

### 2. 零拷贝订单簿

**使用内存映射共享订单簿**:
```rust
use memmap2::MmapMut;

pub struct SharedOrderBook {
    mmap: MmapMut,
    snapshot_offset: usize,
}

impl SharedOrderBook {
    pub fn new(file_path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        file.set_len(1024 * 1024)?; // 1MB

        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(Self {
            mmap,
            snapshot_offset: 0,
        })
    }

    pub fn write_snapshot(&mut self, snapshot: &OrderBookSnapshotMessage) {
        let bytes = snapshot.as_bytes();
        self.mmap[self.snapshot_offset..self.snapshot_offset + bytes.len()]
            .copy_from_slice(bytes);
    }

    pub fn read_snapshot(&self) -> &OrderBookSnapshotMessage {
        unsafe {
            OrderBookSnapshotMessage::from_bytes(&self.mmap[self.snapshot_offset..])
        }
    }
}
```

### 3. SIMD优化

**向量化订单匹配**:
```rust
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn find_matching_price_simd(prices: &[i64; 4], target: i64) -> Option<usize> {
    let prices_vec = _mm256_loadu_si256(prices.as_ptr() as *const __m256i);
    let target_vec = _mm256_set1_epi64x(target);
    let cmp = _mm256_cmpeq_epi64(prices_vec, target_vec);
    let mask = _mm256_movemask_pd(_mm256_castsi256_pd(cmp));

    if mask != 0 {
        Some(mask.trailing_zeros() as usize)
    } else {
        None
    }
}
```

### 4. 性能基准

**延迟测量**:
```rust
use hdrhistogram::Histogram;

pub fn benchmark_market_data_latency() {
    let mut subscriber = MarketDataSubscriber::new();
    subscriber.subscribe_incremental(1).unwrap();

    let mut histogram = Histogram::<u64>::new(3).unwrap();

    for _ in 0..100000 {
        let start = Instant::now();

        // 接收消息
        let msg = subscriber.receive_message().unwrap();

        // 计算时延（从消息时间戳到接收时间）
        let latency = Instant::now().duration_since(start).as_nanos() as u64;
        histogram.record(latency).ok();
    }

    println!("Market Data Latency:");
    println!("  P50:  {} ns", histogram.value_at_percentile(50.0));
    println!("  P95:  {} ns", histogram.value_at_percentile(95.0));
    println!("  P99:  {} ns", histogram.value_at_percentile(99.0));
    println!("  P99.9: {} ns", histogram.value_at_percentile(99.9));
}
```

---

## 附录

### A. 消息类型完整列表

| 消息类型 | 代码 | 方向 | 大小 | 描述 |
|---------|------|------|------|------|
| OrderBookSnapshot | 0x70 | S→C | 576B | 完整订单簿快照 |
| TopOfBookSnapshot | 0x71 | S→C | 128B | 最优买卖价 |
| TradeSnapshot | 0x72 | S→C | 256B | 最近成交快照 |
| AddOrder | 0x78 | S→C | 128B | 新增订单 |
| ModifyOrder | 0x79 | S→C | 128B | 修改订单 |
| DeleteOrder | 0x7A | S→C | 128B | 删除订单 |
| ExecuteOrder | 0x7B | S→C | 128B | 订单成交 |
| Trade | 0x7C | S→C | 128B | 成交事件 |
| DailyStatistics | 0x80 | S→C | 128B | 24小时统计 |
| KlineUpdate | 0x81 | S→C | 128B | K线更新 |
| TickerUpdate | 0x82 | S→C | 128B | Ticker更新 |
| LiquidationEvent | 0x83 | S→C | 128B | 强平事件 |

### B. 多播组地址分配

| 多播地址 | 端口 | 内容 | 备注 |
|---------|------|------|------|
| 239.255.0.x | 9001 | 订单簿快照 | x=交易对分组 |
| 239.255.1.x | 9002 | 订单簿增量 | x=交易对分组 |
| 239.255.2.x | 9003 | 成交数据 | x=交易对分组 |
| 239.255.3.x | 9004 | 统计数据 | x=交易对分组 |

### C. 性能对比

| 指标 | RLTOP Market Data | ITCH 5.0 | WebSocket |
|------|-------------------|----------|-----------|
| 协议类型 | 二进制UDP多播 | 二进制UDP多播 | 文本/二进制TCP |
| 消息大小 | 128-576字节 | 40-50字节 | 200-500字节 |
| 分发时延 | < 500ns | < 1μs | 1-10ms |
| 带宽效率 | 高 | 极高 | 低 |
| 订阅者扩展性 | 极好（多播） | 极好（多播） | 差（每连接） |
| 可靠性 | UDP（需应用层处理） | UDP | TCP |

---

**文档结束**

**版本历史**:
- v1.0.0 (2025-12-06): 初始版本，从RLTOP完整协议中拆分行情数据部分
- 下一版本计划: 添加增量压缩算法、多播组动态订阅、K线生成规范

**相关文档**:
- [RLTOP交易指令协议规范](trading_command_proto.md)
- [RLTOP协议完整设计](command_proto.md)
