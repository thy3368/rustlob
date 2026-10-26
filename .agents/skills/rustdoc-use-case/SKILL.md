---
name: rustdoc-use-case
description: Write Rustdoc for RustLOB `StateMachineV2` and `StateMachineV2Diff` use cases, including commands, errors, state truth, replayable diffs, and boundary behavior.
---

# Rustdoc Use Case

Document the public business contract an adapter author needs. Start from:

- `lib/common/entity/src/use_case/state_machine_v2.rs`
- `lib/common/entity/src/use_case/executor.rs`
- `lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/match_perp_order.rs`
- `.agents/skills/shared/changes_pair_first_rule.md`

## Contract Vocabulary

- `Command` corresponds to `StateMachineV2::Command` and describes the requested business action.
- `StateGiven` is authoritative loaded state.
- `StateChanged` is the after truth returned by `compute_state_changed()`.
- `StateDiff` is replay/persist before/after truth and implements `ReplayableChanges`.
- `check_command()` performs command-only validation.
- `validate_state_given()` performs business validation requiring loaded state.
- `compute_state_changed_only()` derives after truth after those checks.
- `to_replayable_events()` projects replayable events from the diff.

## What To Document

- Command: business purpose, identity fields, and non-obvious field semantics.
- Error: the business rejection represented by each public variant; keep `Display` aligned with `thiserror` messages.
- State truth: identify authoritative fields. For updates, document `UpdatedEntityPair<T>` as authoritative and derive reply, events, and assertions from it.
- Use case: state the business change it derives and that it is deterministic for the same command and state.
- Diff: explain the before/after truth needed for replay or persistence without treating a duplicate `*_after` snapshot as a second truth.

Keep examples small and compilable. Good doctests construct a command, demonstrate an error string, or show `compute_state_changed()` usage. Leave fixture-heavy business flows to unit tests.

Explicitly state relevant boundaries: the use case does not query a database, persist, replay, publish, call HTTP, or shape transport replies. Those concerns belong to `StateSource`, `StateSink`, adapters, and runtime wiring.

Before finishing, verify public commands, important fields, errors, state truth, diffs, and use-case types have concise business-oriented Rustdoc and that unit tests remain the primary business harness.
