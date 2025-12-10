# RLTOP交易指令协议规范 (Trading Command Protocol)

**文档版本**: v1.0.0
**创建日期**: 2025-12-06
**协议族**: RLTOP (RustLob Low-Latency Trading Order Protocol)
**状态**: Draft

---

## 目录

1. [协议概述](#协议概述)
2. [设计原则](#设计原则)
3. [消息格式规范](#消息格式规范)
4. [交易指令消息](#交易指令消息)
5. [响应消息](#响应消息)
6. [会话管理](#会话管理)
7. [错误处理](#错误处理)
8. [安全性](#安全性)
9. [实现指南](#实现指南)
10. [性能优化](#性能优化)

---

## 协议概述

### 协议定位

RLTOP交易指令协议专注于**订单输入和执行回报**，对标NASDAQ OUCH协议，旨在为加密货币和传统金融市场提供超低时延的交易指令传输能力。

**核心特性**:
- **超低时延**: < 1μs订单提交时延（网卡到匹配引擎）
- **零拷贝**: 消息直接映射到内存，无序列化开销
- **固定长度**: 核心消息固定长度，可预测性能
- **缓存友好**: 消息大小对齐缓存行（64/128字节）
- **可扩展**: 预留字段支持未来功能扩展

### 协议分层

```
┌─────────────────────────────────┐
│   Trading Application Layer     │  策略引擎、订单管理
├─────────────────────────────────┤
│  RLTOP Trading Command Protocol │  本协议（订单指令）
├─────────────────────────────────┤
│      Session Layer              │  会话管理、认证、心跳
├─────────────────────────────────┤
│    Transport Layer (TCP/UDP)    │  可靠传输
├─────────────────────────────────┤
│   Kernel Bypass (DPDK/io_uring) │  内核旁路
├─────────────────────────────────┤
│    Physical Layer (Ethernet)    │  10/25/40GbE
└─────────────────────────────────┘
```

### 与行情协议的关系

| 协议 | 职责 | 方向 | 传输 | 时延要求 |
|------|------|------|------|----------|
| **Trading Command** | 订单输入、成交回报 | 双向（请求-响应） | TCP | < 1μs |
| **Market Data** | 市场行情、深度数据 | 单向（推送） | UDP多播 | < 500ns |

**消息类型分离**:
- 交易指令协议：`0x00-0x6F`（会话、订单、执行）
- 行情数据协议：`0x70-0x8F`（快照、增量、统计）
- 系统消息：`0x90-0x9F`（状态、公告）

---

## 设计原则

### 1. 零拷贝原则

所有消息使用`#[repr(C, packed)]`布局，可直接从网络字节流转换为内存结构：

```rust
// 零拷贝转换
unsafe fn from_bytes(bytes: &[u8]) -> &Self {
    assert!(bytes.len() >= std::mem::size_of::<Self>());
    &*(bytes.as_ptr() as *const Self)
}
```

### 2. 固定长度优先

核心订单消息使用固定长度，避免解析开销：
- 新订单（NewOrder）: 192字节
- 取消订单（CancelOrder）: 128字节
- 修改订单（ReplaceOrder）: 128字节
- 成交回报（ExecutionReport）: 192字节

### 3. 缓存对齐

消息大小对齐缓存行，减少false sharing：
- x86-64: 64字节缓存行
- ARM64 (Apple M系列): 128字节缓存行
- 推荐：使用128字节对齐保证跨平台兼容

### 4. 大端序（网络字节序）

所有多字节整数使用大端序（Big-Endian），符合网络协议标准。

### 5. 版本前向兼容

- 主版本号（4 bits）：不兼容变更
- 次版本号（4 bits）：向后兼容新特性
- 预留字段：每个消息预留8-24字节扩展空间

---

## 消息格式规范

### 通用消息头

所有交易指令消息共享相同的消息头：

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MessageHeader {
    /// 魔数：0x524C544F ('RLTO')
    pub magic: u32,

    /// 消息长度（包含header，字节）
    pub length: u16,

    /// 消息类型（见MessageType枚举）
    pub msg_type: u8,

    /// 协议版本（高4位=major, 低4位=minor）
    pub version: u8,

    /// 会话ID（登录后分配）
    pub session_id: u64,

    /// 消息序列号（会话内递增）
    pub seq_num: u64,

    /// 时间戳（纳秒，CLOCK_MONOTONIC）
    pub timestamp: u64,

    /// CRC32校验和（UDP模式，TCP可置0）
    pub checksum: u32,
}

// 总大小: 40 bytes
```

**字段说明**:
- `magic`: 快速协议识别，避免误解析
- `length`: 支持变长消息扩展
- `msg_type`: 消息类型（0x00-0x6F为交易指令）
- `version`: 当前版本1.0 = 0x10
- `session_id`: 多路复用标识
- `seq_num`: 消息顺序保证，检测丢包
- `timestamp`: 发送时间，用于时延测量
- `checksum`: 仅UDP模式校验完整性

### 消息类型枚举

```rust
#[repr(u8)]
pub enum TradingMessageType {
    // 会话管理 (0x00-0x0F)
    Heartbeat           = 0x01,
    Logon               = 0x02,
    Logout              = 0x03,
    TestRequest         = 0x04,
    ResendRequest       = 0x05,

    // 订单操作 (0x10-0x2F)
    NewOrder            = 0x10,
    CancelOrder         = 0x11,
    ReplaceOrder        = 0x12,
    MassCancelOrder     = 0x13,
    OrderStatusRequest  = 0x14,

    // 订单响应 (0x30-0x4F)
    OrderAccepted       = 0x30,
    OrderRejected       = 0x31,
    OrderCanceled       = 0x32,
    OrderReplaced       = 0x33,
    CancelRejected      = 0x34,
    ReplaceRejected     = 0x35,

    // 执行回报 (0x50-0x6F)
    ExecutionReport     = 0x50,
    TradeReport         = 0x51,
    TradeCancelReport   = 0x52,
    PositionReport      = 0x53,
}
```

---

## 交易指令消息

### 1. NewOrder（新订单）

**消息类型**: `0x10`
**方向**: 客户端 → 服务器
**大小**: 192字节（3个缓存行）

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct NewOrderMessage {
    /// 消息头
    pub header: MessageHeader,              // 40 bytes

    /// 客户订单ID（唯一标识，用户自定义）
    pub cl_ord_id: u64,                     // 8 bytes

    /// 交易对ID（内部编码，预先映射）
    pub symbol_id: u32,                     // 4 bytes

    /// 买卖方向
    /// 1 = Buy（买入）
    /// 2 = Sell（卖出）
    pub side: u8,                           // 1 byte

    /// 订单类型
    /// 1 = Market（市价单）
    /// 2 = Limit（限价单）
    /// 3 = Stop（止损单）
    /// 4 = StopLimit（止损限价单）
    pub order_type: u8,                     // 1 byte

    /// 时效性（Time In Force）
    /// 1 = GTC（Good Till Cancel）
    /// 2 = IOC（Immediate Or Cancel）
    /// 3 = FOK（Fill Or Kill）
    /// 4 = GTD（Good Till Date）
    pub time_in_force: u8,                  // 1 byte

    /// 对齐填充
    pub _padding: u8,                       // 1 byte

    /// 订单数量（scaled integer）
    /// 例如：1.5 BTC = 150_000_000（精度10^8）
    pub quantity: u64,                      // 8 bytes

    /// 价格（scaled integer）
    /// 例如：50000.12 USD = 5_000_012_000_000（精度10^10）
    pub price: i64,                         // 8 bytes

    /// 止损价（仅Stop/StopLimit订单）
    pub stop_price: i64,                    // 8 bytes

    /// 最小成交数量（可选，0表示无限制）
    pub min_qty: u64,                       // 8 bytes

    /// 显示数量（冰山订单，0表示全部显示）
    pub display_qty: u64,                   // 8 bytes

    /// 自定义标签（策略ID、会话标识等）
    pub user_tag: u64,                      // 8 bytes

    /// 过期时间（UTC纳秒时间戳，仅GTD订单）
    pub expire_time: u64,                   // 8 bytes

    /// 订单属性位标志
    /// Bit 0: PostOnly（只做Maker）
    /// Bit 1: ReduceOnly（仅平仓）
    /// Bit 2: Close（平仓标志）
    /// Bit 3: Hidden（隐藏订单）
    /// Bit 4-7: Reserved
    pub flags: u8,                          // 1 byte

    /// 账户ID（多账户支持）
    pub account_id: u32,                    // 4 bytes

    /// 预留扩展字段
    pub _reserved: [u8; 18],                // 18 bytes
}

// 总大小: 40 + 152 = 192 bytes (3 * 64字节缓存行)
```

**字段详解**:

1. **cl_ord_id**: 客户端生成的唯一订单ID
   - 建议使用雪花算法或UUID
   - 用于追踪订单生命周期
   - 取消/修改订单时引用此ID

2. **symbol_id**: 预先映射的交易对ID
   - 避免字符串传输开销
   - 需要维护symbol_id ↔ 交易对符号映射表
   - 例如：1=BTCUSDT, 2=ETHUSDT

3. **quantity/price**: 定点数表示
   - 避免浮点数精度问题
   - scaled factor根据资产类别确定
   - 加密货币通常使用10^8（satoshi）

4. **flags**: 订单属性位标志
   - PostOnly: 确保订单进入订单簿（不立即成交）
   - ReduceOnly: 仅减少持仓，防止反向开仓
   - Hidden: 不显示在公开订单簿中

**使用示例**:

```rust
let order = NewOrderMessage {
    header: MessageHeader {
        magic: 0x524C544F,
        length: 192,
        msg_type: 0x10,
        version: 0x10,
        session_id: session.id,
        seq_num: session.next_seq(),
        timestamp: get_monotonic_nanos(),
        checksum: 0, // TCP模式
    },
    cl_ord_id: generate_order_id(),
    symbol_id: 1, // BTCUSDT
    side: 1,      // Buy
    order_type: 2, // Limit
    time_in_force: 1, // GTC
    _padding: 0,
    quantity: 150_000_000,    // 1.5 BTC
    price: 5_000_000_000_000, // 50000.00 USD
    stop_price: 0,
    min_qty: 0,
    display_qty: 0,
    user_tag: 0x1234,
    expire_time: 0,
    flags: 0b00000001, // PostOnly
    account_id: 1,
    _reserved: [0; 18],
};
```

### 2. CancelOrder（取消订单）

**消息类型**: `0x11`
**方向**: 客户端 → 服务器
**大小**: 128字节（2个缓存行）

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CancelOrderMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 原始订单的客户订单ID
    pub orig_cl_ord_id: u64,            // 8 bytes

    /// 新的客户订单ID（用于跟踪取消请求）
    pub cl_ord_id: u64,                 // 8 bytes

    /// 交易对ID（验证用）
    pub symbol_id: u32,                 // 4 bytes

    /// 账户ID
    pub account_id: u32,                // 4 bytes

    /// 预留扩展字段
    pub _reserved: [u8; 32],            // 32 bytes
}

// 总大小: 40 + 56 = 96 bytes → 填充至 128 bytes (2 * 64字节缓存行)
```

**字段说明**:
- `orig_cl_ord_id`: 要取消的订单ID（引用NewOrder的cl_ord_id）
- `cl_ord_id`: 本次取消请求的ID（用于追踪取消操作）
- `symbol_id`: 交易对ID，用于验证和快速路由

**使用场景**:
```rust
// 取消之前提交的订单
let cancel = CancelOrderMessage {
    header: /* ... */,
    orig_cl_ord_id: original_order.cl_ord_id,
    cl_ord_id: generate_order_id(),
    symbol_id: original_order.symbol_id,
    account_id: 1,
    _reserved: [0; 32],
};
```

### 3. ReplaceOrder（修改订单）

**消息类型**: `0x12`
**方向**: 客户端 → 服务器
**大小**: 128字节（2个缓存行）

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ReplaceOrderMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 原始订单ID
    pub orig_cl_ord_id: u64,            // 8 bytes

    /// 新订单ID
    pub cl_ord_id: u64,                 // 8 bytes

    /// 交易对ID
    pub symbol_id: u32,                 // 4 bytes

    /// 修改标志位
    /// Bit 0: 修改价格
    /// Bit 1: 修改数量
    /// Bit 2: 修改显示数量
    /// Bit 3: 修改止损价
    /// Bit 4-7: Reserved
    pub modify_flags: u8,               // 1 byte

    pub _padding: [u8; 3],              // 3 bytes

    /// 新价格（如果Bit 0设置）
    pub new_price: i64,                 // 8 bytes

    /// 新数量（如果Bit 1设置）
    pub new_quantity: u64,              // 8 bytes

    /// 新显示数量（如果Bit 2设置）
    pub new_display_qty: u64,           // 8 bytes

    /// 新止损价（如果Bit 3设置）
    pub new_stop_price: i64,            // 8 bytes

    /// 账户ID
    pub account_id: u32,                // 4 bytes

    /// 预留字段
    pub _reserved: [u8; 12],            // 12 bytes
}

// 总大小: 40 + 88 = 128 bytes (2 * 64字节缓存行)
```

**修改语义**:
- 价格/数量优先级：修改订单会失去原有队列位置
- 仅数量减少：部分交易所保持队列优先级
- modify_flags指示哪些字段有效

**使用示例**:
```rust
// 只修改价格
let replace = ReplaceOrderMessage {
    header: /* ... */,
    orig_cl_ord_id: order.cl_ord_id,
    cl_ord_id: generate_order_id(),
    symbol_id: order.symbol_id,
    modify_flags: 0b00000001, // 仅修改价格
    _padding: [0; 3],
    new_price: 4_900_000_000_000, // 49000.00
    new_quantity: 0, // 忽略
    new_display_qty: 0,
    new_stop_price: 0,
    account_id: 1,
    _reserved: [0; 12],
};
```

### 4. MassCancelOrder（批量取消）

**消息类型**: `0x13`
**方向**: 客户端 → 服务器
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MassCancelOrderMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 批量取消请求ID
    pub cl_ord_id: u64,                 // 8 bytes

    /// 取消范围
    /// 0 = 所有订单
    /// 1 = 指定交易对
    /// 2 = 指定买卖方向
    /// 3 = 指定交易对+方向
    pub mass_cancel_type: u8,           // 1 byte

    /// 买卖方向（type=2,3时有效）
    pub side: u8,                       // 1 byte

    pub _padding: [u8; 2],              // 2 bytes

    /// 交易对ID（type=1,3时有效）
    pub symbol_id: u32,                 // 4 bytes

    /// 账户ID
    pub account_id: u32,                // 4 bytes

    /// 预留字段
    pub _reserved: [u8; 64],            // 64 bytes
}

// 总大小: 128 bytes
```

**使用场景**:
```rust
// 取消BTCUSDT的所有买单
let mass_cancel = MassCancelOrderMessage {
    header: /* ... */,
    cl_ord_id: generate_order_id(),
    mass_cancel_type: 3, // 交易对+方向
    side: 1,             // Buy
    _padding: [0; 2],
    symbol_id: 1,        // BTCUSDT
    account_id: 1,
    _reserved: [0; 64],
};
```

---

## 响应消息

### 1. OrderAccepted（订单接受）

**消息类型**: `0x30`
**方向**: 服务器 → 客户端
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct OrderAcceptedMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 客户订单ID
    pub cl_ord_id: u64,                 // 8 bytes

    /// 交易所订单ID（服务器生成）
    pub order_id: u64,                  // 8 bytes

    /// 交易对ID
    pub symbol_id: u32,                 // 4 bytes

    /// 订单状态
    /// 1 = New（新订单）
    /// 2 = PartiallyFilled（部分成交）
    /// 3 = Filled（完全成交）
    /// 4 = Canceled（已取消）
    /// 5 = Rejected（已拒绝）
    pub order_status: u8,               // 1 byte

    pub _padding: [u8; 3],              // 3 bytes

    /// 接受时间戳（纳秒）
    pub accept_time: u64,               // 8 bytes

    /// 累计成交数量
    pub cum_qty: u64,                   // 8 bytes

    /// 剩余数量
    pub leaves_qty: u64,                // 8 bytes

    /// 预留字段
    pub _reserved: [u8; 40],            // 40 bytes
}

// 总大小: 128 bytes
```

**字段说明**:
- `order_id`: 服务器分配的全局唯一订单ID
- `accept_time`: 服务器接受时间，用于计算时延
- `cum_qty`: 已成交数量（部分成交时>0）
- `leaves_qty`: 剩余未成交数量

### 2. OrderRejected（订单拒绝）

**消息类型**: `0x31`
**方向**: 服务器 → 客户端
**大小**: 128字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct OrderRejectedMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 客户订单ID
    pub cl_ord_id: u64,                 // 8 bytes

    /// 交易对ID
    pub symbol_id: u32,                 // 4 bytes

    /// 拒绝原因代码（见RejectReason枚举）
    pub reject_reason: u16,             // 2 bytes

    pub _padding: [u8; 2],              // 2 bytes

    /// 拒绝文本（固定长度ASCII，截断填充）
    pub reject_text: [u8; 64],          // 64 bytes

    /// 预留字段
    pub _reserved: [u8; 4],             // 4 bytes
}

// 总大小: 128 bytes
```

**拒绝原因枚举**:
```rust
#[repr(u16)]
pub enum RejectReason {
    UnknownSymbol           = 1,    // 未知交易对
    ExchangeClosed          = 2,    // 交易所关闭
    OrderExceedsLimit       = 3,    // 订单超限
    DuplicateOrder          = 4,    // 重复订单ID
    InsufficientBalance     = 5,    // 余额不足
    InvalidPrice            = 6,    // 无效价格
    InvalidQuantity         = 7,    // 无效数量
    UnknownOrder            = 8,    // 未知订单（取消/修改）
    TooLateToCancel         = 9,    // 订单已成交
    RiskCheckFailed         = 10,   // 风控拒绝
    InvalidOrderType        = 11,   // 无效订单类型
    InvalidTimeInForce      = 12,   // 无效时效性
    PriceOutOfRange         = 13,   // 价格超出范围
    SelfTradePrevention     = 14,   // 自成交预防
    PostOnlyWouldCross      = 15,   // PostOnly会立即成交
}
```

### 3. ExecutionReport（成交回报）

**消息类型**: `0x50`
**方向**: 服务器 → 客户端
**大小**: 192字节

```rust
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ExecutionReportMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 客户订单ID
    pub cl_ord_id: u64,                 // 8 bytes

    /// 交易所订单ID
    pub order_id: u64,                  // 8 bytes

    /// 执行ID（唯一成交标识）
    pub exec_id: u64,                   // 8 bytes

    /// 交易对ID
    pub symbol_id: u32,                 // 4 bytes

    /// 买卖方向
    pub side: u8,                       // 1 byte

    /// 订单状态
    pub order_status: u8,               // 1 byte

    /// 执行类型
    /// 1 = New（新订单）
    /// 2 = Trade（成交）
    /// 3 = Canceled（取消）
    /// 4 = Replaced（修改）
    /// 5 = Rejected（拒绝）
    /// 6 = Expired（过期）
    pub exec_type: u8,                  // 1 byte

    pub _padding: u8,                   // 1 byte

    /// 本次成交价格
    pub last_px: i64,                   // 8 bytes

    /// 本次成交数量
    pub last_qty: u64,                  // 8 bytes

    /// 累计成交数量
    pub cum_qty: u64,                   // 8 bytes

    /// 剩余数量
    pub leaves_qty: u64,                // 8 bytes

    /// 平均成交价格
    pub avg_px: i64,                    // 8 bytes

    /// 手续费
    pub commission: i64,                // 8 bytes

    /// 手续费货币ID（1=USDT, 2=BTC等）
    pub commission_currency: u32,       // 4 bytes

    /// Liquidity标志
    /// 0 = None
    /// 1 = Maker（提供流动性）
    /// 2 = Taker（消耗流动性）
    pub liquidity_flag: u8,             // 1 byte

    pub _padding2: [u8; 3],             // 3 bytes

    /// 交易时间戳（撮合时间）
    pub transact_time: u64,             // 8 bytes

    /// 对手方订单ID（可选）
    pub contra_order_id: u64,           // 8 bytes

    /// 成交ID（交易所全局成交流水号）
    pub trade_id: u64,                  // 8 bytes

    /// 预留字段
    pub _reserved: [u8; 16],            // 16 bytes
}

// 总大小: 40 + 152 = 192 bytes (3 * 64字节缓存行)
```

**关键字段**:
- `exec_type`: 区分订单生命周期事件
  - New: 订单已进入订单簿
  - Trade: 发生成交
  - Canceled/Replaced: 订单状态变更
- `liquidity_flag`: Maker/Taker分类（影响手续费率）
- `transact_time`: 撮合引擎的成交时间戳

---

## 会话管理

### 1. Logon（登录）

**消息类型**: `0x02`
**方向**: 客户端 → 服务器
**大小**: 128字节

```rust
#[repr(C)]
pub struct LogonMessage {
    pub header: MessageHeader,          // 40 bytes

    /// API Key的SHA256哈希
    pub api_key_hash: [u8; 32],         // 32 bytes

    /// HMAC-SHA256签名
    /// 签名内容: header(40) + api_key_hash(32)
    pub signature: [u8; 32],            // 32 bytes

    /// 心跳间隔（秒）
    pub heartbeat_interval: u32,        // 4 bytes

    /// 请求的会话模式
    /// 0 = Sync（TCP同步）
    /// 1 = Async（UDP异步）
    /// 2 = Recovery（断线重连）
    pub session_mode: u8,               // 1 byte

    /// 客户端版本
    pub client_version: u8,             // 1 byte

    pub _padding: [u8; 2],              // 2 bytes

    /// 预留字段
    pub _reserved: [u8; 12],            // 12 bytes
}

// 总大小: 128 bytes
```

**认证流程**:
1. 客户端生成时间戳`timestamp`
2. 计算`api_key_hash = SHA256(api_key)`
3. 计算签名：
   ```rust
   let msg_bytes = concat(header, api_key_hash);
   let signature = HMAC_SHA256(secret_key, msg_bytes);
   ```
4. 发送Logon消息
5. 服务器验证签名，分配`session_id`

**安全考虑**:
- API Key永不明文传输
- 使用HMAC防止重放攻击（timestamp在header中）
- 签名验证使用常量时间比较（防止时序攻击）

### 2. Heartbeat（心跳）

**消息类型**: `0x01`
**方向**: 双向
**大小**: 128字节

```rust
#[repr(C, packed)]
pub struct HeartbeatMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 测试请求ID（如果是响应TestRequest）
    pub test_req_id: u64,               // 8 bytes

    /// 预留字段
    pub _reserved: [u8; 76],            // 76 bytes
}

// 总大小: 128 bytes
```

**心跳机制**:
- 客户端和服务器定期发送心跳（间隔由Logon指定）
- 超时2倍心跳间隔未收到心跳，断开连接
- TestRequest可主动要求对方发送心跳

### 3. Logout（登出）

**消息类型**: `0x03`
**方向**: 双向
**大小**: 128字节

```rust
#[repr(C, packed)]
pub struct LogoutMessage {
    pub header: MessageHeader,          // 40 bytes

    /// 登出原因
    /// 0 = UserRequested（用户请求）
    /// 1 = Timeout（心跳超时）
    /// 2 = SequenceGap（序列号缺失）
    /// 3 = SystemShutdown（系统关闭）
    pub logout_reason: u8,              // 1 byte

    pub _padding: [u8; 3],              // 3 bytes

    /// 登出文本
    pub logout_text: [u8; 64],          // 64 bytes

    /// 预留字段
    pub _reserved: [u8; 16],            // 16 bytes
}

// 总大小: 128 bytes
```

---

## 错误处理

### 序列号管理

**序列号规则**:
- 每个会话从1开始递增
- 发送方单调递增
- 接收方检测缺失（gap detection）
- 缺失时发送ResendRequest

**ResendRequest消息**:
```rust
#[repr(C, packed)]
pub struct ResendRequestMessage {
    pub header: MessageHeader,          // 40 bytes
    pub begin_seq_num: u64,             // 8 bytes (起始序列号)
    pub end_seq_num: u64,               // 8 bytes (结束序列号，0=最新)
    pub _reserved: [u8; 72],            // 72 bytes
}
```

### 错误恢复

**连接断开后重连**:
1. 发送Logon（session_mode=Recovery）
2. 携带last_seq_num（上次收到的序列号）
3. 服务器重发缺失消息
4. 恢复正常交易

**最佳实践**:
- 本地持久化订单状态
- 断线后使用OrderStatusRequest查询
- 关键消息记录日志

---

## 安全性

### 1. 认证机制

**HMAC-SHA256签名**:
```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn generate_signature(secret: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(secret).unwrap();
    mac.update(msg);
    let result = mac.finalize();
    result.into_bytes().into()
}

pub fn verify_logon(logon: &LogonMessage, secret: &[u8]) -> bool {
    let msg_bytes = &logon.as_bytes()[..72]; // header + api_key_hash
    let expected_sig = generate_signature(secret, msg_bytes);

    // 常量时间比较防止时序攻击
    use subtle::ConstantTimeEq;
    logon.signature.ct_eq(&expected_sig).into()
}
```

### 2. 完整性校验（UDP模式）

**CRC32校验和**:
```rust
use crc32fast::Hasher;

impl MessageHeader {
    pub fn calculate_checksum(&mut self, payload: &[u8]) {
        let mut hasher = Hasher::new();

        // 排除checksum字段本身
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>() - 4,
            )
        };

        hasher.update(header_bytes);
        hasher.update(payload);
        self.checksum = hasher.finalize();
    }

    pub fn verify_checksum(&self, payload: &[u8]) -> bool {
        let mut temp = *self;
        let expected = self.checksum;
        temp.checksum = 0;
        temp.calculate_checksum(payload);
        temp.checksum == expected
    }
}
```

### 3. 防重放攻击

- 时间戳检查：拒绝timestamp偏离当前时间>5秒的消息
- Nonce机制：Logon消息包含随机数
- 序列号：严格递增，检测重放

---

## 实现指南

### 客户端实现

**基本流程**:
```rust
use std::net::TcpStream;
use std::io::{Read, Write};

pub struct TradingClient {
    stream: TcpStream,
    session_id: u64,
    seq_num: u64,
}

impl TradingClient {
    pub fn connect(addr: &str, api_key: &str, secret: &str) -> Result<Self> {
        let mut stream = TcpStream::connect(addr)?;

        // 配置TCP参数
        configure_tcp_socket(&stream)?;

        // 发送Logon
        let logon = create_logon_message(api_key, secret);
        stream.write_all(logon.as_bytes())?;

        // 接收Logon响应
        let mut buf = [0u8; 128];
        stream.read_exact(&mut buf)?;
        let response = unsafe { LogonMessage::from_bytes(&buf) };

        Ok(Self {
            stream,
            session_id: response.header.session_id,
            seq_num: 1,
        })
    }

    pub fn send_order(&mut self, order: NewOrderMessage) -> Result<()> {
        let mut msg = order;
        msg.header.session_id = self.session_id;
        msg.header.seq_num = self.seq_num;
        msg.header.timestamp = get_monotonic_nanos();
        self.seq_num += 1;

        self.stream.write_all(msg.as_bytes())?;
        Ok(())
    }

    pub fn recv_message(&mut self) -> Result<Box<dyn RltopMessage>> {
        // 读取消息头
        let mut header_buf = [0u8; 40];
        self.stream.read_exact(&mut header_buf)?;
        let header = unsafe { MessageHeader::from_bytes(&header_buf) };

        // 根据长度读取完整消息
        let mut msg_buf = vec![0u8; header.length as usize];
        msg_buf[..40].copy_from_slice(&header_buf);
        self.stream.read_exact(&mut msg_buf[40..])?;

        // 解析消息类型
        parse_message(&msg_buf)
    }
}
```

### 服务器实现

**网关架构**:
```rust
pub struct TradingGateway {
    sessions: DashMap<u64, Session>,
    order_router: Arc<OrderRouter>,
    risk_engine: Arc<RiskEngine>,
}

impl TradingGateway {
    pub async fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        loop {
            // 接收消息
            let msg = recv_message(&mut stream).await?;

            // 分发处理
            match msg.msg_type() {
                0x02 => self.handle_logon(msg, &stream).await?,
                0x10 => self.handle_new_order(msg).await?,
                0x11 => self.handle_cancel_order(msg).await?,
                0x12 => self.handle_replace_order(msg).await?,
                _ => return Err("Unknown message type"),
            }
        }
    }

    async fn handle_new_order(&self, msg: NewOrderMessage) -> Result<()> {
        // 1. 验证会话
        let session = self.sessions.get(&msg.header.session_id)
            .ok_or("Invalid session")?;

        // 2. 检查序列号
        session.validate_sequence(msg.header.seq_num)?;

        // 3. 风控检查
        self.risk_engine.check_order(&msg).await?;

        // 4. 路由到匹配引擎
        self.order_router.route_order(msg).await?;

        Ok(())
    }
}
```

---

## 性能优化

### 1. 零拷贝网络

**TCP零拷贝**:
```rust
use std::os::unix::io::AsRawFd;

pub fn configure_tcp_socket(stream: &TcpStream) -> Result<()> {
    let fd = stream.as_raw_fd();

    unsafe {
        let flag: libc::c_int = 1;

        // 禁用Nagle算法
        libc::setsockopt(
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_NODELAY,
            &flag as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        );

        // 启用零拷贝（Linux 4.14+）
        libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_ZEROCOPY,
            &flag as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        );
    }

    Ok(())
}
```

### 2. 批量发送

**订单批处理**:
```rust
pub struct OrderBatch {
    orders: Vec<NewOrderMessage>,
    buffer: Vec<u8>,
}

impl OrderBatch {
    pub fn add_order(&mut self, order: NewOrderMessage) {
        self.orders.push(order);
    }

    pub fn flush(&mut self, stream: &mut TcpStream) -> Result<()> {
        // 序列化所有订单到缓冲区
        self.buffer.clear();
        for order in &self.orders {
            self.buffer.extend_from_slice(order.as_bytes());
        }

        // 批量发送
        stream.write_all(&self.buffer)?;
        self.orders.clear();
        Ok(())
    }
}
```

### 3. 内存池

**消息预分配**:
```rust
use object_pool::Pool;

pub struct MessagePool {
    new_order_pool: Pool<NewOrderMessage>,
}

impl MessagePool {
    pub fn new(capacity: usize) -> Self {
        Self {
            new_order_pool: Pool::new(capacity, || unsafe {
                std::mem::zeroed()
            }),
        }
    }

    pub fn alloc_new_order(&self) -> object_pool::Reusable<NewOrderMessage> {
        self.new_order_pool.pull(|| unsafe { std::mem::zeroed() })
    }
}
```

### 4. 性能基准

**延迟测量**:
```rust
use std::time::Instant;

pub fn benchmark_order_submission() {
    let mut client = TradingClient::connect("localhost:8080", "key", "secret").unwrap();

    let iterations = 10000;
    let mut latencies = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let order = create_test_order();

        let start = Instant::now();
        client.send_order(order).unwrap();
        let response = client.recv_message().unwrap();
        let end = Instant::now();

        latencies.push(end.duration_since(start).as_nanos());
    }

    // 计算百分位数
    latencies.sort();
    println!("P50: {} ns", latencies[iterations / 2]);
    println!("P95: {} ns", latencies[iterations * 95 / 100]);
    println!("P99: {} ns", latencies[iterations * 99 / 100]);
}
```

---

## 附录

### A. 完整消息类型列表

| 消息类型 | 代码 | 方向 | 大小 | 描述 |
|---------|------|------|------|------|
| Heartbeat | 0x01 | 双向 | 128B | 心跳 |
| Logon | 0x02 | C→S | 128B | 登录 |
| Logout | 0x03 | 双向 | 128B | 登出 |
| TestRequest | 0x04 | 双向 | 128B | 测试请求 |
| ResendRequest | 0x05 | 双向 | 128B | 重传请求 |
| NewOrder | 0x10 | C→S | 192B | 新订单 |
| CancelOrder | 0x11 | C→S | 128B | 取消订单 |
| ReplaceOrder | 0x12 | C→S | 128B | 修改订单 |
| MassCancelOrder | 0x13 | C→S | 128B | 批量取消 |
| OrderStatusRequest | 0x14 | C→S | 128B | 状态查询 |
| OrderAccepted | 0x30 | S→C | 128B | 订单接受 |
| OrderRejected | 0x31 | S→C | 128B | 订单拒绝 |
| OrderCanceled | 0x32 | S→C | 128B | 订单取消 |
| OrderReplaced | 0x33 | S→C | 128B | 订单修改 |
| CancelRejected | 0x34 | S→C | 128B | 取消拒绝 |
| ReplaceRejected | 0x35 | S→C | 128B | 修改拒绝 |
| ExecutionReport | 0x50 | S→C | 192B | 成交回报 |
| TradeReport | 0x51 | S→C | 192B | 交易报告 |
| TradeCancelReport | 0x52 | S→C | 192B | 交易取消 |
| PositionReport | 0x53 | S→C | 192B | 持仓报告 |

### B. 错误代码表

见[错误处理](#错误处理)章节的RejectReason枚举。

### C. 价格精度映射

| 资产类别 | 精度 | Scaled因子 | 示例 |
|---------|------|-----------|------|
| 加密货币 | 8位小数 | 10^8 | 1.5 BTC = 150000000 |
| 外汇 | 5位小数 | 10^5 | 1.23456 EUR/USD = 123456 |
| 股票 | 2位小数 | 10^2 | 123.45 USD = 12345 |
| 期货 | 2位小数 | 10^2 | 5000.50 = 500050 |

---

**文档结束**

**版本历史**:
- v1.0.0 (2025-12-06): 初始版本，从RLTOP完整协议中拆分交易指令部分
- 下一版本计划: 添加批量订单消息、条件单支持、仓位管理消息

**相关文档**:
- [RLTOP行情数据协议规范](market_data_proto.md)
- [RLTOP协议完整设计](command_proto.md)
