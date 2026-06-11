---
name: write-entity
description: Write RustLOB core entities for command-style use cases. Use when creating or refactoring structs in core/src/entity, moving reusable business rules out of use cases or adapters, implementing entity constructors and domain methods, adding Rustdoc, or writing focused entity unit tests. Uses SpotOrder as the reference pattern.
---

# Write Entity

## Overview

Use this skill to create or refactor RustLOB `core/src/entity` types so they carry reusable
business meaning instead of being passive data bags. The reference pattern is
`lib/example/core/src/entity/spot/spot_order.rs`: a rich entity with facts, constructor,
domain queries/calculations, match semantics, Rustdoc, and focused unit tests.

Before changing an entity, read the shared constraints file:

- `.agents/skills/shared/use_case_entity_constraints.md`

For independent entity property tests, use `proptest-entity`. Keep this skill focused on
entity modeling, Rustdoc, business methods, and small inline unit tests.

## Workflow

1. Read the entity, its neighboring entities, and the use cases/adapters that construct or inspect it.
2. Identify business facts that belong on the entity, not in adapters.
3. Add only reusable methods that at least two workflows may naturally need, or that make one workflow's business rule explicit.
4. Keep validation ownership clear: use cases reject commands/state; entities expose facts and invariant checks.
5. Add Rustdoc for the struct, fields, constructor, and business methods.
6. Add focused inline `#[cfg(test)]` unit tests for the entity methods.
   If the entity needs business-state enumeration with `proptest`, switch to
   `proptest-entity` and put those scenarios in a dedicated file.
7. Replace duplicate manual construction or calculation in adapters/use cases with entity methods when it does not couple core to adapter details.
8. Run targeted formatting and tests for the touched package.

## Entity Shape

Prefer this shape:

```rust
/// Business meaning of this stored entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityName {
    /// Field-level business meaning.
    pub field: Type,
}

impl EntityName {
    /// Builds the entity from already validated facts or replayed history.
    pub fn new(...) -> Self {
        Self { ... }
    }

    /// Business query or calculation.
    pub fn business_method(&self) -> ... {
        ...
    }
}
```

Use `Option` for arithmetic that can overflow, e.g. `checked_mul`, `checked_add`, `checked_sub`.
Do not use `unwrap()` or `expect()` in production entity code.

## Business Method Rules

- Every entity must contain domain-semantic methods that a use case can reuse; a plain field bag is not enough.
- Put stable domain vocabulary on the entity: ownership, symbol/product matching, status checks, reserved/releasable amounts, notional calculations, version transitions.
- Keep persistence, event decoding, SQL, HTTP, CLI, and mapper details out of entities.
- Constructors may be permissive when adapters need to rebuild historical state from events. Document that explicitly.
- If a method enforces a command rejection rule, prefer returning a boolean or `Option`; map it to use-case errors in the use case.
- Avoid generic utility methods that do not express business language.
- Do not design the entity as a private helper for just one use case; preserve the many-`use_case` to one-`entity` reuse direction.

## SpotOrder Pattern

For an order-like entity, the pattern is:

```rust
impl SpotOrder {
    pub fn belongs_to_account(&self, account_id: &str) -> bool { ... }
    pub fn trades_symbol(&self, symbol: &str) -> bool { ... }
    pub fn limit_price(&self) -> Option<u64> { ... }
    pub fn notional_quote(&self) -> Option<u64> { ... }
    pub fn remaining_qty(&self) -> Option<u64> { ... }
    pub fn crosses_order(&self, maker: &Self) -> Result<bool, SpotOrderMatchError> { ... }
    pub fn finalize_after_match(
        &self,
        total_fill: u64,
    ) -> Result<SpotOrderFinalization, SpotOrderMatchError> { ... }
}
```

These methods make future place/cancel/match/reconcile use cases reuse the same business meaning
instead of recomputing `qty * price`, comparing raw strings, re-deriving remaining quantity, or
duplicating crossing and finish-state rules in multiple places.

## Rustdoc

Document:

- What business snapshot the entity represents.
- What each public field means in domain terms.
- Whether `new` validates or only assembles already validated/replayed facts.
- Overflow behavior for calculations.
- Which workflow concept a method supports, such as cancellation release or reservation consistency.

Keep Rustdoc concise. Do not describe adapter implementation details.

## Tests

Add inline tests near the entity:

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
