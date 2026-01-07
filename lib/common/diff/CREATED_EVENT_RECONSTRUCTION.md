# Created 事件重构实体指南

## 概述

本指南介绍如何从 `Created` 类型的 `ChangeLogEntry` 中重构实体实例。这是事件溯源系统中的一个核心模式：

```
输入：ChangeLogEntry { ChangeType::Created { fields } } + Type
输出：新的实体实例
```

## 核心 API

### 1. `FromCreatedEvent` Trait

允许类型定义如何从 Created 事件重构自己：

```rust
pub trait FromCreatedEvent: Sized {
    fn from_created_event(entry: &ChangeLogEntry) -> Result<Self, EntityError>;
    fn from_field_map(fields: &HashMap<String, String>) -> Result<Self, EntityError>;
}
```

### 2. 辅助函数

#### `extract_fields_from_created_event`
从 Created 事件中提取字段值映射表：

```rust
let fields = extract_fields_from_created_event(&created_event)?;
// fields: HashMap<String, String>
// 例如：{"id" -> "1", "symbol" -> "\"BTCUSDT\""}
```

#### `parse_field_value`
解析字符串字段值为特定类型：

```rust
let value = parse_field_value("42", "u64")?;  // "42"
let value = parse_field_value("\"hello\"", "string")?;  // "hello"
```

#### `reconstruct_from_created`
使用闭包构造器从 Created 事件重构实体：

```rust
let order = reconstruct_from_created::<Order, _>(&entry, |fields| {
    let id = fields.get("id")?.parse().ok()?;
    let symbol = fields.get("symbol")?;
    Ok(Order { id, symbol })
})?;
```

## 工作流程

### 第一步：生成 Created 事件

#### 方式 1：自动追踪
```rust
let order = Order { id: 1, symbol: "BTCUSDT".to_string(), price: 50000.0 };
let entry = order.track_create()?;  // 生成基础 Created 事件
```

#### 方式 2：手动构建（包含字段信息）
```rust
let created_event = ChangeLogEntry::new(
    "1",
    "Order",
    ChangeType::Created {
        fields: vec![
            FieldChange::new("id", "", "1"),
            FieldChange::new("symbol", "", "\"BTCUSDT\""),
            FieldChange::new("price", "", "50000.0"),
        ],
    },
    1000,  // timestamp
    1,     // sequence
);
```

### 第二步：从事件重构实体

#### 方式 1：使用 `reconstruct_from_created` 闭包

```rust
let order = reconstruct_from_created::<Order, _>(&created_event, |fields| {
    let id = fields
        .get("id")
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or(EntityError::FieldParseError {
            field: "id".to_string(),
            reason: "missing id".to_string(),
        })?;

    let symbol = fields
        .get("symbol")
        .map(|v| {
            if v.starts_with('\"') && v.ends_with('\"') {
                v[1..v.len() - 1].to_string()
            } else {
                v.clone()
            }
        })
        .ok_or(EntityError::FieldParseError {
            field: "symbol".to_string(),
            reason: "missing symbol".to_string(),
        })?;

    let price = fields
        .get("price")
        .and_then(|v| v.parse::<f64>().ok())
        .ok_or(EntityError::FieldParseError {
            field: "price".to_string(),
            reason: "invalid price".to_string(),
        })?;

    Ok(Order { id, symbol, price })
})?;
```

#### 方式 2：实现 `FromCreatedEvent` Trait

```rust
impl FromCreatedEvent for Order {
    fn from_created_event(entry: &ChangeLogEntry) -> Result<Self, EntityError> {
        let fields = extract_fields_from_created_event(entry)?;
        Self::from_field_map(&fields)
    }

    fn from_field_map(fields: &HashMap<String, String>) -> Result<Self, EntityError> {
        let id = fields
            .get("id")
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or(EntityError::FieldParseError {
                field: "id".to_string(),
                reason: "invalid id".to_string(),
            })?;

        let symbol = fields
            .get("symbol")
            .map(|v| strip_quotes(v))
            .ok_or(EntityError::FieldParseError {
                field: "symbol".to_string(),
                reason: "missing symbol".to_string(),
            })?;

        let price = fields
            .get("price")
            .and_then(|v| v.parse::<f64>().ok())
            .ok_or(EntityError::FieldParseError {
                field: "price".to_string(),
                reason: "invalid price".to_string(),
            })?;

        Ok(Order { id, symbol, price })
    }
}

// 使用
let order = Order::from_created_event(&created_event)?;
```

## 字段值格式

Created 事件中的字段格式：

| 类型 | old_value | new_value 示例 | 备注 |
|------|-----------|-----------------|------|
| u64/i64 | "" | "42" | 直接序列化，无引号 |
| f64 | "" | "3.14" | 直接序列化，无引号 |
| bool | "" | "true" | 直接序列化，无引号 |
| String | "" | "\"hello\"" | Debug 格式，**有引号** |

## 完整示例

### 场景：订单创建事件
```rust
#[derive(Debug, Clone)]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
    quantity: u64,
}

// 创建原始订单
let original = Order {
    id: 1,
    symbol: "BTCUSDT".to_string(),
    price: 50000.0,
    quantity: 10,
};

// 步骤 1：生成 Created 事件（包含字段信息）
let created_event = ChangeLogEntry::new(
    "1",
    "Order",
    ChangeType::Created {
        fields: vec![
            FieldChange::new("id", "", "1"),
            FieldChange::new("symbol", "", "\"BTCUSDT\""),
            FieldChange::new("price", "", "50000.0"),
            FieldChange::new("quantity", "", "10"),
        ],
    },
    1000,
    1,
);

// 步骤 2：从事件重构订单
let reconstructed = reconstruct_from_created::<Order, _>(&created_event, |fields| {
    let id = fields.get("id")?.parse().ok()?;
    let symbol = fields.get("symbol").map(|s| strip_quotes(s))?;
    let price = fields.get("price")?.parse().ok()?;
    let quantity = fields.get("quantity")?.parse().ok()?;

    Some(Order { id, symbol, price, quantity }).ok_or(EntityError::Custom("".into()))
})?;

// 步骤 3：验证
assert_eq!(reconstructed.id, original.id);
assert_eq!(reconstructed.symbol, original.symbol);
assert_eq!(reconstructed.price, original.price);
assert_eq!(reconstructed.quantity, original.quantity);
```

## 应用场景

### 1. 事件溯源系统恢复
```
数据库: Created Event → 内存: Order Instance
```

### 2. 分布式系统同步
```
消息队列: Created Event → 副本节点: Order Instance
```

### 3. 快照+事件组合恢复
```
快照: Snapshot<Order> + Created Event → 新节点上重建完整状态
```

### 4. 审计和重演
```
审计日志: [Created Event] + [Updated Events] → 历史时间点的完整状态
```

## 错误处理

```rust
match reconstruct_from_created::<Order, _>(&entry, constructor) {
    Ok(order) => println!("重构成功: {:?}", order),
    Err(EntityError::EntityTypeMismatch { expected, actual }) => {
        eprintln!("类型不匹配: expected {}, got {}", expected, actual);
    }
    Err(EntityError::FieldParseError { field, reason }) => {
        eprintln!("字段 {} 解析失败: {}", field, reason);
    }
    Err(e) => eprintln!("错误: {}", e),
}
```

## 性能注意事项

1. **字段映射开销**：`extract_fields_from_created_event` 创建 HashMap，对大实体可能有性能影响
2. **字符串解析**：多次字段解析可能影响性能，考虑缓存
3. **内存使用**：临时 HashMap 占用内存，大批量重构时考虑流式处理

## 最佳实践

1. **定义清晰的字段名**：保持一致的命名约定
2. **验证字段完整性**：确保所有必需字段都存在于事件中
3. **错误上报**：明确区分不同类型的解析错误
4. **版本管理**：对实体结构变更进行版本控制
5. **测试覆盖**：包含边界情况和错误情况的测试

## 相关资源

- `FromCreatedEvent` trait：自定义重构逻辑
- `extract_fields_from_created_event`：提取字段映射
- `parse_field_value`：类型特定的值解析
- `reconstruct_from_created`：通用重构函数
