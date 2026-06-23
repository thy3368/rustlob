---
name: proptest-entity
description: Write property-based tests for RustLOB core entities with `proptest`. Use when Codex needs to add or review independent entity scenario enums and strategies for files under `core/src/entity`, such as `stored_order.rs`, account snapshots, positions, balances, or any entity whose business state affects command use cases.
---

# Proptest Entity

## Overview

Use this skill when an entity needs its own property-based scenario coverage. Entity proptests
verify reusable business facts on the entity itself; they must not hide inside a specific use
case's `given_state_scenarios.rs`.

Reference pattern:

- Entity: `lib/example/core/src/entity/stored_order.rs`
- Entity scenario file: `lib/example/core/src/entity/stored_order/stored_order_scenarios.rs`

If the task involves `use case` vs `entity`, `behavior method`, `helper/query method`, `aggregate root`, `state machine`, or whether an action should be promoted into a `use case`, read `.agents/skills/shared/use_case_entity_aggregate_boundary.md` before writing properties.

## Workflow

1. Read the entity and the use cases that inspect it.
2. List the entity's meaningful business states before writing generators.
3. Create a dedicated test module next to the entity:
   - for `core/src/entity/stored_order.rs`, use `core/src/entity/stored_order/stored_order_scenarios.rs`
   - mount it from the entity file with `#[cfg(test)] mod stored_order_scenarios;`
4. Define one explicit scenario enum, such as `StoredOrderScenario`.
5. Define one business-aware strategy, such as `stored_order_scenario_strategy()`.
6. Write properties against the correct layer:
   - single-entity `behavior methods`: state-machine transitions, invariants, boundary states
   - aggregate-root `behavior methods`: internal coordination consistency
   - `helper/query methods`: derived values, judgment consistency, overflow or `None` semantics
7. Run targeted `rustfmt` and `cargo test -p <crate> <entity_or_scenario_filter>`.

## Scenario Enum Rules

The enum must name business states, not random data shapes. Examples:

- order states: open, partially filled, filled, canceled
- reservation states: consistent quote reserve, inconsistent quote reserve, consistent base reserve
- execution states: market intent, immediate limit, conditional limit
- account states: sufficient balance, insufficient balance, frozen balance mismatch, locked, disabled
- position states: flat, long, short, liquidatable, margin insufficient

Prefer variants with only the fields needed to construct that state:

```rust
#[derive(Debug, Clone)]
enum StoredOrderScenario {
    BuyLimitWithConsistentQuoteReserve { qty: u64, price: u64 },
    BuyLimitWithInconsistentQuoteReserve { qty: u64, price: u64, reserved_quote: u64 },
    OpenUnfilled { qty: u64, price: u64 },
    PartiallyFilled { qty: u64, price: u64, filled_qty: u64 },
    Filled { qty: u64, price: u64 },
    Canceled { qty: u64, price: u64, filled_qty: u64 },
}
```

## Strategy Rules

- Use `prop_map` / `prop_flat_map` to build valid domain-shaped entities.
- Keep ranges realistic and overflow-safe unless overflow is the scenario being tested.
- Use `prop_filter_map` only for branch-specific invalid scenarios where inequality matters.
- Generate boundary states explicitly: zero when allowed, one, max safe amount, exact mismatch.
- Do not generate meaningless arbitrary strings when the business uses stable examples such as
  `trader-1`, `BTCUSDT`, or `order-42`.

## Properties To Prefer

Entity properties should protect reusable business behavior:

- reservation consistency matches side, quantity, price, and frozen amount
- notional calculations return `None` for market intent or overflow
- lifecycle methods match business status, such as open/partially filled being cancelable
- release amounts match what cancel/reconcile use cases need
- `created_field_changes()` includes business fields needed to replay the entity
- `diff()` emits changes for lifecycle fields such as `status` and `filled_qty`

Avoid tautologies:

- Do not assert that a field equals itself after constructing the same fixture.
- Do not compute expected values by calling the method under test.
- Do not put use-case error mapping in entity proptests; use cases map entity facts to errors.
- Do not test cross-aggregate coordination here.
- Do not test use-case orchestration semantics here.
- Do not test command-level error mapping here.

## Composition With Use Cases

When a use case depends on entity state:

- entity scenario enum lives in the entity's own scenario file when testing entity behavior
- use-case-specific scenario enum may wrap or adapt entity scenarios when validating a command
- `given_state_scenarios.rs` should compose command/entity scenarios instead of embedding opaque
  fixtures

Example split:

- `entity/stored_order/stored_order_scenarios.rs` tests `StoredOrder` business behavior
- `use_case/.../cancel_order/stored_order_scenarios.rs` describes cancel-specific interpretations
- `use_case/.../cancel_order/given_state_scenarios.rs` composes loaded order/account/missing state

## Checklist

- The entity has an explicit scenario enum.
- The scenario file is independent from a specific use case unless it is intentionally
  use-case-specific.
- Variants enumerate supported business states such as open, partially filled, filled, canceled.
- Properties assert entity business methods, replay fields, or diff behavior.
- Comments are in Chinese for this repository.
- Tests run with a focused filter and no generated regression file is left unless it records a
  still-valid bug.
