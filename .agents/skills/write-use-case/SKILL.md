---
name: write-use-case
description: Write RustLOB command-style use cases around `MiStateMachineV2`. Use when Codex needs to add or refactor files such as `*/use_case/**/*.rs`, define error/command/given-state/after-changes types, implement `pre_check_command`, `validate_against_given_state`, `compute_after_changes_unchecked`, or keep loading, persistence, replay, publish, and reply shaping out of the core use case.
---

# Write Use Case

## Overview

Implement use cases around `MiStateMachineV2Unchecked` / `MiStateMachineV2`: business rules live in the state machine implementation, and state loading, persistence, replay, publish, and transport reply shaping stay outside the core.

Treat source code as the contract of truth. This skill summarizes the expected shape, but if the skill and the trait differ, follow the current code.

Start from these source files:
- Contract: `lib/common/entity/src/use_case/state_machine_v2.rs`
- Runtime executor contract: `lib/common/entity/src/use_case/mi_family_executor.rs`
- Current calibration implementation: `lib/example/core/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order.rs`
- Happy-path spec example: `lib/example/core/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order/compute_replayable_events_happy_path.rs`
- Adapter executor examples: `lib/example/inbound_adapter/src/trading/http.rs` and `lib/example/inbound_adapter/src/trading/cli.rs`
- Shared canonical `use_case` / `entity` facts: `.agents/skills/shared/use_case_entity_constraints.md`
- Shared changes pair-first rule: `.agents/skills/shared/changes_pair_first_rule.md`
- Shared entity classification reference: `.agents/skills/shared/entity_four_color_classification.md`

Read `.agents/skills/shared/use_case_entity_constraints.md` before writing or refactoring a use case. It is the shared canonical reference for `use case` / `entity` boundary facts, `aggregate role`, MI chain root, and replay/version semantics.

Read `.agents/skills/shared/changes_pair_first_rule.md` before shaping `AfterChanges` or a replayable before/after changes type. In the V2 model, bind that rule to the actual business truth returned by `compute_after_changes()` and to any optional `ReplayableChanges` projection.

If the task requires deciding how a `changes/entity` object should be classified, or whether it should be an `entity`, `value object`, `role object`, or `description/policy`, read `.agents/skills/shared/entity_four_color_classification.md` before writing or refactoring.

If the task is to critique or score a use case:
- use `.agents/skills/shared/use_case_review_scorecard.md` as the canonical scoring rubric
- use `clean-architecture` for layer and boundary judgment
- use the checker in `.agents/skills/check-use-case-definition/` or `scripts/check_use_case_business_definition.py` when the task needs a scripted score

Model core business use cases as an `MiStateMachineV2` family. When an adapter or runtime path needs orchestration around loading, persistence, replay, and publishing, use `MiStateMachineFamilyExecutor` with `MiFamilyExecutionSpec` and `MiFamilyOutbound`. `UseCaseReplyMapper` may shape adapter replies from execution results or events, but it is not part of the core use case contract.

## Workflow

1. Define the types in this order:
- `Error`
- `Command`
- `GivenState`
- `AfterChanges`
- optional replayable before/after changes type
- `UseCase` or `Machine`

2. Shape business output around `AfterChanges`.
- `type AfterChanges` is the business truth returned by `compute_after_changes()`.
- `compute_after_changes()` is the stable public entry supplied by `MiStateMachineV2`.
- `compute_after_changes_unchecked()` is the implementation hook and should assume pre-check and state validation already passed.
- `AfterChanges` must carry business semantics, not just wrap `Vec<EntityReplayableEvent>`.
- If `AfterChanges` itself is sufficient for replay/persist/audit, implement `ReplayableChanges` directly on it. `PlaceHyperliquidPerpOrderChanges` is the calibration example.
- For update scenarios, default to pair-first modeling with `UpdatedEntityPair<T>` or an equivalent before/after pair.
- Avoid maintaining a duplicate `*_after` snapshot when it is directly projectable from a pair's `after`.

3. Implement `MiStateMachineV2Unchecked`.
- `type Command` should implement `IssuedByParty` when a business party issues the command.
- `pre_check_command()` only does cheap checks on the command itself.
- `validate_against_given_state()` checks business invariants that need loaded state.
- Explicitly reject branch mismatch or state mismatch with a domain error.
- `compute_after_changes_unchecked()` derives strong typed domain after truth by driving entity or aggregate business methods.
- Never call `compute_after_changes_unchecked()` from tests or adapters as the normal public path; use `compute_after_changes()`.

4. Add before/after replay truth only when needed.
- If replay requires owned before state and after truth to be merged, implement `MiStateMachineOwnedV2BeforeAfter`.
- `merge_before_and_after(given_state, after)` should extract authoritative before truth from owned `GivenState` and pair it with `AfterChanges`.
- `compute_before_after_changes()` should be the replayable path when the use case needs stable before/after changes.
- Keep one truth path: `compute_after_changes()` first, then optional merge/projection.

5. Keep these concerns out of the core use case.
- Do not call DB, cache, broker, HTTP, filesystem, VM registry lookup, or other adapter/infra APIs from the use case.
- Do not persist, replay, or publish events inside the use case.
- Do not map domain changes or events to HTTP or API replies inside the use case.
- Do not measure latency inside the business logic.
- Do not call another `use_case` from inside the current `use_case` unless the current use case is intentionally modeling a higher-level business pipeline in core and still returns one coherent `AfterChanges`.

6. For adapter or runtime execution paths, wire the MI family executor.
- `MiFamilyExecutionSpec<F>` derives `F::Command` from the adapter-side request.
- `MiFamilyOutbound<F>` owns outbound stages: `load_given_state`, `persist`, `replay`, and `publish`.
- `MiStateMachineFamilyExecutor::execute()` fixes the runtime sequence: pre-check command -> load authoritative state -> validate against state -> compute unchecked after changes -> merge before/after -> project replayable events -> persist -> replay -> publish.
- The executor returns `MiFamilyExecutionResult<F::BeforeAfterChanges>`, including merged changes and projected events.
- Execution errors map to `MiFamilyExecutionError<BusinessError, OutboundError>`: business errors, event projection errors, and per-stage outbound errors stay distinguishable.
- Keep request parsing, command mapping, outbound implementation, error-to-transport mapping, and optional reply shaping in adapter/runtime code.

## Project Rules

- Prefer names that read as a business sentence:
  - `PlaceHyperliquidPerpOrderError`
  - `PlaceHyperliquidPerpOrderCmd`
  - `PlaceHyperliquidPerpOrderState`
  - `PlaceHyperliquidPerpOrderChanges`
  - `PlaceHyperliquidPerpOrderUseCase`
- `GivenState` should be a domain snapshot, not a repository handle.
- `AfterChanges` should represent pure business facts before projection, not transport responses and not a raw event bag.
- update 型 `AfterChanges` 默认先建模 authoritative pair，再考虑是否真的需要额外业务结果字段。
- Keep `Error` domain-specific. Avoid stringly typed `Result<_, String>` as the main API.
- Keep `compute_after_changes()` deterministic for the same command and state.
- Keep `to_replayable_events()` deterministic for the same changes.
- `party_id` belongs to the business command, not to technical metadata.
- `trace_id` is only for tracing. Do not use it as the idempotency key.
- Treat `entity` as a reusable core collaborator, not a private struct owned by one use case.
- If business logic is likely reusable across use cases, prefer a domain-semantic entity method over duplicating the rule in the use case.
- `use case` 负责业务边界与跨聚合协调，不只是串 `method call`。
- 优先复用 `entity` / `aggregate` 的 `behavior method`。
- 不要把 `helper/query method` 当成业务完成条件本身。
- 聚合内协调可留在 `aggregate method`。
- 跨聚合协调必须保留在 `use case`。

## Testing

For a new use case, add the smallest useful test set inline in the same file unless the crate already has a stronger convention.

Required tests:
- `pre_check_command()` rejects malformed command-only input.
- `validate_against_given_state()` rejects invalid state transitions and branch mismatch.
- `compute_after_changes()` produces the expected strong typed after truth through the public V2 chain.
- `compute_after_changes_unchecked()` is covered indirectly through `compute_after_changes()`.
- `to_replayable_events()` projects the expected replayable events from changes when `AfterChanges` or before/after changes implements `ReplayableChanges`.
- `compute_before_after_changes()` covers merge order and before/after truth when `MiStateMachineOwnedV2BeforeAfter` exists.

Testing split:
- Happy-path tests should follow `place_perp_order/compute_replayable_events_happy_path.rs`: first assert changes, then assert replayable events and their order.
- Keep `pre_check_command()` and `validate_against_given_state()` tests separate from the happy-path spec file.
- Use `proptest` only to add invariant coverage; do not replace happy-path spec tests with it.

Adapter/runtime path tests:
- When implementing `MiFamilyExecutionSpec`, test adapter request to `F::Command` mapping, especially business identity fields and idempotency keys.
- When implementing `MiFamilyOutbound`, test each outbound stage's error maps to the matching `MiFamilyExecutionError` variant.
- Test replayable event projection order before adapter reply shaping.
- Cover the `MiStateMachineFamilyExecutor` success path for runtime wiring when the adapter owns that wiring.
- Keep `UseCaseReplyMapper` tests scoped to adapter reply shaping; do not make it part of core use case tests.

## Output Checklist

Before finishing, verify:
- The use case type only encodes business logic.
- `AfterChanges` is the pure business result, not an adapter or transport DTO.
- replayable events are projected from the same changes, not derived by a separate business path.
- `pre_check_command()` only checks command-local facts.
- `validate_against_given_state()` owns loaded-state invariants and branch mismatch checks.
- `compute_after_changes_unchecked()` is not used as the adapter/public entry.
- Optional before/after replay truth is derived by `MiStateMachineOwnedV2BeforeAfter`, not by a second business path.
- External state loading happens outside the use case.
- Side effects happen outside the use case.
- Reply shaping happens outside the core use case.
- Adapter/runtime orchestration uses `MiStateMachineFamilyExecutor` when a full load/persist/replay/publish path is needed.
- `MiFamilyExecutionSpec` maps adapter request data into a business command without moving business rules into the adapter.
- `MiFamilyOutbound` owns technical side effects and preserves stage-specific error mapping.
- The core use case does not directly invoke outer technical workflow machinery.
- Any reusable business rule that belongs on an entity is not duplicated inline in the use case.
- The use case is not repeating an existing `entity` / `aggregate behavior method` without reason.
- The use case has not wrongly pushed cross-aggregate coordination into one `aggregate method`.
- Any method reused from an entity or aggregate is truly a business evolution method, not a helper disguised with an action name.
- Tests cover direct V2 hook behavior and any replayable projection that matters.
