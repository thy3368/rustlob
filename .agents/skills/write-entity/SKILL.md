---
name: write-entity
description: Write RustLOB core entities for command-style use cases. Use when creating or refactoring structs in core/src/entity, moving reusable entity rules out of use cases or adapters, implementing entity constructors and entity methods, adding Rustdoc, or writing focused entity unit tests. Uses BalanceV2 as the reference pattern.
---

# Write Entity

## Overview

Use this skill to create or refactor RustLOB `core/src/entity` types so they carry reusable
business meaning instead of being passive data bags. The reference pattern is
`lib/example/entity/src/entity/account/balance_v2.rs`: `BalanceV2Record` is the stable persistence
and replay record, while `BalanceV2<Open/Closed>` uses typestate to constrain lifecycle operations.

Before changing an entity, read these shared constraints:

- `.agents/skills/shared/entity_four_color_classification.md`
- `.agents/skills/shared/use_case_entity_constraints.md` as the canonical reference for `use case` /
  `entity` boundary facts, aggregate role, and replay/version semantics
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
   invariant checks, and legal lifecycle progression. Follow the BalanceV2 boundary: the record
   carries stable persistence/replay facts, while typestate carries lifecycle-operation constraints.
8. For stateful entities, document the lifecycle states, legal transitions, and the boundary between
   entity state and persistence records. Validate all inputs and derive every new value with checked
   arithmetic before committing state or version changes; failures leave state unchanged.
9. Add Rustdoc for the struct, fields or exposed facts, constructor, and entity methods.
10. Add focused inline `#[cfg(test)]` unit tests for the entity methods. If the entity needs
    business-state enumeration with `proptest`, switch to `proptest-entity`.
11. Replace duplicate manual construction or calculation in adapters/use cases with entity methods
    when it does not couple core to adapter details.
12. Run targeted formatting and tests for the touched package.

## Entity Shape

Prefer a stable record plus a domain entity wrapper when persistence and lifecycle need different
responsibilities:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceV2Record {
    // stable identity, amounts, version/LSN, and serde-compatible lifecycle status
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceV2<S = Open> {
    record: BalanceV2Record,
    stage: PhantomData<S>,
}
```

Keep the serialized shape flat. If a lifecycle status is added to an existing record, use serde
compatibility defaults where appropriate (for example, an omitted status decodes as `open`). When
decoding into a typestate wrapper, verify that the persisted status agrees with the target stage.

For lifecycle operations such as `freeze`, `unfreeze_and_pay`, and `close`:

- validate amounts and available/frozen balances before mutation;
- derive all new balances and the incremented version with checked arithmetic first;
- commit state and version only after every derivation succeeds;
- preserve state on errors; and
- when consuming an open entity, make a failed `close` return the unchanged open entity together
  with the error.

Use `Option` for arithmetic that can overflow, e.g. `checked_mul`, `checked_add`, `checked_sub`.
Do not use `unwrap()` or `expect()` in production entity code; existing test conventions may use
those helpers in tests.

Expose high-level business facts, lifecycle transitions, and requirement/summary objects. Keep
intermediate derivations private to the entity module.

For replay and persistence metadata, keep the responsibilities distinct:

- `FieldDiff` describes created/changed persisted fields;
- entity identity identifies the stable business entity;
- `partition` selects the persistence/sharding partition;
- LSN/version orders entity revisions; and
- `replay_field_type` declares the field value representation used during replay.

The record should implement the applicable `Entity`, `Shard`, and `FieldDiff` contracts without
turning version into an ordinary field diff when the event protocol represents it as LSN.

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

## Four-Color and Boundary Rules

Classify the entity using the four-color model before implementation. Preserve the distinction
between entity facts and use-case orchestration: an entity owns identity, invariants, derived facts,
and legal state transitions; a use case owns command admission, cross-aggregate sequencing,
persistence, and publication. Keep adapter concerns outside the entity.

Prefer constructors with the narrowest visibility that fits the architecture (`pub(crate)` when
callers are inside the crate; public only when the boundary requires it). Do not weaken semantics
with placeholder types or empty branches.

The reusable document chain `SpotTrade -> SettlementTransferVoucher -> BalanceLedgerEntryV2`
remains a required reference where relevant, even though order matching is no longer the reference
entity pattern.

## Verification Guidance

Tests for a BalanceV2-style entity should cover, as applicable:

- construction facts and lifecycle status;
- successful lifecycle transitions;
- invalid amounts and insufficient balances;
- Decimal and version overflow;
- close preconditions and boundary behavior;
- flat serialization compatibility, including old records defaulting to open;
- typestate/status mismatch on decode; and
- stable diff/replay metadata, entity identity, partition, and LSN behavior.

Rustdoc should explain commands, errors, state truth, replayable diffs, and boundary behavior when
those concepts apply. Validate with focused tests and formatting, for example:

```bash
rustfmt path/to/entity.rs path/to/changed_adapter.rs
cargo test -p crate_name
```

If `cargo fmt --all` or workspace tests fail due to unrelated files, report the exact blocker and
keep verification scoped to touched packages.
