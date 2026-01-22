# EntityManager - 字段级变更追踪

## 概览

EntityManager 实现了**字段级别**的变更追踪，不需要序列化整个实体状态，而是精确记录每个字段的变更信息。

## 核心数据结构

### FieldChange - 字段变更记录

```rust
#[derive(Debug, Clone)]
pub struct FieldChange {
    pub field_name: String,   // 字段名称
    pub old_value: String,    // 旧值（字符串形式）
    pub new_value: String,    // 新值（字符串形式）
}
```

### ChangeLogEntry - 变更日志条目

```rust
#[derive(Debug, Clone)]
pub struct ChangeLogEntry {
    pub entity_id: String,
    pub entity_type: String,
    pub change_type: ChangeType,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Created,
    Updated { changed_fields: Vec<FieldChange> },
    Deleted
}
```

## 使用示例

### 基本用法

```rust
use prep_proc::proc::repo::EntityManager::{EntityManager, FieldChange};

#[derive(Debug, Clone)]
struct User {
    id: String,
    name: String,
    age: i32,
    email: String,
}

let user = User {
    id: "user_001".to_string(),
    name: "Alice".to_string(),
    age: 25,
    email: "alice@example.com".to_string(),
};

let mut manager = EntityManager::new(user);

// 记录字段变更
let changes = vec![
    FieldChange {
        field_name: "name".to_string(),
        old_value: "Alice".to_string(),
        new_value: "Alice Smith".to_string(),
    },
    FieldChange {
        field_name: "age".to_string(),
        old_value: "25".to_string(),
        new_value: "26".to_string(),
    },
];

// 执行更新
let entry = manager.update(
    |user| {
        user.name = "Alice Smith".to_string();
        user.age = 26;
    },
    changes
).unwrap();
```

### 单字段更新

```rust
// 只更新一个字段
let changes = vec![
    FieldChange {
        field_name: "email".to_string(),
        old_value: "alice@example.com".to_string(),
        new_value: "alice.smith@example.com".to_string(),
    },
];

manager.update(
    |user| {
        user.email = "alice.smith@example.com".to_string();
    },
    changes
).unwrap();
```

## 测试结果

### 测试 1: 多字段变更追踪

```
=== ChangeLogEntry 字段变更验证 ===
✓ entity_id: example_id
✓ entity_type: prep_proc::proc::repo::EntityManager::tests::TestEntity
✓ 记录了 2 个字段变更
  ✓ 字段: value | 旧值: 100 → 新值: 150
  ✓ 字段: name | 旧值: Initial → 新值: Updated
✓ timestamp: 1765732119 (current: 1765732119)

=== 字段变更追踪验证通过! ===
```

**验证项**:
- ✅ 正确记录 2 个字段变更
- ✅ 字段名称准确
- ✅ 旧值和新值准确记录
- ✅ 时间戳正确

### 测试 2: 多次更新

```
=== 第一次更新 ===
✓ 变更字段数: 1
  - value: 50 → 75

=== 第二次更新 ===
✓ 变更字段数: 1
  - name: Original → Modified

=== 多次更新测试通过! ===
```

**验证项**:
- ✅ 每次更新独立记录
- ✅ 只记录实际变更的字段
- ✅ 支持连续多次更新

## 优势对比

### 🚫 旧方案：序列化整个状态

```rust
// 旧方案
pub struct ChangeLogEntry {
    pub old_state: Option<Vec<u8>>,  // 序列化整个对象
    pub new_state: Option<Vec<u8>>,  // 序列化整个对象
}
```

**缺点**:
- ❌ 存储空间大（每次 2 × 对象大小）
- ❌ 难以快速查看变更内容（需要反序列化）
- ❌ 无法直接知道哪些字段变更了
- ❌ 需要实体实现 Serialize trait

### ✅ 新方案：字段级追踪

```rust
// 新方案
pub struct ChangeLogEntry {
    pub change_type: ChangeType,  // 包含字段变更列表
}

pub struct FieldChange {
    pub field_name: String,
    pub old_value: String,
    pub new_value: String,
}
```

**优点**:
- ✅ 存储空间小（只存字段名和值的字符串）
- ✅ 可读性强（直接查看变更内容）
- ✅ 精确追踪（知道具体哪些字段变了）
- ✅ 无需序列化（不需要 Serialize trait）
- ✅ 易于查询和过滤（按字段名查询变更）

## 性能对比

### 示例实体
```rust
struct TestEntity {
    id: String,        // "test_1"
    value: i64,        // 100 → 150
    name: String,      // "Initial" → "Updated"
}
```

| 方案 | 存储大小 | 可读性 | 查询效率 |
|------|---------|--------|----------|
| 序列化 | 74 bytes (2×37) | 低（需反序列化） | 低 |
| 字段追踪 | ~60 bytes | 高（直接可读） | 高 |

## 适用场景

### ✅ 推荐使用

1. **审计日志** - 需要清晰记录谁改了什么
2. **权限控制** - 基于字段级别的权限
3. **变更通知** - 只通知相关字段的订阅者
4. **增量同步** - 只同步变更的字段
5. **UI 渲染** - 只更新变更的字段对应的 UI

### ⚠️ 注意事项

1. **手动记录** - 需要手动指定变更的字段
2. **类型安全** - 值存储为字符串，需要自行转换
3. **一致性** - 确保 field_changes 与实际变更一致

## 高级用法

### 宏辅助自动记录

可以实现一个宏来自动生成字段变更记录：

```rust
// 未来可以实现的宏
track_changes!(entity, {
    entity.name = "New Name";
    entity.age += 1;
});
// 自动生成 FieldChange 列表
```

### 类型安全的字段值

可以扩展 `FieldChange` 支持类型化的值：

```rust
pub enum FieldValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub struct FieldChange {
    pub field_name: String,
    pub old_value: FieldValue,
    pub new_value: FieldValue,
}
```

### 变更差异计算

可以添加辅助函数来计算两个实体的差异：

```rust
impl EntityManager<T> where T: Diff {
    pub fn diff(&self, other: &T) -> Vec<FieldChange> {
        self.entity.diff(other)
    }
}
```

## API 参考

### EntityManager::update

```rust
pub fn update<F>(
    &mut self,
    updater: F,
    field_changes: Vec<FieldChange>
) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
where
    F: FnOnce(&mut T)
```

**参数**:
- `updater`: 更新函数，接收可变引用并修改实体
- `field_changes`: 字段变更列表，明确指定哪些字段变更了

**返回**:
- `Ok(ChangeLogEntry)`: 包含变更信息的日志条目
- `Err(...)`: 时间戳获取失败等错误

## 最佳实践

### 1. 保持一致性

```rust
// ✅ 好的做法：field_changes 与实际变更一致
let changes = vec![
    FieldChange {
        field_name: "age".to_string(),
        old_value: old_age.to_string(),
        new_value: new_age.to_string(),
    },
];
manager.update(|user| {
    user.age = new_age;  // 与 changes 一致
}, changes);

// ❌ 坏的做法：不一致
let changes = vec![
    FieldChange { field_name: "age", ... },
];
manager.update(|user| {
    user.name = "foo";  // 改了 name 但 changes 里没记录
}, changes);
```

### 2. 使用辅助函数

```rust
fn record_field_change<T: ToString>(
    field_name: &str,
    old_value: T,
    new_value: T
) -> FieldChange {
    FieldChange {
        field_name: field_name.to_string(),
        old_value: old_value.to_string(),
        new_value: new_value.to_string(),
    }
}

// 使用
let change = record_field_change("age", 25, 26);
```

### 3. 验证变更

```rust
fn validate_changes(changes: &[FieldChange]) -> bool {
    changes.iter().all(|c| c.old_value != c.new_value)
}
```

## 总结

EntityManager 的字段级变更追踪提供了：

- ✅ **精确追踪** - 字段级别的变更记录
- ✅ **高可读性** - 直接查看变更内容
- ✅ **低存储开销** - 只存字段名和值
- ✅ **易于查询** - 支持按字段名过滤
- ✅ **无侵入性** - 不需要实体实现特殊 trait

这种设计非常适合需要详细审计日志和精确变更追踪的场景。
