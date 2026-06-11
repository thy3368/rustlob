---
name: write-use-case-happy-path-tests
description: Write RustLOB happy-path specification tests for `CommandUseCase2::compute_replayable_events`. Use when Codex should add or rewrite use case tests that prove covered business scenarios from existing use case code, using a business matrix, `Rule/Given/When/Then`, and event-order assertions.
---

# Write Use Case Happy Path Tests

Use this skill when the task is to add or rewrite happy-path tests for `compute_replayable_events`.

This is a single-entry skill:
- Start from existing use case code.
- Do not start from a freeform user prompt.
- Do not invent business rules that cannot be read from the current command, state, entity, and event logic.

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
- Inspect `compute_replayable_events()`.
- Inspect the command, given state, error type, and entity methods it relies on.
- Identify the emitted event kinds and their business order.

2. Derive business scenarios from code, not branch names.
- Extract the business action, actors, success outcomes, and finish states.
- Identify scenario dimensions such as side, execution intent, time-in-force, or equivalent domain axes.
- Identify the observable business facts in emitted replayable events.

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
- Good:
  - `ioc_limit_taker_with_no_crossing_maker_rejects_single_taker_update`
  - `gtc_limit_taker_partially_fills_and_stops_at_first_non_crossing_maker`
- Bad:
  - `test_compute_events_1`
  - `should_work`
  - `returns_three_events`

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

Prefer helpers that encode business facts, such as:
- `assert_trade_event_for_accounts(...)`
- `assert_order_update_event(...)`

Do not add helpers that only rename a trivial `assert_eq!`.

## Driving Development

Use the tests to prove covered business scenarios and expose missing ones.

Required mindset:
- First derive the matrix.
- Then identify the uncovered happy-path cell.
- Then write the minimal test that proves that cell.

Treat these as design signals:
- If a required successful scenario cannot be expressed cleanly with current command/state shape, that is a design gap.
- If assertions require large amounts of fixture noise, extract helpers or narrow the scenario.
- If a test name sounds technical instead of business-facing, the scenario is probably underspecified.

## Output Shape

When you add or rewrite a happy-path file, aim for this structure:
- file header with purpose + matrix + current coverage
- local helper builders and assertion helpers
- business-sentence test names
- `Rule/Given/When/Then` comments
- `arrange/act/assert` sections

Preserve existing project terminology when the file already has a strong naming convention.

## Anti-Patterns

Do not:
- start from a user prompt alone
- infer business rules that are not backed by code
- mirror internal branch names into test names
- write tests that only prove `Ok(...)` or `events.len()`
- hide the key Given conditions inside opaque fixtures
- mix happy path coverage with `pre_check_command()` or `validate_against_state()` concerns in the same file

## Output Checklist

Before finishing, verify:
- You read the real `compute_replayable_events()` first.
- The file header includes a scenario matrix and `current coverage`.
- Every test name is a business sentence.
- Every test uses `Rule/Given/When/Then`.
- Every test body uses `arrange/act/assert`.
- Event order is asserted when multiple events are emitted.
- `filled_qty` and `status_reason` assertions match the business semantics.
- Each test clearly protects one matrix cell or one core happy-path rule.
