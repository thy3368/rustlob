---
name: use-case-happy-test-tdd
description: Write or rewrite RustLOB `CommandUseCase4` happy-path specification tests, or run strict happy-path TDD repair for coverage gaps. Use when Codex should add spec-only happy tests for `compute_changes()` / `to_replayable_events()`, review happy coverage, complete a business matrix, choose one missing matrix cell, write the failing spec test first, and make the minimal core business repair.
---

# Use Case Happy Test TDD

Use this skill as the single happy-path test entry point for RustLOB `CommandUseCase4` use cases.

It supports two modes:

- `Spec-only mode`: use when the user asks to add, write, rewrite, or improve happy-path spec tests. Start from real code and current semantics. Do not repair business code by default; if a new test reveals a behavior gap, report it unless the user also asked for a fix.
- `TDD repair mode`: use when the user asks for review, coverage gaps, TDD, repair, or fixing happy-path behavior. Run the strict loop: `matrix -> scorecard -> choose one missing cell -> failing spec test -> minimal business repair -> matrix gap closed`.

## Scope

This skill only covers:
- `CommandUseCase4::compute_changes(...)`
- `ReplayableChanges::to_replayable_events()`
- supported successful business semantics
- happy-path event contracts, including event order when multiple events matter
- minimal business repair in the use case during TDD repair mode
- dependent entity repair only when the missing happy semantics are a reusable domain rule

This skill does not cover:
- `pre_check_command()`
- `validate_against_state()`
- property tests
- workflow tests
- adapter tests
- infra tests
- pushing business rules into adapter or infra

## Read First

Load these files before writing or reviewing happy-path tests:
- Main calibration example: `lib/example/core/src/use_case/trading/spot/match_order/compute_replayable_events_happy_path.rs`
- Style template: `lib/example/core/src/use_case/trading/spot/match_order/compute_replayable_events_spec_style_template.rs`
- Contract: `lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`
- Shared canonical `use_case` / `entity` facts: `.agents/skills/shared/use_case_entity_constraints.md`
- Shared `Changes` rule: `.agents/skills/shared/changes_pair_first_rule.md`

Load these references from this skill as needed:
- `references/example_breakdown.md` for calibration-test structure
- `references/checklist.md` before finishing spec-only test work
- `references/tdd-loop.md` for TDD repair mode
- `references/review-scorecard.md` for coverage review
- `references/matrix-completion.md` for matrix reconstruction

Read the calibration example as style input. Read `.agents/skills/shared/changes_pair_first_rule.md` before deriving assertions from `Changes`, reviewing happy-path semantics, or repairing `Changes`.
Read `.agents/skills/shared/use_case_entity_constraints.md` before review or repair when the task involves `use case` vs `entity`, aggregate role, `MI chain root`, or replay/version semantics.
If stronger architecture policy is needed, treat it as a separate source rather than as part of the shared constraints file.

## Mode Selection

Choose `Spec-only mode` when the user asks for:
- writing or rewriting happy-path tests
- adding missing happy spec coverage without asking to repair production code
- turning current successful behavior into business-spec tests

Choose `TDD repair mode` when the user asks for:
- review or scoring of current happy coverage
- finding coverage gaps
- TDD
- repair, fix, or implementation changes driven by a missing happy scenario
- closing one missing happy matrix cell end to end

If the request is ambiguous, default to `Spec-only mode` for test-writing wording and `TDD repair mode` for review/fix wording.

## Common Workflow

1. Read the real current code first.
- Inspect `compute_changes()`.
- Inspect the command, state, changes, projection logic, error type, and entity methods the use case relies on.
- Read current happy-path tests and helper terminology before proposing new coverage.
- Start from repository semantics, not an imagined spec.

2. Derive business scenarios from semantics.
- Extract the business action, actors, success outcomes, and finish states.
- Derive axes from domain meaning, not from branch names alone.
- Include these observation dimensions where applicable:
  - actor / intent
  - liquidity relation or matching condition
  - finish state
  - event shape
  - status transition
- Add use-case-specific axes when needed, such as side, execution mode, time-in-force, fill pattern, or multi-party interaction shape.

3. Write the happy-path matrix before adding any test.
- At the top of the test file or in the review output, list scenario dimensions, outcome kinds, event expectations, and current coverage.
- Show covered cells and missing cells.
- Every proposed test must map to one concrete matrix cell.

4. Choose the smallest useful scenario set.
- Each test should protect one core business rule or one concrete matrix cell.
- Avoid multiple unrelated rules in one test.
- Do not add cases that only repeat the same rule with cosmetic input changes.

5. Write spec-style tests.
- Name tests as business sentences, preferably `who + condition + outcome`.
- Use comments in this exact shape:
  - `Rule:`
  - `Given:`
  - `When:`
  - `Then:`
- Comments must explain business meaning, not Rust mechanics.
- Structure each body as `arrange`, `act`, `assert`.

6. Assert business facts and event order.
- Assert `Changes` first when the business truth lives there.
- For update scenarios, assert pair-first semantics before touching projected event fields.
- Assert `changes.to_replayable_events()` after asserting `Changes` when event projection matters.
- Assert `events.len()`, but never stop there.
- Assert identity fields that make each event meaningful.
- Assert trade price, quantity, and side when trade facts matter.
- Assert maker update events for makers whose state changes.
- Assert taker update events for the taker finish state.
- Assert order status transitions and versions on update events.
- Assert exact event order for multi-event flows when order is part of replay or business meaning.
- If `filled_qty` did not change, assert `None`, not `Some(0)`.
- If the scenario ends with a business reject or cancel reason, assert `status_reason`.
- If the scenario ends as a normal successful fill or partial fill without reject semantics, assert `status_reason == None`.

Prefer helpers that encode business facts, such as:
- `assert_trade_event_for_accounts(...)`
- `assert_order_update_event(...)`

Do not add helpers that only rename a trivial `assert_eq!`.

## Spec-Only Mode

Use this mode to produce happy-path spec tests without defaulting into production repair.

Required output:
1. the happy-path matrix or the matrix header added to the test file
2. the selected scenario cells or scenario set
3. the new or rewritten spec tests
4. verification that the tests compile/run, or the exact reason verification could not be completed

Before finishing, verify:
- the file header includes scenario dimensions, outcome kinds, event expectations, and `current coverage`
- every test maps to one matrix cell or one core happy-path rule
- every test name is a business sentence
- every test uses `Rule/Given/When/Then`
- every test body uses `arrange/act/assert`
- `Changes` semantics are asserted before replayable-event projection details when both are relevant
- update scenarios verify pair `after` and projected update events one by one
- event order is asserted when multiple events are emitted
- `filled_qty` and `status_reason` assertions match the business semantics

If a spec-only test exposes a production behavior gap, do not silently repair it. State the gap and ask or proceed only if the user's request included fixing behavior.

## TDD Repair Mode

Use this mode to close one missing happy-path matrix cell end to end.

Required workflow:

1. Reconstruct the happy-path business matrix from semantics.
- Show scenario axes.
- Show currently covered cells.
- Show missing cells.

2. Review current coverage with the fixed scorecard.
- Use the exact five sections defined in `references/review-scorecard.md`.
- No freeform-only review is allowed.
- Treat duplicate pair + duplicate `*_after` fields as a scorecard violation, not a harmless style choice.

3. Select one missing highest-value matrix cell by default.
- Default unit of work is exactly one missing matrix cell.
- Prefer the highest-value gap that closes a real business-semantic blind spot.
- Do not batch-fill multiple cells unless the user explicitly asks.

4. Write the happy-path spec test first.
- Use spec-style comments and business naming.
- Tie the test to the exact chosen matrix cell.
- Keep the test focused on one happy-path rule.

5. Confirm the failure exposes a real business gap.
- Run the test or otherwise prove the current coverage/behavior is missing that cell.
- If the test passes already, do not force a repair. Re-evaluate the chosen cell.

6. Apply the smallest business repair.
- Prefer repairing the use case first.
- Only repair an entity when the missing happy semantics are a reusable business rule that belongs in the domain model.
- Do not change adapter or infra behavior to satisfy a core happy-path business gap.
- Prefer missing `Changes` semantics first; repair event projection separately only when the business change is already correct and the replayable contract is wrong.
- If update `Changes` currently keeps pair plus duplicate `after`, repair the pair-first shape before tolerating dual-track assertions.

7. Verify the exact matrix gap is now closed.
- Re-run the new test and relevant nearby tests.
- State which exact matrix cell is now covered.
- Stop after closing that one cell unless the user explicitly asks for more.

Required output, in order:
1. the happy-path matrix
2. the fixed five-part review scorecard
3. the chosen single matrix cell
4. the new failing spec test
5. the minimal business repair location and rationale
6. the verification that the chosen matrix cell is now closed

If the current happy coverage is already sufficient for the user's ask:
- still produce the matrix
- still produce the fixed review scorecard
- state that no higher-value missing cell remains for this task
- do not invent extra tests or broad rewrites

## Repair Boundary

Apply this boundary strictly in TDD repair mode:
- prefer repairing the use case first when the gap is cross-aggregate coordination or business-boundary semantics
- allow single-entity repair when the missing semantics are a reusable entity `behavior method`
- allow aggregate-root repair when the missing semantics are same-aggregate consistency behavior
- if the missing gap is only a derived judgment or calculation, do not promote it into an independent `use case`
- never move business repair into adapter or infra
- prioritize repairing `Changes` into pair-first semantics over adding tests that normalize duplicate fields

If you recommend entity repair, explain why the rule is reusable domain behavior rather than one use case's orchestration detail.

## Validation Scenarios

Validate skill behavior against these scenario types:

1. Spec-only happy-path test writing.
- Expected behavior:
  - read real code first
  - write the matrix first
  - add business-sentence tests with `Rule/Given/When/Then`
  - assert `Changes` before event projection
  - avoid repairing production code unless asked

2. Partial happy coverage in a real use case.
- Expected behavior:
  - emit the matrix first
  - use the fixed scorecard to identify a concrete gap
  - choose exactly one missing matrix cell in TDD repair mode
  - tie the new spec test to that exact cell

3. Missing happy semantics that belong in an entity rule.
- Expected behavior:
  - explain why the rule is reusable domain logic
  - allow repair in the entity
  - leave adapter and infra untouched

4. Use case with already-sufficient happy coverage for the current ask.
- Expected behavior:
  - still perform the review when in TDD repair mode
  - find no higher-value missing cell to add
  - avoid inventing extra tests or broad rewrites
