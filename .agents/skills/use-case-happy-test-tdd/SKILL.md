---
name: use-case-happy-test-tdd
description: Write RustLOB `StateMachineV2` happy-path specification tests or run strict TDD repair, including replay projections through `StateMachineV2Diff` when required.
---

# Use Case Happy Test TDD

Use this as the happy-path entry point for current RustLOB state-machine use cases.

## Modes

- **Spec-only:** add or improve happy-path tests from current semantics. Do not repair production behavior unless requested.
- **TDD repair:** for review, gaps, TDD, repair, or fix requests, build the matrix, select one highest-value missing cell, write the failing test first, make the smallest core repair, and verify that cell.

## Read First

- Contract: `lib/common/entity/src/use_case/state_machine_v2.rs`
- Calibration test: `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order/compute_replayable_events_happy_path.rs`
- Calibration implementation: `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order.rs`
- Pair-first rule: `.agents/skills/shared/changes_pair_first_rule.md`
- `references/example_breakdown.md` for test structure
- `references/checklist.md` before finishing spec-only work
- `references/tdd-loop.md`, `references/review-scorecard.md`, and `references/matrix-completion.md` for repair/review work

## Scope

Test `StateMachineV2::compute_state_changed()` and, when replay truth matters, `StateMachineV2Diff::compute_state_diff()` plus `ReplayableChanges::to_replayable_events()`.

Do not test `StateMachineExecutor` outbound adapter details, databases, brokers, HTTP, query adapters, or legacy query execution here. Command rejection and loaded-state rejection belong in focused `check_command()` and `validate_state_given()` tests, not the happy-path file.

## Workflow

1. Read the command, `StateGiven`, `StateChanged`, optional `StateDiff`, entity behavior, and existing tests.
2. Build a business matrix from actor/intent, branch, matching condition, finish state, status transition, event shape, and domain-specific axes.
3. Map each proposed test to one matrix cell; avoid cosmetic duplicates.
4. Name tests as business sentences and use `Rule`, `Given`, `When`, `Then` comments with arrange/act/assert bodies.
5. Execute and assert in this order:
   1. construct the command
   2. construct given state
   3. call `compute_state_changed()`
   4. assert `StateChanged` business truth
   5. call `compute_state_diff()` or `to_replayable_events()` when replay applies
   6. assert event order, entity identity/version, field changes, and LSN/logical timestamp projection at the boundary that writes them

For update scenarios, assert `UpdatedEntityPair<T>` before projected events. Assert meaningful event fields, not only vector length. Preserve exact event order when replay semantics depend on it. Distinguish unchanged fields (`None`) from a change to zero.

## TDD Repair

In repair mode, report the fixed scorecard from `references/review-scorecard.md`, choose one missing matrix cell, demonstrate the failing targeted test, implement only the required core/entity change, and rerun the narrow test plus relevant regression scope. Do not broaden the production refactor or move business rules into adapters.

## Completion

- The matrix records dimensions, expected outcomes/events, current coverage, and the chosen cell.
- Business truth is asserted before replay projection.
- Pair-first truth, event order, versions, field changes, and relevant metadata are explicit.
- Spec-only work does not silently change production behavior.
- Verification results or the exact blocker are reported.
