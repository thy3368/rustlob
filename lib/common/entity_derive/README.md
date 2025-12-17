# Entity Derive - 自动实体追踪 Proc Macro

为 Rust 实体自动生成变更追踪、Diff 和状态回放功能。

## 快速开始

```rust
use entity_derive::Trackable;

#[derive(Debug, Clone, PartialEq, Trackable)]
#[entity(serde = "bincode")]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
}

fn main() {
    let mut order = Order { id: 1, symbol: "BTCUSDT".into(), price: 50000.0 };

    // 🎯 自动追踪创建
    let create_log = order.track_create().unwrap();

    // 🎯 自动追踪更新（方式1：闭包）
    let update_log = order.track_update(|o| {
        o.price = 51000.0;
    }).unwrap();

    // 🎯 自动追踪删除
    let delete_log = order.track_delete().unwrap();
}
```

## 核心功能

### 1. 自动追踪 (Auto Track)

```rust
// 追踪创建
let entry = order.track_create()?;

// 追踪更新 - 闭包方式（自动 diff）
let entry = order.track_update(|o| {
    o.price = 51000.0;
    o.quantity = 2.0;
})?;

// 追踪更新 - 比较方式
let old_order = order.clone();
order.price = 51000.0;
let entry = order.track_update_from(&old_order)?;

// 追踪删除
let entry = order.track_delete()?;
```

### 2. 自动 Diff

```rust
#[derive(Diff)]
struct Order {
    id: u64,
    price: f64,
    #[diff(skip)]  // 跳过此字段
    cache: String,
}

let changes = old_order.diff(&new_order);
// changes: Vec<FieldChange>
```

### 3. 自动 Replay

```rust
#[derive(Replayable)]
struct Order {
    id: u64,
    price: f64,
    #[replay(skip)]  // 跳过回放
    cache: String,
}

order.replay(&change_log_entry)?;
```

### 4. 一键派生 (Trackable)

```rust
// 自动派生 Entity + Diff + Replayable
#[derive(Debug, Clone, Trackable)]
#[entity(serde = "bincode")]
struct Order {
    id: u64,
    symbol: String,
    price: f64,
}
```

## 配置属性

### Entity 属性

```rust
#[entity(
    id = "order_id",              // ID 字段名（默认: id）
    type_name = "CustomOrder",    // 类型名（默认: 结构体名）
    serde = "bincode"             // 序列化方式: bincode|json|custom
)]
```

### Diff 属性

```rust
#[diff(skip)]  // 跳过字段 diff
```

### Replay 属性

```rust
#[replay(skip)]                    // 跳过字段回放
#[replay(parse = "parse_decimal")] // 自定义解析函数
```

## 运行示例

```bash
cargo run --example order_example
```

## 性能特性

- ✅ **零运行时开销** - 编译时代码生成
- ✅ **无反射** - 直接字段访问
- ✅ **低延迟** - bincode 序列化 < 100ns
- ✅ **Clean Architecture** - 无外部框架依赖

## 完整示例

```bash
lib/common/entity_derive/examples/order_example.rs
```

## License

MIT
