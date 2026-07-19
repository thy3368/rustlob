# Entity Method Constraints

这是 entity 方法设计的 shared architecture policy。它不是
`lib/common/entity/src/entity.rs` 或 `MiStateMachineV2Unchecked` 的代码事实源；分类事实仍以
`.agents/skills/shared/entity_four_color_classification.md` 为准，`use case` / `entity`
边界事实仍以 `.agents/skills/shared/use_case_entity_constraints.md` 为准。

Use this file when deciding whether a method belongs on an entity, how to classify that method, and
which entity methods a use case or adapter may reasonably call.

## Method Categories

Split candidate entity methods into four buckets:

- `Behavior Method` / `行为方法`
  - 创建、推进、关闭、取消、拒绝、消耗、释放、补偿等会改变业务生命周期或合法状态的方法。
  - 也可以基于 entity / aggregate root 已稳定的业务事实，派生直接下游业务单据。
  - Aggregate-root behavior may coordinate multiple objects inside the same aggregate.
- `Business Query Method` / `业务查询方法`
  - 回答身份、归属、资产、市场、状态、资格、可用量、剩余量、业务摘要、业务要求等事实的方法。
  - It returns stable business facts, qualifications, summaries, or requirement objects without changing state.
- `Invariant Check Method` / `不变量检查方法`
  - 检查金额守恒、执行状态、版本关系、内部字段自洽、可撤销/可终止资格等方法。
  - It only states whether a self-contained truth condition holds.
- `Technical/Internal Method` / `技术/内部方法`
  - `diff`、`replay_*`、trait plumbing、serde/persistence/wire helpers, private
    `recompute_*` / `derive_*`, and intermediate calculations.
  - Keep these private or module-internal unless a trait requires otherwise.

## Archetype Bias

- `Moment-Interval` usually favors lifecycle `Behavior Method`.
- `Party/Place/Thing` often favors `Business Query Method` and `Invariant Check Method`.
- Promoted `Role` entities must justify their lifecycle and should not exist as label-only wrappers.
- `Description` objects should usually stay out of behavior-heavy entity design.
- `EntityMethodBias` is a governance signal, not an allowlist; use it to review placement together
  with the four-color archetype.

## Admission Rules

Allow a candidate to become an entity method only when it is one of these:

- `Business Query Method`: identity, ownership, market/product identity, lifecycle status,
  availability, remaining amount, business summary, or another fact naturally owned by this entity.
- `Invariant Check Method`: whether the entity is internally consistent, cancelable, matchable,
  eligible, expired, terminal, or otherwise self-consistent.
- `Behavior Method`: applying fill, finishing after a match, canceling by user, rejecting under a
  named business reason, consuming or releasing a reservation, or another state-changing business
  progression; it may also derive direct downstream business documents, such as order placement
  deriving a freeze ledger entry, order matching deriving `SpotTrade`, or order cancellation
  deriving an unfreeze ledger entry.
- `Technical/Internal Method`: small calculations needed to implement public entity methods.

Do not expose these as use-case-visible entity methods:

- Naked getters that only mirror fields without naming a stable business dependency.
- Raw internal collections, legs, entries, children, or field material that makes the use case
  reimplement entity rules.
- Intermediate matching/calculation steps such as raw `remaining_qty`, `limit_price`,
  `crosses_order`, `should_enter_matching`, `matched_status_for`, maker price selection, or trade
  quantity derivation. Keep these private or module-internal and expose a higher-level decision,
  lifecycle method, requirement, or summary.
- Cross-aggregate orchestration, such as moving balances, consuming reservations, deciding outbound
  persistence, or sequencing authorization.
- Adapter, infra, persistence, replay decoding, SQL, HTTP, CLI, or wire-format logic.
- Process glue that only serves one use case and has no reusable business vocabulary.

## Encapsulation

- 默认优先采用“私有字段 + 公共业务方法 + 最小只读 getter”的封装方式。
- `use case` 不允许直接修改 entity 成员值；状态推进必须通过 entity 暴露的高语义生命周期方法完成。
- 不要为了让 use case 继续改字段而补 setter、`*_mut()` getter，或返回内部字段的可变引用。
- getter 只用于读取外部真实依赖的 authoritative business facts，不能机械生成全字段 getter。
- 如果 entity 内部持有 `Vec<_>`、`legs`、`entries`、`children` 等集合，默认不要直接公开整包原始集合。
  优先提供带业务语义的访问器或 summary/query API。
- 如果必须返回内部子对象引用，子对象自身也要遵循最小 getter 暴露，而不是退化成公开字段包。

简短对照：

- 反例：`pub legs: Vec<Leg>`，外部自己遍历并区分 principal / fee。
- 正例：`legs` 保持私有，entity 提供 `principal_legs()`、`fee_legs()`、`total_fee()` 等语义接口。

## Constructors

- Constructors may be permissive when adapters need to rebuild historical state from events.
  Document that explicitly.
- `new(...)` 是否公开要按边界决定；如果只是 core 内部装配已校验事实、回放历史状态，默认优先
  `pub(crate)` 或更窄可见性，而不是直接对外公开。
- 如果构造器的可接受输入比业务方法更宽，Rustdoc 必须明确“仅装配事实，不承担校验”，避免外部把它
  误当成完整业务入口。
- 构造器只有在表达业务创建入口时才可标为 BDD behavior；如果只是装配已校验事实或回放历史状态，
  不标，并说明“仅装配事实”。

## Document Chain / 单据链

- 上游单据是下游单据的工厂来源：稳定的业务事实派生可以放在上游单据 entity 上。
- 单据链表达业务事实派生链，例如：
  - `SpotTrade -> SettlementTransferVoucher`
  - `HyperliquidPerpTrade -> SettlementTransferVoucher`
  - `SettlementTransferVoucher -> BalanceLedgerEntryV2`
- 这类方法属于有业务语义的单据派生方法，不是普通 getter，也不是 adapter glue。
- 上游 entity 方法只能生成直接下游单据，不跨级生成间接单据，也不编排多个聚合。
- 上游单据工厂方法只生成下游单据，不执行下游单据自身的状态应用行为。
- 下游单据自身的不变量、构造校验、状态应用行为仍由下游单据负责。
- 需要跨聚合状态修改、余额应用、持久化、发布时，由 `use case` 编排。
- 不要把 `apply_to(balance)`、账户余额变更、仓位变更、余额记账、外部发布、持久化等动作
  归入上游单据工厂方法。

## Rustdoc Tags

- 只有 public 的真实 `Behavior Method` 可添加：
  `/// 可 BDD 规格化的聚合根行为：<业务动作>。`
- `Business Query Method`、`Invariant Check Method`、`Technical/Internal Method` 不添加该标识。
- If a method enforces a command rejection rule, prefer returning a boolean or `Option`; map it to
  use-case errors in the use case.
- Document whether `new` validates or only assembles already validated/replayed facts.
- Document overflow behavior for calculations.
- Document which workflow concept a method supports, such as cancellation release or reservation
  consistency.

## Reservation Calibration

For `lib/example/core/src/entity/reservation/model.rs`, keep the four-way split strict:

- `Behavior Method` and tag it:
  - `new`, because it is currently the business creation entry for a frozen reservation.
  - `consume`, because it reduces frozen amount.
  - `release`, because it restores frozen amount.
- `Business Query Method` and do not tag it:
  - `belongs_to_account`
  - `is_asset`
  - `is_for_order`
  - `is_active`
  - `can_consume`
  - `can_release`
- `Invariant Check Method` and do not tag it:
  - `has_consistent_amounts`
- `Technical/Internal Method` and do not tag it:
  - `recompute_terminal_state`
  - `diff`
  - `replay_*`

This calibration matches the current `Reservation` implementation: public fields still exist for
serialization/replay compatibility, while lifecycle progression goes through `new`, `consume`, and
`release`.

## SpotOrderV2 Calibration

For `SpotOrderV2`, treat direct downstream document derivation as part of behavior methods, not as a
separate fifth category:

- `place`: creates a spot order and derives the direct downstream freeze ledger entry.
- `match_with_makers`: advances taker/maker order state and derives direct downstream `SpotTrade`
  documents.
- `cancel`: cancels the order, releases its internal reservation, and derives the direct downstream
  unfreeze ledger entry.

## Tests

Add focused inline tests near the entity:

- `Behavior Method` tests state transitions, failure semantics, and invariants.
- Aggregate-root methods test consistency across the aggregate's internal objects.
- `Business Query Method` tests calculations and boolean judgments.
- `Invariant Check Method` tests consistency and qualification judgments.
- Constructor tests store all fields when the constructor is part of the entity contract.
- Boolean business queries cover true and false cases.
- Calculations cover expected values, overflow, or inconsistency paths when relevant.
- Release/transition methods return business-critical amounts.

Prefer small fixture builders like `sample_order()` over repeated setup. If the entity needs
business-state enumeration with `proptest`, switch to `proptest-entity`.

## Adapter And Use Case Cleanup

After adding entity methods, search for duplicated raw struct construction or repeated business
calculations:

```bash
rg -n "EntityName \\{|qty \\* price|reserved_quote|account_id ==" path/to/crate
```

Replace only where the dependency direction remains clean:

- `core` entities must not depend on adapters.
- `use_case` may call entity methods.
- `outbound_adapter` may construct entities and call entity methods.
- `use_case` must not call another `use_case`; shared business meaning should be reused through
  entity methods or higher-level orchestration.
