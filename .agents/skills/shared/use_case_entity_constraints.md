# Use Case And Entity Shared Constraints

这些约束是 RustLOB 在 `use_case` 与 `entity` 设计上的共享硬规则。
任何编写、评审、重构 `use_case` 或 `entity` 的 skill，都应先读取并遵守本文件。

## Core Boundary Rules

- `use_case` 是单一业务动作的边界，输入是 `command/query`，输出是业务结果或可回放事件。
- `entity` 承载核心业务事实、不变式和可复用的领域语义方法。
- `use_case -> entity` 是允许的依赖方向。
- `entity` 不依赖 `use_case`。
- `entity` 不知道 `command/query`。

## Use Case Rules

- `use_case` 之间不允许互相调用。
- 一个业务动作必须收敛在单一 `use_case` 中，不要在一个 `use_case` 内直接调用另一个 `use_case`。
- 跨用例协作只能放在更上层编排中完成，例如 `workflow`、`process`、`composition root`、executor 之外的业务编排层。
- `use_case` 负责命令级校验、状态相关校验，以及基于状态推导业务结果或事件。
- `use_case` 不依赖具体 DB、HTTP、SDK、ORM、broker、runtime 等外部技术。

## Entity Rules

- `use_case` 与 `entity` 是多对一关系：多个 `use_case` 可以复用同一个 `entity`。
- `entity` 不能反向绑定某个特定 `use_case`，不能演化成某个单独流程的私有数据对象。
- `entity` 必须包含有领域语义的方法，而不是只有字段和 getter/setter。
- `entity methods` 要区分为 `behavior method` 与 `helper/query method`。
- `behavior method` 与 `state machine` 强相关，负责本实体或本聚合内部的合法业务演化。
- 聚合根可以用 `behavior method` 协调同一聚合内部多个对象的一致性。
- `helper/query method` 负责业务判断、业务查询、派生计算，但不应伪装成独立 `use case`。
- 跨聚合业务边界与协调必须保留在 `use_case`，不要下沉到 `entity` / `aggregate` method。
- 这些方法应承载可复用的业务规则、业务查询、业务计算或不变式判断，供多个 `use_case` 复用。
- 贫血 `entity` 是违规信号：如果一个类型只搬运字段、业务规则全堆在 `use_case` 或 adapter 中，应优先考虑把可复用规则下沉回 `entity`。

## Review Checklist

检查 `use_case` / `entity` 设计时，至少确认：

- 是否存在 `use_case` 互调。
- 是否把跨用例协作错误地放进单个 `use_case`。
- 是否把 `entity` 设计成只服务某一个 `use_case` 的私有流程对象。
- `entity` 是否退化为贫血数据结构。
- 可复用业务规则是否错误地散落在 adapter 或多个 `use_case` 中，而不是沉淀到 `entity`。
