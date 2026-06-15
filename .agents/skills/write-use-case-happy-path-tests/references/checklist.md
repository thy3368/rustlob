# Happy Path Spec Test Checklist

Use this checklist before finishing a `compute_output_and_events(...).events` happy-path test file.

## Read Path

- Read the actual `compute_output_and_events()` implementation first.
- Read the related command, given state, error, and entity methods.
- Read existing helpers and test terminology before inventing new names.

## Matrix

- Write the scenario matrix at the file top before adding cases.
- Include scenario dimensions, outcomes, and event expectations.
- Include `current coverage`.
- Be able to explain which matrix cell each test covers.

## Naming

- Every test name is a business sentence.
- Prefer `who + condition + outcome`.
- Avoid technical names like `returns_ok`, `works`, or `case_1`.

## Comment Structure

- Every test includes `Rule:`.
- Every test includes `Given:`.
- Every test includes `When:`.
- Every test includes `Then:`.
- Comments describe business meaning, not Rust mechanics.

## Body Structure

- Every test body is split into `arrange`, `act`, and `assert`.
- Important scenario facts are visible in the test body.
- Critical business conditions are not hidden in opaque fixtures.

## Assertions

- `events.len()` is asserted, but not used as the only proof.
- Trade event existence is asserted when a trade must happen.
- Maker update events are asserted when maker state changes.
- Taker update events are asserted for the finish state.
- Multi-event order is asserted explicitly.
- `filled_qty` unchanged is asserted as `None`.
- `status_reason` is asserted when reject/cancel semantics matter.
- Normal successful completion asserts `status_reason == None` when appropriate.

## Scope Discipline

- The file only covers happy-path `compute_output_and_events(...).events`.
- `pre_check_command()` tests are elsewhere.
- `validate_against_state()` tests are elsewhere.
- Property-test concerns are elsewhere.

## Final Quality Bar

- Each test protects one core business rule.
- Removing a test would remove a real business guarantee, not just duplicate another case.
- The file reads like a compact requirements spec for supported successful scenarios.
