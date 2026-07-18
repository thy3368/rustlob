---
name: write-entity
description: Write RustLOB core entities for command-style use cases. Use when creating or refactoring structs in core/src/entity, moving reusable entity rules out of use cases or adapters, implementing entity constructors and entity methods, adding Rustdoc, or writing focused entity unit tests. Uses SpotOrderV2 as the reference pattern.
---

# Write Entity

## Overview

Use this skill to create or refactor RustLOB `core/src/entity` types so they carry reusable
business meaning instead of being passive data bags. The reference pattern is
`lib/example/core/src/entity/spot/spot_order_v2.rs`: an entity with high-semantic business entry
points and private/internal derivation details.

`SpotOrderV2` is the current reference for entity API shape. Older `SpotOrder` code may be useful
as legacy contrast, but it is not the standard for new entity design.

Before changing an entity, read these shared constraints:

- `.agents/skills/shared/entity_four_color_classification.md`
- `.agents/skills/shared/use_case_entity_constraints.md` as the canonical reference for `use case` /
  `entity` boundary facts, `aggregate role`, `MI chain root`, and `replay/version` semantics
- `.agents/skills/shared/entity_method_constraints.md` for entity method classification, admission
  rules, Rustdoc tagging, focused tests, and use case / adapter cleanup guidance

If stronger architecture policy is needed, use a dedicated policy document or skill instead of
assuming it is encoded in the shared constraints file.

For independent entity property tests, use `proptest-entity`. Keep this skill focused on entity
modeling, Rustdoc, entity methods, and small inline unit tests.

## Workflow

1. Read the entity, its neighboring entities, and the use cases/adapters that construct or inspect it.
2. Classify the target object with `.agents/skills/shared/entity_four_color_classification.md`
   before deciding whether it should be an `entity`, `aggregate root`, `value object`, or remain
   `description/policy`.
3. Identify business facts that belong on the entity, not in adapters.
4. If the entity participates in a document flow / 单据链, identify stable upstream-document to
   downstream-document factory methods first, such as
   `SettlementTransferVoucher -> BalanceLedgerEntryV2`.
5. Check each candidate method against `.agents/skills/shared/entity_method_constraints.md` before
   exposing it to use cases.
6. Add only reusable methods that at least two workflows may naturally need, or that make one
   workflow's business rule explicit.
7. Keep validation ownership clear: use cases reject commands/state; entities expose facts,
   invariant checks, and legal lifecycle progression.
8. Add Rustdoc for the struct, fields or exposed facts, constructor, and entity methods.
9. Add focused inline `#[cfg(test)]` unit tests for the entity methods. If the entity needs
   business-state enumeration with `proptest`, switch to `proptest-entity`.
10. Replace duplicate manual construction or calculation in adapters/use cases with entity methods
   when it does not couple core to adapter details.
11. Run targeted formatting and tests for the touched package.

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

## Document Chain Factories

For document-flow entities, model the upstream document as the factory source for downstream
documents when the derivation is a stable business fact. Examples:

- `SpotTrade -> SettlementTransferVoucher`
- `HyperliquidPerpTrade -> SettlementTransferVoucher`
- `SettlementTransferVoucher -> BalanceLedgerEntryV2`

Name these methods with `from_*` / `derive_*` wording that exposes the document derivation, rather
than generic `build_*` construction. The factory should only derive and return the downstream
document. It must not apply balances, mutate other aggregates, persist events, publish messages, or
run downstream document behavior such as `apply_to(balance)`.

Downstream document invariants and behavior remain owned by the downstream entity. Cross-aggregate
state changes and persistence/publication sequencing remain use case orchestration.

## SpotOrderV2 Pattern

For an order-like entity, use `SpotOrderV2` as the current real pattern:

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

## Validation

Run targeted commands rather than full workspace commands when the workspace is dirty or has known
unrelated failures:

```bash
rustfmt path/to/entity.rs path/to/changed_adapter.rs
cargo test -p crate_name
```

If `cargo fmt --all` or workspace tests fail due to unrelated files, report the exact blocker and
keep verification scoped to touched packages.
