# LOB 快照功能使用指南

## 概述

`SymbolLob` trait 现在支持快照能力，允许您保存和恢复 LOB（Limit Order Book）的完整状态。这对于以下场景非常有用：

- **事件溯源（Event Sourcing）**: 定期创建快照，减少事件回放时间
- **持久化存储**: 将 LOB 状态保存到磁盘或数据库
- **灾难恢复**: 从快照快速恢复系统状态
- **时间旅行调试**: 保存特定时间点的状态用于问题排查
- **分布式系统同步**: 在多个节点间同步 LOB 状态

## 核心类型

### LobSnapshot

LOB 快照数据结构，包含以下信息：

```rust
pub struct LobSnapshot {
    /// 交易对符号
    pub symbol: Symbol,
    /// 快照时间戳（纳秒）
    pub timestamp: u64,
    /// 快照序列号
    pub sequence: u64,
    /// 序列化的 LOB 状态数据
    pub data: Vec<u8>,
    /// 最佳买价（快照时）
    pub best_bid: Option<Price>,
    /// 最佳卖价（快照时）
    pub best_ask: Option<Price>,
    /// 最后成交价（快照时）
    pub last_price: Option<Price>,
}
```

### SymbolLob Trait 方法

#### create_snapshot

创建 LOB 快照：

```rust
fn create_snapshot(&self, timestamp: u64, sequence: u64) -> Result<LobSnapshot, RepoError>
```

**参数**:
- `timestamp`: 快照时间戳（纳秒）
- `sequence`: 快照序列号（用于排序和版本控制）

**返回**:
- `Ok(LobSnapshot)`: 成功创建快照
- `Err(RepoError::SnapshotNotSupported)`: 默认实现，表示不支持快照
- `Err(RepoError::SerializationFailed)`: 序列化失败

#### restore_from_snapshot

从快照恢复 LOB 状态：

```rust
fn restore_from_snapshot(&mut self, snapshot: &LobSnapshot) -> Result<(), RepoError>
```

**参数**:
- `snapshot`: LOB 快照数据

**返回**:
- `Ok(())`: 成功恢复状态
- `Err(RepoError::SnapshotNotSupported)`: 默认实现，表示不支持快照
- `Err(RepoError::DeserializationFailed)`: 反序列化失败
- `Err(RepoError::SymbolMismatch)`: 快照的交易对与当前 LOB 不匹配

## 使用示例

### 1. 基础使用

```rust
use base_types::{OrderId, Price, Symbol};
use lob_repo::core::symbol_lob_repo::{LobSnapshot, SymbolLob};

// 假设您有一个实现了 SymbolLob trait 并支持快照的 LOB
let mut lob = MyLob::new(Symbol::new("BTCUSDT"));

// 添加订单...
lob.add_order(order1)?;
lob.add_order(order2)?;

// 创建快照
let timestamp = current_nanos();
let sequence = 1;
let snapshot = lob.create_snapshot(timestamp, sequence)?;

// 保存快照到磁盘
save_to_disk(&snapshot)?;

// 从快照恢复
let loaded_snapshot = load_from_disk()?;
lob.restore_from_snapshot(&loaded_snapshot)?;
```

### 2. 实现快照支持

要为您的 LOB 实现类添加快照支持，需要：

#### 步骤 1: 为订单类型添加序列化支持

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MyOrder {
    order_id: OrderId,
    symbol: Symbol,
    side: Side,
    price: Price,
    quantity: Quantity,
    filled_quantity: Quantity,
}

impl Order for MyOrder {
    fn order_id(&self) -> OrderId { self.order_id }
    fn price(&self) -> Price { self.price }
    fn quantity(&self) -> Quantity { self.quantity }
    fn filled_quantity(&self) -> Quantity { self.filled_quantity }
    fn side(&self) -> Side { self.side }
    fn symbol(&self) -> Symbol { self.symbol }
}
```

#### 步骤 2: 实现 create_snapshot 方法

```rust
impl SymbolLob<MyOrder> for MyLob {
    // ... 其他方法实现 ...

    fn create_snapshot(&self, timestamp: u64, sequence: u64) -> Result<LobSnapshot, RepoError> {
        use serde::Serialize;

        // 定义快照数据结构
        #[derive(Serialize)]
        struct SnapshotData {
            orders: Vec<MyOrder>,
            // 其他需要保存的状态...
        }

        // 收集所有订单
        let orders: Vec<MyOrder> = self.collect_all_orders();

        let snapshot_data = SnapshotData { orders };

        // 序列化
        let data = bincode::serialize(&snapshot_data)
            .map_err(|e| RepoError::SerializationFailed(e.to_string()))?;

        // 创建快照
        Ok(LobSnapshot::new(
            self.symbol,
            timestamp,
            sequence,
            data,
            self.best_bid(),
            self.best_ask(),
            self.last_price(),
        ))
    }

    fn restore_from_snapshot(&mut self, snapshot: &LobSnapshot) -> Result<(), RepoError> {
        use serde::Deserialize;

        // 验证交易对匹配
        if snapshot.symbol != self.symbol {
            return Err(RepoError::SymbolMismatch {
                expected: self.symbol.to_string(),
                actual: snapshot.symbol.to_string(),
            });
        }

        #[derive(Deserialize)]
        struct SnapshotData {
            orders: Vec<MyOrder>,
        }

        // 反序列化
        let snapshot_data: SnapshotData = bincode::deserialize(&snapshot.data)
            .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

        // 清空当前状态
        self.clear();

        // 恢复订单
        for order in snapshot_data.orders {
            self.add_order(order)?;
        }

        // 恢复市场数据
        self.bid_max = snapshot.best_bid;
        self.ask_min = snapshot.best_ask;
        self.last_trade_price = snapshot.last_price;

        Ok(())
    }
}
```

### 3. 事件溯源模式

结合事件溯源使用快照：

```rust
struct EventStore {
    snapshots: HashMap<u64, LobSnapshot>,
    events: Vec<OrderEvent>,
}

impl EventStore {
    /// 定期创建快照（例如每 100 个事件）
    fn maybe_snapshot(&mut self, lob: &impl SymbolLob<MyOrder>) -> Result<(), RepoError> {
        const SNAPSHOT_INTERVAL: u64 = 100;

        if self.events.len() % SNAPSHOT_INTERVAL as usize == 0 {
            let sequence = self.events.len() as u64;
            let snapshot = lob.create_snapshot(current_nanos(), sequence)?;
            self.snapshots.insert(sequence, snapshot);
        }

        Ok(())
    }

    /// 重建 LOB 状态（快照 + 增量事件）
    fn rebuild(&self, lob: &mut impl SymbolLob<MyOrder>, up_to_sequence: u64) -> Result<(), RepoError> {
        // 1. 找到最近的快照
        let snapshot_seq = self.snapshots
            .keys()
            .filter(|&&seq| seq <= up_to_sequence)
            .max()
            .copied();

        // 2. 从快照恢复
        if let Some(seq) = snapshot_seq {
            let snapshot = &self.snapshots[&seq];
            lob.restore_from_snapshot(snapshot)?;

            // 3. 应用快照之后的增量事件
            for event in &self.events {
                if event.sequence > seq && event.sequence <= up_to_sequence {
                    self.apply_event(lob, event)?;
                }
            }
        } else {
            // 没有快照，从头回放所有事件
            for event in &self.events {
                if event.sequence <= up_to_sequence {
                    self.apply_event(lob, event)?;
                }
            }
        }

        Ok(())
    }
}
```

### 4. 持久化到磁盘

```rust
use std::fs::File;
use std::io::{Read, Write};

/// 保存快照到文件
fn save_snapshot_to_file(snapshot: &LobSnapshot, path: &str) -> Result<(), std::io::Error> {
    let serialized = bincode::serialize(snapshot)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let mut file = File::create(path)?;
    file.write_all(&serialized)?;
    Ok(())
}

/// 从文件加载快照
fn load_snapshot_from_file(path: &str) -> Result<LobSnapshot, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    bincode::deserialize(&buffer)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}

// 使用示例
let snapshot = lob.create_snapshot(current_nanos(), 1)?;
save_snapshot_to_file(&snapshot, "btcusdt_snapshot.bin")?;

let restored_snapshot = load_snapshot_from_file("btcusdt_snapshot.bin")?;
lob.restore_from_snapshot(&restored_snapshot)?;
```

### 5. 分布式系统同步

```rust
/// 在节点间同步 LOB 状态
async fn sync_lob_state(
    source_lob: &impl SymbolLob<MyOrder>,
    target_node: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建快照
    let snapshot = source_lob.create_snapshot(current_nanos(), 1)?;

    // 通过网络发送快照
    send_snapshot_to_node(target_node, &snapshot).await?;

    Ok(())
}

async fn receive_lob_state(
    lob: &mut impl SymbolLob<MyOrder>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 接收快照
    let snapshot = receive_snapshot().await?;

    // 恢复状态
    lob.restore_from_snapshot(&snapshot)?;

    Ok(())
}
```

## 性能考虑

### 快照频率

- **高频快照**: 恢复速度快，但存储开销大
  - 适用于：实时交易系统，需要快速恢复
  - 建议：每 10-100 个事件创建一次快照

- **低频快照**: 存储开销小，但恢复速度慢
  - 适用于：历史数据存储，不常恢复
  - 建议：每 1000-10000 个事件创建一次快照

### 序列化格式选择

| 格式 | 性能 | 大小 | 跨语言 | 推荐场景 |
|------|------|------|--------|----------|
| Bincode | ⭐⭐⭐⭐⭐ | 小 | ❌ | Rust 内部使用 |
| MessagePack | ⭐⭐⭐⭐ | 小 | ✅ | 跨语言系统 |
| JSON | ⭐⭐⭐ | 大 | ✅ | 调试和可读性 |
| Protobuf | ⭐⭐⭐⭐ | 小 | ✅ | 企业级系统 |

### 压缩

对于大型快照，建议使用压缩：

```rust
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

fn compress_snapshot(snapshot: &LobSnapshot) -> Result<Vec<u8>, std::io::Error> {
    let serialized = bincode::serialize(snapshot)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&serialized)?;
    encoder.finish()
}

fn decompress_snapshot(compressed: &[u8]) -> Result<LobSnapshot, std::io::Error> {
    let mut decoder = GzDecoder::new(compressed);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    bincode::deserialize(&buffer)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}
```

## 错误处理

### RepoError 变体

```rust
pub enum RepoError {
    /// 不支持快照功能（默认实现）
    SnapshotNotSupported,

    /// 序列化失败
    SerializationFailed(String),

    /// 反序列化失败
    DeserializationFailed(String),

    /// 交易对不匹配
    SymbolMismatch {
        expected: String,
        actual: String,
    },

    // ... 其他错误类型 ...
}
```

### 错误处理示例

```rust
match lob.create_snapshot(timestamp, sequence) {
    Ok(snapshot) => {
        println!("快照创建成功: 序列号 {}", snapshot.sequence);
        // 保存快照...
    }
    Err(RepoError::SnapshotNotSupported) => {
        eprintln!("警告: 此 LOB 实现不支持快照功能");
    }
    Err(RepoError::SerializationFailed(msg)) => {
        eprintln!("序列化失败: {}", msg);
    }
    Err(e) => {
        eprintln!("创建快照失败: {}", e);
    }
}
```

## 测试

完整的测试用例参见 `tests/snapshot_test.rs`：

```bash
# 运行快照测试
cargo test --package lob_repo --test snapshot_test

# 运行所有测试
cargo test --package lob_repo
```

## 最佳实践

1. **快照命名**: 使用有意义的命名规则
   ```
   {symbol}_{timestamp}_{sequence}.snapshot
   例如: BTCUSDT_1234567890_100.snapshot
   ```

2. **版本控制**: 在快照中包含版本信息，以支持向后兼容

3. **验证恢复**: 恢复后验证数据完整性
   ```rust
   lob.restore_from_snapshot(&snapshot)?;
   assert_eq!(lob.symbol(), snapshot.symbol);
   assert_eq!(lob.best_bid(), snapshot.best_bid);
   ```

4. **原子操作**: 使用事务保证快照创建和保存的原子性

5. **清理策略**: 定期清理旧快照
   ```rust
   // 只保留最近 N 个快照
   const MAX_SNAPSHOTS: usize = 10;
   if snapshots.len() > MAX_SNAPSHOTS {
       snapshots.sort_by_key(|s| s.sequence);
       snapshots.truncate(MAX_SNAPSHOTS);
   }
   ```

## 参考实现

- `LocalLobHashMap`: 参见 `lib/common/lob/src/adapter/local_lob_hashmap_impl.rs`
- 测试用例: `lib/common/lob/tests/snapshot_test.rs`
- 示例代码: `lib/common/lob/examples/snapshot_example.rs`（计划添加）

## 相关文档

- [Entity Traits 设计文档](../../diff/ENTITY_TRAITS.md)
- [Event Sourcing 指南](../../diff/EVENT_SOURCING.md)
- [LOB 仓储接口文档](./README.md)

## 版本历史

- v0.1.0 (2025-12-17): 初始版本，添加基础快照支持
