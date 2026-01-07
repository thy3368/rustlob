# ChangeTracker 集成文档

## 概述

成功将 `diff_tracker` crate 的 `ChangeTracker` 能力集成到 `diff` crate 的 `diff_types.rs` 中，提供了完整的实体变更追踪解决方案。

## 集成内容

### 核心组件

从 `diff_tracker/src/tracker/mod.rs` 集成到 `diff/src/diff/diff_types.rs`:

1. **ChangeTracker 结构体** - 变更追踪器
2. **track! 宏** - 自动追踪字段变更的便捷宏
3. **追踪函数**:
   - `track_with_tracker()` - 手动追踪
   - `track_auto()` - 自动追踪（基于 Diff trait）
   - `track_changes()` - track_auto 的别名

### 架构设计

```
diff_types.rs 现在提供三种变更追踪方式:

1. TrackableEntity + Diff + Replay trait
   └─ 完整的实体追踪和回放能力

2. ChangeTracker (手动追踪)
   ├─ record(field_name, old_value, new_value)
   ├─ set(field_name, field, new_value)
   └─ track! 宏

3. 便捷函数
   ├─ track_with_tracker() - 基于 ChangeTracker
   └─ track_auto() - 基于 Diff trait
```

## 使用示例

### 方式 1: 使用 trait 系统

```rust
use diff::{TrackableEntity, Diff, Replay, ChangeLogEntry};

#[derive(Debug, Clone)]
struct Order {
    id: u64,
    status: String,
    price: f64,
}

// 实现三个 trait
impl TrackableEntity for Order {
    type Id = u64;
    fn entity_id(&self) -> Self::Id { self.id }
    fn entity_type() -> &'static str { "Order" }
    fn to_bytes(&self) -> Result<Vec<u8>, String> { /* ... */ }
    fn from_bytes(data: &[u8]) -> Result<Self, String> { /* ... */ }
}

impl Diff for Order {
    fn diff(&self, other: &Self) -> Vec<FieldChange> { /* ... */ }
}

impl Replay for Order {
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String> { /* ... */ }
}

// 自动获得 Trackable trait
```

### 方式 2: 使用 ChangeTracker（手动追踪）

```rust
use diff::{ChangeTracker, track};

let mut order = Order::new();
let mut tracker = ChangeTracker::new();

// 方式 2a: 使用 track! 宏
track!(tracker, order.status = "confirmed".to_string());
track!(tracker, order.price = 150.0);

// 方式 2b: 使用 set 方法
tracker.set("status", &mut order.status, "confirmed".to_string());

// 方式 2c: 使用 record 方法
let old_price = order.price;
order.price = 150.0;
tracker.record("price", old_price, order.price);

let changes = tracker.into_changes();
```

### 方式 3: 使用便捷函数

```rust
use diff::{track_with_tracker, track_auto, track};

// 3a: 手动追踪（不需要实现 Diff trait）
let mut order = Order::new();
let entry = track_with_tracker(&mut order, |o, tracker| {
    track!(tracker, o.status = "confirmed".to_string());
    track!(tracker, o.price = 150.0);
}).unwrap();

// 3b: 自动追踪（需要实现 Diff trait）
let mut order = Order::new();
let entry = track_auto(&mut order, |o| {
    o.status = "confirmed".to_string();
    o.price = 150.0;
}).unwrap();
```

## API 对比

### ChangeTracker 方法

| 方法 | 签名 | 用途 |
|------|------|------|
| `new()` | `Self` | 创建追踪器 |
| `record()` | `(field_name, old_value, new_value)` | 手动记录变更 |
| `set()` | `(field_name, field, new_value)` | 更新并记录 |
| `into_changes()` | `Vec<FieldChange>` | 获取变更（消耗） |
| `changes()` | `&[FieldChange]` | 获取变更（借用） |
| `has_changes()` | `bool` | 是否有变更 |
| `clear()` | `()` | 清空变更 |

### 追踪函数对比

| 函数 | 需要 Diff trait | 需要 Clone | 性能 | 灵活性 |
|------|----------------|-----------|------|--------|
| `track_with_tracker()` | ❌ | ❌ | 高 | 高 |
| `track_auto()` | ✅ | ✅ | 中 | 中 |
| `track_changes()` | ✅ | ✅ | 中 | 中 |

## 选择指南

### 选择 ChangeTracker（手动追踪）如果：
- ✅ 需要精确控制哪些字段被追踪
- ✅ 不想实现 Diff trait
- ✅ 性能要求高（避免 clone）
- ✅ 复杂的变更逻辑

### 选择 Diff trait（自动追踪）如果：
- ✅ 实体结构简单
- ✅ 需要追踪所有字段变更
- ✅ 可以接受 clone 开销
- ✅ 想要简洁的代码

### 选择完整 trait 系统如果：
- ✅ 需要 Event Sourcing
- ✅ 需要快照和回放
- ✅ 需要完整的实体追踪能力
- ✅ 构建复杂的状态管理系统

## 测试结果

```bash
$ cargo test --package diff --lib diff_types

running 10 tests
test diff::diff_types::tests::test_change_tracker_record ... ok
test diff::diff_types::tests::test_change_tracker_set ... ok
test diff::diff_types::tests::test_diff ... ok
test diff::diff_types::tests::test_replay ... ok
test diff::diff_types::tests::test_snapshot ... ok
test diff::diff_types::tests::test_track_auto ... ok
test diff::diff_types::tests::test_track_auto_with_no_changes ... ok
test diff::diff_types::tests::test_track_with_no_changes ... ok
test diff::diff_types::tests::test_track_with_tracker ... ok
test diff::diff_types::tests::test_trackable_entity ... ok

test result: ok. 10 passed; 0 failed
```

## 导出的 API

从 `diff` crate 可以直接使用：

```rust
use diff::{
    // 核心数据结构
    ChangeType, FieldChange, ChangeLogEntry, EntitySnapshot, TrackingResult,

    // 核心 trait
    TrackableEntity, Diff, Replay, Trackable,

    // 变更追踪器
    ChangeTracker,

    // 便捷函数
    track_with_tracker, track_auto, track_changes,

    // 宏（需要 #[macro_use]）
    track,
};
```

## track! 宏使用

```rust
#[macro_use]
extern crate diff;

use diff::{ChangeTracker, track};

fn example() {
    let mut entity = MyEntity::new();
    let mut tracker = ChangeTracker::new();

    // 自动追踪字段变更
    track!(tracker, entity.field1 = new_value1);
    track!(tracker, entity.field2 = new_value2);
    track!(tracker, entity.nested.field = value);  // 支持嵌套字段

    let changes = tracker.into_changes();
}
```

## 性能考虑

### ChangeTracker（手动追踪）
- **优点**:
  - 无需 clone
  - 精确控制追踪
  - 零额外分配
- **缺点**:
  - 需要手动记录每个字段

### Diff trait（自动追踪）
- **优点**:
  - 代码简洁
  - 自动检测所有变更
- **缺点**:
  - 需要 clone 整个实体
  - 需要实现 Diff trait
  - 字段较多时性能开销大

### 推荐实践

```rust
// 热路径：使用 ChangeTracker
fn hot_path_update(order: &mut Order) {
    let mut tracker = ChangeTracker::new();
    tracker.set("price", &mut order.price, new_price);
    // 只追踪需要的字段
}

// 非关键路径：使用 track_auto
fn cold_path_update(order: &mut Order) {
    track_auto(order, |o| {
        o.status = "updated".to_string();
        o.price = 100.0;
    }).unwrap();
}
```

## 与 diff_tracker crate 的关系

**现状**: `diff_tracker` crate 现在依赖 `diff` crate

**建议**: 可以考虑废弃 `diff_tracker` crate，统一使用 `diff` crate

**迁移路径**:
```rust
// 旧代码（diff_tracker）
use diff_tracker::{ChangeTracker, track_with_tracker};

// 新代码（diff）
use diff::{ChangeTracker, track_with_tracker};
// API 完全兼容，只需改变导入路径
```

## 架构优势

### 统一的变更追踪系统

```
diff crate (统一追踪系统)
  ├─ TrackableEntity trait (实体标识)
  ├─ Diff trait (差异检测)
  ├─ Replay trait (状态回放)
  ├─ Trackable trait (完整能力)
  ├─ ChangeTracker (手动追踪器)
  └─ 便捷函数 (简化使用)
```

### 关键改进

1. **单一依赖**: 只需依赖 `diff` crate
2. **灵活选择**: 三种追踪方式适应不同场景
3. **类型安全**: 关联类型确保类型安全
4. **零成本抽象**: 编译期多态，无运行时开销
5. **完整测试**: 10 个测试全部通过

## 下一步计划

### 可选增强

1. **派生宏支持**:
   ```rust
   #[derive(Trackable)]
   struct Order { /* ... */ }
   ```

2. **异步支持**:
   ```rust
   pub async fn track_async<T, F>(entity: &mut T, updater: F)
   where T: TrackableEntity,
         F: Future<Output = ()>
   ```

3. **批量追踪**:
   ```rust
   pub fn track_batch<T>(entities: &mut [T], updater: F)
   where T: Trackable
   ```

4. **压缩支持**: EntitySnapshot 的自动压缩

## 总结

成功将 `ChangeTracker` 能力集成到 `diff_types.rs`，提供了完整的实体变更追踪解决方案：

- ✅ **三种追踪方式**: trait 系统、手动追踪、自动追踪
- ✅ **灵活选择**: 根据场景选择最合适的方式
- ✅ **完整测试**: 10 个测试全部通过
- ✅ **统一 API**: 所有功能从 `diff` crate 导出
- ✅ **向后兼容**: 与 `diff_tracker` API 完全兼容

**版本**: v1.0.0
**更新时间**: 2025-12-16
**测试状态**: ✅ 10/10 passed
