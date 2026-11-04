---
name: rustdoc-use-case
description: Write or refresh Rustdoc for RustLOB command, StateGiven, business error, StateChanged, and optional StateDiff use-case contracts, including meaningful public fields and small stable doctests.
---

# Rustdoc Use Case

为 RustLOB use case 的公开业务契约编写 Rustdoc。目标读者是 core 调用者与 adapter
作者；文档应让他们理解输入、authoritative state、业务拒绝、输出真相与边界，而不必阅读实现。

开始前读取当前事实来源，不从旧模板猜测 trait 名或方法名：

- `lib/common/entity/src/use_case/use_case_v2.rs`：`UseCase` / `StateDiff` 契约
- `lib/common/entity/src/use_case/executor.rs`：运行时编排与副作用顺序
- `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/match_perp_order.rs`：代表性实现
- `.agents/skills/shared/changes_pair_first_rule.md`：update changes 的共享规则

## 当前契约

- `UseCase` 是协调多个聚合或多个业务对象的 use-case family，不是单聚合内部状态演进。
- `UseCase::Command` 必须实现 `LogEntryMetadata`。文档应说明业务命令语义；只有调用者确实
  需要时才解释 LSN、逻辑时间或请求标识等日志元数据。
- `StateGiven` 是已经加载的 authoritative state，可以组合多个聚合与业务上下文。
- `StateChanged` 是 case 级 after truth；它可以是多对象重组结果或业务结果，不要求是单实体快照。
- `compute_state_changed()` 是公开统一入口，固定依次调用 `check_command()`、
  `validate_state_given()`、`compute_state_changed_only()`。不要引导外部直接调用 only hook。
- `StateDiff` 是按需实现的 replay / persist / audit 扩展，不是每个 `UseCase` 的必需部分。
  `compute_state_diff()` 消费 owned `StateGiven`，先通过 `compute_state_changed()` 计算 after truth，
  再由 `do_compute_state_diff()` 合并必要的 before/after truth。
- `StateDiff::StateDiff` 实现 `ReplayableChanges`。`to_replayable_events()` 只把唯一 changes truth
  投影为事件，不创建另一套业务真相。

## 文档对象

根据文件的公开面选择相关项，不要求机械套用固定标题或模板。

### Command

说明命令请求的业务动作、业务身份字段、幂等或顺序语义，以及单位、精度、时间基准、零值等
无法由类型直接表达的约束。区分不依赖状态的 `check_command()` 与依赖已加载状态的
`validate_state_given()`，但不要把每一行校验实现复制进类型文档。

### StateGiven

说明已加载状态代表什么、哪些对象或字段是 authoritative，以及 adapter 必须保证的排序、完整性
或匹配条件。状态由 adapter / `StateSource` 加载；不要暗示 core use case 会查询数据库或补加载。

### Error

顶层文档说明该错误类型覆盖的业务拒绝范围。公开 variant 应说明触发它的业务条件，并与
`thiserror` 的 `#[error(...)]` / `Display` 含义一致。区分业务拒绝与 adapter、存储、发布等
运行时故障，不把后者伪装成 use case error。

### Use Case

说明它协调哪些业务对象、派生什么业务变化，以及同一 command 与 state 下的确定性。
若边界容易误解，明确指出它不负责加载、持久化、回放、发布、鉴权或 transport reply shaping。
不要把单聚合内部规则描述成 use-case family 的编排职责。

### StateChanged And StateDiff

说明 strong-typed changes 携带哪些业务事实、哪些字段是 authoritative，以及 create、update 或
其他结果的区别。若实现 `StateDiff`，再说明它为 replay / persist / audit 保留了哪些 before/after
truth，以及事件如何由这些 truth 投影。

update changes 必须遵守 pair-first：

- `UpdatedEntityPair<T>` 是实体更新的 authoritative truth。
- reply、replay events、Rustdoc 与测试断言都从 pair 或其投影推导。
- 禁止把仅等价于 `pair.after` 的 `*_after` 字段写成并列真相。
- 只有额外字段承载 pair 无法无损表达的独立业务语义时才保留，并在字段文档中解释该语义。

### Public Fields

仅当字段名与类型不足以表达业务含义时补字段 Rustdoc，例如身份角色、单位、排序要求、时间基准、
authoritative status 或与其他字段的约束。避免用同义句重复字段名，也不要记录易变的私有算法步骤。

## Core 与运行时边界

使用 `core / adapter / infra` 语义保持职责准确：

- core `UseCase` 校验 command 与 given state，并计算业务 truth。
- adapter 把 transport 输入转换为 command、提供 `StateSource` / `StateSink`，并塑造 transport reply。
- infra 实现数据库、事件系统、网络与其他外部副作用。

`UseCaseExecutor` 的运行时顺序是：command pre-check、加载 authoritative state、计算 diff
（内部执行 state validation、after computation、before/after merge）、投影事件、写入日志元数据、
检查重复事件 key、persist、replay、publish。文档可以在相关时说明这个边界，但不得把这些固定
副作用写成 core use case 自己的职责。

## Doctest

仅为小型、稳定、可编译的公开 API 用法添加 doctest，例如构造 command、展示一个稳定的错误
字符串，或调用 `compute_state_changed()`。优先使用公开 re-export 路径，并确保例子不依赖私有
fixture、外部服务或大段状态搭建。复杂业务矩阵、边界组合、diff/event 一致性继续由单元测试或
BDD 测试承担；不要为了增加示例而把 Rustdoc 变成第二套测试 harness。

## 完成检查

- trait、关联类型和方法名与当前 `use_case_v2.rs` 一致。
- command、given state、error、use case、公开 changes/diff 都有必要且简洁的业务 Rustdoc。
- 只有确有非显然业务含义的 public 字段增加字段文档。
- update changes 明确遵守 pair-first，事件只是唯一 changes truth 的投影。
- 文档没有把数据库、发布、回放或 HTTP reply shaping 归入 core use case。
- doctest 小而稳定；业务覆盖仍以正式测试为准。
