---
name: use-case-happy-test-tdd
description: Strict RustLOB `CommandUseCase4` happy-path TDD for reviewing happy coverage, completing happy-path business matrices, adding one missing happy scenario at a time, and driving minimal core business fixes from a failing spec test.
---

# Use Case Happy Test TDD

Use this skill when the task is not just to write a happy-path test, but to use one missing happy-path spec test as a business repair loop for a RustLOB `CommandUseCase4` use case.

This skill is stricter than `write-use-case-happy-path-tests`:
- it must review current happy coverage first
- it must reconstruct the full happy-path business matrix first
- it must use a fixed review scorecard
- it must choose exactly one highest-value missing matrix cell by default
- it must write the missing spec test before repairing code
- it must stop once that one matrix gap is demonstrably closed

Keep this skill independent. Do not rewrite or subsume `write-use-case-happy-path-tests`.

## Scope

This skill only covers:
- `CommandUseCase4::compute_changes(...)`
- `ReplayableChanges::to_replayable_events()`
- supported successful business semantics
- happy-path event contracts, including event order when multiple events matter
- minimal business repair in the use case
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

Load these files before doing any review or TDD work:
- Main calibration example: `lib/example/core/src/use_case/trading/spot/match_order/compute_replayable_events_happy_path.rs`
- Existing happy-test calibration breakdown: `.agents/skills/write-use-case-happy-path-tests/references/example_breakdown.md`
- Contract: `lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`
- Shared boundary reference: `.agents/skills/shared/use_case_entity_aggregate_boundary.md`
- Shared `Changes` rule: `.agents/skills/shared/changes_pair_first_rule.md`

Load these references from this skill:
- `references/tdd-loop.md`
- `references/review-scorecard.md`
- `references/matrix-completion.md`

Read the calibration example as style input. Apply the stricter TDD workflow from this skill on top of it.
If the task involves `use case` vs `entity`, `behavior method`, `helper/query method`, `aggregate root`, `state machine`, or whether an action should be promoted into a `use case`, read `.agents/skills/shared/use_case_entity_aggregate_boundary.md` before review or repair.
Read `.agents/skills/shared/changes_pair_first_rule.md` before reviewing happy-path semantics or repairing `Changes`.

## Required Workflow

1. Read the real current code first.
- Read the real use case implementation.
- Read the command, state, changes, projection logic, error, and entity methods the use case relies on.
- Read the current happy-path tests before proposing new coverage.
- Start from the repository's real business semantics, not an imagined spec.

2. Reconstruct the happy-path business matrix from semantics.
- Derive scenario axes from domain meaning, not from branch names alone.
- Every matrix must include these observation dimensions where applicable:
  - actor / intent
  - liquidity relation or matching condition
  - finish state
  - event shape
  - status transition
- Add use-case-specific axes when needed, but do not reduce the matrix to branch-by-branch coverage.

3. Write the matrix before adding any test.
- Show the scenario axes.
- Show the currently covered cells.
- Show the missing cells.
- Every proposed test must map to one concrete matrix cell.

4. Review current coverage with the fixed scorecard.
- Use the exact five sections defined in `references/review-scorecard.md`.
- No freeform-only review is allowed.
- Treat duplicate pair + duplicate `*_after` fields as a scorecard violation, not a harmless style choice.

5. Select one missing highest-value matrix cell by default.
- Default unit of work is exactly one missing matrix cell.
- Prefer the highest-value gap that closes a real business-semantic blind spot.
- Do not batch-fill multiple cells unless the user explicitly asks.

6. Write the happy-path spec test first.
- Use spec-style comments and business naming.
- Tie the test to the exact chosen matrix cell.
- Keep the test focused on one happy-path rule.

7. Confirm the failure exposes a real business gap.
- Run the test or otherwise prove the current coverage/behavior is missing that cell.
- If the test passes already, do not force a repair. Re-evaluate the chosen cell.

8. Apply the smallest business repair.
- Prefer repairing the use case first.
- Only repair an entity when the missing happy semantics are a reusable business rule that belongs in the domain model.
- Do not change adapter or infra behavior to satisfy a core happy-path business gap.
- Prefer missing `Changes` semantics first; repair event projection separately only when the business change is already correct and the replayable contract is wrong.
- If update `Changes` currently keeps pair plus duplicate `after`, repair the pair-first shape before tolerating dual-track assertions.

9. Verify the exact matrix gap is now closed.
- Re-run the new test and relevant nearby tests.
- State which exact matrix cell is now covered.
- Stop after closing that one cell unless the user explicitly asks for more.

## TDD Stop Rule

The default stopping condition is strict:
- one missing matrix cell is chosen
- one new happy spec test is written first
- the test initially fails or the gap is otherwise proven missing
- the minimal core business repair makes that test pass
- the exact matrix cell now covered is named explicitly

Do not continue to add more happy-path cells by default after that point.

## Repair Boundary

Apply this boundary strictly:
- prefer repairing the use case first when the gap is cross-aggregate coordination or business-boundary semantics
- allow single-entity repair when the missing semantics are a reusable entity `behavior method`
- allow aggregate-root repair when the missing semantics are same-aggregate consistency behavior
- if the missing gap is only a derived judgment or calculation, do not promote it into an independent `use case`
- never move business repair into adapter or infra
- prioritize repairing `Changes` into pair-first semantics over adding tests that normalize duplicate fields

If you recommend entity repair, explain why the rule is reusable domain behavior rather than one use case's orchestration detail.

## Output Contract

Always produce, in order:
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

## Validation Scenarios

Validate the skill behavior against these scenario types:

1. Partial happy coverage in a real use case.
- Expected behavior:
  - emit the matrix first
  - use the fixed scorecard to identify a concrete gap
  - choose exactly one missing matrix cell
  - tie the new spec test to that exact cell

2. Missing happy semantics that belong in an entity rule.
- Expected behavior:
  - explain why the rule is reusable domain logic
  - allow repair in the entity
  - leave adapter and infra untouched

3. Use case with already-sufficient happy coverage for the current ask.
- Expected behavior:
  - still perform the review
  - find no higher-value missing cell to add
  - avoid inventing extra tests or broad rewrites

## Acceptance Criteria

Before finishing a task under this skill, verify that the workflow:
- starts from real code, not a freeform imagined rule set
- emits a matrix before new test design
- uses the fixed five-part review scorecard
- maps each new test to one exact matrix cell
- repairs one cell only by default
- asserts `Changes` semantics before replayable-event projection when both matter
- does not normalize duplicate pair + `*_after` fields in new tests; it pushes repair back into `Changes`
- treats event order as part of the business contract when multiple events matter
- keeps business repair in core, never adapter or infra
- only pushes logic into an entity when the rule is reusable domain behavior
