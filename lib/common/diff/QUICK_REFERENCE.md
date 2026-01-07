# Created 事件重构 - 快速参考

## 核心概念（60 秒快速理解）

```
Created Event = 包含初始字段信息的 ChangeLogEntry
重构 = 从事件的字段信息构建新实体实例

Input:  ChangeLogEntry::Created { fields: [id, symbol, price, ...] }
Output: Order { id: 1, symbol: "BTCUSDT", price: 50000.0, ... }
```

## 最简单的使用方式：使用 Derive 宏（推荐）

```rust
use diff::Entity;

// 1. 定义实体，只需添加 #[derive(Entity)]
#[derive(Debug, Clone, PartialEq, Entity)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
    quantity: u64,
    status: String,
}

// 2. 从 Created 事件一行代码重构（无需手动编写解析逻辑）
let order = Order::from_created_event(&event)?;
```

**优势**：
- ✅ 零样板代码
- ✅ 自动处理类型解析
- ✅ 自动去掉字符串引号
- ✅ 完整的错误处理
- ✅ 编译时代码生成（零运行时开销）

## 高级：手动控制和自定义

如果需要自定义重构逻辑或处理特殊类型，可使用底层 API：

```rust
use diff::reconstruct_from_created;

// 使用闭包进行自定义构造
let order = reconstruct_from_created::<Order, _>(&event, |fields| {
    Ok(Order {
        id: fields.get("id")?.parse().ok()?,
        symbol: fields.get("symbol")?.trim_matches('"').to_string(),
        price: fields.get("price")?.parse().ok()?,
        quantity: fields.get("quantity")?.parse().ok()?,
        status: fields.get("status")?.trim_matches('"').to_string(),
    })
})?;
```

## 三个必知的辅助函数

### 1. 提取字段
```rust
let fields = extract_fields_from_created_event(&event)?;
// Result: HashMap<String, String>
// 例：{"id" -> "1", "symbol" -> "\"BTCUSDT\""}
```

### 2. 解析字段值
```rust
parse_field_value("42", "u64")?;        // ✓
parse_field_value("3.14", "f64")?;      // ✓
parse_field_value("\"hello\"", "string")?;  // ✓ 自动去掉引号
```

### 3. 通用重构
```rust
reconstruct_from_created::<T, _>(&event, |fields| {
    // 你的构造逻辑
    Ok(T { /* ... */ })
})?
```

## Derive 宏的属性

### `#[created(skip)]` - 跳过字段

对于无法自动解析的复杂类型，可使用 `#[created(skip)]`：

```rust
#[derive(Entity)]
struct OrderWithCache {
    id: u64,
    symbol: String,
    #[created(skip)]  // 此字段使用 Default::default()
    cache: Arc<Mutex<HashMap<String, Value>>>,
}
```

### 其他属性继续有效

```rust
#[derive(Entity)]
#[entity(id = "order_id", type_name = "CustomOrder")]
struct Order {
    order_id: u64,
    #[diff(skip)]
    #[replay(skip)]
    #[created(skip)]
    internal_state: String,
}
```

## 字段值格式速查表

| 类型 | new_value 示例 | 自动处理 | 备注 |
|------|-----------------|---------|------|
| u64 | `"42"` | ✅ | 直接使用 |
| i64 | `"-100"` | ✅ | 直接使用 |
| f64 | `"3.14"` | ✅ | 直接使用 |
| bool | `"true"` | ✅ | 直接使用 |
| String | `"\"hello\""` | ✅ | **自动去掉引号** |

## 常见模式

### 模式 1：基础 Derive（无需手动代码）
```rust
#[derive(Entity)]
struct SimpleOrder {
    id: u64,
    symbol: String,
    price: f64,
}

// 一行代码！
let order = SimpleOrder::from_created_event(&event)?;
```

### 模式 2：跳过复杂字段
```rust
#[derive(Entity)]
struct ComplexOrder {
    id: u64,
    symbol: String,
    #[created(skip)]
    cache: Arc<Mutex<Vec<String>>>,
}

let order = ComplexOrder::from_created_event(&event)?;
// cache 字段自动使用 Default::default()
```

### 模式 3：自定义构造逻辑（使用底层 API）
```rust
let order = reconstruct_from_created::<Order, _>(&event, |fields| {
    // 完全自定义的构造逻辑
    Ok(Order {
        id: fields.get("id")?.parse().ok()?,
        symbol: fields.get("symbol")?.trim_matches('"').to_string(),
        price: fields.get("price")?.parse().ok()?,
    })
})?;
```

## 错误处理

```rust
match Order::from_created_event(&event) {
    Ok(order) => { /* 重构成功 */ }
    Err(EntityError::EntityTypeMismatch { .. }) => { /* 类型不匹配 */ }
    Err(EntityError::FieldParseError { field, reason }) => {
        eprintln!("Failed to parse field '{}': {}", field, reason);
    }
    Err(e) => { /* 其他错误 */ }
}
```

## 手动实现 `FromCreatedEvent` Trait

如果 derive 宏不满足需求，可手动实现：

```rust
use diff::FromCreatedEvent;

impl FromCreatedEvent for MyType {
    fn from_created_event(entry: &ChangeLogEntry) -> Result<Self, EntityError> {
        let fields = extract_fields_from_created_event(entry)?;
        Self::from_field_map(&fields)
    }

    fn from_field_map(fields: &HashMap<String, String>)
        -> Result<Self, EntityError>
    {
        // 你的自定义逻辑
        Ok(Self { /* ... */ })
    }
}

// 使用
let obj = MyType::from_created_event(&event)?;
```

## 一句话总结

**使用 `#[derive(Entity)]` 一行代码重构实体，或使用底层 API 进行自定义构造。**

## 更多资源

- **Derive 宏详细指南**：`FROM_CREATED_EVENT_DERIVE.md`
- **底层 API 详细指南**：`CREATED_EVENT_RECONSTRUCTION.md`
- **实现细节**：`IMPLEMENTATION_SUMMARY.md`
- **测试示例**：`lib/common/entity_derive/tests/entity_derive_test.rs`
- **API 文档**：`lib/common/diff/src/diff/diff_types.rs` 中的 trait 和函数注释

