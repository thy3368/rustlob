# Diff 和 Replay 设计约束

## 核心原则

**Track 和 Replay 都必须要求 entity 具有 ID 和 timestamp，才能保证 replay 的正确性。**

## 为什么需要 Entity ID？

### 问题场景
```rust
// ❌ 错误：没有 entity_id，无法区分不同实体的变更
let mut order1 = Order { value: 100 };
let mut order2 = Order { value: 200 };

let entry = track_changes(&mut order1, |o| o.value = 150);

// 危险！无法确定这个 entry 是属于 order1 还是 order2
order2.replay(&entry)?;  // 可能错误地将 order1 的变更应用到 order2
```

### 正确设计
```rust
// ✅ 正确：有 entity_id，可以验证实体匹配
impl TrackableEntity for Order {
    type Id = u64;
    fn entity_id(&self) -> Self::Id { self.id }
    // ...
}

let mut order1 = Order { id: 1, value: 100 };
let mut order2 = Order { id: 2, value: 200 };

let entry = track_auto(&mut order1, |o| o.value = 150);

// 安全：replay 前会检查 entity_id
order1.replay(&entry)?;  // ✅ 成功，entity_id 匹配
order2.replay(&entry)?;  // ❌ 失败，entity_id 不匹配
```

### can_replay 验证逻辑

```rust
pub trait Replay: TrackableEntity {
    fn can_replay(&self, entry: &ChangeLogEntry) -> bool {
        // 验证 entity_id 和 entity_type 都匹配
        self.entity_id().to_string() == entry.entity_id
            && Self::entity_type() == entry.entity_type
    }

    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
        if !self.can_replay(entry) {
            return Err(format!(
                "Cannot replay: entity mismatch (expected {}:{:?}, got {}:{})",
                Self::entity_type(),
                self.entity_id(),
                entry.entity_type,
                entry.entity_id
            ));
        }
        // 应用变更...
    }
}
```

## 为什么需要 Timestamp？

### 问题场景：顺序错乱
```rust
// ❌ 没有 timestamp，无法确定变更顺序
let entry1 = ChangeLogEntry { /* price: 100 -> 150 */ };
let entry2 = ChangeLogEntry { /* price: 150 -> 200 */ };

// 如果顺序错乱回放
order.replay(&entry2)?;  // price = 200
order.replay(&entry1)?;  // price = 150 (错误！应该是 200)
```

### 正确设计
```rust
// ✅ 使用 timestamp 和 sequence 保证顺序
pub struct ChangeLogEntry {
    pub entity_id: String,
    pub entity_type: String,
    pub change_type: ChangeType,
    pub timestamp: u64,    // 纳秒时间戳
    pub sequence: u64,     // 序列号（解决时间戳冲突）
}

// 回放时按顺序排序
let mut logs = vec![entry1, entry2, entry3];
logs.sort_by_key(|e| (e.timestamp, e.sequence));

for entry in logs {
    entity.replay(&entry)?;
}
```

## 为什么需要 Sequence？

### 问题场景：时间戳冲突
```rust
// 高频交易系统中，可能在同一纳秒内发生多个变更
let entry1 = ChangeLogEntry { timestamp: 1000, /* ... */ };
let entry2 = ChangeLogEntry { timestamp: 1000, /* ... */ };  // 冲突！

// 无法确定哪个先发生
```

### 正确设计
```rust
// ✅ 使用 sequence 解决时间戳冲突
let entry1 = ChangeLogEntry {
    timestamp: 1000,
    sequence: 1,  // 先发生
    // ...
};

let entry2 = ChangeLogEntry {
    timestamp: 1000,
    sequence: 2,  // 后发生
    // ...
};

// 排序逻辑
logs.sort_by_key(|e| (e.timestamp, e.sequence));
```

## 设计约束总结

### TrackableEntity trait（必须实现）

```rust
pub trait TrackableEntity: Clone + Debug + Send + Sync {
    type Id: Debug + Clone + PartialEq + ToString;

    // 🔴 必需：提供实体唯一标识
    fn entity_id(&self) -> Self::Id;

    // 🔴 必需：提供实体类型名称
    fn entity_type() -> &'static str where Self: Sized;

    // 🔴 必需：序列化/反序列化
    fn to_bytes(&self) -> Result<Vec<u8>, String>;
    fn from_bytes(data: &[u8]) -> Result<Self, String> where Self: Sized;

    // 可选：快照方法（有默认实现）
    fn create_snapshot(&self, timestamp: u64, sequence: u64) -> Result<EntitySnapshot, String>;
    fn from_snapshot(snapshot: &EntitySnapshot) -> Result<Self, String> where Self: Sized;
}
```

### ChangeLogEntry（必需字段）

```rust
pub struct ChangeLogEntry {
    pub entity_id: String,        // 🔴 必需：标识哪个实体
    pub entity_type: String,      // 🔴 必需：标识实体类型
    pub change_type: ChangeType,  // 🔴 必需：变更内容
    pub timestamp: u64,           // 🔴 必需：变更时间（纳秒）
    pub sequence: u64,            // 🔴 必需：序列号（解决冲突）
}
```

### EntitySnapshot（必需字段）

```rust
pub struct EntitySnapshot {
    pub entity_id: String,      // 🔴 必需：标识哪个实体
    pub entity_type: String,    // 🔴 必需：标识实体类型
    pub timestamp: u64,         // 🔴 必需：快照时间
    pub sequence: u64,          // 🔴 必需：快照序列号
    pub data: Vec<u8>,          // 🔴 必需：序列化数据
}
```

## Event Sourcing 场景

### 完整的回放流程

```rust
struct EventStore {
    snapshots: HashMap<String, EntitySnapshot>,
    logs: Vec<ChangeLogEntry>,
}

impl EventStore {
    /// 重建实体到指定时间点
    fn rebuild<T: Trackable>(&self, entity_id: &str, up_to_sequence: u64)
        -> Result<T, String>
    {
        // 1. 加载最近的快照
        let snapshot = self.snapshots.get(entity_id)
            .ok_or("No snapshot found")?;

        // 🔴 验证：快照必须有 entity_id
        if snapshot.entity_id != entity_id {
            return Err("Snapshot entity_id mismatch".to_string());
        }

        let mut entity = T::from_snapshot(snapshot)?;

        // 2. 按 (timestamp, sequence) 排序日志
        let mut relevant_logs: Vec<_> = self.logs.iter()
            .filter(|e| e.entity_id == entity_id)
            .filter(|e| e.sequence > snapshot.sequence)
            .filter(|e| e.sequence <= up_to_sequence)
            .collect();

        relevant_logs.sort_by_key(|e| (e.timestamp, e.sequence));

        // 3. 顺序回放
        for entry in relevant_logs {
            // 🔴 验证：每次回放前检查 entity_id 匹配
            if !entity.can_replay(entry) {
                return Err(format!("Cannot replay entry for entity {}", entry.entity_id));
            }
            entity.replay(entry)?;
        }

        Ok(entity)
    }
}
```

## 安全保证

### 类型安全

```rust
// ✅ 编译期保证：ID 类型匹配
impl TrackableEntity for Order {
    type Id = u64;  // 强制 entity_id 返回 u64
    fn entity_id(&self) -> u64 { self.id }
}

impl TrackableEntity for User {
    type Id = String;  // 强制 entity_id 返回 String
    fn entity_id(&self) -> String { self.user_id.clone() }
}
```

### 运行时安全

```rust
// ✅ 运行时验证：entity_id 和 entity_type 都匹配
fn can_replay(&self, entry: &ChangeLogEntry) -> bool {
    self.entity_id().to_string() == entry.entity_id     // ID 匹配
        && Self::entity_type() == entry.entity_type     // 类型匹配
}
```

## 常见错误

### ❌ 错误 1：忽略 can_replay 检查

```rust
// ❌ 危险：直接应用变更，不检查实体匹配
impl Replay for Order {
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
        // 没有调用 can_replay 检查！
        match &entry.change_type {
            ChangeType::Updated { changed_fields } => {
                // 直接应用变更...
            }
            _ => {}
        }
        Ok(())
    }
}
```

### ✅ 正确 1：始终检查 can_replay

```rust
// ✅ 安全：先检查实体匹配
impl Replay for Order {
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
        if !self.can_replay(entry) {
            return Err(format!(
                "Entity mismatch: expected {}:{}, got {}:{}",
                Self::entity_type(), self.entity_id(),
                entry.entity_type, entry.entity_id
            ));
        }

        match &entry.change_type {
            ChangeType::Updated { changed_fields } => {
                // 安全地应用变更...
            }
            _ => {}
        }
        Ok(())
    }
}
```

### ❌ 错误 2：忽略时间戳排序

```rust
// ❌ 危险：按插入顺序回放
for entry in logs {
    entity.replay(entry)?;  // 可能顺序错乱
}
```

### ✅ 正确 2：按时间戳排序回放

```rust
// ✅ 安全：按 (timestamp, sequence) 排序
logs.sort_by_key(|e| (e.timestamp, e.sequence));

for entry in logs {
    entity.replay(entry)?;
}
```

### ❌ 错误 3：跨实体应用变更

```rust
// ❌ 危险：将 order1 的变更应用到 order2
let entry = track_auto(&mut order1, |o| o.price = 100);
order2.replay(&entry)?;  // 错误！
```

### ✅ 正确 3：验证实体 ID

```rust
// ✅ 安全：replay 会自动检查
let entry = track_auto(&mut order1, |o| o.price = 100);

order1.replay(&entry)?;  // ✅ 成功，ID 匹配
order2.replay(&entry)?;  // ❌ 失败，ID 不匹配，返回错误
```

## 最佳实践

### 1. 使用单调递增的序列号

```rust
struct SequenceGenerator {
    current: AtomicU64,
}

impl SequenceGenerator {
    fn next(&self) -> u64 {
        self.current.fetch_add(1, Ordering::SeqCst)
    }
}
```

### 2. 使用高精度时间戳

```rust
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64  // 纳秒级精度
}
```

### 3. 验证时间戳单调性

```rust
impl EventStore {
    fn append(&mut self, entry: ChangeLogEntry) -> Result<(), String> {
        if let Some(last) = self.logs.last() {
            // 验证时间戳单调递增（允许相等）
            if entry.timestamp < last.timestamp {
                return Err("Timestamp must be monotonic".to_string());
            }
            // 如果时间戳相等，验证序列号递增
            if entry.timestamp == last.timestamp && entry.sequence <= last.sequence {
                return Err("Sequence must be monotonic for same timestamp".to_string());
            }
        }
        self.logs.push(entry);
        Ok(())
    }
}
```

## 总结

### 设计约束必要性

| 约束 | 作用 | 后果（如果缺失） |
|------|------|----------------|
| **entity_id** | 标识实体 | 无法区分不同实体的变更 |
| **entity_type** | 验证类型 | 可能将错误类型的变更应用到实体 |
| **timestamp** | 保证顺序 | 变更可能乱序回放 |
| **sequence** | 解决冲突 | 同一时刻的变更无法排序 |

### 核心原则

1. **🔴 entity_id 是必需的** - 没有 ID 无法追踪和回放
2. **🔴 timestamp 是必需的** - 没有时间戳无法保证顺序
3. **🔴 sequence 是必需的** - 解决高频场景下的时间戳冲突
4. **🔴 类型验证是必需的** - 防止类型混淆导致的错误

### 安全保证

- ✅ **编译期安全**: 关联类型保证 ID 类型匹配
- ✅ **运行时安全**: can_replay() 验证实体和类型匹配
- ✅ **顺序安全**: (timestamp, sequence) 确保回放顺序
- ✅ **数据安全**: 序列化/反序列化保证数据完整性

**版本**: v1.0.0
**更新**: 2025-12-16
