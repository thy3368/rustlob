# FromCreatedEvent Derive Macro 指南

## 概述

`#[derive(Entity)]` 现在可以自动为结构体实现 `FromCreatedEvent` trait，使得从 `Created` 事件重构实体变得无需编写样板代码。

## 基础用法

### 最简单的例子

```rust
use diff::{Entity, FromCreatedEvent, ChangeLogEntry, ChangeType, FieldChange};

#[derive(Debug, Clone, PartialEq, Entity)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
    quantity: u64,
    status: String,
}

// 自动生成的 FromCreatedEvent 实现让你可以直接使用：
let event = ChangeLogEntry::new(
    "1",
    "Order",
    ChangeType::Created {
        fields: vec![
            FieldChange::new("id", "", "1"),
            FieldChange::new("symbol", "", "\"BTCUSDT\""),
            FieldChange::new("price", "", "50000.0"),
            FieldChange::new("quantity", "", "10"),
            FieldChange::new("status", "", "\"pending\""),
        ],
    },
    1000, 1,
);

// 一行代码重构实体！
let order = Order::from_created_event(&event)?;
assert_eq!(order.id, 1);
assert_eq!(order.symbol, "BTCUSDT");
```

## 支持的字段类型

自动生成的代码支持以下字段类型：

### 整数类型
```rust
#[derive(Entity)]
struct Data {
    u64_field: u64,
    i64_field: i64,
    u32_field: u32,
    i32_field: i32,
    // ... 其他 u8, u16, i8, i16, usize, isize
}
```
**处理方式**：直接 `parse()` 字符串值

### 浮点数类型
```rust
#[derive(Entity)]
struct Data {
    f64_field: f64,
    f32_field: f32,
}
```
**处理方式**：直接 `parse()` 字符串值

### 布尔值
```rust
#[derive(Entity)]
struct Data {
    flag: bool,
}
```
**处理方式**：直接 `parse()` 字符串值（支持 "true" 和 "false"）

### 字符串
```rust
#[derive(Entity)]
struct Data {
    name: String,
    description: String,
}
```
**处理方式**：自动去掉 Debug 格式的引号（`"\"hello\""` → `"hello"`）

### 其他类型
```rust
#[derive(Entity)]
struct Data {
    #[created(skip)]  // 使用 Default::default()
    custom_type: ComplexType,
}
```
**处理方式**：使用 `Default::default()`

## 属性支持

### `#[created(skip)]` - 跳过字段

用于跳过无法自动解析的字段，使用 `Default::default()` 代替：

```rust
#[derive(Debug, Clone, PartialEq, Entity)]
struct Order {
    id: u64,
    symbol: String,
    #[created(skip)]
    cache: String,  // 不从事件中解析，使用 ""
}

let order = Order::from_created_event(&event)?;
assert_eq!(order.cache, "");  // Default for String
```

### 其他属性继续有效

```rust
#[derive(Debug, Clone, PartialEq, Entity)]
#[entity(id = "order_id", type_name = "CustomOrder")]
struct Order {
    order_id: u64,
    #[diff(skip)]
    #[replay(skip)]
    #[created(skip)]
    internal_state: String,
}
```

## 两种调用方式

### 方式 1：从 ChangeLogEntry 直接构造

```rust
let order = Order::from_created_event(&event)?;
```

**优势**：
- 自动提取和验证事件类型
- 完整的错误处理
- 更清晰的意图

### 方式 2：从字段映射表构造

```rust
use std::collections::HashMap;

let mut fields = HashMap::new();
fields.insert("id".to_string(), "1".to_string());
fields.insert("symbol".to_string(), "\"BTCUSDT\"".to_string());
// ...

let order = Order::from_field_map(&fields)?;
```

**优势**：
- 灵活处理已提取的字段
- 方便单元测试

## 完整示例

### 基础重构

```rust
use diff::{Entity, FromCreatedEvent, ChangeLogEntry, ChangeType, FieldChange};

#[derive(Debug, Clone, Entity)]
struct Trade {
    id: u64,
    pair: String,
    amount: f64,
    price: f64,
}

// 创建 Created 事件
let event = ChangeLogEntry::new(
    "1",
    "Trade",
    ChangeType::Created {
        fields: vec![
            FieldChange::new("id", "", "1"),
            FieldChange::new("pair", "", "\"BTC/USD\""),
            FieldChange::new("amount", "", "0.5"),
            FieldChange::new("price", "", "45000.00"),
        ],
    },
    1000, 1,
);

// 从事件重构
let trade = Trade::from_created_event(&event)?;

// 使用
println!("Trade {}: {} {} @ ${}",
    trade.id, trade.amount, trade.pair, trade.price);
```

### 包含可选字段

```rust
#[derive(Debug, Clone, Entity)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,

    // 这些字段不需要从事件中解析
    #[created(skip)]
    filled_qty: u64,

    #[created(skip)]
    created_at: std::time::SystemTime,
}

let order = Order::from_created_event(&event)?;
assert_eq!(order.filled_qty, 0);  // 使用 Default::default()
```

### 多个数值类型

```rust
#[derive(Debug, Clone, Entity)]
struct Quote {
    id: u64,
    bid: f64,
    ask: f64,
    spread: f32,
    depth: i32,
}

let event = ChangeLogEntry::new(
    "1",
    "Quote",
    ChangeType::Created {
        fields: vec![
            FieldChange::new("id", "", "1"),
            FieldChange::new("bid", "", "100.50"),
            FieldChange::new("ask", "", "100.75"),
            FieldChange::new("spread", "", "0.25"),
            FieldChange::new("depth", "", "100"),
        ],
    },
    2000, 1,
);

let quote = Quote::from_created_event(&event)?;
```

## 错误处理

### 缺少字段

```rust
let result = Order::from_created_event(&event);

match result {
    Err(diff::EntityError::FieldParseError { field, reason }) => {
        eprintln!("Failed to parse field '{}': {}", field, reason);
    }
    _ => {}
}
```

### 无效的字段值

```rust
let invalid_event = ChangeLogEntry::new(
    "2",
    "Order",
    ChangeType::Created {
        fields: vec![
            FieldChange::new("id", "", "not_a_number"),  // ❌ 无效
            FieldChange::new("symbol", "", "\"BTCUSDT\""),
        ],
    },
    3000, 2,
);

let result = Order::from_created_event(&invalid_event);
assert!(result.is_err());  // 会返回 FieldParseError
```

## 与手写版本的对比

### 手写版本（需要闭包）
```rust
let order = reconstruct_from_created::<Order, _>(&event, |fields| {
    Ok(Order {
        id: fields.get("id")?.parse().ok()?,
        symbol: fields.get("symbol")?
            .trim_matches('"').to_string(),
        price: fields.get("price")?.parse().ok()?,
        quantity: fields.get("quantity")?.parse().ok()?,
        status: fields.get("status")?
            .trim_matches('"').to_string(),
    })
})?;
```

### Derive 版本（无需样板代码）
```rust
let order = Order::from_created_event(&event)?;
```

**差异**：消除了所有样板代码！

## 最佳实践

1. **使用 derive，除非需要自定义逻辑**
   ```rust
   // ✅ 好
   #[derive(Entity)]
   struct SimpleOrder { /* ... */ }

   // ❌ 避免过度复杂化
   impl FromCreatedEvent for SimpleOrder { /* 手动实现 */ }
   ```

2. **为复杂字段使用 `#[created(skip)]`**
   ```rust
   #[derive(Entity)]
   struct Order {
       id: u64,
       #[created(skip)]
       internal_cache: Arc<Mutex<HashMap<String, Value>>>,
   }
   ```

3. **使用 `#[created(skip)]` 配合 `#[replay(skip)]`**
   ```rust
   #[derive(Entity)]
   struct Order {
       id: u64,
       #[diff(skip)]
       #[replay(skip)]
       #[created(skip)]
       transient_data: String,
   }
   ```

4. **验证事件完整性**
   ```rust
   match Order::from_created_event(&event) {
       Ok(order) => {
           // 处理成功情况
       }
       Err(e) => {
           // 记录错误，可能是数据格式问题
           warn!("Failed to reconstruct order: {}", e);
       }
   }
   ```

## 性能特性

- **编译时代码生成**：没有运行时开销
- **零分配路径**：对于简单数值类型，分配最少
- **错误快速失败**：缺少字段立即返回错误

## 故障排除

### "Cannot parse 'x' as type Y"
字段值无法转换为目标类型。检查：
- 字段值格式是否正确
- 数值是否在类型范围内

### "Missing field 'x'"
Created 事件中缺少必需字段。检查：
- 事件构造是否包含所有字段
- 字段名称是否与结构体字段匹配

### String 字段包含引号
String 字段值应该是 `"\"hello\""` 格式（Debug 格式）。自动生成的代码会处理这些引号。

## 未来扩展

可能的改进方向：
- 支持嵌套结构体
- 支持自定义类型的转换函数
- 支持字段重命名属性（`#[created(rename = "...")]`）
