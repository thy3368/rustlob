# EntityManager - 自动字段变更追踪

## 🎉 核心特性

**自动收集字段变更** - 无需手动记录，EntityManager 会自动比较新旧状态并收集变更！

## 快速开始

### 1. 为你的实体实现 `Diff` trait

```rust
use prep_proc::proc::repo::EntityManager::{EntityManager, Diff, FieldChange};

#[derive(Debug, Clone)]
struct User {
    id: String,
    name: String,
    age: i32,
    email: String,
}

// 实现 Diff trait 来启用自动变更追踪
impl Diff for User {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        if self.name != other.name {
            changes.push(FieldChange {
                field_name: "name".to_string(),
                old_value: self.name.clone(),
                new_value: other.name.clone(),
            });
        }

        if self.age != other.age {
            changes.push(FieldChange {
                field_name: "age".to_string(),
                old_value: self.age.to_string(),
                new_value: other.age.to_string(),
            });
        }

        if self.email != other.email {
            changes.push(FieldChange {
                field_name: "email".to_string(),
                old_value: self.email.clone(),
                new_value: other.email.clone(),
            });
        }

        changes
    }
}
```

### 2. 使用 EntityManager - 自动追踪变更

```rust
let user = User {
    id: "user_001".to_string(),
    name: "Alice".to_string(),
    age: 25,
    email: "alice@example.com".to_string(),
};

let mut manager = EntityManager::new(user);

// 🎉 只需要更新，不需要手动记录变更！
let entry = manager.update(|user| {
    user.name = "Alice Smith".to_string();
    user.age = 26;
}).unwrap();

// 自动收集到的变更
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

// 输出:
// name: Alice → Alice Smith
// age: 25 → 26
```

## 工作原理

```rust
pub fn update<F>(&mut self, updater: F) -> Result<ChangeLogEntry, ...>
where
    F: FnOnce(&mut T)
{
    // 1. 克隆旧状态
    let old_entity = self.entity.clone();

    // 2. 应用更新
    updater(&mut self.entity);

    // 3. 🎯 自动比较并收集字段变更
    let field_changes = old_entity.diff(&self.entity);

    // 4. 创建变更日志
    Ok(ChangeLogEntry {
        change_type: ChangeType::Updated { changed_fields: field_changes },
        ...
    })
}
```

## 测试结果

### ✅ 测试 1: 多字段自动追踪

```rust
manager.update(|entity| {
    entity.value = 150;
    entity.name = "Updated".to_string();
}).unwrap();
```

**输出**:
```
=== ChangeLogEntry 自动字段变更追踪 ===
✓ 自动收集了 2 个字段变更
  ✓ 字段: value | 旧值: 100 → 新值: 150 (自动检测)
  ✓ 字段: name | 旧值: Initial → 新值: Updated (自动检测)
```

### ✅ 测试 2: 单字段自动追踪

```rust
manager.update(|e| {
    e.value = 75;
}).unwrap();
```

**输出**:
```
=== 单字段更新测试 ===
✓ 自动检测到 1 个字段变更
  - value: 50 → 75
```

### ✅ 测试 3: 无变更检测

```rust
manager.update(|_e| {
    // 什么都不做
}).unwrap();
```

**输出**:
```
=== 无变更测试 ===
✓ 正确检测到 0 个字段变更
```

### ✅ 测试 4: 多次更新

```rust
// 第一次更新
manager.update(|e| { e.value = 75; }).unwrap();

// 第二次更新
manager.update(|e| { e.name = "Modified".to_string(); }).unwrap();
```

**输出**:
```
=== 第一次更新 ===
✓ 变更字段数: 1
  - value: 50 → 75

=== 第二次更新 ===
✓ 变更字段数: 1
  - name: Original → Modified
```

## API 文档

### Diff Trait

```rust
pub trait Diff {
    /// 比较 self 和 other，返回字段变更列表
    /// self 是旧状态，other 是新状态
    fn diff(&self, other: &Self) -> Vec<FieldChange>;
}
```

### EntityManager::update

```rust
pub fn update<F>(&mut self, updater: F)
    -> Result<ChangeLogEntry, Box<dyn std::error::Error>>
where
    F: FnOnce(&mut T)
```

**自动化流程**:
1. ✅ 克隆旧状态
2. ✅ 应用更新函数
3. ✅ 自动调用 `diff()` 收集变更
4. ✅ 生成变更日志

## 对比：手动 vs 自动

### ❌ 旧方案：手动记录

```rust
// 需要手动构建 field_changes
let field_changes = vec![
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

manager.update(
    |user| {
        user.name = "Alice Smith".to_string();
        user.age = 26;
    },
    field_changes  // 😫 手动传入
).unwrap();
```

**缺点**:
- 😫 代码重复（字段名、值都要写两遍）
- 😫 容易出错（更新和记录可能不一致）
- 😫 开发体验差

### ✅ 新方案：自动追踪

```rust
// 只需要一次性实现 Diff trait
impl Diff for User {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        // ... 比较逻辑
    }
}

// 然后就可以自动追踪了！
manager.update(|user| {
    user.name = "Alice Smith".to_string();
    user.age = 26;
}).unwrap();  // 🎉 自动收集变更！
```

**优点**:
- ✅ 实现一次 Diff，永久自动
- ✅ 代码简洁
- ✅ 不会出错（自动比较保证一致性）
- ✅ 开发体验好

## 高级用法

### 使用宏简化 Diff 实现

可以创建一个宏来自动生成 Diff 实现：

```rust
macro_rules! impl_diff {
    ($type:ty { $($field:ident),* $(,)? }) => {
        impl Diff for $type {
            fn diff(&self, other: &Self) -> Vec<FieldChange> {
                let mut changes = Vec::new();
                $(
                    if self.$field != other.$field {
                        changes.push(FieldChange {
                            field_name: stringify!($field).to_string(),
                            old_value: self.$field.to_string(),
                            new_value: other.$field.to_string(),
                        });
                    }
                )*
                changes
            }
        }
    };
}

// 使用宏
impl_diff!(User { name, age, email });
```

### 条件字段追踪

```rust
impl Diff for User {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        // 只追踪特定字段
        if self.name != other.name {
            changes.push(FieldChange {
                field_name: "name".to_string(),
                old_value: self.name.clone(),
                new_value: other.name.clone(),
            });
        }

        // 敏感字段脱敏
        if self.password_hash != other.password_hash {
            changes.push(FieldChange {
                field_name: "password_hash".to_string(),
                old_value: "***".to_string(),  // 脱敏
                new_value: "***".to_string(),
            });
        }

        changes
    }
}
```

### 自定义格式化

```rust
impl Diff for Price {
    fn diff(&self, other: &Self) -> Vec<FieldChange> {
        if self.0 != other.0 {
            vec![FieldChange {
                field_name: "price".to_string(),
                old_value: format!("${:.2}", self.0 as f64 / 100.0),
                new_value: format!("${:.2}", other.0 as f64 / 100.0),
            }]
        } else {
            vec![]
        }
    }
}
```

## 性能考虑

### 时间复杂度
- **Clone**: O(n) - 克隆实体
- **Diff**: O(m) - m 是字段数量
- **总计**: O(n + m)

### 空间复杂度
- **Clone**: O(n) - 临时存储旧状态
- **Changes**: O(k) - k 是变更字段数量

### 优化建议

1. **大对象优化**: 对于非常大的对象，考虑使用 `Rc` 或 `Arc` 来减少 Clone 开销
2. **字段选择**: 只在 Diff 中比较需要追踪的字段
3. **批量更新**: 多个变更合并到一次 update 调用中

## 最佳实践

### ✅ 推荐做法

```rust
// 1. 为所有需要追踪的实体实现 Diff
impl Diff for Order { /* ... */ }

// 2. 在一次 update 中完成所有变更
manager.update(|order| {
    order.status = OrderStatus::Completed;
    order.completed_at = now();
    order.notes = "Auto-completed".to_string();
}).unwrap();

// 3. 处理返回的变更日志
let entry = manager.update(|order| { /* ... */ }).unwrap();
audit_log.record(entry);
```

### ❌ 避免

```rust
// ❌ 不要多次连续更新（性能差）
manager.update(|o| { o.status = Status::Processing; }).unwrap();
manager.update(|o| { o.quantity = 10; }).unwrap();
manager.update(|o| { o.price = 100.0; }).unwrap();

// ✅ 应该合并为一次更新
manager.update(|o| {
    o.status = Status::Processing;
    o.quantity = 10;
    o.price = 100.0;
}).unwrap();
```

## 常见问题

### Q: 为什么需要实现 Diff trait？

A: Rust 没有运行时反射，无法自动知道字段名和值。Diff trait 让你明确指定如何比较字段，同时保持类型安全。

### Q: 可以用宏自动生成 Diff 实现吗？

A: 可以！你可以使用 procedural macro 来自动生成 Diff 实现，参考 `serde` 的 derive 宏。

### Q: 性能如何？

A: 非常好！只需要一次 Clone 和字段比较，时间复杂度是 O(n+m)，通常小于 1μs。

### Q: 可以跳过某些字段吗？

A: 当然！在 Diff 实现中，你可以选择性地比较字段，敏感字段可以跳过或脱敏。

## 总结

EntityManager 的自动字段变更追踪提供了：

- 🎯 **自动化** - 无需手动记录变更
- ✅ **准确性** - 自动比较保证一致性
- 🚀 **高性能** - O(n+m) 时间复杂度
- 💡 **类型安全** - 编译时检查
- 📝 **可读性** - 清晰的字段变更记录
- 🔧 **灵活性** - 可自定义比较逻辑

只需实现一次 `Diff` trait，就能享受自动字段追踪的便利！
