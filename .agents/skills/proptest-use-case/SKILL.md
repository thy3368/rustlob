---
name: proptest-use-case
description: Write property-based tests for RustLOB command-style use cases with `proptest`. Use when Codex needs to add or review `proptest!` tests for `CommandUseCase2`, design generators for commands or state snapshots, verify executor and pipeline invariants, or distinguish strong business properties from weak tautological tests.
---

# Proptest Use Case

## Overview

Use `proptest` to verify business invariants of RustLOB use cases, not just sample inputs. Good properties explore success and rejection paths, assert side effects on `CommandUseCaseExecutor2`, and encode domain rules that would matter if broken.

Start from these source files:
- Contract: `lib/common/cmd_handler/src/use_case_def2.rs`
- Real property-test examples:
  - `lib/core/l1/src/use_case/command_handler/receive_and_admit_transactions.rs`
  - `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs`
- Shared proptest examples:
  - `lib/common/cmd_handler/src/use_case_proptest_examples/good.rs`
  - `lib/common/cmd_handler/src/use_case_proptest_examples/bad.rs`

Read the shared good and bad examples before writing new properties.

## Workflow

1. Choose the property before writing the generator.
- Prefer business invariants such as:
  - invalid commands fail before any side effects
  - rejected state transitions do not persist, replay, or publish events
  - successful execution emits events consistent with command and state
  - reply mapping preserves counts or identities derived from domain events
- Avoid properties that only restate what a prepared fixture already contains.

2. Generate command and state spaces that cover both happy and unhappy paths.
- Use `prop_map` to build valid domain-shaped values.
- Prefer one strategy that can hit multiple branches over many tiny one-branch tests.
- Keep generators business-aware: generate realistic `party_id`, amounts, statuses, limits, and counts.

3. Test the right level.
- Use direct method properties for pure logic in `pre_check_command`, `validate_against_state`, or `compute_replayable_events`.
- Use `CommandUseCaseExecutor2` properties when orchestration matters:
  - load is or is not called
  - pipeline is or is not called
  - event counts and output shapes match the business rules

4. Assert side effects explicitly.
- Count `persist`, `replay`, and `publish` calls with atomics or counters.
- For rejected commands, assert those counters stay at zero.
- For successful commands, assert each expected stage runs exactly once when that is the contract.

5. Keep the property meaningful.
- `trace_id` is for tracing only; do not build a property that treats it as the business identity.
- `command_id` is the business command identity; properties about retries or deduplication should key off that.
- `party_id` belongs to the business command; properties may assert actor-sensitive validation or emitted event ownership.

## Good vs Bad

- Good example characteristics:
  - property covers both acceptance and rejection branches
  - property checks side effects and emitted events
  - use case actually computes business outcomes
- Bad example characteristics:
  - property only reasserts precomputed state answers
  - role is technical, not business-facing
  - command identity and trace identity are confused
  - the test is tautological and unlikely to catch regressions

## Output Checklist

- The property names the business invariant it protects.
- The generators can hit both valid and invalid paths.
- The assertions check business behavior or side effects, not tautologies.
- The test uses `prop_assert!` or `prop_assert_eq!` consistently inside `proptest!`.
- If executor orchestration matters, the property asserts pipeline call counts.
