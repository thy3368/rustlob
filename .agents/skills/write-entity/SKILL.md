---
name: write-entity
description: Write RustLOB core entities for command-style use cases. Use when creating or refactoring structs in core/src/entity, moving reusable business rules out of use cases or adapters, implementing entity constructors and domain methods, adding Rustdoc, or writing focused entity unit tests. Uses SpotOrderV2 as the reference pattern.
---

# Write Entity

## Overview

Use this skill to create or refactor RustLOB `core/src/entity` types so they carry reusable
business meaning instead of being passive data bags. The reference pattern is
`lib/example/core/src/entity/spot/spot_order_v2.rs`: an entity with a small set of
high-semantic business entry points and private/internal derivation details.

`SpotOrderV2` is the current reference for entity API shape. Older `SpotOrder` code may be useful
as legacy contrast, but it is not the standard for new entity design.

Before changing an entity, read the shared constraints file:

- `.agents/skills/shared/entity_four_color_classification.md`
- `.agents/skills/shared/use_case_entity_constraints.md` as the only shared canonical reference for `use case` / `entity` boundary facts, `aggregate role`, `MI chain root`, and `replay/version` semantics

If stronger architecture policy is needed, use a dedicated policy document or skill instead of assuming it is encoded in the shared constraints file.

For independent entity property tests, use `proptest-entity`. Keep this skill focused on
entity modeling, Rustdoc, business methods, and small inline unit tests.

## Workflow

1. Read the entity, its neighboring entities, and the use cases/adapters that construct or inspect it.
2. Classify the target object with `.agents/skills/shared/entity_four_color_classification.md` before deciding whether it should be an `entity`, `aggregate root`, `value object`, or remain `description/policy`.
3. Identify business facts that belong on the entity, not in adapters.
4. Decide whether each candidate method passes the domain method admission rules below before
   exposing it to use cases.
5. Add only reusable methods that at least two workflows may naturally need, or that make one workflow's business rule explicit.
6. Keep validation ownership clear: use cases reject commands/state; entities expose facts and invariant checks.
7. Add Rustdoc for the struct, fields, constructor, and business methods.
8. Add focused inline `#[cfg(test)]` unit tests for the entity methods.
   If the entity needs business-state enumeration with `proptest`, switch to
   `proptest-entity` and put those scenarios in a dedicated file.
9. Replace duplicate manual construction or calculation in adapters/use cases with entity methods when it does not couple core to adapter details.
10. Run targeted formatting and tests for the touched package.

## Entity Shape

Prefer this shape:

```rust
/// Business meaning of this stored entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityName {
    /// Field-level business meaning.
    field: Type,
    legs: Vec<EntityLeg>,
}

impl EntityName {
    /// Builds the entity from already validated facts or replayed history.
    pub(crate) fn new(...) -> Self {
        Self { ... }
    }

    /// Business query or calculation.
    pub fn business_method(&self) -> ... {
        ...
    }

    /// Exposes only the fact that outside workflows actually depend on.
    pub fn field(&self) -> &Type {
        &self.field
    }

    /// Returns only the business-relevant subset instead of the raw collection.
    pub fn principal_legs(&self) -> impl Iterator<Item = &EntityLeg> {
        self.legs.iter().filter(|leg| leg.is_principal())
    }
}
```

Use `Option` for arithmetic that can overflow, e.g. `checked_mul`, `checked_add`, `checked_sub`.
Do not use `unwrap()` or `expect()` in production entity code.
默认优先采用“私有字段 + 公共业务方法 + 最小只读 getter”的封装方式，不要为了测试方便、
调用方便或序列化方便，就把业务字段直接做成 `pub`。只有当公开字段本身就是审慎设计后的稳定
业务接口，或确有明确 serde / 存储边界需求时，才把字段公开，并在 Rustdoc 里说明原因。
use case 不允许直接修改 entity 成员值；状态推进必须通过 entity 暴露的高语义生命周期方法完成。
不要为了让 use case 继续改字段而补 setter、`*_mut()` getter，或返回内部字段的可变引用。
getter 只用于读取外部真实依赖的稳定业务事实，不能机械生成全字段 getter。

## Business Method Rules

- Every entity must contain domain-semantic methods that a use case can reuse; a plain field bag is not enough.
- First decide the entity's `four_color_archetype`; do not assume every entity should have the same rich-behavior shape.
- Split methods explicitly into three buckets:
  - single-entity `behavior methods`
  - aggregate-root `behavior methods`
  - `helper/query methods`
- Method bias should follow archetype:
  - `Moment-Interval` usually favors `behavior methods`
  - `Party/Place/Thing` often favors qualification, ownership, and derived `helper/query methods`
  - promoted `Role` entities must justify their lifecycle and should not exist as label-only wrappers
  - `Description` objects should usually stay out of behavior-heavy `entity` design
- Put stable domain vocabulary on the entity: ownership, symbol/product matching, status checks, reserved/releasable amounts, notional calculations, version transitions.
- `behavior methods` must be lifecycle actions or aggregate-internal lifecycle actions; they represent legal business evolution, not generic utilities.
- Aggregate-root `behavior methods` may coordinate multiple objects inside the same aggregate.
- `helper/query methods` may answer business questions or derived values, but they must not pretend to be independent business actions.
- Requirement-style methods should be high-semantic entries such as `hold_requirement`,
  `cancel_release_requirements`, `terminal_release_requirements`, or
  `fee_consume_requirement_for_trade`. Prefer one method that returns a named business
  requirement/summary over several public methods that expose raw ingredients.
- Keep persistence, event decoding, SQL, HTTP, CLI, and mapper details out of entities.
- Constructors may be permissive when adapters need to rebuild historical state from events. Document that explicitly.
- `new(...)` 是否公开要按边界决定；如果只是 core 内部装配已校验事实、回放历史状态，默认优先
  `pub(crate)` 或更窄可见性，而不是直接对外公开。
- 如果构造器的可接受输入比业务方法更宽，Rustdoc 必须明确“仅装配事实，不承担校验”，避免外部把它
  误当成完整业务入口。
- If a method enforces a command rejection rule, prefer returning a boolean or `Option`; map it to use-case errors in the use case.
- Avoid generic utility methods that do not express business language.
- Do not design the entity as a private helper for just one use case; preserve the many-`use_case` to one-`entity` reuse direction.
- Do not put cross-aggregate coordination, authorization sequencing, or external dependency decisions into an entity.
- Do not wrap a pure helper in a business-verb name.
- Use cases should orchestrate across aggregates and map entity outcomes to command errors; they
  should not assemble entity-internal matching, remaining, price, or release rules from raw fields.
- Prefer names like `cancel`, `release`, `consume`, `apply_fill` for evolution methods.
- Prefer names like `reserve_funds_for_order`, `release_hold` for aggregate-internal coordination.
- Prefer names like `is_*`, `can_*`, `remaining_*`, `available_*` for query/helper methods.
- getter 只暴露外部真实依赖的 authoritative business facts，不要机械镜像全部字段。
- 如果 entity 内部持有 `Vec<_>`、`legs`、`entries`、`children` 等集合，默认不要直接公开整包原始集合。
  优先提供带业务语义的访问器，例如按用途、账户、状态过滤后的查询方法。
- 如果必须返回内部子对象引用，子对象自身也要遵循最小 getter 暴露，而不是退化成公开字段包。

简短对照：

- 反例：`pub legs: Vec<Leg>`，外部自己遍历并区分 principal / fee。
- 正例：`legs` 保持私有，entity 提供 `principal_legs()`、`fee_legs()`、`total_fee()` 等语义接口。

## Entity Domain Method Admission Rules

Allow a candidate to become an entity domain method only when it is one of these:

- `stable self business fact`: identity, ownership, market/product identity, lifecycle status,
  invariant consistency, or another fact naturally owned by this entity.
- `self invariant or qualification`: e.g. whether the entity is internally consistent, cancelable,
  matchable, eligible, expired, or terminal.
- `self lifecycle progression`: e.g. applying fill, finishing after a match, canceling by user, or
  rejecting under a named business reason.
- `requirement or summary derived from self facts`: e.g. hold, release, consume, settlement, or
  audit summaries that downstream use cases can apply to authoritative aggregates.
- `entity-module internal derivation`: small calculations needed to implement the public business
  entries. Keep these private or module-internal; do not expose them as use-case-visible API.

Do not expose these as use-case-visible entity methods:

- Naked getters that only mirror fields without naming a stable business dependency.
- Internal collections, legs, entries, children, or raw field material that makes the use case
  reimplement entity rules.
- Intermediate matching/calculation steps such as raw `remaining_qty`, `limit_price`,
  `crosses_order`, `should_enter_matching`, `matched_status_for`, maker price selection, or trade
  quantity derivation. Keep these private or module-internal and expose a higher-level decision or
  lifecycle method.
- Cross-aggregate orchestration, such as moving balances, consuming reservations, deciding outbound
  persistence, or sequencing authorization.
- Adapter, infra, persistence, replay decoding, SQL, HTTP, CLI, or wire-format logic.
- Process glue that only serves one use case and has no reusable business vocabulary.

## SpotOrderV2 Pattern

For an order-like entity, use `SpotOrderV2` as the pattern:

```rust
impl SpotOrderV2 {
    pub fn new(...) -> Self { ... }

    pub fn belongs_to_account(&self, account_id: &str) -> bool { ... }
    pub fn trades_asset(&self, asset: u32) -> bool { ... }
    pub fn trades_symbol(&self, symbol: &str) -> bool { ... }
    pub fn has_consistent_execution_state(&self) -> bool { ... }
    pub fn has_consistent_reserved_quote(&self) -> bool { ... }
    pub fn has_consistent_reserved_base(&self) -> bool { ... }

    pub fn hold_requirement(&self) -> Option<SpotOrderHoldRequirement> { ... }
    pub fn cancel_release_requirements(...) -> SpotOrderReleaseRequirements { ... }
    pub fn terminal_release_requirements(...) -> SpotOrderReleaseRequirements { ... }
    pub fn fee_consume_requirement_for_trade(...) -> Result<SpotOrderFeeConsumeRequirement, ...> { ... }

    pub(crate) fn fill(&mut self, added_fill_qty: u64) -> Result<(), SpotOrderV2MatchError> { ... }
    pub(crate) fn finish_after_match(&mut self, added_fill_qty: u64) -> Result<(), SpotOrderV2MatchError> { ... }
    pub(crate) fn cancel_by_user(&mut self) -> Result<(), SpotOrderV2MatchError> { ... }
    pub(crate) fn reject_as_bad_alo(&mut self) -> Result<(), SpotOrderV2MatchError> { ... }
    pub(crate) fn reject_as_no_liquidity(&mut self) -> Result<(), SpotOrderV2MatchError> { ... }

    fn remaining_qty(&self) -> Option<u64> { ... }
    fn limit_price(&self) -> Option<u64> { ... }
    fn crosses_order(&self, maker: &Self) -> Result<bool, SpotOrderV2MatchError> { ... }
    fn should_enter_matching(&self, best_maker: Option<&Self>) -> Result<bool, SpotOrderV2MatchError> { ... }
    fn matched_status_for(&self, next_filled_qty: u64) -> SpotOrderStatus { ... }
}
```

Expose high-level business facts, lifecycle transitions, and requirement/summary objects. Keep
intermediate derivations inside the entity module, including remaining quantity, limit price,
crossing checks, maker price selection, trade quantity, and matched-status calculation.

Use case code may call `SpotOrderV2` to ask for a business decision or apply a lifecycle action,
then coordinate other aggregates such as balances, reservations, fees, trades, and outbound events.
It should not reconstruct `qty - filled_qty`, `qty * price`, crossing semantics, or release reason
selection from public fields.
It also must not mutate `SpotOrderV2` fields directly. Legal lifecycle progression goes through
high-semantic entity methods such as `fill`, `finish_after_match`, `cancel_by_user`,
`reject_as_bad_alo`, and `reject_as_no_liquidity`. Do not add setters or mutable getters to bypass
that boundary.

## Rustdoc

Document:

- What business snapshot the entity represents.
- Its `four_color_archetype` and why it deserves to be an `entity`.
- What each public field means in domain terms, or why a getter is the correct exposed fact boundary.
- Whether `new` validates or only assembles already validated/replayed facts.
- Overflow behavior for calculations.
- Which workflow concept a method supports, such as cancellation release or reservation consistency.

Keep Rustdoc concise. Do not describe adapter implementation details.

## Tests

Add inline tests near the entity:

- `behavior methods` test state transitions, failure semantics, and invariants.
- Aggregate-root methods test consistency across the aggregate's internal objects.
- `helper/query methods` test calculations and boolean judgments.
- Constructor stores all fields.
- Boolean business queries return true and false cases.
- Calculations return expected values.
- Overflow or inconsistency paths are covered when relevant.
- Release/transition methods return business-critical amounts.

Prefer small fixture builders like `sample_order()` over repeated setup.

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
- `use_case` must not call another `use_case`; shared business meaning should be reused through entity methods or higher-level orchestration.

## Validation

Run targeted commands rather than full workspace commands when the workspace is dirty or has
known unrelated failures:

```bash
rustfmt path/to/entity.rs path/to/changed_adapter.rs
cargo test -p crate_name
```

If `cargo fmt --all` or workspace tests fail due to unrelated files, report the exact blocker and
keep verification scoped to touched packages.
