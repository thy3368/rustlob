# Changes Pair-First Shared Rule

这些约束是 RustLOB 在 `CommandUseCase4::Changes` 设计上的共享硬规则。
任何编写、评审、测试、文档化 `Changes` 的 skill，都应先读取并遵守本文件。

## Rule Goal

- `Changes` 是业务变化的唯一真相，也是 update 语义的 authoritative source。
- replayable events、reply、Rustdoc、happy-path 断言，都是从 `Changes` 往外投影，不应再并列维护第二条业务真相路径。

## Default Strategy

- create 场景：可以直接保留新实体或新业务结果，只要字段本身就是业务真相。
- update 场景：默认使用 `UpdatedEntityPair<T>` 或等价的 before/after pair-first 表达。
- 先把业务变化建模到 pair，再从 pair 的 `after` 投影出 replayable event、reply 字段或测试断言。

## Forbidden Shape

- 不要同时维护 `UpdatedEntityPair<T>` 与语义重复的 `*_after` 快照。
- 不要让调用方面对“双真相”歧义：既有 pair，又有一个可由 pair `after` 直接算出的重复结果。
- 不要把 `Changes` 退化成“pair 一份、after 再抄一份、事件再拼一份”的三轨并存结构。

反例含义：
- 调用方不应需要猜“这里该信 pair 还是信 `*_after`”。
- 如果 `*_after` 只是 `pair.after.clone()`、字段子集、或无新业务语义的别名，它就是违规重复。

## Allowed Exception

只有当单独暴露的 `after` 结果本身是独立业务结果，而不是 pair `after` 的简单投影时，才允许保留：

- 该结果承载了额外业务语义，而不只是更新后实体副本。
- 该结果面向调用方有独立含义，且不能被 pair `after` 无损替代。
- 保留它后不会制造“哪一个字段才是 authoritative truth”的歧义。

如果拿不准，默认回到 pair-first。

## Calibration Example

- `CancelSpotOrderChanges` 是校准正例：
  - update 后的订单变化应优先由 pair 表达。
  - 事件、reply、测试断言优先从 pair `after` 投影。
  - 不应再并列维护一个语义重复的 `cancelled_order_after` 一类字段。

## Review / Test / Doc Consequences

- review 时，duplicate pair + duplicate `*_after` 不是风格差异，而是明确 violation。
- happy-path tests 应先断言 pair 语义，再断言 `to_replayable_events()` 的投影一致性。
- Rustdoc 应明确说明哪些字段是 authoritative truth，update 场景优先按业务语义 pair 表达。
