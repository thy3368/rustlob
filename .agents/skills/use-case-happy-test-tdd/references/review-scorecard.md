# Review Scorecard

This review contract is mandatory. Do not replace it with freeform review notes.

Always output these five sections in this exact order:
- `matrix completeness`
- `business naming`
- `assertion quality`
- `event order`
- `rule isolation`

Each section must contain:
- current weakness
- evidence from current tests
- next matrix cell to add or fix

## Section Guidance

### `matrix completeness`

Check whether the current tests cover the meaningful happy-path business cells implied by the use case semantics.

Score down when:
- the header has no matrix
- scenario axes are missing
- covered cells are not named
- tests exist but cannot be mapped back to concrete cells

### `business naming`

Check whether test names and `Rule/Given/When/Then` comments read as business rules rather than mechanics.

Score down when:
- names describe implementation behavior only
- names hide actor, condition, or outcome
- comments explain Rust steps instead of business meaning

### `assertion quality`

Check whether assertions prove `Changes` business facts first, not just event count or generic success.

Score down when:
- `Changes` semantics matter but are not asserted
- tests stop at `events.len()`
- event identity fields are not checked
- status transitions are under-asserted
- replayable-event projection matters but is not asserted after `Changes`

### `event order`

Check whether multi-event happy paths assert order when order is part of the business contract.

Score down when:
- multiple events are emitted
- order matters for replay/business meaning
- tests only assert set membership or count

### `rule isolation`

Check whether each test protects one matrix cell or one core happy-path rule.

Score down when:
- one test mixes unrelated rules
- fixture noise hides the protected rule
- several business cells are implicitly bundled without being named

## Review Discipline

- The scorecard is a decision tool for choosing the next highest-value missing cell.
- The next matrix cell must be concrete enough to become one new spec test immediately.
- If no worthwhile missing cell remains for the current ask, say so explicitly in the relevant sections and final recommendation.
