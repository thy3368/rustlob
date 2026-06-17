# Rust之从0-1低时延CEX：不究竟的红皮教科书领域事件

## 问题的根源

"领域事件"这个概念在 DDD 实践中普遍存在一个隐患：它很容易退化为一种**技术通知机制**，而非真正的业务语义载体。

当我们说"发布一个 `OrderStatusUpdated` 事件"时，我们实际上在做的是：状态先变了，然后通知别人。事件是**结果的副产品**，而不是**变更的原因**。

这就是"不究竟"的本质——领域事件在大多数实践中只是一个**通知**，而不是**事实**。

---

## 两个"事件"概念的混淆

在系统设计中，"事件"这个词承载了两种截然不同的语义：

| 维度 | 领域事件（传统） | 事件溯源中的事件 |
|------|----------------|----------------|
| 本质 | 状态变更后的通知 | 状态变更的唯一原因 |
| 与状态的关系 | 状态是主体，事件是附属 | 事件是主体，状态是衍生物 |
| 可靠性 | 可能丢失、可能与状态不同步 | 是系统的唯一事实来源 |
| 设计时机 | 事后补充 | 事前设计 |
| 领域语义 | 应该有，但常常退化 | 必须有，是设计起点 |

这两个概念共用同一个词，却指向完全不同的架构地位。混淆它们，是大量系统设计问题的根源。

---

## 领域事件为何"不究竟"

### 1. 状态与事件的脱节

```rust
// 传统做法：状态先变，事件后发
order.status = OrderStatus::Cancelled;  // 状态已改变
event_bus.publish(order_cancelled_event);  // 事件可能失败、可能丢失
// 此时 order 已是 Cancelled，但事件未必到达消费者
```

事件是状态变更的**影子**，而不是**原因**。如果事件发布失败，状态已经改变了——系统进入不一致状态。

### 2. 事件可以被忽略

传统领域事件的消费者是可选的。没有消费者，事件消失，没有任何影响。这意味着事件不是系统的核心，只是一个可选的扩展点。

### 3. 无法重建历史

传统领域事件不保证完备性。你无法从事件流重建出完整的系统状态，因为事件只是通知，不是状态变更的完整记录。

### 4. 领域语义的退化

```rust
// 退化：ChangeLog 里只有技术字段，看不出业务发生了什么
ChangeLog { entity_type: "Order", change_type: Updated { changed_fields: [
    FieldChange { field_name: "status", old_value: "Pending", new_value: "Cancelled" }
]}}
// 消费者必须自己推断：为什么取消？谁取消的？

// 究竟：entity_type 本身就是业务语义，ChangeLog 携带完整上下文
ChangeLog { entity_type: "OrderCancelledByUser", change_type: Updated { changed_fields: [
    FieldChange { field_name: "cancelled_by",  old_value: "",        new_value: "user_42" },
    FieldChange { field_name: "cancel_reason", old_value: "",        new_value: "InsufficientFunds" },
    FieldChange { field_name: "status",        old_value: "Pending", new_value: "Cancelled" },
]}}
```

在压力下，开发者倾向于用通用的技术事件替代具体的业务事件，领域语义逐渐消失。

---

## 事件的统一：升华而非替代

事件溯源不是一种新技术，而是对"领域事件"这个概念的**彻底实现**。

它回答了一个根本问题：**如果领域事件真的重要，为什么不让它成为状态的唯一来源？**

```rust
// 事件溯源：事件是原因，状态是结果
// 用 track_update 产生 ChangeLog，再包装成 DomainEvent
let change_log = order.track_update(|o| o.status = OrderStatus::Cancelled)?;
let event = DomainEvent::new(change_log, order.clone());

// 状态从事件流中派生，永远与事件同步
let current_state = event_store
    .iter()
    .fold(Order::default(), |mut acc, e| { acc.replay(e.change_log()); acc });
```

### 升华后的事件具备三个特性

**必然性**：每一次状态变更，必然对应一个具有领域语义的事件。没有事件，就没有状态变更。

**纯粹性**：事件是状态变化的唯一原因。没有旁路，没有直接的状态修改。

**完备性**：事件的集合就是业务历史的全部。可以从任意时间点重建状态，可以审计每一次变更。

---

## 实践路径

### 具备溯源能力的领域事件

"究竟"的领域事件必须同时携带两样东西：**发生了什么**（变更记录）和**变更后的状态**（领域对象）。

这正是 `DomainEvent<T>` 的设计：

```rust
pub struct DomainEvent<T> {
    change_log: ChangeLog,  // 溯源记录：what changed, when, in what sequence
    state: T,               // 领域对象快照：the entity after the change
}
```

`ChangeLog` 是溯源能力的载体：

```rust
pub struct ChangeLog {
    entity_id: String,       // 哪个实体
    entity_type: String,     // 什么类型
    change_type: ChangeType, // Created / Updated { changed_fields } / Deleted
    timestamp: u64,          // 纳秒时间戳
    sequence: u64,           // 全局单调递增序号，保证顺序
}
```

`ChangeType::Updated` 中的 `changed_fields: Vec<FieldChange>` 记录了每个字段的 `old_value` 和 `new_value`，使得：

- **向前重建**：从初始状态 replay 事件流，可还原任意时刻的状态
- **向后审计**：每个 `DomainEvent` 自带完整的变更上下文，无需查询外部系统
- **因果可追溯**：`sequence` 保证事件的全局顺序，`timestamp` 提供物理时间锚点

传统领域事件只有 `T`（或者更糟，只有一个 ID）。`DomainEvent<T>` 把 `ChangeLog` 和 `T` 绑定在一起，事件本身就是完整的事实，不依赖外部状态才能被理解。

```rust
// 使用示例：订单取消产生具备溯源能力的领域事件
let change_log = order.track_update(|o| o.status = OrderStatus::Cancelled)?;
let event = DomainEvent::new(change_log, order.clone());

// 消费者可以独立理解这个事件，无需查询数据库
let (log, state) = event.into_parts();
// log.change_type => Updated { changed_fields: [{ field: "status", old: "Pending", new: "Cancelled" }] }
// state => 取消后的完整订单对象
```

### 通过 DomainEvent 的 ChangeLog 进行持久化

`DomainEvent<T>` 的持久化只需要写 `ChangeLog`，不需要写整个实体快照。状态是事件流的衍生物，可以随时 replay 重建。

```rust
// 持久化：只存 ChangeLog，不存实体快照
async fn persist(event: &DomainEvent<Order>, store: &EventStore) -> Result<(), StoreError> {
    let log = event.change_log();
    // ChangeLog 是 serde::Serialize，可直接序列化写盘
    store.append(log).await
}

### 从 ChangeLog 重放生成 SQL 写入实体数据库

`ChangeLog` 携带了足够的信息，可以直接翻译成对应实体表的 SQL，无需加载实体对象。

```rust
/// 将 ChangeLog 翻译成 SQL 并执行，实现事件驱动的数据库投影
fn apply_changelog_to_db(log: &ChangeLog, db: &mut DbConn) -> Result<(), DbError> {
    match log.change_type() {
        ChangeType::Created { fields } => {
            // INSERT INTO {entity_type} (id, field1, field2, ...) VALUES (...)
            let cols: Vec<&str> = fields.iter().map(|f| f.field_name.as_ref()).collect();
            let vals: Vec<&str> = fields.iter().map(|f| f.new_value.as_str()).collect();
            let sql = format!(
                "INSERT INTO {} (id, {}) VALUES ('{}', {})",
                log.entity_type(),
                cols.join(", "),
                log.entity_id(),
                vals.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ")
            );
            db.execute(&sql)?;
        }

        ChangeType::Updated { changed_fields } => {
            // UPDATE {entity_type} SET field1='new1', field2='new2' WHERE id='{id}'
            // sequence 作为乐观锁，防止乱序重放
            let set_clause: Vec<String> = changed_fields
                .iter()
                .map(|f| format!("{} = '{}'", f.field_name, f.new_value))
                .collect();
            let sql = format!(
                "UPDATE {} SET {}, last_sequence = {} WHERE id = '{}' AND last_sequence < {}",
                log.entity_type(),
                set_clause.join(", "),
                log.sequence(),
                log.entity_id(),
                log.sequence(),
            );
            db.execute(&sql)?;
        }

        ChangeType::Deleted => {
            // DELETE FROM {entity_type} WHERE id='{id}'
            let sql = format!(
                "DELETE FROM {} WHERE id = '{}'",
                log.entity_type(),
                log.entity_id()
            );
            db.execute(&sql)?;
        }
    }
    Ok(())
}

// 使用：消费 DomainEvent，提取 ChangeLog 写库
async fn handle_event(event: DomainEvent<Order>, db: &mut DbConn) -> Result<(), DbError> {
    let (log, _state) = event.into_parts();
    apply_changelog_to_db(&log, db)
}
```

关键点：
- `entity_type` 对应表名，`entity_id` 对应主键，`changed_fields` 对应列
- `sequence` 作为乐观锁，保证乱序到达的事件不会覆盖更新的状态
- 写库路径完全由 `ChangeLog` 驱动，不依赖实体对象，可独立部署为投影服务



```


---

## 结论

领域事件"不究竟"，根本原因在于它在大多数实践中只是一个通知机制，而不是状态变更的原因。

事件溯源是让领域事件变得"究竟"的架构保障——它从根本上颠倒了事件与状态的主从关系：**不是状态变更产生事件，而是事件产生状态**。

这不是两个不同的概念，而是同一个概念的两种实现深度：一个停留在通知层面，一个深入到事实层面。

> 事件溯源是领域事件的终极形态。它解决的不是技术问题，而是概念的彻底性问题。
