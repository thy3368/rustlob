---
name: write-use-case-happy-path-tests
description: Write RustLOB happy-path specification tests for `CommandUseCase4::compute_changes(...)` and `ReplayableChanges::to_replayable_events()`. Use when Codex should add or rewrite use case tests that prove covered business scenarios from existing use case code, using a business matrix, `Rule/Given/When/Then`, and event-order assertions.
---

# Write Use Case Happy Path Tests

Use this skill when the task is to add or rewrite happy-path tests for `compute_changes()` and, when needed, its replayable-event projection contract.

This is a single-entry skill:
- Start from existing use case code.
- Do not start from a freeform user prompt.
- Do not invent business rules that cannot be read from the current command, state, entity, changes, and event-projection logic.

This skill only covers use case happy-path specification tests. It does not cover:
- `pre_check_command()`
- `validate_against_state()`
- property tests
- workflow / adapter tests
- entity unit tests

## Read First

Load these files before writing tests:
- Main calibration example: `lib/example/core/src/use_case/trading/spot/match_order/compute_replayable_events_happy_path.rs`
- Style template: `lib/example/core/src/use_case/trading/spot/match_order/compute_replayable_events_spec_style_template.rs`
- Contract: `lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`

Read these references from this skill when needed:
- `references/example_breakdown.md`
- `references/checklist.md`

## Workflow

1. Read the real use case before naming any test.
- Inspect `compute_changes()`.
- Inspect the command, given state, error type, changes type, `to_replayable_events()`, and entity methods it relies on.
- Identify the business facts inside `Changes`, then the emitted event kinds and their business order.

2. Derive business scenarios from code, not branch names.
- Extract the business action, actors, success outcomes, and finish states.
- Identify scenario dimensions such as side, execution intent, time-in-force, or equivalent domain axes.
- Identify the observable business facts in `Changes`.
- Identify which replayable-event facts are contractually important enough to assert.

3. Write the happy-path matrix first.
- At the top of the test file, list:
  - scenario dimensions
  - outcome kinds
  - event expectations
  - current coverage
- If you add a new test, be able to point to the matrix cell it covers.

4. Choose the smallest useful scenario set.
- Each test should protect one core business rule.
- Avoid multiple unrelated rules in one test.
- Do not add cases that only repeat the same rule with cosmetic input changes.

5. Name tests as business sentences.
- Prefer `who + condition + outcome`.

6. Write spec comments in this exact shape.
- `Rule:`
- `Given:`
- `When:`
- `Then:`

The comments should explain business meaning, not Rust mechanics.

7. Structure the body in this exact order.
- `arrange`
- `act`
- `assert`

8. Assert business facts and event order.
- Assert `Changes` first when the business truth lives there.
- Assert whether a trade event exists when a trade must happen.
- Assert maker update events for makers whose state changes.
- Assert taker update events for the taker finish state.
- For multi-event flows, assert order explicitly.
- Treat event order as part of the business spec, not an implementation detail.

## Assertion Rules

- Assert `events.len()`, but never stop there.
- Assert the business identity fields that make the event meaningful.
- Assert trade price / qty / side when trade facts matter.
- Assert order status transitions and versions on update events.
- If `filled_qty` did not change, assert `None`, not `Some(0)`.
- If the scenario ends with a business reject/cancel reason, assert `status_reason`.
- If the scenario ends as a normal successful fill or partial fill without reject semantics, assert `status_reason == None`.
- When event projection matters, assert `changes.to_replayable_events()` after asserting the business fields inside `changes`.

Prefer helpers that encode business facts, such as:
- `assert_trade_event_for_accounts(...)`
- `assert_order_update_event(...)`

Do not add helpers that only rename a trivial `assert_eq!`.

## Output Checklist

Before finishing, verify:
- You read the real `compute_changes()` and `to_replayable_events()` first.
- The file header includes a scenario matrix and `current coverage`.
- Every test name is a business sentence.
- Every test uses `Rule/Given/When/Then`.
- Every test body uses `arrange/act/assert`.
- Every test asserts `Changes` business semantics before event projection details when both are relevant.
- Event order is asserted when multiple events are emitted.
- `filled_qty` and `status_reason` assertions match the business semantics.
- Each test clearly protects one matrix cell or one core happy-path rule.
