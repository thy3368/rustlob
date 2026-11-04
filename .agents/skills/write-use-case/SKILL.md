---
name: write-use-case
description: Create or refactor RustLOB command-style use cases that coordinate multiple aggregates or business objects through `UseCase`, optionally adding `StateDiff` for replayable before/after truth.
---

# Write Use Case

为 RustLOB 的多聚合或多业务对象 command-style 业务目标实现 core use case。单实体或单聚合内部的状态演进属于 entity；跨聚合协调、command 校验和 authoritative state 校验属于 use case。状态加载、持久化、回放、发布、transport 映射与 reply shaping 留在 adapter / infra。

## Read First

- Contract: `lib/common/entity/src/use_case/use_case_v2.rs`
- Runtime orchestration: `lib/common/entity/src/use_case/executor.rs`
- Calibration use case: `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order.rs`
- Happy-path specification: `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order/compute_replayable_events_happy_path.rs`
- Executor wiring: `lib/example/use_case_executor/src/trading/spot/spot_block_executor.rs`
- Shared boundaries: `.agents/skills/shared/use_case_entity_constraints.md`
- Pair-first rule: `.agents/skills/shared/changes_pair_first_rule.md`

源码是最终事实源。塑造 update changes 前必须读取 pair-first rule。

## Modeling Boundary

- 仅当一个业务目标协调多个聚合或多个业务对象时实现 `UseCase`。它们应共享可匹配的 authoritative `StateGiven`、case truth 和业务编排边界。
- 将可复用的单实体不变量、构造规则和内部状态演进放到 entity 的公开业务方法中；use case 分别驱动这些方法，不让任一聚合直接装载、导航或调用另一个聚合。
- `Command` 是业务命令并实现 `LogEntryMetadata`。业务身份留在 command 中，不用 `trace_id` 代替幂等键。
- `StateGiven` 是已加载的 authoritative domain state，不是 repository handle。
- `check_command()` 只校验不依赖状态的 command facts。
- `validate_state_given()` 校验依赖已加载状态的业务不变量，并显式拒绝 branch/state mismatch。
- `compute_state_changed_only()` 在校验通过后协调业务对象并推导 case 级 after truth。调用方和测试使用公开链路 `compute_state_changed()`。
- 对同一 command 与 state，业务计算必须确定。

## Truth Model

`StateChanged` 是 case 级唯一 after truth。它可以同时包含 create 事实、update pair、跨对象重组结果或命令分支结果；不要求是单实体 after snapshot。若这些事实已足以稳定投影事件，允许 `StateChanged` 自身实现 `ReplayableChanges`，无需为了形式而增加 `StateDiff`。

仅当 replay、persist、diff 或 audit 需要把 owned `StateGiven` 的 before truth 与 `StateChanged` 合并为稳定 before/after truth 时实现 `StateDiff`：

- `do_compute_state_diff(StateGiven, StateChanged)` 消费并合并唯一业务真相；调用方使用 `compute_state_diff()`。
- `StateDiff::StateDiff` 实现 `ReplayableChanges`。
- update 默认用 `UpdatedEntityPair<T>`。pair 是 authoritative truth，禁止再保存语义重复的 `*_after`。
- create 可以直接携带新实体或独立业务结果。
- `to_replayable_events()` 只能从上述唯一真相确定性投影，遵守稳定事件顺序、实体版本和 pair-first 规则；不要维护另一条独立事件业务路径。

## Runtime Boundary

仅在 adapter/runtime wiring 使用 `UseCaseExecutor`、`StateSource` 和 `StateSink`。`UseCaseExecutor::execute()` 的实际顺序是：

```text
check_command
-> StateSource::load_given_state
-> StateDiff::compute_state_diff
   (validate_state_given -> compute_state_changed_only -> do_compute_state_diff)
-> ReplayableChanges::to_replayable_events
-> 将 command 的 lsn 和 logical_ts 写入每个 event
-> 拒绝重复 (lsn, entity_type, entity_id)
-> StateSink::persist
-> StateSink::replay
-> StateSink::publish
```

返回值是 `ExecutionResult<StateDiff>`。业务错误映射到 `ExecutionError::Business`，加载与 outbound 错误分别映射到 `LoadState`、`Persist`、`Replay`、`Publish`。事件投影失败映射到 `ProjectEvents`；重复事件键的实际路径是 `ExecutionError::ProjectEvents(EntityError::Custom(...))`，不要依赖未使用的独立错误路径。

执行器不承载 HTTP、数据库查询设计、业务规则、request-to-command、error-to-transport、metrics 或 reply shaping。

## Tests

Core use case tests:

- 通过 `check_command()` 覆盖 malformed command，包括比较边界值。
- 通过 `validate_state_given()` 覆盖 authoritative state rejection 和 branch/state mismatch。
- 通过 `compute_state_changed()` 断言成功的 case-level after truth。
- 先断言 create facts 和 update pairs；需要 replay 时，再断言 `compute_state_diff()`（若实现）与 `to_replayable_events()` 的顺序、字段和实体版本。
- update 测试先验证 pair，再验证投影，不直接拼装第二份 after truth。

Executor wiring tests 单独覆盖 state load、各 outbound error variant、command `lsn` / `logical_ts` 注入、事件顺序及重复事件键拒绝。

## Completion Check

- use case 确实协调多个聚合或多个业务对象；单实体内部演进已归入 entity。
- core 不含 DB、cache、broker、filesystem、HTTP、持久化执行、回放、发布或 reply shaping。
- `StateChanged` 是唯一 after truth；只有确需 owned before/after truth 时才有 `StateDiff`。
- update 遵守 pair-first，create 与 update 均从唯一业务真相投影。
- 生产路径不新增 `unwrap()`、`expect()`、无法解释的 `unsafe` 或吞错。
- 关注低延迟路径中的分配、clone、锁竞争、cache locality 与序列化开销。
