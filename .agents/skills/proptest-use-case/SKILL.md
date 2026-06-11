---
name: proptest-use-case
description: Write property-based tests for RustLOB command-style use cases with `proptest`. Use when Codex needs to add or review `proptest!` tests for `CommandUseCase2`, design generators for commands or state snapshots, verify executor and pipeline invariants, or distinguish strong business properties from weak tautological tests.
---

# Proptest Use Case

## Overview

Use `proptest` to verify business invariants of RustLOB use cases, not just sample inputs. Good properties start from happy paths to validate that the command design can express required business scenarios and that `compute_replayable_events` maps those scenarios into complete business events. Rejection paths and boundary failures come after that.

For independent entity scenario coverage under `core/src/entity`, use `proptest-entity`.
This skill stays focused on command/use-case properties and executor behavior.

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

2. Start with happy-path coverage for command design and event completeness.
- First list the successful business scenarios the command is required to support.
- Generate those scenarios as valid commands and states before adding invalid-command or invalid-state generators.
- Use the happy-path property to check two things:
  - command completeness: the command shape can express each required scenario without adapter-specific hacks or missing fields.
  - event completeness: `compute_replayable_events` emits all business facts and state changes required by that scenario.
- If a required successful scenario cannot be generated naturally, treat it as a command design gap.
- Domain-specific fields are examples, not universal rules. For example, order commands may need supported execution intent, time-in-force, trigger behavior, side, or client order id; funding commands may need amount, account, currency, or balance-state scenarios.

3. Generate command and state spaces that cover both happy and unhappy paths.
- Every command with business validation must have its own scenario enum and strategy, preferably
  in a dedicated file such as `command_scenarios.rs`.
  - The enum should cover valid command shapes, invalid command-only inputs, boundary values, and
    deterministic error priority when multiple command fields are invalid.
  - Command scenario tests should target `pre_check_command()` and must not depend on loaded state.
- Every entity whose state affects a use case must have independent scenario support. Use
  `proptest-entity` for entity-owned scenario enums and strategies; use-case scenario
  files may wrap them or define use-case-specific interpretations.
- Use case `given_state_scenarios.rs` should compose command/entity scenario enums instead of
  hiding entity state variation inside ad hoc fixtures.
- Use `prop_map` to build valid domain-shaped values.
- Prefer one strategy that can hit multiple branches over many tiny one-branch tests.
- Keep generators business-aware: generate realistic `party_id`, amounts, statuses, limits, and counts.

4. Test the right level.
- Use direct method properties for pure logic in `pre_check_command`, `validate_against_state`, or `compute_replayable_events`.
- Use `CommandUseCaseExecutor2` properties when orchestration matters:
  - load is or is not called
  - pipeline is or is not called
  - event counts and output shapes match the business rules

5. Assert side effects explicitly.
- Count `persist`, `replay`, and `publish` calls with atomics or counters.
- For rejected commands, assert those counters stay at zero.
- For successful commands, assert each expected stage runs exactly once when that is the contract.

6. Keep the property meaningful.
- `trace_id` is for tracing only; do not build a property that treats it as the business identity.
- `command_id` is the business command identity; properties about retries or deduplication should key off that.
- `party_id` belongs to the business command; properties may assert actor-sensitive validation or emitted event ownership.

## Good vs Bad

- Good example characteristics:
  - happy-path properties validate command design completeness and `compute_replayable_events` completeness
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
- Commands with business validation have an explicit `*CommandScenario` enum and strategy.
- Entities that affect validation or event generation have independent scenario coverage via
  `proptest-entity` or a clearly use-case-specific wrapper.
- Given-state properties compose command/entity scenarios instead of embedding all variations in one
  opaque fixture.
- Happy-path generators cover required successful business scenarios before rejection cases.
- Happy-path assertions prove both command expressiveness and emitted event completeness.
- The generators can hit both valid and invalid paths.
- The assertions check business behavior or side effects, not tautologies.
- The test uses `prop_assert!` or `prop_assert_eq!` consistently inside `proptest!`.
- If executor orchestration matters, the property asserts pipeline call counts.
