# EntityManager - 终极方案：track! 宏

## 🎉 最佳方案：使用 `track!` 宏

### 三种方式全面对比

#### ❌ 方式1：手动 record（不推荐）

```rust
manager.update(|entity, tracker| {
    tracker.record("value", entity.value, 150);
    entity.value = 150;
});
```

**问题**:
- ❌ 两行代码，容易不同步
- ❌ 需要手动写字段名字符串
- ❌ 可能拼错字段名
- ❌ old_value 可能写错

#### ⚠️ 方式2：tracker.set()（推荐）

```rust
manager.update(|entity, tracker| {
    tracker.set("value", &mut entity.value, 150);
});
```

**优势**:
- ✅ 一行代码，自动同步
- ✅ 不会不同步

**问题**:
- ⚠️ 仍需手动写字段名字符串
- ⚠️ 字段名容易拼错
- ⚠️ 没有编译时检查

#### ✨ 方式3：track! 宏（最佳）

```rust
manager.update(|entity, tracker| {
    track!(tracker, entity.value = 150);
});
```

**优势**:
- ✅ 最简洁的语法
- ✅ 自动获取字段名（不会拼错）
- ✅ 接近原生赋值语法
- ✅ 自动同步
- ✅ 代码最少

## 完整对比表

| 特性 | record() | set() | track!宏 |
|------|----------|-------|----------|
| 代码行数 | 2行 | 1行 | 1行 |
| 同步性 | ❌ 手动 | ✅ 自动 | ✅ 自动 |
| 字段名获取 | ❌ 手动字符串 | ❌ 手动字符串 | ✅ 自动获取 |
| 拼写错误 | ❌ 可能 | ❌ 可能 | ✅ 不可能 |
| 语法简洁度 | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 易错性 | ❌ 高 | ⚠️ 中 | ✅ 低 |
| 推荐度 | ❌ | ✅ | ⭐⭐⭐⭐⭐ |

## 使用示例

### 基本用法

```rust
use prep_proc::track;  // 导入宏

let mut manager = EntityManager::new(user);

manager.update(|entity, tracker| {
    track!(tracker, entity.name = "Alice Smith".to_string());
    track!(tracker, entity.age = 26);
    track!(tracker, entity.email = "alice@example.com".to_string());
}).unwrap();
```

### 与原生语法对比

```rust
// 原生 Rust 赋值
entity.value = 150;
entity.name = "Updated".to_string();

// 使用 track! 宏（几乎一样！）
track!(tracker, entity.value = 150);
track!(tracker, entity.name = "Updated".to_string());
```

**相似度**: 99%！只需在前面加 `track!(tracker,`

### 复杂场景

```rust
manager.update(|order, tracker| {
    // 条件更新
    if order.quantity > 100 {
        track!(tracker, order.price = calculate_bulk_price());
        track!(tracker, order.discount = 0.1);
    }

    // 计算后更新
    let new_total = order.quantity * order.price;
    track!(tracker, order.total = new_total);

    // 字符串拼接
    track!(tracker, order.notes = format!("Processed at {}", now()));
}).unwrap();
```

### 嵌套字段

```rust
#[derive(Clone)]
struct Order {
    id: String,
    customer: Customer,
}

#[derive(Clone)]
struct Customer {
    name: String,
    email: String,
}

manager.update(|order, tracker| {
    // 嵌套字段也能自动获取完整路径
    track!(tracker, order.customer.name = "New Name".to_string());
    track!(tracker, order.customer.email = "new@email.com".to_string());
}).unwrap();

// 字段名会自动记录为 "order.customer.name" 和 "order.customer.email"
```

## 实际测试结果

```
=== 三种方式对比 ===

方式1: tracker.record()
  tracker.record("value", entity.value, 150);
  entity.value = 150;
  问题: 容易不同步

方式2: tracker.set()
  tracker.set("value", &mut entity.value, 150);
  优势: 自动同步

方式3: track! 宏
  track!(tracker, entity.value = 150);
  优势: 最简洁，自动获取字段名

实际效果:
  entity.value : 50 → 100
  entity.name : Test → Macro Test

=== 对比测试完成! ===
```

## 宏实现原理

```rust
#[macro_export]
macro_rules! track {
    ($tracker:expr, $($field:tt).+ = $value:expr) => {{
        // 1. stringify! 自动将字段路径转为字符串
        // 2. &mut $($field).+ 获取字段的可变引用
        // 3. $value 是新值
        $tracker.set(stringify!($($field).+), &mut $($field).+, $value);
    }};
}
```

**关键技术**:
- `$($field:tt).+`: 匹配任意字段路径（如 `entity.value` 或 `order.customer.name`）
- `stringify!()`: 将标识符转为字符串字面量
- 宏展开时自动获取字段名，编译时检查

## 迁移指南

### 从 set() 迁移到 track!

```rust
// 旧代码（set）
manager.update(|entity, tracker| {
    tracker.set("value", &mut entity.value, 150);
    tracker.set("name", &mut entity.name, "Updated".to_string());
});

// 新代码（track!）- 只需删除字段名字符串
manager.update(|entity, tracker| {
    track!(tracker, entity.value = 150);
    track!(tracker, entity.name = "Updated".to_string());
});
```

**迁移步骤**:
1. 将 `tracker.set("field_name", &mut entity.field_name, value)`
2. 改为 `track!(tracker, entity.field_name = value)`
3. 删除字段名字符串
4. 删除 `&mut`

## 完整示例

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

    manager.update(|order, tracker| {
        // 🎉 使用 track! 宏 - 语法最简洁
        track!(tracker, order.status = OrderStatus::Processing);
        track!(tracker, order.quantity = 120);
        track!(tracker, order.price = 48.5);
        track!(tracker, order.notes = "Bulk discount applied".to_string());
    }).unwrap();
}
```

## 高级技巧

### 批量更新

```rust
manager.update(|entity, tracker| {
    let updates = vec![
        ("Alice", 26, "alice@example.com"),
        // ...
    ];

    for (name, age, email) in updates {
        track!(tracker, entity.name = name.to_string());
        track!(tracker, entity.age = age);
        track!(tracker, entity.email = email.to_string());
    }
}).unwrap();
```

### 条件追踪

```rust
manager.update(|entity, tracker| {
    // 只有真正改变时才追踪
    if entity.value != new_value {
        track!(tracker, entity.value = new_value);
    }
}).unwrap();
```

### 混合使用

```rust
manager.update(|entity, tracker| {
    // 大部分用 track! 宏
    track!(tracker, entity.value = 150);

    // 特殊情况用 set()（如需要自定义格式）
    tracker.set(
        "price",
        &mut entity.price,
        new_price
    );

    // 不需要追踪的直接赋值
    entity.internal_flag = true;
}).unwrap();
```

## 最佳实践

### ✅ 推荐

```rust
// 1. 默认使用 track! 宏
manager.update(|e, t| {
    track!(t, e.value = 150);
    track!(t, e.name = "Updated".to_string());
}).unwrap();

// 2. 变量名简化（tracker -> t，entity -> e）
// 因为 track! 宏让代码更简洁，所以可以用更短的变量名

// 3. 一行一个字段，清晰易读
track!(t, e.field1 = value1);
track!(t, e.field2 = value2);
track!(t, e.field3 = value3);
```

### ❌ 避免

```rust
// ❌ 不要混用 track! 和手动赋值（容易混淆）
manager.update(|e, t| {
    track!(t, e.value = 150);
    e.name = "Updated".to_string();  // 为什么这个不追踪？
});

// ✅ 应该明确区分
manager.update(|e, t| {
    // 追踪的字段
    track!(t, e.value = 150);

    // 明确说明不追踪的原因
    e.internal_cache = None;  // 内部缓存，不需要审计
});
```

## 性能

- **零运行时开销**: 宏在编译时展开
- **与 set() 完全相同**: 宏只是语法糖
- **无额外分配**: 直接操作字段引用

## 总结

`track!` 宏是 **EntityManager 的终极方案**：

- 🎯 **最简洁** - 接近原生赋值语法
- ✅ **最安全** - 自动获取字段名，不会拼错
- 🚀 **最高效** - 零运行时开销
- 💡 **最易用** - 几乎不需要学习成本

**强烈推荐所有场景都使用 `track!` 宏！**

## 快速参考

```rust
// 导入
use prep_proc::track;

// 使用
manager.update(|entity, tracker| {
    track!(tracker, entity.field = new_value);
}).unwrap();

// 就这么简单！
```
