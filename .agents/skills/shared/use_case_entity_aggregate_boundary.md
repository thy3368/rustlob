# Use Case / Entity / Aggregate Boundary Reference

这是 RustLOB 关于 `use case`、`entity behavior method`、`helper/query method`、`aggregate root` 边界的 canonical reference。
当 skill 任务涉及这些边界问题时，应先读本文件，再输出判断、设计、实现或评审结论。

## Core Split

用三分法先判断当前语义属于哪一层：

- `use case`
- `entity behavior method`
- `entity helper/query method`

如果一个方法落在聚合根上，再继续区分它是在做：

- 单实体自身演化
- 同一聚合内部多个对象的一致性协调

不要跳过这一步直接按“代码写在哪里方便”来放置职责。

## Use Case

`use case` 是一个独立业务动作的边界，不只是“调几个 method 的壳”。

它负责：

- 表达独立的业务意图边界
- 承载 `command / state / changes` 的协调
- 编排一个或多个 `entity` / `aggregate` / `policy`
- 定义组合成功与组合失败的业务语义
- 承担跨聚合的顺序控制与整体成立判断

判定它更像 `use case` 的信号：

- 独立授权
- 独立失败语义
- 独立审计意义
- 独立状态推进

如果一个动作符合这些特征，就不应把它压扁成某个 `entity` 上的普通方法。

## Entity Behavior Method

`behavior method` 表达实体的合法业务演化。

它和 `state machine` 强相关：方法本质上就是“受业务约束的状态迁移边”。

这类方法通常：

- 改变实体状态，或产出与状态迁移直接相关的业务结果
- 检查本实体或本聚合内部的不变量
- 使用领域动词命名，例如 `cancel`、`release`、`consume`、`apply_fill`

它分两类：

### 单实体 behavior

适用于单个实体自身的合法迁移，例如：

- `Order::cancel`
- `Order::apply_fill`

### 聚合根 behavior

适用于同一聚合内多个子实体 / 值对象的一致性协调。

聚合根可以接收外部事实或参数来推进本聚合内部状态，但边界仍然只到“本聚合内部”。

例如：

- `AccountAggregate::reserve_funds_for_order`
- `AccountAggregate::release_hold`

## Helper / Query Method

`helper/query method` 负责：

- 业务判断
- 业务查询
- 派生计算

这类方法可以很重要，但它们不承载独立业务动作语义。

常见命名：

- `is_*`
- `can_*`
- `remaining_*`
- `available_*`

例如：

- `Order::remaining_qty`
- `Order::is_cancelable`

不要把纯判断或派生计算用“业务动词名”包装成假动作。

## Aggregate Boundary Rules

按边界分层：

- 单实体内状态迁移 -> 可放 `entity behavior method`
- 同一聚合内多个对象的一致性协调 -> 可放 `aggregate root behavior`
- 跨聚合业务协调 -> 应放 `use case`

精确规则：

- 不要把跨聚合业务编排塞进 `entity method`
- `entity method` 可以接收外部事实/参数来推进自身或本聚合内部状态
- 但它不应承担多个聚合之间的顺序控制、整体成立判断、组合失败语义

如果一个动作需要同时协调 `OrderAggregate`、`FundHoldAggregate`、`AccountAggregate` 等多个独立业务真相边界，它就应留在 `use case`。

## State Machine Rule

当问题涉及“状态机该放在哪里”时，先用下面口径回答：

- `state machine` 的合法迁移边，优先落在 `behavior method`
- 单实体状态机落在单实体方法
- 聚合内部联合状态推进落在聚合根方法
- 跨聚合的业务步骤编排，不是某个实体自己的状态机边，而是 `use case` 的职责

不要把“有状态变化”误解为“一定属于 use case”。
也不要把“是业务动作”误解为“一定应该成为 entity method”。

## Decision Mnemonic

用这组口诀快速判定：

- 独立授权 / 独立失败 / 独立审计 / 独立状态推进 -> 更像 `use case`
- 只负责本实体合法迁移 -> 更像单实体 `behavior method`
- 负责同一聚合内部协调 -> 更像聚合根 `behavior method`
- 只返回判断 / 派生值 -> 更像 `helper/query method`

## Positive Examples

- `Order::cancel`
  - 单实体 `behavior method`
- `Order::apply_fill`
  - 单实体 `behavior method`
- `AccountAggregate::reserve_funds_for_order`
  - 聚合根 `behavior method`
- `Order::remaining_qty`
  - `helper/query method`
- `Order::is_cancelable`
  - `helper/query method`
- `cancel_order` use case 协调 `OrderAggregate` + `FundHoldAggregate`
  - `use case`

## Anti-Patterns

- 不要把 `helper/query method` 假装成 business action
- 不要把跨聚合编排塞进某个 `entity` / `aggregate` method
- 不要因为某个方法名字像业务动作，就默认它真的承载了业务演化
- 不要把 `use case` 退化成只搬运字段、只拼接 trivial helper 结果的外壳

## Output Rule For Skills

如果 skill 任务涉及下面任一问题，先引用本文件的边界口径再继续：

- `use case` vs `entity`
- `behavior method`
- `helper/query method`
- `aggregate root` 该管什么
- `state machine` 与实体方法关系
- 某个动作“该不该升格成 use case”
