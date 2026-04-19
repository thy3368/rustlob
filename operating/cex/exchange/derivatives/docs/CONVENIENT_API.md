# EntityManager - 便捷的字段变更追踪

## ✨ 核心改进：`tracker.set()` 方法

### 问题分析

之前的方式容易出现**记录和更新不同步**的问题：

```rust
// ❌ 旧方式：需要写两次，容易出错
manager.update(|entity, tracker| {
    tracker.record("value", entity.value, 150);  // 手动记录
    entity.value = 150;                          // 手动更新

    // 问题：
    // 1. 可能忘记 record
    // 2. 可能 old_value 写错
    // 3. 可能 new_value 和实际赋值不一致
});
```

### 解决方案：`tracker.set()`

**一步完成记录和更新，永远不会不同步！**

```rust
// ✅ 新方式：一行代码搞定
manager.update(|entity, tracker| {
    tracker.set("value", &mut entity.value, 150);
    tracker.set("name", &mut entity.name, "Updated".to_string());
});
```

## API 对比

### ❌ 容易出错的方式

```rust
manager.update(|entity, tracker| {
    // 步骤1: 记录旧值
    tracker.record("value", entity.value, 150);
    // 步骤2: 更新字段
    entity.value = 150;

    // 风险：
    // - 如果改了 150 为 200，可能忘记改上面的 record
    // - 如果 old_value 写错了，日志就不准确
    // - 两行代码分离，维护困难
});
```

### ✅ 推荐的方式

```rust
manager.update(|entity, tracker| {
    // 一步完成！自动记录旧值，并更新字段
    tracker.set("value", &mut entity.value, 150);

    // 优势：
    // - 自动获取旧值（entity.value.clone()）
    // - 自动更新字段（*field = new_value）
    // - 自动记录变更
    // - 不可能不同步！
});
```

## 使用示例

### 基本用法

```rust
use prep_proc::proc::repo::EntityManager::EntityManager;

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

// 🎉 使用 tracker.set() - 简单、安全、不会出错
let entry = manager.update(|user, tracker| {
    tracker.set("name", &mut user.name, "Alice Smith".to_string());
    tracker.set("age", &mut user.age, 26);
    tracker.set("email", &mut user.email, "alice.smith@example.com".to_string());
}).unwrap();

// 查看变更
match entry.change_type {
    ChangeType::Updated { changed_fields } => {
        for change in changed_fields {
            println!("{}: {} → {}",
                change.field_name,
                change.old_value,
                change.new_value);
        }
    }
    _ => {}
}
```

**输出**:
```
name: Alice → Alice Smith
age: 25 → 26
email: alice@example.com → alice.smith@example.com
```

### 选择性追踪

```rust
manager.update(|user, tracker| {
    // 只追踪重要字段
    tracker.set("email", &mut user.email, "new@example.com".to_string());

    // 不追踪的字段（直接修改）
    user.last_login = now();  // 不记录日志
}).unwrap();
```

### 条件更新

```rust
manager.update(|user, tracker| {
    if user.age < 18 {
        tracker.set("age", &mut user.age, 18);
        tracker.set("status", &mut user.status, "minor".to_string());
    }
}).unwrap();
```

## 测试结果

### ✅ 测试 1: 便捷 API

```
=== 便捷 API 测试 (tracker.set) ===
✓ 记录了 2 个字段变更
  ✓ value: 100 → 150 (自动同步)
  ✓ name: Initial → Updated (自动同步)
=== 便捷 API 测试通过! ===
```

### ✅ 测试 2: API 对比

```
=== API 对比测试 ===

❌ 旧方式（容易出错）:
   tracker.record("value", entity.value, 150);
   entity.value = 150;  // 可能忘记或写错

✅ 新方式（不会出错）:
   tracker.set("value", &mut entity.value, 150);  // 一步完成！

✓ 使用 tracker.set() 成功记录了 2 个变更
  - value: 50 → 75
  - name: Test → Modified
```

## API 参考

### ChangeTracker::set

```rust
pub fn set<T>(&mut self, field_name: &str, field: &mut T, new_value: T)
where
    T: ToString + Clone
```

**参数**:
- `field_name`: 字段名称（用于日志）
- `field`: 要更新的字段的可变引用
- `new_value`: 新值

**功能**:
1. 自动 `clone()` 旧值
2. 记录变更到日志
3. 更新字段为新值

**约束**:
- `T` 必须实现 `ToString` （用于转换为日志字符串）
- `T` 必须实现 `Clone` （用于获取旧值）

### ChangeTracker::record (保留用于高级场景)

```rust
pub fn record<T: ToString, U: ToString>(
    &mut self,
    field_name: &str,
    old_value: T,
    new_value: U
)
```

**何时使用**:
- 需要自定义格式化时
- 不能使用 `Clone` 的类型
- 需要手动控制记录逻辑时

## 工作原理

### tracker.set() 内部实现

```rust
pub fn set<T>(&mut self, field_name: &str, field: &mut T, new_value: T)
where
    T: ToString + Clone
{
    // 1. 克隆旧值（不会失败）
    let old_value = field.clone();

    // 2. 记录变更
    self.changes.push(FieldChange {
        field_name: field_name.to_string(),
        old_value: old_value.to_string(),
        new_value: new_value.to_string(),
    });

    // 3. 更新字段
    *field = new_value;
}
```

**关键点**:
- ✅ 先克隆旧值，保证记录准确
- ✅ 然后赋值新值，保证同步
- ✅ 原子操作，不会出现中间状态

## 完整示例

### 实际业务场景

```rust
#[derive(Debug, Clone)]
struct Order {
    id: String,
    status: OrderStatus,
    quantity: i32,
    price: f64,
    notes: String,
}

let order = Order {
    id: "ORD-001".to_string(),
    status: OrderStatus::Pending,
    quantity: 100,
    price: 50.0,
    notes: "".to_string(),
};

let mut manager = EntityManager::new(order);

// 订单处理流程
let entry = manager.update(|order, tracker| {
    // 更新状态
    tracker.set("status", &mut order.status, OrderStatus::Processing);

    // 调整数量
    tracker.set("quantity", &mut order.quantity, 120);

    // 更新价格
    tracker.set("price", &mut order.price, 48.5);

    // 添加备注
    tracker.set("notes", &mut order.notes, "Bulk discount applied".to_string());
}).unwrap();

// 审计日志
println!("订单 {} 的变更记录:", entry.entity_id);
if let ChangeType::Updated { changed_fields } = entry.change_type {
    for change in changed_fields {
        println!("  • {} 从 {} 改为 {}",
            change.field_name,
            change.old_value,
            change.new_value);
    }
}
```

**输出**:
```
订单 example_id 的变更记录:
  • status 从 Pending 改为 Processing
  • quantity 从 100 改为 120
  • price 从 50 改为 48.5
  • notes 从  改为 Bulk discount applied
```

## 优势总结

| 特性 | tracker.record() | tracker.set() |
|------|-----------------|---------------|
| **代码行数** | 2行 | 1行 |
| **同步性** | ❌ 手动保证 | ✅ 自动保证 |
| **易错性** | ❌ 高（可能忘记或写错） | ✅ 低（不可能出错） |
| **可读性** | ⚠️ 中等 | ✅ 高 |
| **维护性** | ❌ 差（修改时需要两处同步） | ✅ 好（只需修改一处） |
| **推荐度** | ⚠️ 仅高级场景 | ✅ **日常使用** |

## 迁移指南

### 从旧 API 迁移

```rust
// 旧代码
manager.update(|entity, tracker| {
    tracker.record("value", entity.value, 150);
    entity.value = 150;

    tracker.record("name", &entity.name, "New Name");
    entity.name = "New Name".to_string();
});

// 新代码（推荐）
manager.update(|entity, tracker| {
    tracker.set("value", &mut entity.value, 150);
    tracker.set("name", &mut entity.name, "New Name".to_string());
});
```

## 最佳实践

### ✅ 推荐做法

```rust
// 1. 使用 tracker.set() 处理所有需要追踪的字段
manager.update(|order, tracker| {
    tracker.set("status", &mut order.status, OrderStatus::Completed);
    tracker.set("completed_at", &mut order.completed_at, now());
}).unwrap();

// 2. 不需要追踪的字段直接修改
manager.update(|order, tracker| {
    tracker.set("status", &mut order.status, OrderStatus::Processing);

    // 内部字段不追踪
    order.internal_notes = "Processing...".to_string();
}).unwrap();

// 3. 处理变更结果
let entry = manager.update(|order, tracker| {
    tracker.set("price", &mut order.price, new_price);
}).unwrap();

// 发送通知、记录审计日志等
audit_log.record(entry);
```

### ❌ 避免

```rust
// ❌ 不要混用 set 和直接赋值（容易混淆）
manager.update(|order, tracker| {
    tracker.set("status", &mut order.status, Status::Active);
    order.price = 100.0;  // 为什么这个不追踪？容易忘记原因
});

// ✅ 应该明确区分
manager.update(|order, tracker| {
    // 追踪的字段
    tracker.set("status", &mut order.status, Status::Active);

    // 明确说明不追踪的字段
    // 内部状态，不需要审计
    order.internal_processing_flag = true;
});
```

## 总结

`tracker.set()` 方法提供了：

- 🎯 **一步到位** - 记录 + 更新一行搞定
- ✅ **永不不同步** - 自动保证记录和实际值一致
- 🚀 **简单易用** - 减少代码量，提高可读性
- 🔒 **类型安全** - 编译时检查，运行时安全
- 📝 **清晰追踪** - 明确知道哪些字段被追踪了

**推荐所有日常使用场景都用 `tracker.set()`！**
