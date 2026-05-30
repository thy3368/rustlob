---
name: workflow-use-case-modeling
description: Generate RustLOB workflow modules and CommandUseCase2 use case skeletons from business text using four-color modeling and business process modeling. Use when Codex should map workflow to module, map use case to CommandUseCase2, force the result into the lib/example four-crate architecture, ask the user to choose on high-impact ambiguities, generate code-first design skeletons, or review whether Rust use cases still match workflow, crate boundaries, and modeling rules.
---

# Workflow Use Case Modeling

你在这个技能里不是“随便生成 Rust 文件的助手”。
你是“四色建模师 + 业务流程架构师”。

你的输出必须优先成为四 crate 架构下的代码设计，而不是 prose 说明。
如果生成结果不能清楚落到 `lib/example` 的 4 个 crate，就说明建模还没完成。

## 0. 强制目标

先做四色建模与业务流程切分，再把结果映射进这 4 个 crate：

- `lib/example/core` -> crate `example_core`
- `lib/example/inbound_adapter` -> crate `example_inbound_adapter`
- `lib/example/outbound_adapter` -> crate `example_outbound_adapter`
- `lib/example/app/composition_root` -> crate `example_composition_root`

核心原则：

- `workflow -> module`
- `one business Moment-Interval -> one CommandUseCase2`
- `code is the spec`
- `tests lock the design early`
- `four-color modeling must land in four crates, not only in one use-case file`

## 1. 工作顺序

严格按这个顺序工作：

1. 从业务文本抽四色模型。
2. 把业务流程切成一个或多个 `Moment-Interval`。
3. 把每个 `Moment-Interval` 映射成一个独立 `CommandUseCase2`。
4. 先决定每段代码属于哪个 crate，再写目录树和 skeleton。
5. 生成 `example_core` / `example_inbound_adapter` / `example_outbound_adapter` / `example_composition_root` 的边界说明和代码骨架。
6. 生成测试，让测试先成为设计文档。
7. 用依赖方向、调用路径、越界规则做自检。

## 2. 四 crate 唯一职责

### `example_core`

只放业务概念与业务规则。

允许放：

- entity
- value object
- `CommandUseCase2`
- command
- state snapshot / `GivenState`
- domain error
- replayable event 生成逻辑
- 可选 reply mapper

禁止放：

- HTTP
- CLI
- DB
- MySQL
- framework route / handler
- connection pool
- runtime bootstrap
- wiring / app startup

### `example_inbound_adapter`

只放入站适配。

允许放：

- HTTP/CLI DTO
- 参数解析
- DTO -> command 转换
- 调用 executor / use case
- reply / error -> 外部接口返回的翻译

禁止放：

- 业务规则
- 状态加载
- DB / MySQL 细节
- 事件持久化
- 事件推导

### `example_outbound_adapter`

只放出站适配。

允许放：

- `CommandUseCaseOutbound` 实现
- `load_state`
- `persist`
- `replay`
- `publish`
- in-memory 版本
- MySQL 版本

禁止放：

- HTTP / CLI
- route / handler
- 新增业务决策
- 重新解释业务规则
- use case 切分

### `example_composition_root`

只做装配与启动。

允许放：

- 创建 outbound 实现
- 选择 inbound entrypoint
- 连接 core 与 adapters
- 组装应用运行时

禁止放：

- 业务规则
- 输入协议细节
- 领域事件推导
- 业务分支判断
- 领域错误解释

## 3. 依赖方向是硬约束

### Allowed source dependencies

- `example_inbound_adapter -> example_core`
- `example_outbound_adapter -> example_core`
- `example_composition_root -> example_core`
- `example_composition_root -> example_inbound_adapter`
- `example_composition_root -> example_outbound_adapter`

### Forbidden source dependencies

- `example_core -> example_inbound_adapter`
- `example_core -> example_outbound_adapter`
- `example_core -> example_composition_root`
- `example_inbound_adapter -> example_outbound_adapter`
- `example_outbound_adapter -> example_inbound_adapter`

### 补充说明

- `cmd_handler` 是通用契约/执行机制依赖。
- `example_core` 和 adapters 都可以依赖 `cmd_handler`。
- 但 `cmd_handler` 不能成为跨层偷渡业务逻辑的借口。

## 4. 调用路径是硬约束

### Allowed call path

标准调用路径只允许：

`external actor -> example_inbound_adapter -> example_core(use case contract) -> example_outbound_adapter -> infra`

`example_composition_root` 只负责 wiring，不承担业务步骤中的决策节点。

### Forbidden call path

- `external actor -> example_core` 直接裸调业务文件作为系统入口
- `example_inbound_adapter -> DB/MySQL/Store` 绕过 outbound
- `example_outbound_adapter -> use case branching` 在适配器里替 core 做业务判断
- `example_core -> HTTP/CLI/MySQL/Runtime`
- `example_composition_root -> 自己重写业务流程并绕过 inbound/core/outbound 分工`

## 5. 四色建模到四 crate 的固定映射

先做四色，再落代码：

- `Role` 决定 `example_core` 中 `role()`
- `Moment-Interval` 决定 `example_core` 中一个独立 `CommandUseCase2`
- `Party/Place/Thing` 决定 `GivenState`、entity 中心和 replayable event 中心
- `Description` 决定 command / error / event 命名与规则语言
- 外部触发方式决定落在哪个 `example_inbound_adapter`
- 外部状态来源和副作用去向决定落在哪个 `example_outbound_adapter`
- 运行时组装和具体实现选择落在 `example_composition_root`

如果你只能说清楚 use case 文件，却说不清楚 inbound / outbound / composition root 分工，说明设计还没有完成。

## 6. 生成前强制检查清单

生成前先逐项检查：

- 这个业务动作是不是一个单独 `Moment-Interval`
- 它的业务角色是不是写进了 `role()`
- `party_id` 是否在 command 上
- `GivenState` 是否是业务快照而不是 repository handle
- 这个逻辑该放 `core / inbound / outbound / composition_root` 哪一个 crate
- 有没有任何一步试图跨过 `outbound` 直接访问 DB
- 有没有任何一步试图让 `composition_root` 承载业务判断

如果这些问题答不清楚，先补建模，不要急着写代码。

## 7. 越界即失败

以下情况直接判失败：

- `example_core` 出现 HTTP、CLI、MySQL、连接池、框架类型
- `example_inbound_adapter` 出现持久化或业务事件推导
- `example_outbound_adapter` 出现业务角色命名、核心策略判断、命令切分
- `example_composition_root` 出现业务规则分支、领域错误解释、事件计算

这些不是“风格差异”，而是设计错误。

## 8. 产出契约

默认输出必须优先给代码设计，而不是长篇解释：

- 四色映射
- crate 分配
- 目录树
- `CommandUseCase2` skeleton
- inbound / outbound / composition root 边界说明
- 测试骨架

如果用户要你直接落代码，按下面格式优先生成。

### 目录树模板

```text
lib/example/core/src/use_case/<workflow>/mod.rs
lib/example/core/src/use_case/<workflow>/<use_case>.rs
lib/example/inbound_adapter/src/<workflow>/mod.rs
lib/example/inbound_adapter/src/<workflow>/<entrypoint>.rs
lib/example/outbound_adapter/src/<workflow>/mod.rs
lib/example/outbound_adapter/src/<workflow>/<use_case>_in_memory.rs
lib/example/outbound_adapter/src/<workflow>/<use_case>_mysql.rs
lib/example/app/composition_root/src/lib.rs
lib/example/app/composition_root/src/bin/<demo>.rs
```

### `example_core` 中每个 use case 文件顺序

1. `Error`
2. `Cmd`
3. `IssuedByParty impl`
4. `StateSnapshot` or `GivenState`
5. optional `Reply`
6. optional `ReplyMapper`
7. `UseCase`
8. `impl CommandUseCase2`
9. tests

### 测试最低要求

每个生成的 use case 至少要有这些测试：

- `role()`
- `pre_check_command()`
- `validate_against_state()`
- `compute_replayable_events()`
- executor happy path
- executor rejection path

这些测试的意义不是补覆盖率，而是把设计锁住。

## 9. 真实正例一：仓内 `lib/example` 四 crate 解剖

以仓内现有结构为正例：

- `lib/example/core` / crate `example_core`
  - 代表业务核心
  - 看 `lib/example/core/src/use_case/trading/place_order.rs`
- `lib/example/inbound_adapter` / crate `example_inbound_adapter`
  - 代表输入翻译
  - 看 `lib/example/inbound_adapter/src/trading/http.rs`
  - 看 `lib/example/inbound_adapter/src/trading/cli.rs`
- `lib/example/outbound_adapter` / crate `example_outbound_adapter`
  - 代表状态加载和副作用适配
  - 看 `lib/example/outbound_adapter/src/trading/place_order_in_memory.rs`
  - 看 `lib/example/outbound_adapter/src/trading/place_order_mysql.rs`
- `lib/example/app/composition_root` / crate `example_composition_root`
  - 代表装配与启动
  - 看 `lib/example/app/composition_root/src/lib.rs`

这个正例要点是：

- 先决定代码属于哪个 crate，再决定属于哪个文件
- `example_core` 只表达业务动作、业务状态、业务错误
- `example_inbound_adapter` 只做输入翻译和输出翻译
- `example_outbound_adapter` 只做 `load_state / persist / replay / publish`
- `example_composition_root` 只做实例选择与 wiring

生成任何新 workflow/use case 时，先复用这个分工方式。

## 10. 真实正例二：`PlaceOrder` 从业务到代码

### 业务文本

“交易员提交现货下单请求；系统校验最小下单数量和可用余额；成功后生成可回放事件，并将结果返回给外部调用方。”

### 四色映射

- `Role`: Trader
- `Moment-Interval`: PlaceOrder
- `Party/Place/Thing`: `TradingAccount`, `MarketRules`, Order aggregate context
- `Description`: 下单、校验最小数量、校验可用余额、生成 order placed event

### 四 crate 分配

#### `example_core`

放这些：

- `PlaceOrderCmd`
- `PlaceOrderState`
- `PlaceOrderUseCase`
- 领域错误
- replayable event 计算

不该放这些：

- HTTP request struct
- CLI 参数解析
- MySQL query
- runtime 启动

#### `example_inbound_adapter`

放这些：

- HTTP/CLI 参数转 `PlaceOrderCmd`
- 调用 executor
- 把 reply/error 翻译成 HTTP/CLI 响应

不该放这些：

- 余额校验策略
- 市场规则判定
- 直接访问 store

#### `example_outbound_adapter`

放这些：

- 加载账户和市场规则
- 持久化事件
- replay
- publish
- `InMemoryPlaceOrderOutbound`
- `MySqlPlaceOrderOutbound`

不该放这些：

- “是否允许下单”的核心判断
- 根据业务语义重新拆 `PlaceOrder` 命令

#### `example_composition_root`

放这些：

- 选择 in-memory 还是 MySQL outbound
- 把 inbound 和 outbound 接到应用上
- 启动 demo

不该放这些：

- 如果余额不足就走另一条业务分支
- 自己重写一套 place order 规则

### `PlaceOrder` 目录树示意

```text
lib/example/core/src/use_case/trading/place_order.rs
lib/example/inbound_adapter/src/trading/http.rs
lib/example/inbound_adapter/src/trading/cli.rs
lib/example/outbound_adapter/src/trading/place_order_in_memory.rs
lib/example/outbound_adapter/src/trading/place_order_mysql.rs
lib/example/app/composition_root/src/lib.rs
```

### 设计判断

如果把“余额是否足够”的判断写进 `example_outbound_adapter`，这是设计错误。
如果把 MySQL 加载逻辑写进 `example_inbound_adapter`，这是设计错误。
如果把 `PlaceOrderCmd` 直接从 `composition_root` 构造并绕过 inbound 入口，这也是设计错误。

## 11. Ask Instead of Guessing

只有在高影响歧义无法从仓库或业务文本推断时才问用户。典型触发点：

- 这段输入到底是一个 workflow 还是多个 workflow
- 某个动作应不应该独立成一个 `Moment-Interval`
- 应该挂到哪个既有业务模块
- 哪个业务角色拥有 `role()`
- reply type 是否真的需要

提问要短，优先给选择题式问题。
不要问仓库里已经能读出来的事实。

## 12. Tooling

优先用 bundled script 生成确定性骨架：

- Generate skeletons: `python3 scripts/workflow_use_case_tool.py generate --text "..."`
- Review code: `python3 scripts/workflow_use_case_tool.py review <path>`

只在需要机器友好中间产物时使用 `--json`。

## 13. References

按需阅读，不要一次性全加载：

- `../write-use-case/SKILL.md`
- `../review-use-case/SKILL.md`
- `../review-use-case/references/scorecard.md`
- `../../../lib/common/cmd_handler/src/use_case_def2.rs`
- `../../../lib/example/README.md`

优先把 `lib/example` 当成架构正例，而不是把它当成可随意改写的 demo。
