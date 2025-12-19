# Rust之从0-1低时延CEX：领域层BinLog/极度收敛仓储层SQL编写/Event Sourcing 架构详解

> **文档描述**: 本文档详细解析 rustlob 中如何实现 Event Sourcing 架构，以 MySqlDbRepo 为核心，通过 BDD 测试场景展示系统的完整工作流程。

---

## 目录

1. [Event Sourcing 基本概念](#event-sourcing-基本概念)
2. [核心组件](#核心组件)
3. [完整工作流程](#完整工作流程)
4. [事件类型详解](#事件类型详解)
5. [BDD 场景分析](#bdd-场景分析)
6. [数据库设计](#数据库设计)
7. [关键特性](#关键特性)

---

## Event Sourcing 基本概念

### 什么是 Event Sourcing?

Event Sourcing 是一种架构模式，其核心思想是：**不存储实体的当前状态，而是存储导致该状态的所有事件序列**。

```
传统架构                    Event Sourcing 架构
Order 表 (当前状态)         ChangeLog 表 (事件序列)
─────────────────          ──────────────────
order_id: 1                event_id: 1
price: 51000  ─────────>   event_type: Created
quantity: 150              timestamp: T1
filled: 50
status: filled             event_id: 2
                           event_type: Updated
                           timestamp: T2
                           changed_fields: [{price: 50000→51000}]

                           event_id: 3
                           event_type: Updated
                           timestamp: T3
                           changed_fields: [{quantity: 100→150}]
```

### 核心优势

| 优势 | 说明 |
|------|------|
| **完整审计日志** | 每一个变更都被记录，支持完整的数据追踪 |
| **事件重放** | 可以从任意时间点重构实体状态 |
| **时间旅行** | 支持查询历史状态（"10分钟前订单的价格是多少？") |
| **故障恢复** | 实体崩溃后可通过事件重放恢复 |
| **并发安全** | 基于事件序列的强一致性保证 |
| **CQRS 友好** | 支持读写分离架构 |

---

## 核心组件

### 1. Entity Trait

**作用**: 定义实体的核心行为

```rust
pub trait Entity {
    type Id;

    // 获取实体的唯一标识符
    fn entity_id(&self) -> Self::Id;

    // 获取实体类型（作为表名）
    fn entity_type() -> &'static str;

    // 计算两个版本实体的差异
    fn diff(&self, other: &Self) -> Vec<FieldChange>;

    // 从变更事件回放状态
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), EntityError>;

    // 生成创建事件
    fn track_create(&self) -> Result<ChangeLogEntry, EntityError>;

    // 生成更新事件（带闭包处理修改）
    fn track_update<F>(&mut self, f: F) -> Result<ChangeLogEntry, EntityError>
    where
        F: FnOnce(&mut Self);

    // 生成删除事件
    fn track_delete(&self) -> Result<ChangeLogEntry, EntityError>;
}
```

**由 `entity_derive` 宏自动生成**，开发者只需标记：

```rust
#[derive(Debug, Clone, entity_derive::Entity)]
struct TestEntity {
    id: u64,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    filled_quantity: Quantity,
    side: Side,
}
```

### 2. ChangeLogEntry（事件）

**作用**: 表示系统中发生的一个不可变事件

```rust
pub struct ChangeLogEntry {
    /// 实体唯一标识符
    pub entity_id: String,

    /// 实体类型（对应表名）
    pub entity_type: String,

    /// 事件类型
    pub change_type: ChangeType,

    /// 事件时间戳（毫秒）
    pub timestamp: u64,

    /// 事件序列号（递增）
    pub sequence: u64,
}

pub enum ChangeType {
    // 实体创建事件：包含所有初始字段
    Created {
        fields: Vec<FieldChange>
    },

    // 实体更新事件：只包含变更的字段
    Updated {
        changed_fields: Vec<FieldChange>
    },

    // 实体删除事件：无额外信息
    Deleted,
}

pub struct FieldChange {
    pub field_name: String,      // 字段名
    pub old_value: String,       // 旧值（Debug 格式）
    pub new_value: String,       // 新值（Debug 格式）
}
```

### 3. MySqlDbRepo（事件仓储）

**作用**: 处理事件持久化和重放

```rust
pub struct MySqlDbRepo<E: Entity> {
    connection: Option<mysql::PooledConn>,
    _entity: std::marker::PhantomData<E>
}

impl<E: Entity + FromCreatedEvent> DBRepo for MySqlDbRepo<E> {
    type E = E;

    /// 核心方法：回放单个事件
    fn replay_event(&mut self, event: &ChangeLogEntry) -> Result<(), RepoError>;
}
```

---

## 完整工作流程

### 核心流程图

```
┌─────────────────────────────────────────────────────────────┐
│                      Event Sourcing 流程                      │
└─────────────────────────────────────────────────────────────┘

1️⃣  创建实体
   ┌─────────────┐
   │  新实体对象  │
   └──────┬──────┘
          │
          ▼
   ┌──────────────────────┐
   │  entity.track_create()  │
   └──────┬───────────────┘
          │ 生成 Created 事件
          ▼
   ┌─────────────────────┐
   │  ChangeLogEntry     │
   │  - entity_id: "1"   │
   │  - change_type:     │
   │    Created {        │
   │      fields: [...]  │
   │    }                │
   │  - timestamp: T1    │
   │  - sequence: 1      │
   └──────┬──────────────┘
          │
          ▼
   ┌──────────────────────────┐
   │  repo.replay_event()      │
   │  (事件仓储处理)            │
   └──────┬───────────────────┘
          │
          ├─→ 检查实体类型匹配 ✓
          ├─→ 检查实体不存在 ✓
          ├─→ 从事件重构实体
          ├─→ INSERT 到数据库
          │
          ▼
   ┌──────────────────────┐
   │  数据库表 (TestEntity) │
   │  entity_id │ price │  │
   │    1       │50000  │  │
   └──────────────────────┘


2️⃣  更新实体（关键！）
   ┌──────────────────────┐
   │  已有实体对象         │
   │  price: 50000        │
   │  quantity: 100       │
   └──────┬───────────────┘
          │
          ▼
   ┌──────────────────────────┐
   │  entity.track_update(|e| {│
   │      e.price = 51000;    │
   │      e.quantity = 150;   │
   │  })                      │
   └──────┬───────────────────┘
          │
          ├─→ 创建副本
          ├─→ 应用修改
          ├─→ 计算 Diff
          │    old: {price: 50000, quantity: 100}
          │    new: {price: 51000, quantity: 150}
          │    diff: [
          │      {price: 50000→51000},
          │      {quantity: 100→150}
          │    ]
          │
          ▼
   ┌──────────────────────────────────┐
   │  ChangeLogEntry (Updated 事件)    │
   │  - entity_id: "1"                │
   │  - change_type: Updated {        │
   │      changed_fields: [           │
   │        {field: price, 50000→51000}
   │        {field: quantity, 100→150}│
   │      ]                           │
   │    }                             │
   │  - timestamp: T2                 │
   │  - sequence: 2                   │
   └──────┬───────────────────────────┘
          │
          ▼
   ┌──────────────────────────┐
   │  repo.replay_event()      │
   │  (事件仓储处理)            │
   └──────┬───────────────────┘
          │
          ├─→ 检查实体类型匹配 ✓
          ├─→ 检查实体已存在 ✓
          ├─→ 加载当前实体
          ├─→ 应用变更 (entity.replay)
          ├─→ UPDATE 数据库
          │
          ▼
   ┌──────────────────────────┐
   │  数据库表 (TestEntity)    │
   │  entity_id│price│quantity│
   │    1      │51000│ 150    │
   └──────────────────────────┘


3️⃣  删除实体
   ┌──────────────────────┐
   │  entity.track_delete()│
   └──────┬───────────────┘
          │
          ▼
   ┌──────────────────────┐
   │  ChangeLogEntry      │
   │  (Deleted 事件)       │
   │  - sequence: 3       │
   └──────┬───────────────┘
          │
          ▼
   ┌──────────────────────────┐
   │  repo.replay_event()      │
   │  (DELETE 操作)             │
   └──────────────────────────┘
```

---

## 事件类型详解

### Created 事件（创建）

**场景**: 新实体首次创建

```
输入: 新的实体对象
    TestEntity {
        id: 1,
        symbol: "BTCUSDT",
        price: 50000,
        quantity: 100,
        filled_quantity: 0,
        side: Buy
    }

输出: ChangeLogEntry
    {
        entity_id: "1",
        entity_type: "TestEntity",
        change_type: Created {
            fields: [
                FieldChange { field: "id", old: "", new: "1" },
                FieldChange { field: "symbol", old: "", new: "BTCUSDT" },
                FieldChange { field: "price", old: "", new: "50000" },
                FieldChange { field: "quantity", old: "", new: "100" },
                FieldChange { field: "filled_quantity", old: "", new: "0" },
                FieldChange { field: "side", old: "", new: "Buy" }
            ]
        },
        timestamp: 1000,
        sequence: 1
    }

数据库操作:
    INSERT INTO TestEntity (
        entity_id, entity_type, timestamp, sequence,
        id, symbol, price, quantity, filled_quantity, side
    ) VALUES (
        '1', 'TestEntity', 1000, 1,
        '1', 'BTCUSDT', '50000', '100', '0', 'Buy'
    )

特点:
  ✓ old_value 总是空
  ✓ 包含所有字段（完整快照）
  ✓ 支持实体重构 (FromCreatedEvent)
  ✓ 幂等性：重复应用 Created 事件无害
```

### Updated 事件（更新）

**场景**: 实体状态变化

```
输入: 实体修改
    Before: price = 50000, quantity = 100
    After:  price = 51000, quantity = 150

计算过程:
    1. 创建实体副本
    2. 应用修改闭包
    3. 与原实体计算 Diff

       diff() 的逻辑:
       for field in entity_fields {
           if self.field != other.field {
               changes.push(FieldChange {
                   field_name: field,
                   old_value: format!("{:?}", self.field),
                   new_value: format!("{:?}", other.field),
               })
           }
       }

输出: ChangeLogEntry
    {
        entity_id: "1",
        entity_type: "TestEntity",
        change_type: Updated {
            changed_fields: [
                FieldChange { field: "price", old: "50000", new: "51000" },
                FieldChange { field: "quantity", old: "100", new: "150" }
            ]
        },
        timestamp: 2000,
        sequence: 2
    }

数据库操作:
    UPDATE TestEntity SET
        timestamp = 2000,
        sequence = 2,
        price = '51000',
        quantity = '150'
    WHERE entity_id = '1' AND entity_type = 'TestEntity'

特点:
  ✓ 只包含变更的字段（增量）
  ✓ 保留 old_value 用于审计
  ✓ 要求实体已存在（OrderNotFound 失败）
  ✓ 无法更新不存在的实体
```

### Deleted 事件（删除）

**场景**: 实体被删除

```
输入: 实体 ID

输出: ChangeLogEntry
    {
        entity_id: "1",
        entity_type: "TestEntity",
        change_type: Deleted,
        timestamp: 3000,
        sequence: 3
    }

数据库操作:
    DELETE FROM TestEntity
    WHERE entity_id = '1' AND entity_type = 'TestEntity'

特点:
  ✓ 无额外数据
  ✓ 幂等性：重复删除不报错
  ✓ 后续更新失败（OrderNotFound）
```

---

## BDD 场景分析

### 场景 1: 单字段更新

```
✓ 演示: 单个字段变更事件生成
✓ 验证: entity.track_update 能正确识别变更

Given:  price = 50000
When:   price 改为 51000
Then:   事件包含 1 个 FieldChange
```

### 场景 2: 多字段更新

```
✓ 演示: 多个字段同时变更
✓ 验证: 所有变更都被捕获

Given:  price = 3000, quantity = 100, filled_quantity = 0
When:   price→3500, quantity→150, filled_quantity→50
Then:   事件包含 3 个 FieldChange
```

### 场景 3: 不存在实体的更新失败

```
✓ 演示: 安全保证机制
✓ 验证: 无法更新从未创建的实体

Given:  repo 中无此实体
When:   调用 repo.replay_event(Updated)
Then:   返回 OrderNotFound 错误
        │
        └─→ 这是关键的一致性保证！
```

### 场景 4: 连续多次更新

```
✓ 演示: 事件序列重放
✓ 验证: 事件序列号递增

Step 1: 创建   (sequence: 1)
        INSERT ...

Step 2: 更新价格 (sequence: 2)
        UPDATE ... SET price = 2

Step 3: 更新成交量 (sequence: 3)
        UPDATE ... SET filled_quantity = 5000

结果: 实体最终状态 = 经过所有事件重放得到
```

### 场景 5: 事件包含完整变更信息

```
✓ 演示: 事件的完整性
✓ 验证: 能否从事件重构实体

Given:  price: 200 → 250
        filled_quantity: 0 → 25

Then:   事件 changed_fields 包含:
        [
            { field: "price", old: "200", new: "250" },
            { field: "filled_quantity", old: "0", new: "25" }
        ]
```

### 场景 6: 删除实体后无法更新

```
✓ 演示: 状态机约束
✓ 验证: 已删除实体的操作被拒绝

Step 1: 创建实体
        entity_exists = true

Step 2: 删除实体
        entity_exists = false

Step 3: 尝试更新
        entity_exists? → false
        返回 OrderNotFound ✓
```

### 场景 7: Diff 计算准确性

```
✓ 演示: Diff 算法
✓ 验证: 变更识别

Given:
    v1: price = 30, quantity = 100, filled = 0
    v2: price = 35, quantity = 100, filled = 50

When:  diffs = v1.diff(v2)

Then:  diffs = [
    { field: "price", old: "30", new: "35" },
    { field: "filled_quantity", old: "0", new: "50" }
]
    (quantity 未变，不出现在 diffs 中)
```

### 场景 8: 完整生命周期

```
✓ 演示: 整个系统流程
✓ 验证: 端到端的一致性

Timeline:
  T1: Created   → INSERT (sequence: 1)
  T2: Updated   → UPDATE (sequence: 2)
  T3: Deleted   → DELETE (sequence: 3)
  T4: Deleted   → DELETE (sequence: 4, 幂等)

数据库状态演变:
  After T1: 实体存在，初始值
  After T2: 实体存在，更新后的值
  After T3: 实体删除，记录删除时间
  After T4: 幂等，无副作用
```

---

## 数据库设计

### 表结构（多表设计）

每个实体类型对应一个表：

```sql
-- Order 表
CREATE TABLE Order (
    entity_id VARCHAR(255) PRIMARY KEY,
    entity_type VARCHAR(100),
    timestamp BIGINT,
    sequence BIGINT,
    price DECIMAL(18,8),
    quantity DECIMAL(18,8),
    filled_quantity DECIMAL(18,8),
    side VARCHAR(10),
    -- 元数据
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    INDEX idx_timestamp (timestamp),
    INDEX idx_entity_type (entity_type)
) ENGINE=InnoDB;

-- Trade 表（不同实体类型）
CREATE TABLE Trade (
    entity_id VARCHAR(255) PRIMARY KEY,
    entity_type VARCHAR(100),
    timestamp BIGINT,
    sequence BIGINT,
    order_id VARCHAR(255),
    price DECIMAL(18,8),
    quantity DECIMAL(18,8),
    -- ...
) ENGINE=InnoDB;
```

### 审计表（可选）

```sql
-- 保存完整事件历史
CREATE TABLE ChangeLog (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    entity_id VARCHAR(255),
    entity_type VARCHAR(100),
    change_type ENUM('Created', 'Updated', 'Deleted'),
    old_values JSON,
    new_values JSON,
    timestamp BIGINT,
    sequence BIGINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    UNIQUE KEY unique_event (entity_type, entity_id, sequence),
    INDEX idx_entity (entity_type, entity_id),
    INDEX idx_timestamp (timestamp)
) ENGINE=InnoDB;
```

---

## 关键特性

### 1. 幂等性（Idempotence）

**定义**: 重复应用同一事件多次，效果与应用一次相同

```rust
// 创建事件幂等性
let event = entity.track_create()?;
repo.replay_event(&event)?;  // 第一次：INSERT
repo.replay_event(&event)?;  // 第二次：已存在，返回 Ok（不插入）

// 删除事件幂等性
repo.replay_event(&delete_event)?;  // 第一次：DELETE
repo.replay_event(&delete_event)?;  // 第二次：已删除，返回 Ok（不删除）
```

**实现机制** (在 MySqlDbRepo::replay_event):

```rust
ChangeType::Created { .. } => {
    // 检查：实体是否已存在
    if self.entity_exists(...)? {
        return Ok(());  // ← 幂等处理
    }
    self.insert_entity(...)?;
    Ok(())
}

ChangeType::Deleted => {
    // 检查：实体是否存在
    if !self.entity_exists(...)? {
        return Ok(());  // ← 幂等处理
    }
    self.delete_entity(...)?;
    Ok(())
}
```

### 2. 事件序列化（Event Serialization）

**为什么需要?**

- 事件必须持久化到数据库
- 支持跨系统传输（消息队列、RPC）
- 用于时间旅行和重放

**格式**:

```
字段值序列化为字符串（Debug 格式）：
- 数值: "50000"
- 字符串: "\"BTCUSDT\"" (带转义)
- 类型: 隐含在字段名中
```

### 3. 事件排序（Event Ordering）

**两个维度**:

1. **时间戳** (timestamp): 事件发生的真实时间
   - 用于时间查询
   - 可能因时钟偏差不连续

2. **序列号** (sequence): 递增的逻辑顺序
   - 保证事件的因果关系
   - 检测事件丢失

```rust
// 序列号必须连续
expected_sequence = last_sequence + 1;
if event.sequence != expected_sequence {
    Err("事件丢失或顺序错误")
}
```

### 4. 状态重构（State Reconstruction）

**概念**: 从事件序列重建任意时间点的实体状态

```rust
// 重构当前状态
fn reconstruct_current(entity_id: &str) -> Result<Entity> {
    let events = db.load_events_for(entity_id)?;

    let mut entity = Entity::default();

    for event in events {
        entity.replay(&event)?;
    }

    Ok(entity)  // 最终状态
}

// 时间旅行：重构历史状态
fn reconstruct_at_time(entity_id: &str, target_time: u64) -> Result<Entity> {
    let events = db.load_events_for(entity_id)?;

    let mut entity = Entity::default();

    for event in events {
        // 只应用 timestamp 早于 target_time 的事件
        if event.timestamp <= target_time {
            entity.replay(&event)?;
        } else {
            break;
        }
    }

    Ok(entity)  // target_time 时的状态
}
```

### 5. 一致性保证（Consistency Guarantees）

| 保证 | 机制 | 示例 |
|------|------|------|
| **强一致性** | 事件序列化 | 事件 seq 1, 2, 3 顺序执行 |
| **原子性** | 单事件单操作 | 一个事件 = 一个 SQL 操作 |
| **持久性** | ACID 数据库 | MySQL 事务 |
| **隔离性** | 乐观锁 | 通过 sequence 检测冲突 |

### 6. 时间戳管理

```rust
pub trait TimestampProvider {
    fn now() -> u64;  // 毫秒
}

// 确保事件有正确的时间戳
let timestamp = TimestampProvider::now();
let event = ChangeLogEntry {
    timestamp,
    // ...
};

// 用于：
// 1. 时间范围查询
// 2. 超时检测
// 3. 时间旅行
```

---

## 常见问题

### Q1: 如何处理并发更新?

**问题**: 两个线程同时更新同一实体

**解决方案**: 基于序列号的乐观锁

```rust
// 线程 A
event_a = entity.track_update(...)?;  // seq: 2
repo.replay_event(&event_a)?;         // OK

// 线程 B
event_b = entity.track_update(...)?;  // seq: 2 (冲突!)
repo.replay_event(&event_b)?;         // 可能冲突

// 检测: 如果 sequence 重复，表示并发冲突
```

### Q2: 如何查询过去某时刻的状态?

**答**: 加载所有 timestamp 早于目标时间的事件，重放

```rust
let past_state = load_events(entity_id)
    .filter(|e| e.timestamp <= target_time)
    .fold(Entity::new(), |mut entity, event| {
        entity.replay(&event).ok();
        entity
    });
```

### Q3: 如何避免事件表过大?

**答**: 定期快照 + 增量事件

```
Snapshot (T1): 完整状态
Events (T1 ~ T1000): 增量
Snapshot (T1001): 新的完整状态
Events (T1001 ~ T2000): 增量
```

### Q4: 实体删除后能否恢复?

**答**: 是的，删除只是一个事件

```rust
// 通过事件日志恢复删除
// 1. 加载所有事件（包括 Deleted）
// 2. 重放到 Deleted 前的状态
// 3. 忽略 Deleted 事件即可恢复
```

---

## 总结

Event Sourcing 的精妙之处在于：

1. **完全的可追踪性** - 每个变更都被记录
2. **灵活的重放** - 可从任意时间点恢复
3. **强一致性** - 基于事件序列的顺序保证
4. **审计友好** - 内置完整审计日志
5. **容错能力** - 故障后可通过重放恢复

这个实现展示了如何在 Rust 中优雅地实现 Event Sourcing 模式！

---

**文档版本**: 1.0
**最后更新**: 2025-12-19
**参考文件**:
- `/lib/common/db_repo/src/adapter/mysql_db_repo.rs`
- `/lib/common/db_repo/tests/mysql_db_repo_update_bdd.rs`
