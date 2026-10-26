---
name: write-use-case
description: Write or refactor RustLOB command-style use cases built on `StateMachineV2` and, when replayable before/after truth is required, `StateMachineV2Diff`.
---

# Write Use Case

Implement business rules in the state machine. Keep state loading, persistence, replay, publishing, transport mapping, and reply shaping outside core.

## Read First

- Contract: `lib/common/entity/src/use_case/state_machine_v2.rs`
- Runtime orchestration: `lib/common/entity/src/use_case/executor.rs`
- Calibration use case: `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order.rs`
- Happy-path specification: `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order/compute_replayable_events_happy_path.rs`
- Executor example: `lib/example/use_case_executor/src/trading/spot/spot_block_executor.rs`
- Shared boundaries: `.agents/skills/shared/use_case_entity_constraints.md`
- Pair-first rule: `.agents/skills/shared/changes_pair_first_rule.md`

Source code is authoritative if these instructions drift.

## Core Contract

Define domain-specific `Error`, `Command`, `StateGiven`, `StateChanged`, and the use-case type. Implement `StateMachineV2`:

- `Command` is the business command and implements the required log-entry metadata contract.
- `StateGiven` is loaded authoritative domain state, never a repository handle.
- `StateChanged` is strong typed after truth, not an event bag or transport response.
- `check_command()` validates command-only facts.
- `validate_state_given()` validates invariants that require loaded state, including branch or state mismatch.
- `compute_state_changed_only()` drives entity or aggregate behavior and derives after truth after validation succeeds.
- Callers and tests use `compute_state_changed()`, the public validation-and-computation chain.

Keep `compute_state_changed()` deterministic for the same command and state. Put reusable single-entity rules on entity behavior methods; retain cross-aggregate coordination in the use case.

## Replayable Truth

Implement `StateMachineV2Diff` only when persistence, replay, diff, or audit needs owned before/after truth:

- `StateDiff` implements `ReplayableChanges`.
- `do_compute_state_diff(StateGiven, StateChanged)` merges authoritative before truth with after truth.
- Callers use `compute_state_diff()`.
- Updates default to `UpdatedEntityPair<T>`; do not duplicate its `after` as a semantically identical `*_after` field.
- `to_replayable_events()` is a deterministic projection from `StateDiff`, returning `EntityReplayableEvent` or `EntityError` as defined by the current contract.

Read the pair-first shared rule before shaping update changes.

## Runtime Boundary

Use `StateMachineExecutor`, `StateSource`, and `StateSink` only in adapter/runtime wiring. `StateMachineExecutor::execute()` performs this sequence:

```text
check_command
-> StateSource::load_given_state
-> StateMachineV2Diff::compute_state_diff
-> ReplayableChanges::to_replayable_events
-> write command lsn and logical_ts into every event
-> StateSink::persist
-> StateSink::replay
-> StateSink::publish
```

It returns `ExecutionResult<StateDiff>`. Business failures map to `ExecutionError::Business`, projection failures (including duplicate event keys) to `ExecutionError::ProjectEvents`, and outbound failures to `LoadState`, `Persist`, `Replay`, or `Publish`.

The executor does not own HTTP handling, database query design, business rules, request-to-command mapping, error-to-transport mapping, metrics, or reply shaping.

## Tests

- Test malformed command input through `check_command()`.
- Test loaded-state rejection through `validate_state_given()`.
- Test successful business truth through `compute_state_changed()`.
- When replay matters, test `compute_state_diff()` and then `to_replayable_events()`.
- Assert pair truth before projected event fields, including event order and entity versions.
- Test executor wiring separately, including each outbound error variant, command metadata projection, and duplicate event-key rejection.

## Completion Check

- Core code contains no DB, cache, broker, filesystem, HTTP, persistence, replay, publish, or reply-shaping work.
- `StateChanged` is the single after truth.
- `StateDiff` is the single replayable before/after truth.
- Business identity remains in the command; `trace_id` is not used as an idempotency key.
- Production code adds no `unwrap()`, `expect()`, unexplained `unsafe`, or swallowed errors.
