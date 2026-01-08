# Vec订单簿 RTO=0 RPO=0 快速参考指南

一页纸的核心概念速查手册。

## 核心概念速览

```
┌──────────────────────────────────────┐
│ Vec订单簿架构 (LocalLob<O>)           │
├──────────────────────────────────────┤
│ 内存结构:                             │
│  • orders: Vec<Option<OrderNode>>    │  紧凑顺序存储
│  • bids/asks: Vec<PricePoint>        │  价格→订单映射
│  • order_index: HashMap              │  O(1)查询
│  • 链表: 同价格订单链 (时间优先)     │
└──────────────────────────────────────┘

两大关键接口:
├─ RepoSnapshot trait
│  ├─ create_snapshot() → 完整状态克隆
│  └─ restore_from_snapshot() → 状态恢复
│
└─ EventReplay trait
   ├─ replay_event() → 应用单个事件
   └─ replay_events() → 批量应用事件

恢复流程 (RTO ~1-2秒):
┌────────────┐ ┌──────────────┐ ┌────────────┐
│快照加载    │→│增量事件回放   │→│一致性验证  │
│(100ms)    │ │(<1s)         │ │(50ms)     │
└────────────┘ └──────────────┘ └────────────┘
```

## 文件位置速查

| 模块 | 文件 | 关键方法 |
|-----|-----|--------|
| 快照/回放接口 | `lib/common/lob/src/core/repo_snapshot_support.rs` | RepoSnapshot, EventReplay traits |
| Vec实现 | `lib/common/lob/src/adapter/local_lob_impl.rs` | LocalLob实现 |
| 事件类型 | `lib/common/diff/src/lib.rs` | ChangeLogEntry, ChangeType |

## 日常操作速查

### 创建快照
```rust
let snapshot = lob.create_snapshot(
    timestamp_ns,     // u64
    sequence_number   // u64
)?;

// 保存到磁盘
let serialized = bincode::serialize(&snapshot)?;
tokio::fs::write("snapshot.bin", serialized).await?;
```

### 从快照恢复
```rust
let snapshot_data = tokio::fs::read("snapshot.bin").await?;
let snapshot: LocalLob<O> = bincode::deserialize(&snapshot_data)?;

let mut lob = LocalLob::new(symbol);
lob.restore_from_snapshot(&snapshot)?;
```

### 回放事件
```rust
// 单个事件
lob.replay_event(&event)?;

// 批量事件
lob.replay_events(&events)?;

// 从特定序列号开始回放
lob.replay_from_sequence(&events, start_sequence)?;
```

### 持久化事件
```rust
// 写入WAL
wal_writer.write_event(event).await?;

// 批量fsync
wal_writer.flush().await?;

// 立即同步fsync
wal_writer.sync_immediate().await?;
```

## 数据结构速查

### 事件类型
```rust
pub struct ChangeLogEntry {
    entity_id: String,           // 订单ID
    change_type: ChangeType,     // Created|Updated|Deleted
    timestamp: u64,              // ns
    sequence: u64,               // 全局序列号
}

pub enum ChangeType {
    Created { /* 订单字段 */ },
    Updated { changed_fields: BTreeMap<String, (String, String)> },
    Deleted,
}
```

### 价格点结构
```rust
struct PricePoint {
    first_order_idx: Option<usize>,  // 链表头
    last_order_idx: Option<usize>,   // 链表尾
}

struct OrderNode<O> {
    order: O,
    next_idx: Option<usize>,  // 同价格下一个订单
}
```

## 性能基准速查

| 操作 | 耗时(1000订单) | 优化后 |
|-----|---------------|-------|
| 创建快照 | 50-100ms | 20-30ms |
| 快照反序列化 | 20-30ms | 8-12ms |
| 回放100事件 | 100-200ms | 5-10ms |
| 完整恢复 | 500-1000ms | 200-300ms |

## 常用模式速查

### 模式1: 定期快照
```rust
// 每5秒或1000个事件
let should_snapshot =
    elapsed_secs >= 5 ||
    event_count >= 1000;

if should_snapshot {
    let snapshot = lob.create_snapshot(now(), seq)?;
    snapshot_mgr.save_snapshot(&snapshot, now(), seq).await?;
}
```

### 模式2: 故障后恢复
```rust
// 1. 加载快照
let (snapshot, meta) = snapshot_mgr.load_latest_snapshot().await?;

// 2. 恢复到快照状态
lob.restore_from_snapshot(&snapshot)?;

// 3. 加载后续事件
let events = load_events(meta.sequence + 1)?;

// 4. 回放增量
lob.replay_from_sequence(&events, meta.sequence + 1)?;
```

### 模式3: 事件去重优化
```rust
// 同订单的多个Updated事件 → 仅保留最后一个
let mut latest_events = HashMap::new();
for event in events {
    latest_events.insert(event.entity_id.clone(), event);
}

for (_, event) in latest_events {
    lob.replay_event(&event)?;
}
```

### 模式4: 增量快照
```rust
pub struct IncrementalSnapshot {
    base_snapshot: LocalLob<O>,
    delta_events: Vec<ChangeLogEntry>,
}

fn compact(&mut self) -> Result<LocalLob<O>> {
    let mut result = self.base_snapshot.clone();
    result.replay_events(&self.delta_events)?;
    self.base_snapshot = result.clone();
    self.delta_events.clear();
    Ok(result)
}
```

## 故障恢复决策树

```
故障发生
  │
  ├─→ 快照可用？
  │    ├─→ 是 → 加载快照(100ms)
  │    │       └─→ 回放增量(<1s)
  │    │           └─→ RTO=1-2s ✓
  │    │
  │    └─→ 否 → 使用WAL恢复
  │           └─→ 全量回放(5-10s)
  │               └─→ RTO=5-10s (可接受)
  │
  └─→ 快照和WAL都不可用？
       └─→ 从备用节点恢复
           └─→ 可能丢失数据 (RPO>0)
               └─→ 需要人工干预
```

## 监控指标速查

```
关键指标:
├─ RTO (Recovery Time Objective)
│  └─ 目标: <2秒
│  └─ 告警: >2秒
│
├─ RPO (Recovery Point Objective)
│  └─ 目标: 0 (无丢失)
│  └─ 告警: >0
│
├─ 快照大小
│  └─ 监控: 内存占用增长
│  └─ 告警: >100MB
│
├─ 事件缓冲区大小
│  └─ 监控: 待fsync事件数
│  └─ 告警: >10000
│
└─ 恢复频率
   └─ 监控: 恢复操作次数/小时
   └─ 告警: >1次/小时
```

## 序列号管理速查

```
事件序列号特性:
├─ 全局唯一且单调递增: 1, 2, 3, 4, ...
├─ 无缝隙: 不能跳过序列号
├─ 幂等性: 重复回放相同序列号无害

快照序列号含义:
├─ 快照时刻: Seq_snap = 100
├─ 快照包含事件: 1-100
├─ 回放开始: Seq >= 101

查询方法:
├─ 获取当前序列号: wal_writer.current_sequence()
├─ 快照序列号: snapshot_metadata.sequence
└─ 回放起点: snapshot_seq + 1
```

## 配置速查

### WAL配置
```rust
WalConfig {
    batch_threshold: 100,      // 100个事件后fsync
    batch_timeout_ms: 100,     // 100ms后fsync
    max_buffer_size: 10000,    // 最多缓存10000个事件
}
```

### 快照配置
```rust
SnapshotConfig {
    prefix: "lob_repo".to_string(),     // 文件前缀
    enable_compression: true,       // 启用LZ4压缩
    retention_count: 10,            // 保留10个快照
}
```

### 恢复配置
```rust
RecoveryConfig {
    auto_recovery: true,            // 启用自动恢复
    recovery_timeout_secs: 30,      // 30秒超时
    validate_consistency: true,     // 验证一致性
}
```

## 错误处理速查

```rust
// 序列化错误
如果 bincode::serialize() 失败
→ 检查Order类型是否实现Serialize trait

// 反序列化错误
如果 bincode::deserialize() 失败
→ 检查快照版本兼容性
→ 检查Order类型是否实现Deserialize trait

// 事件回放失败
如果 replay_event() 返回错误
→ 检查事件序列完整性
→ 检查事件格式正确性
→ 检查订单是否存在

// 快照恢复失败
如果 restore_from_snapshot() 失败
→ 检查快照文件完整性
→ 检查校验和
→ 尝试备用快照
```

## 线程安全速查

```
Arc<Mutex<>> 保护:
├─ WAL Writer 的文件句柄
├─ 事件缓冲区
└─ 快照文件访问

LocalLob 本身:
├─ 不是Sync (无法跨线程共享)
└─ 通过Arc<Mutex<LocalLob>>共享

async/await 使用:
├─ tokio::fs 用于异步文件I/O
├─ tokio::sync::Mutex 用于异步互斥
└─ 避免阻塞tokio运行时
```

## 调试技巧速查

### 打印快照信息
```rust
println!("Snapshot: Seq={}, Size={}KB",
    snapshot_metadata.sequence,
    snapshot_metadata.size / 1024);
```

### 验证事件顺序
```rust
let mut prev_seq = 0;
for event in &events {
    assert!(event.sequence > prev_seq, "Sequence gap!");
    prev_seq = event.sequence;
}
```

### 追踪恢复流程
```rust
eprintln!("[Recovery] Phase 1: Loading snapshot...");
let (snapshot, meta) = snapshot_mgr.load_latest_snapshot().await?;
eprintln!("[Recovery] Snapshot seq={}, replayed {} events",
    meta.sequence, recovery_result.events_replayed);
```

### 检查一致性
```rust
// 订单总数
println!("Total orders: {}", lob.order_index.len());

// 最佳价格
println!("Best bid: {:?}, best ask: {:?}",
    lob.best_bid(), lob.best_ask());

// 订单详情
for (id, order) in &lob.order_index {
    println!("Order {}: qty={}", id, order.quantity());
}
```

## 参考文档

| 文档 | 用途 |
|-----|------|
| [VEC_LOB_SNAPSHOT_AND_REPLAY.md](./VEC_LOB_SNAPSHOT_AND_REPLAY.md) | 详细原理说明 |
| [VEC_LOB_IMPLEMENTATION_GUIDE.md](./VEC_LOB_IMPLEMENTATION_GUIDE.md) | 完整实现指南 |
| [local_lob_impl.rs](./src/adapter/local_lob_impl.rs) | 源代码实现 |
| [repo_snapshot_support.rs](./src/core/repo_snapshot_support.rs) | 接口定义 |

---

**快速参考版本**: v1.0
**最后更新**: 2025-12-18
