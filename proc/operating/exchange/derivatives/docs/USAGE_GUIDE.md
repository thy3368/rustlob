# EntityManager 使用指南

## 完整解决方案总览

EntityManager 提供了两种字段变更追踪模式，分别适用于不同的使用场景：

### 模式对比表

| 使用场景 | 推荐方法 | 优势 | 示例 |
|---------|---------|------|------|
| **直接字段赋值** | `update()` + `track!` 宏 | 最简洁，自动获取字段名 | `track!(t, e.value = 150);` |
| **通过方法修改** | `update_auto()` + `Diff` trait | 自动检测所有变更 | `entity.increment_value();` |

---

## 模式 1: update() + track! 宏

### 适用场景
- ✅ **直接字段赋值** - 在 update 闭包中直接修改字段值
- ✅ **需要选择性追踪** - 只追踪部分重要字段，其他字段不记录
- ✅ **简单业务逻辑** - 变更逻辑直观清晰

### 使用方法

```rust
use prep_proc::track;  // 导入宏

let mut manager = EntityManager::new(order);

// 使用 track! 宏追踪字段变更
let entry = manager.update(|entity, tracker| {
    // ✨ 语法接近原生赋值，只需在前面加 track!(tracker,
    track!(tracker, entity.value = 150);
    track!(tracker, entity.name = "Updated".to_string());

    // 不需要追踪的字段可以直接赋值
    entity.internal_flag = true;  // 不会被追踪
}).unwrap();
```

### 核心特性

**1. 自动获取字段名**
```rust
// ✅ track! 宏自动获取字段名
track!(tracker, entity.price = 100.0);

// 等价于（但不需要手写）：
tracker.set("entity.price", &mut entity.price, 100.0);
```

**2. 支持嵌套字段**
```rust
// 自动记录完整路径
track!(tracker, entity.customer.name = "Alice".to_string());
// 字段名会自动记录为 "entity.customer.name"
```

**3. 零运行时开销**
- 宏在编译时展开
- 与手写代码性能完全相同
- 无额外内存分配

### 完整示例

```rust
use prep_proc::track;

#[derive(Debug, Clone)]
struct Order {
    id: String,
    status: OrderStatus,
    quantity: i32,
    price: f64,
    notes: String,
}

fn process_order(order: Order) {
    let mut manager = EntityManager::new(order);

    let entry = manager.update(|order, tracker| {
        // 🎉 使用 track! 宏 - 语法最简洁
        track!(tracker, order.status = OrderStatus::Processing);
        track!(tracker, order.quantity = 120);
        track!(tracker, order.price = 48.5);
        track!(tracker, order.notes = "Bulk discount applied".to_string());
    }).unwrap();

    // 处理变更日志
    println!("变更记录:");
    if let ChangeType::Updated { changed_fields } = entry.change_type {
        for change in changed_fields {
            println!("  {}: {} → {}",
                change.field_name,
                change.old_value,
                change.new_value);
        }
    }
}
```

**输出**:
```
变更记录:
  order.status: Pending → Processing
  order.quantity: 100 → 120
  order.price: 50 → 48.5
  order.notes:  → Bulk discount applied
```

---

## 模式 2: update_auto() + Diff trait

### 适用场景
- ✅ **通过方法修改实体** - 调用实体的业务方法
- ✅ **复杂业务逻辑** - 方法内部修改多个字段
- ✅ **自动追踪所有变更** - 不想手动指定每个字段

### 使用方法

**步骤 1: 为实体实现 Diff trait**

```rust
use prep_proc::proc::repo::EntityManager::{Diff, FieldChange};

#[derive(Debug, Clone)]
struct Order {
    id: String,
    status: OrderStatus,
    quantity: i32,
    price: f64,
}

// 实现 Diff trait
impl Diff for Order {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        // 比较每个需要追踪的字段
        if self.status != other.status {
            changes.push(FieldChange {
                field_name: "status".to_string(),
                old_value: self.status.to_string(),
                new_value: other.status.to_string(),
            });
        }

        if self.quantity != other.quantity {
            changes.push(FieldChange {
                field_name: "quantity".to_string(),
                old_value: self.quantity.to_string(),
                new_value: other.quantity.to_string(),
            });
        }

        if self.price != other.price {
            changes.push(FieldChange {
                field_name: "price".to_string(),
                old_value: self.price.to_string(),
                new_value: other.price.to_string(),
            });
        }

        changes
    }
}
```

**步骤 2: 为实体添加业务方法**

```rust
impl Order {
    /// 业务方法：增加数量
    pub fn add_quantity(&mut self, amount: i32) {
        self.quantity += amount;
    }

    /// 业务方法：应用折扣
    pub fn apply_discount(&mut self, percentage: f64) {
        self.price = self.price * (1.0 - percentage);
    }

    /// 业务方法：完成订单
    pub fn complete(&mut self) {
        self.status = OrderStatus::Completed;
    }

    /// 复杂业务逻辑：批量处理
    pub fn process_bulk_order(&mut self) {
        if self.quantity > 100 {
            self.apply_discount(0.1);  // 10% 折扣
            self.status = OrderStatus::Processing;
        }
    }
}
```

**步骤 3: 使用 update_auto() 自动追踪**

```rust
let mut manager = EntityManager::new(order);

// 🎯 调用业务方法，变更自动被追踪
let entry = manager.update_auto(|order| {
    order.add_quantity(20);         // 方法调用
    order.apply_discount(0.05);     // 方法调用
    order.complete();               // 方法调用
}).unwrap();

// 所有变更都被自动检测到
println!("变更记录:");
if let ChangeType::Updated { changed_fields } = entry.change_type {
    for change in changed_fields {
        println!("  {}: {} → {}",
            change.field_name,
            change.old_value,
            change.new_value);
    }
}
```

**输出**:
```
变更记录:
  quantity: 100 → 120
  price: 50 → 47.5
  status: Pending → Completed
```

### 核心特性

**1. 自动检测所有变更**
- 通过 `Clone` 保存旧状态
- 调用 `Diff::diff()` 自动比较
- 无需手动记录每个变更

**2. 支持复杂业务逻辑**
```rust
manager.update_auto(|order| {
    // 调用复杂业务方法
    order.process_bulk_order();
    // 内部可能修改多个字段，全部自动追踪
}).unwrap();
```

**3. 灵活控制追踪粒度**
```rust
impl Diff for Order {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        // 只追踪重要字段
        if self.price != other.price {
            changes.push(FieldChange {
                field_name: "price".to_string(),
                old_value: self.price.to_string(),
                new_value: other.price.to_string(),
            });
        }

        // 敏感字段脱敏
        if self.password_hash != other.password_hash {
            changes.push(FieldChange {
                field_name: "password".to_string(),
                old_value: "***".to_string(),
                new_value: "***".to_string(),
            });
        }

        changes
    }
}
```

---

## 两种模式的对比

### 功能对比

| 特性 | track! 宏 | update_auto() |
|------|-----------|---------------|
| **语法简洁度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **直接赋值** | ✅ 完美支持 | ⚠️ 可以但不推荐 |
| **方法调用** | ❌ 不支持 | ✅ 完美支持 |
| **选择性追踪** | ✅ 灵活控制 | ⚠️ 需在 Diff 中实现 |
| **初始成本** | ✅ 零成本（导入宏） | ⚠️ 需实现 Diff trait |
| **运行时开销** | ✅ 零开销 | ⚠️ 需 Clone 实体 |
| **编译时检查** | ✅ 字段名自动获取 | ✅ 类型安全 |

### 性能对比

#### track! 宏
```rust
// 无额外开销
track!(tracker, entity.value = 150);

// 编译后等价于：
tracker.set("entity.value", &mut entity.value, 150);
// 直接操作字段，无 Clone
```

#### update_auto()
```rust
// 需要 Clone 整个实体
manager.update_auto(|entity| {
    entity.increment_value();
});

// 内部实现：
let old = entity.clone();      // 额外开销：Clone
updater(&mut entity);
let changes = old.diff(&entity); // 额外开销：比较
```

**结论**: 对于小型实体（< 1KB），性能差异可忽略；对于大型实体，track! 宏更优。

---

## 实战场景示例

### 场景 1: 用户资料更新（使用 track! 宏）

```rust
#[derive(Clone)]
struct UserProfile {
    id: String,
    name: String,
    email: String,
    age: i32,
}

fn update_user_profile(user: UserProfile, new_name: String, new_age: i32) {
    let mut manager = EntityManager::new(user);

    let entry = manager.update(|user, tracker| {
        // 只追踪实际变更的字段
        track!(tracker, user.name = new_name);
        track!(tracker, user.age = new_age);

        // 内部字段不追踪
        user.last_updated = now();
    }).unwrap();

    // 发送审计日志
    audit_log::record(entry);
}
```

**为什么用 track! 宏？**
- 直接字段赋值
- 只追踪重要字段（name, age）
- 语法简洁

---

### 场景 2: 订单状态机（使用 update_auto()）

```rust
#[derive(Clone)]
struct Order {
    id: String,
    status: OrderStatus,
    quantity: i32,
    total: f64,
}

impl Order {
    /// 业务方法：处理订单
    pub fn process(&mut self) {
        self.status = OrderStatus::Processing;
        self.validate_quantity();
        self.calculate_total();
    }

    /// 业务方法：取消订单
    pub fn cancel(&mut self) {
        self.status = OrderStatus::Cancelled;
        self.total = 0.0;
    }

    fn validate_quantity(&mut self) {
        if self.quantity < 1 {
            self.quantity = 1;
        }
    }

    fn calculate_total(&mut self) {
        self.total = self.quantity as f64 * 100.0;
    }
}

impl Diff for Order {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        // 实现略...
    }
}

fn process_order(order: Order) {
    let mut manager = EntityManager::new(order);

    // 调用业务方法，自动追踪所有变更
    let entry = manager.update_auto(|order| {
        order.process();  // 内部修改多个字段
    }).unwrap();

    // 变更自动记录
    notify_user(entry);
}
```

**为什么用 update_auto()？**
- 通过方法修改实体
- 方法内部逻辑复杂
- 想自动追踪所有变更

---

### 场景 3: 混合使用

某些场景可能需要混合使用：

```rust
// 情况 1: 大部分直接赋值 + 少量方法调用
manager.update_auto(|entity| {
    // 方法调用（自动追踪）
    entity.apply_business_rules();

    // 直接赋值（也会被追踪）
    entity.manual_flag = true;
}).unwrap();

// 情况 2: 只追踪部分变更
manager.update(|entity, tracker| {
    // 追踪重要字段
    track!(tracker, entity.status = Status::Active);

    // 调用方法但不追踪内部变更
    entity.recalculate();  // 内部修改不会被记录
}).unwrap();
```

---

## 最佳实践

### ✅ 推荐做法

1. **优先使用 track! 宏**（大多数场景）
   ```rust
   manager.update(|e, t| {
       track!(t, e.value = 150);
   }).unwrap();
   ```

2. **方法修改使用 update_auto()**
   ```rust
   manager.update_auto(|e| {
       e.business_method();
   }).unwrap();
   ```

3. **为实体提供 Diff 实现**（即使现在不用）
   ```rust
   impl Diff for MyEntity {
       fn diff(&self, other: &Self) -> Vec<FieldChange> {
           // 实现一次，永久可用
       }
   }
   ```

### ❌ 避免

1. **不要混用方式导致混淆**
   ```rust
   // ❌ 坏的做法
   manager.update(|e, t| {
       track!(t, e.value = 150);  // track! 宏
       e.some_method();           // 为什么这个不追踪？
   }).unwrap();
   ```

2. **不要在 update_auto() 中手动记录**
   ```rust
   // ❌ 不需要（自动检测）
   manager.update_auto(|e| {
       e.value = 150;  // 会自动检测，不需要手动记录
   }).unwrap();
   ```

---

## 性能建议

### 小型实体（< 1KB）
- 两种方法性能差异可忽略
- 按场景选择最合适的

### 大型实体（> 10KB）
- 优先使用 `track!` 宏（避免 Clone 开销）
- 或在 Diff 中只比较变更可能性大的字段

### 高频更新场景
- 使用 `track!` 宏（零运行时开销）
- 避免频繁 Clone

---

## 总结

### 快速决策树

```
你要修改实体吗？
├─ 是 → 是直接赋值还是调用方法？
│   ├─ 直接赋值 → 使用 update() + track! 宏 ✅
│   └─ 调用方法 → 使用 update_auto() + Diff trait ✅
└─ 否 → 不需要 EntityManager
```

### 一句话总结

- **直接赋值** → `track!(tracker, entity.field = value)`
- **方法调用** → `manager.update_auto(|e| e.method())`

两种方式各有优势，根据实际场景选择最合适的方案！
