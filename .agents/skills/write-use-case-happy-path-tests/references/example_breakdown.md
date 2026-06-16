# Example Breakdown

Use `lib/example/core/src/use_case/trading/spot/match_order/compute_replayable_events_happy_path.rs` as the main calibration example for this skill.

## What Makes It A Good Reference

It does not treat the test file as a bag of fixtures. It treats the file as a compact business spec for `compute_changes()` first, then `to_replayable_events()` where event projection is part of the contract.

The file has five important layers:
1. file header
2. scenario builders
3. changes and event assertion helpers
4. business-sentence tests
5. explicit event-order assertions

## 1. File Header

The header explains:
- what the file is proving
- which scenario dimensions exist
- which cells are already covered

This matters because happy-path tests are not just examples. They are a coverage map for the supported business scenarios.

Minimum useful header content:
- scenario axes
- outcome kinds
- event expectation shape
- current coverage

## 2. Builders Are Business Setup, Not Generic Fixtures

Helpers like `taker_buy_limit(...)`, `maker_sell(...)`, and `taker_buy_market(...)` keep each test readable.

Good builder properties:
- names use domain terms
- they expose the variables that matter to the scenario
- they avoid unrelated noise in each test body

Bad builder properties:
- hide critical scenario facts
- use generic names like `fixture1`
- require readers to jump through unrelated setup to understand the `Given`

## 3. Assertion Helpers Protect Business Facts

`assert_trade_event_for_accounts(...)` is a good helper because it encodes trade identity and trade facts:
- trade id
- maker/taker ids
- account ids
- side
- price
- qty

`assert_order_update_event(...)` is a good helper because it captures the update-event contract:
- old/new version
- optional `filled_qty`
- status
- optional `status_reason`

These helpers keep the tests focused on business meaning rather than event parsing boilerplate.

When a V4 use case has rich `Changes`, add helpers that assert:
- created domain facts
- updated entity before/after pairs
- finish-state semantics before any event projection details

## 4. Test Names Must Read Like Business Rules

Examples from the file:
- `gtc_limit_taker_matches_multiple_makers_and_fills_completely`
- `ioc_limit_taker_with_no_crossing_maker_rejects_single_taker_update`
- `market_ioc_buy_with_no_liquidity_rejects_with_market_reason`

These names work because they answer:
- who is acting
- under what condition
- what business outcome must happen

Bad replacements would be:
- `test_gtc_case`
- `compute_events_returns_ok`
- `returns_one_reject_event`

Those names describe mechanics, not business rules.

## 5. Rule / Given / When / Then Is Doing Real Work

Each block has a job:

- `Rule:` states the policy being protected.
- `Given:` states the business setup that makes the rule relevant.
- `When:` states the use case action.
- `Then:` states the expected business outcomes and event shape.

Do not waste these blocks on Rust trivia such as:
- “create state”
- “call function”
- “assert result”

Those are mechanics. The comments should explain why this scenario exists.

## 6. Changes First, Event Order Second

For `CommandUseCase4`, the first assertion target is `Changes`:
- created trades
- updated maker/taker entities
- before/after snapshots
- finish-state semantics

Only after the business truth is asserted should the test project replayable events when event shape or order is part of the contract.

## 7. Event Order Is Part Of The Spec

For multi-event happy paths, the file asserts exact order:
- trade first
- maker update next
- taker update last

This is not ornamental. For event-sourced use cases, order often encodes business meaning and replay semantics.

If a scenario emits more than one event and order matters, assert it directly.

## 8. How To Reuse This Pattern On Another Use Case

When you move to another `compute_changes` implementation:
1. read the use case and entity methods
2. read the `Changes` type and `to_replayable_events()` projection
3. identify scenario axes
4. write the matrix at the top of the file
5. build small domain helpers
6. write tests as business rules
7. assert `Changes` first, then emitted event facts and order when needed

If you cannot complete step 2 cleanly, stop and inspect the domain model again. The problem is usually unclear business semantics, not missing test syntax.
