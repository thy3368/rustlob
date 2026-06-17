# TDD Loop

This skill uses one strict loop:

`matrix -> scorecard -> choose one missing cell -> failing spec test -> minimal business repair -> matrix gap closed`

## Default Unit Of Work

The default unit of work is exactly one missing happy-path matrix cell.

Do not:
- batch-fill multiple missing cells
- add a family of similar tests in one pass
- broaden the task into general coverage cleanup

Only do more than one cell when the user explicitly asks.

## Required Sequence

1. Read the real use case, domain state, entity methods, `Changes`, projection logic, error, and current happy tests.
2. Build the full happy-path business matrix from semantics.
3. Review current coverage with the fixed scorecard.
4. Choose one highest-value missing matrix cell.
5. Write the missing happy-path spec test first.
6. Confirm the failure reveals a real missing business behavior or missing coverage.
7. Repair the smallest amount of core business code needed.
8. Re-run verification and name the exact matrix cell now covered.

## Stop Condition

Stop once all of these are true:
- the chosen matrix cell was missing at the start
- the new spec test was written first
- the test initially failed or the gap was otherwise demonstrated
- the minimal repair now makes it pass
- you can name the exact matrix cell that is now closed

Do not continue by default after this point.

## Repair Rule

Prefer repairing the use case first.

Prefer repairing missing `Changes` semantics before replayable-event projection.

Allow entity repair only when:
- the missing happy semantics are a reusable business rule
- the rule belongs to the domain model
- keeping it in the use case would duplicate or misplace domain logic

Do not satisfy a happy-path core business gap by changing adapter or infra behavior.
