# StateMachineV2Diff Pair-First Shared Rule

这些约束适用于 `StateMachineV2Diff::StateDiff` 与 replayable changes。编写、评审、测试或文档化 update changes 前先遵守本规则。

## Authoritative Truth

- update 默认使用 `UpdatedEntityPair<T>` 或等价 before/after pair。
- pair 是业务变化的 authoritative truth。
- replay events、reply、Rustdoc 和 happy-path assertions 都从 pair 或其 projected changes 推导。
- create 场景可以直接携带新实体或独立业务结果。

## Forbidden Duplication

不要同时维护 pair 与语义重复的 `*_after`：若该字段只是 `pair.after.clone()`、字段子集或无新增业务含义的别名，就形成双真相。也不要独立拼装一条与 pair 无关的事件业务路径。

只有当单独结果承载 pair 的 `after` 无法无损表达的独立业务语义时才可保留；文档必须说明其独立含义。拿不准时使用 pair-first。

## Calibration

在 `StateMachineV2Diff` update changes 中，取消订单应以订单 pair 表达。事件、reply 和测试断言从 pair 的 `after` 投影，不再并列保存重复的 `cancelled_order_after`。

Review 将 duplicate pair + duplicate `*_after` 视为 violation。Happy-path tests 先断言 pair，再断言 `to_replayable_events()` 的一致性；Rustdoc 明确指出 pair 是 authoritative truth。
