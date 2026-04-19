# 通用事件溯源框架 (Generic Event Sourcing Framework)

一个基于 Rust 的通用事件溯源实现，支持任何领域实体的事件记录和状态回放。

## ✨ 核心特性

- ✅ **通用设计**: 使用泛型和 trait，支持任何类型的领域实体
- ✅ **事件溯源**: 完整记录实体的所有状态变更
- ✅ **状态回放**: 从事件序列重建实体状态
- ✅ **事件存储**: 高性能内存事件存储（支持扩展为持久化存储）
- ✅ **事件流**: 支持事件订阅和流式处理
- ✅ **快照支持**: 可选的快照机制优化大量事件回放
- ✅ **类型安全**: 编译时保证类型正确性
- ✅ **线程安全**: 所有 trait 要求 `Send + Sync`

## 🏗️ 架构设计

```
┌─────────────────────────────────────────────────────────┐
│                     核心 Traits                          │
│                                                          │
│  EntityId       ← 实体唯一标识符                         │
│  DomainEvent    ← 领域事件接口                           │
│  Entity         ← 实体接口                               │
│  EventSourced   ← 事件溯源能力                           │
│  EventStore     ← 事件存储接口                           │
│  Snapshot       ← 快照支持（可选）                       │
└─────────────────────────────────────────────────────────┘
              │
              │ 实现
              ▼
┌─────────────────────────────────────────────────────────┐
│               具体实现（示例）                           │
│                                                          │
│  OrderId        : EntityId                               │
│  OrderEvent     : DomainEvent                            │
│  InternalOrder  : Entity + EventSourced                  │
│                                                          │
│  PositionId     : EntityId                               │
│  PositionEvent  : DomainEvent                            │
│  Position       : Entity + EventSourced                  │
└─────────────────────────────────────────────────────────┘
```

## 📚 使用指南

### 1. 定义实体 ID

```rust
use crate::proc::event_sourcing::EntityId;

// 实现 EntityId trait
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(String);

impl EntityId for OrderId {}
```

### 2. 定义领域事件

```rust
use crate::proc::event_sourcing::{DomainEvent, EventId};

#[derive(Debug, Clone)]
pub enum OrderEvent {
    Created {
        event_id: EventId,
        timestamp: u64,
        order_id: OrderId,
        quantity: f64,
        price: f64,
    },
    PartiallyFilled {
        event_id: EventId,
        timestamp: u64,
        order_id: OrderId,
        filled_qty: f64,
    },
    Cancelled {
        event_id: EventId,
        timestamp: u64,
        order_id: OrderId,
    },
}

impl DomainEvent for OrderEvent {
    type Id = OrderId;

    fn event_id(&self) -> EventId {
        match self {
            OrderEvent::Created { event_id, .. } => *event_id,
            OrderEvent::PartiallyFilled { event_id, .. } => *event_id,
            OrderEvent::Cancelled { event_id, .. } => *event_id,
        }
    }

    fn timestamp(&self) -> u64 {
        match self {
            OrderEvent::Created { timestamp, .. } => *timestamp,
            OrderEvent::PartiallyFilled { timestamp, .. } => *timestamp,
            OrderEvent::Cancelled { timestamp, .. } => *timestamp,
        }
    }

    fn entity_id(&self) -> &Self::Id {
        match self {
            OrderEvent::Created { order_id, .. } => order_id,
            OrderEvent::PartiallyFilled { order_id, .. } => order_id,
            OrderEvent::Cancelled { order_id, .. } => order_id,
        }
    }

    fn event_type(&self) -> &'static str {
        match self {
            OrderEvent::Created { .. } => "OrderCreated",
            OrderEvent::PartiallyFilled { .. } => "OrderPartiallyFilled",
            OrderEvent::Cancelled { .. } => "OrderCancelled",
        }
    }
}
```

### 3. 定义实体并实现事件溯源

```rust
use crate::proc::event_sourcing::{Entity, EventSourced, EventApplyError};

#[derive(Debug, Clone)]
pub struct Order {
    id: OrderId,
    created_at: u64,
    quantity: f64,
    price: f64,
    filled_quantity: f64,
    status: OrderStatus,
}

impl Entity for Order {
    type Id = OrderId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn created_at(&self) -> u64 {
        self.created_at
    }
}

impl EventSourced for Order {
    type Event = OrderEvent;

    fn from_event(event: &OrderEvent) -> Result<Self, EventApplyError> {
        match event {
            OrderEvent::Created {
                order_id,
                timestamp,
                quantity,
                price,
                ..
            } => Ok(Order {
                id: order_id.clone(),
                created_at: *timestamp,
                quantity: *quantity,
                price: *price,
                filled_quantity: 0.0,
                status: OrderStatus::Pending,
            }),
            _ => Err(EventApplyError::InvalidEventType {
                expected: "Created".to_string(),
                actual: event.event_type().to_string(),
            }),
        }
    }

    fn apply_event(&mut self, event: &OrderEvent) -> Result<(), EventApplyError> {
        // 验证实体 ID
        if event.entity_id() != &self.id {
            return Err(EventApplyError::EntityIdMismatch);
        }

        match event {
            OrderEvent::Created { .. } => {
                Err(EventApplyError::InvalidEventType {
                    expected: "non-Created".to_string(),
                    actual: "Created".to_string(),
                })
            }
            OrderEvent::PartiallyFilled { filled_qty, .. } => {
                self.filled_quantity = *filled_qty;
                self.status = OrderStatus::PartiallyFilled;
                Ok(())
            }
            OrderEvent::Cancelled { .. } => {
                self.status = OrderStatus::Cancelled;
                Ok(())
            }
        }
    }
}
```

### 4. 使用事件存储

```rust
use crate::proc::event_sourcing::{EventStore, EventId};
use crate::proc::event_store::InMemoryEventStore;

// 创建事件存储
let mut store: InMemoryEventStore<OrderEvent> = InMemoryEventStore::new();

// 保存事件
let event1 = OrderEvent::Created {
    event_id: EventId::new(1),
    timestamp: 1000,
    order_id: OrderId("ORDER_001".to_string()),
    quantity: 10.0,
    price: 50000.0,
};

store.save_event(event1.clone()).unwrap();

// 获取实体的所有事件
let events = store.get_events(&OrderId("ORDER_001".to_string())).unwrap();

// 回放事件重建实体
let order = Order::replay(&events).unwrap();

println!("Order ID: {:?}", order.id());
println!("Status: {:?}", order.status);
```

### 5. 事件流和订阅

```rust
// 获取事件流
let stream = store.subscribe(EventId::new(0)).unwrap();

// 处理事件流
for event in stream {
    println!("Processing event: {:?}", event.event_type());
    // 处理事件...
}
```

### 6. 时间范围查询

```rust
// 查询指定时间范围的事件
let events = store.get_events_by_time_range(1000, 2000).unwrap();

// 获取指定事件 ID 之后的所有事件
let events = store.get_events_after(EventId::new(10)).unwrap();
```

## 🔧 扩展自定义实体

框架完全通用，可以轻松扩展到任何实体类型：

### 示例：Position（持仓）实体

```rust
// 1. 定义 Position ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositionId(u64);

impl EntityId for PositionId {}

// 2. 定义 Position 事件
#[derive(Debug, Clone)]
pub enum PositionEvent {
    Opened { event_id: EventId, timestamp: u64, position_id: PositionId, ... },
    SizeIncreased { event_id: EventId, timestamp: u64, position_id: PositionId, ... },
    Closed { event_id: EventId, timestamp: u64, position_id: PositionId, ... },
}

impl DomainEvent for PositionEvent { ... }

// 3. 实现 Position 实体
pub struct Position { ... }

impl Entity for Position { ... }
impl EventSourced for Position { ... }

// 4. 使用相同的事件存储
let mut position_store: InMemoryEventStore<PositionEvent> = InMemoryEventStore::new();
```

## 📊 性能特性

### 时间复杂度

| 操作 | 复杂度 | 说明 |
|-----|--------|------|
| 保存事件 | O(log n) | BTreeMap 插入 |
| 查询实体事件 | O(m) | m = 实体事件数 |
| 时间范围查询 | O(n) | n = 总事件数 |
| 事件 ID 查询 | O(log n) | BTreeMap 范围查询 |
| 回放事件 | O(m) | m = 实体事件数 |

### 内存使用

- 每个事件: ~8-64 bytes（取决于事件数据大小）
- 索引开销: ~24 bytes/事件（BTreeMap + HashMap）
- 总内存: O(n × 事件大小)

### 优化建议

1. **使用快照**: 对于事件数量大的实体（>1000 事件），使用快照减少回放时间
2. **事件压缩**: 定期将旧事件序列化为快照
3. **持久化存储**: 扩展为 PostgreSQL/MongoDB 等持久化后端
4. **分片**: 按实体类型或 ID 范围分片存储

## 🧪 测试

运行所有事件溯源测试：

```bash
# 运行通用框架测试
cargo test --lib event_sourcing

# 运行事件存储测试
cargo test --lib event_store

# 运行所有事件相关测试
cargo test --lib event
```

测试覆盖率：
- ✅ 事件 ID 排序
- ✅ 实体创建和事件应用
- ✅ 事件回放
- ✅ 事件序列验证
- ✅ 事件流处理
- ✅ 事件存储 CRUD
- ✅ 时间范围查询
- ✅ 错误处理

## 🎯 最佳实践

### 1. 事件设计原则

```rust
// ✅ 好的事件设计：不可变、自包含
OrderEvent::Created {
    event_id: EventId::new(1),
    timestamp: 1000,
    order_id: OrderId("001"),
    quantity: 10.0,  // 包含所有必要信息
    price: 50000.0,
}

// ❌ 不好的事件设计：缺少必要信息
OrderEvent::Created {
    event_id: EventId::new(1),
    order_id: OrderId("001"),
    // 缺少 timestamp 和业务数据
}
```

### 2. 事件顺序保证

```rust
// 使用 validate_event_sequence 验证事件顺序
if let Err(e) = Order::validate_event_sequence(&events) {
    eprintln!("Invalid event sequence: {}", e);
    return;
}
```

### 3. 错误处理

```rust
match order.apply_event(&event) {
    Ok(()) => println!("Event applied successfully"),
    Err(EventApplyError::EntityIdMismatch) => {
        eprintln!("Event belongs to different entity");
    }
    Err(EventApplyError::InvalidStateTransition { from, to }) => {
        eprintln!("Invalid transition: {} -> {}", from, to);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### 4. 快照策略

```rust
// 每 100 个事件创建一次快照
if event_count % 100 == 0 {
    let snapshot = order.create_snapshot();
    snapshot_store.save(snapshot);
}

// 回放时从最近的快照开始
let snapshot = snapshot_store.get_latest(order_id)?;
let mut order = Order::from_snapshot(snapshot)?;
let events_after_snapshot = store.get_events_after(snapshot.last_event_id)?;
for event in events_after_snapshot {
    order.apply_event(&event)?;
}
```

## 📦 依赖

框架无外部依赖，仅使用 Rust 标准库：
- `std::collections::{BTreeMap, HashMap}`
- `std::fmt::Debug`
- `std::hash::Hash`

## 🔗 相关资源

- [Event Sourcing Pattern](https://martinfowler.com/eaaDev/EventSourcing.html)
- [CQRS and Event Sourcing](https://docs.microsoft.com/en-us/azure/architecture/patterns/cqrs)
- [Domain-Driven Design](https://www.domainlanguage.com/ddd/)

## 📄 许可证

This is part of the RustLOB project.

---

**生成时间**: 2025-12-15
**框架版本**: v1.0.0
