---
name: product-manager-red-test-gate
description: Use Hyperliquid as a competitor source to discover missing product capabilities and define them as real Rust entity/use_case Red tests, preferably with rstest fixtures and parameterized cases. Use when asked to mine requirements, find feature gaps, define product acceptance tests, or create PM-style Red tests.
---

# Product Manager Red Test Gate

Use this skill when turning competitor-discovered product gaps into executable Rust acceptance tests for RustLOB. The main output is real `entity` or `use_case` test code. Do not create YAML, Markdown, or prose-only requirement intermediates.

## Product-Manager Output

- Convert only accepted competitor gaps into real Rust tests.
- Do not generate `capability_gap.yaml`, Markdown backlogs, or pure textual requirements as the primary artifact.
- Do not use `red_spec!`, `panic!("RED SPEC")`, empty tests, or other pseudo-tests in place of meaningful assertions.
- Treat the test itself as the product acceptance line.

## Competitor Source

- Treat Hyperliquid as the first competitor fact source.
- Extract business capabilities from actions, APIs, WebSocket behavior, block samples, and observable examples.
- Do not copy competitor behavior unconditionally. Translate each accepted capability into RustLOB business semantics before writing tests.
- Keep wire names such as `updateLeverage` only as source trace comments when useful; the Rust test names and assertions should speak RustLOB domain language.

## Test Responsibility Split

- `core.entity` tests verify domain object behavior, invariants, state transitions, and boundary conditions. Entities must not know commands, queries, adapters, HTTP, WebSocket, or external schema details.
- `core.use_case` tests verify `Command + GivenState -> Changes / Events / Error`.
- `adapter.inbound` may translate Hyperliquid facts into candidate capabilities.
- `adapter.outbound` may scan, report, or connect external references.
- `infra` owns `cargo test`, CI, filesystem access, Hyperliquid docs/API calls, and block samples.
- Do not let adapter, wire, serialization, endpoint, or schema details pollute `core.entity` or `core.use_case` tests.

## Red, Green, Refactor

- Red means generating a real failing test that defines a clear acceptance line.
- Green means the smallest implementation that makes the accepted test pass.
- Refactor means improving structure while keeping the tests Green.
- A Red test may fail by compilation failure, assertion failure, or business behavior failure.
- Do not make tests falsely Green by adding `#[ignore]`, deleting assertions, weakening semantics, adding empty branches, swallowing errors, or using placeholder types.

## DISCOVERED and REJECTED Inventory Tests

- Use `DISCOVERED` and `REJECTED` only as ignored inventory tests inside Rust code.
- Inventory tests must not block default `cargo test`.
- A `REJECTED` inventory test must include a concrete `reason`.
- Only `RED` and `GREEN` tests are ordinary blocking tests.

## rstest Usage

- Prefer `rstest` for code-level BDD when it improves clarity.
- Use `#[fixture]` for Given state.
- Use `#[rstest]` with `#[case(...)]` for scenario matrices.
- `rstest` may reduce boilerplate, but it must not hide the business assertion.

## Canonical Red Test Shape

```rust
use rstest::{fixture, rstest};

// competitor: hyperliquid
// feature: updateLeverage
// state: RED
// target: use_case
#[rstest]
#[case(5, 10)]
#[case(10, 20)]
fn update_leverage_changes_existing_position_risk_state(
    #[case] before_leverage: u64,
    #[case] after_leverage: u64,
) {
    let given = given_user_has_btc_perp_position_with_leverage(before_leverage);
    let command = command_update_leverage_to(after_leverage);

    let changes = UpdatePerpLeverageUseCase::default()
        .compute_changes(command, given)
        .expect("合法杠杆调整应该成功");

    assert!(changes.contains_leverage_changed_to(after_leverage));
    assert!(changes.contains_position_risk_recalculated());
    assert!(changes.to_replayable_events().is_ok());
}
```

## Inventory Test Shapes

Use ignored tests only for capabilities that are discovered or rejected but not accepted as current blocking acceptance work.

```rust
// competitor: hyperliquid
// feature: twapOrder
// state: DISCOVERED
// target: use_case
// reason: 已发现竞品能力，但业务语义尚未确认
#[test]
#[ignore = "DISCOVERED: not yet accepted into implementation backlog"]
fn discovered_hyperliquid_twap_order() {}
```

```rust
// competitor: hyperliquid
// feature: approveBuilderFee
// state: REJECTED
// target: use_case
// reason: 当前产品阶段不支持 builder fee 模型
#[test]
#[ignore = "REJECTED: not in current product scope"]
fn rejected_hyperliquid_approve_builder_fee() {}
```

## Implementation Guardrails

- Locate the business boundary before writing tests: `core.entity`, `core.use_case`, adapter, or infra.
- Prefer Chinese comments and assertion messages when explaining product semantics in this RustLOB repo.
- Keep Red tests close to the module that should own the behavior.
- Add or reuse `rstest` dependencies only where the local crate already permits test dependencies; avoid broad workspace dependency churn unless required.
- When a competitor capability is not accepted, leave it as an ignored inventory test with `DISCOVERED` or `REJECTED` instead of turning it into a blocking Red test.
