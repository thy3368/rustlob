# Entity Traits 快速参考

## 快速开始 (5分钟)

### 1. 实现三个 trait

```rust
use diff::diff_types::*;

#[derive(Debug, Clone, PartialEq)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
}

// ✅ Trait 1: TrackableEntity
impl TrackableEntity for Order {
    type Id = u64;
    fn entity_id(&self) -> Self::Id { self.id }
    fn entity_type() -> &'static str { "Order" }
    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(format!("{}:{}:{}", self.id, self.symbol, self.price).into_bytes())
    }
    fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let s = String::from_utf8(data.to_vec()).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = s.split(':').collect();
        Ok(Self {
            id: parts[0].parse().unwrap(),
            symbol: parts[1].to_string(),
            price: parts[2].parse().unwrap(),
        })
    }
}

// ✅ Trait 2: Diff
impl Diff for Order {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        let mut changes = Vec::new();
        if self.symbol != other.symbol {
            changes.push(FieldChange::new("symbol", &self.symbol, &other.symbol));
        }
        if self.price != other.price {
            changes.push(FieldChange::new("price", self.price.to_string(), other.price.to_string()));
        }
        changes
    }
}

// ✅ Trait 3: Replay
impl Replay for Order {
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
        if !self.can_replay(entry) {
            return Err("Entity mismatch".to_string());
        }
        match &entry.change_type {
            ChangeType::Updated { changed_fields } => {
                for field in changed_fields {
                    match field.field_name.as_str() {
                        "symbol" => self.symbol = field.new_value.clone(),
                        "price" => self.price = field.new_value.parse().unwrap(),
                        _ => {}
                    }
                }
                Ok(())
            }
            ChangeType::Deleted => Err("Cannot replay on deleted entity".to_string()),
            ChangeType::Created => Ok(()),
        }
    }
}

// 🎉 自动获得 Trackable trait！
```

### 2. 使用追踪功能

```rust
fn main() {
    let mut order = Order { id: 1, symbol: "BTC".to_string(), price: 50000.0 };
    let updated = Order { id: 1, symbol: "BTC".to_string(), price: 51000.0 };

    // 检测变更
    let changes = order.diff(&updated);
    println!("Changes: {:?}", changes);

    // 创建日志
    let entry = ChangeLogEntry::new(
        "1", "Order",
        ChangeType::Updated { changed_fields: changes },
        1000, 1
    );

    // 回放变更
    order.replay(&entry).unwrap();
    assert_eq!(order.price, 51000.0);

    // 创建快照
    let snapshot = order.create_snapshot(2000, 2).unwrap();

    // 恢复快照
    let restored = Order::from_snapshot(&snapshot).unwrap();
    assert_eq!(restored, order);
}
```

## API 速查

### TrackableEntity

| 方法 | 返回值 | 用途 |
|------|--------|------|
| `entity_id()` | `Self::Id` | 获取实体ID |
| `entity_type()` | `&'static str` | 获取实体类型 |
| `to_bytes()` | `Result<Vec<u8>, String>` | 序列化 |
| `from_bytes(data)` | `Result<Self, String>` | 反序列化 |
| `create_snapshot(ts, seq)` | `Result<EntitySnapshot, String>` | 创建快照 |
| `from_snapshot(snap)` | `Result<Self, String>` | 恢复快照 |

### Diff

| 方法 | 返回值 | 用途 |
|------|--------|------|
| `diff(other)` | `Vec<FieldChange>` | 比较状态 |
| `has_changes(other)` | `bool` | 是否有变更 |

### Replay

| 方法 | 返回值 | 用途 |
|------|--------|------|
| `replay(entry)` | `Result<(), String>` | 应用变更 |
| `can_replay(entry)` | `bool` | 检查是否可回放 |

## 常见模式

### 模式1: Event Store

```rust
struct EventStore {
    logs: Vec<ChangeLogEntry>,
}

impl EventStore {
    fn track<T: Trackable>(&mut self, old: &T, new: &T, seq: u64) {
        let changes = old.diff(new);
        if !changes.is_empty() {
            let entry = ChangeLogEntry::new(
                old.entity_id().to_string(),
                T::entity_type(),
                ChangeType::Updated { changed_fields: changes },
                current_timestamp(),
                seq,
            );
            self.logs.push(entry);
        }
    }

    fn rebuild<T: Trackable>(&self, initial: T) -> Result<T, String> {
        let mut entity = initial;
        for entry in &self.logs {
            if entity.can_replay(entry) {
                entity.replay(entry)?;
            }
        }
        Ok(entity)
    }
}
```

### 模式2: 乐观锁

```rust
struct Versioned<T> {
    entity: T,
    version: u64,
}

impl<T: Trackable> Versioned<T> {
    fn update(&mut self, new: T) -> Result<(), String> {
        if self.entity.entity_id() != new.entity_id() {
            return Err("ID mismatch".to_string());
        }
        if !self.entity.has_changes(&new) {
            return Ok(()); // 无变更
        }
        self.entity = new;
        self.version += 1;
        Ok(())
    }
}
```

### 模式3: 快照 + 增量

```rust
struct SnapshotStore<T: Trackable> {
    snapshot: Option<EntitySnapshot>,
    logs: Vec<ChangeLogEntry>,
}

impl<T: Trackable> SnapshotStore<T> {
    fn save(&mut self, entity: &T, seq: u64) -> Result<(), String> {
        if seq % 100 == 0 {
            // 每100个变更创建快照
            self.snapshot = Some(entity.create_snapshot(current_timestamp(), seq)?);
            self.logs.clear(); // 清理旧日志
        }
        Ok(())
    }

    fn load(&self) -> Result<T, String> {
        let snapshot = self.snapshot.as_ref()
            .ok_or("No snapshot")?;
        let mut entity = T::from_snapshot(snapshot)?;
        for log in &self.logs {
            entity.replay(log)?;
        }
        Ok(entity)
    }
}
```

## 性能提示

### ✅ 推荐做法

```rust
// 1. 使用 Bincode 序列化（性能最优）
use bincode;
fn to_bytes(&self) -> Result<Vec<u8>, String> {
    bincode::serialize(self).map_err(|e| e.to_string())
}

// 2. 预分配容量
let mut changes = Vec::with_capacity(expected_fields);

// 3. 使用 &str 而非 String
changes.push(FieldChange::new("field", old_str, new_str));

// 4. 批量回放
for entry in batch {
    entity.replay(entry)?;
}
```

### ❌ 避免做法

```rust
// ❌ 避免：频繁创建 String
for i in 0..1000 {
    let s = format!("field_{}", i); // 每次分配
}

// ✅ 改进：复用字符串
let mut buf = String::with_capacity(20);
for i in 0..1000 {
    buf.clear();
    use std::fmt::Write;
    write!(&mut buf, "field_{}", i).unwrap();
}

// ❌ 避免：嵌套序列化
fn to_bytes(&self) -> Result<Vec<u8>, String> {
    Ok(serde_json::to_string(self).unwrap().into_bytes()) // 两次转换
}

// ✅ 改进：直接序列化
fn to_bytes(&self) -> Result<Vec<u8>, String> {
    bincode::serialize(self).map_err(|e| e.to_string())
}
```

## 错误处理模板

```rust
impl Replay for MyEntity {
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
        // 1. 验证实体匹配
        if !self.can_replay(entry) {
            return Err(format!(
                "Entity mismatch: expected {}:{:?}, got {}:{}",
                Self::entity_type(), self.entity_id(),
                entry.entity_type, entry.entity_id
            ));
        }

        // 2. 模式匹配
        match &entry.change_type {
            ChangeType::Created => Ok(()),
            ChangeType::Updated { changed_fields } => {
                // 3. 应用变更
                for field in changed_fields {
                    self.apply_field_change(field)?;
                }
                Ok(())
            }
            ChangeType::Deleted => {
                Err("Cannot replay on deleted entity".to_string())
            }
        }
    }
}

// 辅助方法
impl MyEntity {
    fn apply_field_change(&mut self, field: &FieldChange) -> Result<(), String> {
        match field.field_name.as_str() {
            "field1" => {
                self.field1 = field.new_value.parse()
                    .map_err(|e| format!("Parse error for field1: {}", e))?;
            }
            "field2" => {
                self.field2 = field.new_value.clone();
            }
            unknown => {
                eprintln!("Warning: Unknown field '{}'", unknown);
            }
        }
        Ok(())
    }
}
```

## 测试模板

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_id() {
        let entity = create_entity();
        assert_eq!(entity.entity_id(), expected_id);
    }

    #[test]
    fn test_diff_no_change() {
        let e1 = create_entity();
        let e2 = e1.clone();
        assert!(e1.diff(&e2).is_empty());
    }

    #[test]
    fn test_diff_with_change() {
        let old = create_entity();
        let mut new = old.clone();
        new.field = new_value;

        let changes = old.diff(&new);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].field_name, "field");
    }

    #[test]
    fn test_snapshot_roundtrip() {
        let entity = create_entity();
        let snapshot = entity.create_snapshot(1000, 1).unwrap();
        let restored = Entity::from_snapshot(&snapshot).unwrap();
        assert_eq!(entity, restored);
    }

    #[test]
    fn test_replay() {
        let mut entity = create_entity();
        let entry = create_update_entry();
        entity.replay(&entry).unwrap();
        assert_eq!(entity.field, expected_value);
    }

    #[test]
    fn test_replay_wrong_entity() {
        let mut entity1 = create_entity_with_id(1);
        let entry = create_entry_for_id(2);
        assert!(entity1.replay(&entry).is_err());
    }
}
```

## 故障排查

### 问题: 序列化失败

```rust
// 症状: to_bytes() 返回错误

// 检查1: 确认所有字段都是可序列化的
#[derive(Serialize, Deserialize)] // 需要所有字段支持
struct Entity { /* ... */ }

// 检查2: 使用正确的序列化库
use bincode; // 或 serde_json

// 检查3: 添加错误信息
fn to_bytes(&self) -> Result<Vec<u8>, String> {
    bincode::serialize(self)
        .map_err(|e| format!("Serialization error: {}", e))
}
```

### 问题: 回放失败

```rust
// 症状: replay() 返回错误

// 检查1: 验证实体ID和类型
assert!(entity.can_replay(&entry));

// 检查2: 检查字段名是否匹配
match field.field_name.as_str() {
    "correct_name" => { /* ... */ },
    unknown => {
        eprintln!("Unknown field: {}", unknown);
        // 继续而不是返回错误（向前兼容）
    }
}

// 检查3: 处理解析错误
field.new_value.parse()
    .map_err(|e| format!("Parse error for {}: {}", field.field_name, e))?
```

### 问题: 性能问题

```rust
// 症状: diff() 太慢

// 优化1: 减少不必要的比较
if self.rarely_changed_field != other.rarely_changed_field {
    // 只在实际变化时处理
}

// 优化2: 使用位掩码追踪变更
struct Entity {
    data: Data,
    dirty_mask: u64, // 每位对应一个字段
}

// 优化3: 批量处理
let changes: Vec<_> = entities.par_iter()
    .filter_map(|e| {
        let new = updated_entities.get(&e.entity_id())?;
        let changes = e.diff(new);
        if changes.is_empty() { None } else { Some(changes) }
    })
    .collect();
```

## 进阶技巧

### 技巧1: 条件序列化

```rust
impl TrackableEntity for Entity {
    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        // 只序列化必要字段，减少快照大小
        let slim = SlimEntity {
            id: self.id,
            essential_field: self.essential_field,
            // 省略临时字段
        };
        bincode::serialize(&slim).map_err(|e| e.to_string())
    }
}
```

### 技巧2: 压缩快照

```rust
use flate2::write::GzEncoder;
use flate2::Compression;

fn to_bytes(&self) -> Result<Vec<u8>, String> {
    let data = bincode::serialize(self).map_err(|e| e.to_string())?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(&data).map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())
}
```

### 技巧3: 异步回放

```rust
#[async_trait]
trait AsyncReplay: TrackableEntity {
    async fn replay_async(&mut self, entry: &ChangeLogEntry) -> Result<(), String>;
}

impl AsyncReplay for Entity {
    async fn replay_async(&mut self, entry: &ChangeLogEntry) -> Result<(), String> {
        // 可以调用异步方法
        let external_data = fetch_from_db(entry.entity_id).await?;
        self.replay(entry)?;
        Ok(())
    }
}
```

## 相关资源

- 📖 完整文档: [ENTITY_TRAITS.md](./ENTITY_TRAITS.md)
- 🧪 测试示例: `lib/common/diff/src/diff/diff_types.rs` (tests 模块)
- 🎯 使用案例: [待添加]

**版本**: 1.0.0
**更新**: 2025-12-16
