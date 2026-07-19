---
name: write-bdd-tests
description: Write RustLOB BDD tests that reverse-validate domain design. Use when Codex should add or review BDD tests for entity behavior methods or command-style use cases, especially to verify MI causal chains, document-chain correctness, behavior method signatures, business invariants, command plus given state yields changes, replayable events, and whether entity/use case design matches the domain. Do not use for adapter, workflow, HTTP, database, UI, or generic end-to-end BDD tests.
---

# Write BDD Tests

## Purpose

Use BDD as a design falsification tool, not as coverage decoration. A good RustLOB BDD test should make a domain design claim executable: given a meaningful business situation, the entity or use case either advances the correct business fact chain or exposes that the current design is wrong.

This skill only targets core business design. Do not use it to demonstrate routing, transport, persistence, macro output, logs, or full-process tutorials.

## Allowed BDD Types

1. Entity behavior method BDD
   - Test public aggregate-root or entity behavior methods.
   - Validate method signatures, state progression, downstream document derivation, causal pointers, and invariants.
   - Treat ordinary getters, converters, mappers, and technical helpers as non-BDD targets.

2. Use case BDD
   - Test command-style use cases as `command + authoritative state -> changes/events`.
   - Validate the business target, MI causal chain, document chain, conservation rules, and terminal facts.
   - Prefer `compute_changes(...)` and `ReplayableChanges::to_replayable_events()` surfaces when available.

## Read First

Before writing or reviewing tests, read the relevant shared constraints from the repository:

- Entity behavior BDD:
  - `.agents/skills/shared/entity_method_constraints.md`
  - `.agents/skills/shared/entity_use_case_api_policy.md`
- Use case BDD:
  - `.agents/skills/shared/use_case_entity_constraints.md`
  - `.agents/skills/shared/changes_pair_first_rule.md`
- MI causal chain or document-chain assertions:
  - `.agents/skills/shared/mi_chain.md`
  - If the task is mainly causal-chain review or scoring, combine with `review-mi-causal-chain`.

Also read the target entity/use case code and nearby tests before editing. Let existing naming, fixtures, builders, and assertion style drive the test shape.

## Entity Behavior BDD Workflow

Start from public behavior methods and their Rustdoc/business comments. Confirm the method name and parameters are business language, not adapter or persistence language.

Write scenarios that prove:

- The method advances only this aggregate/entity state, or derives a direct downstream business document.
- Inputs express the business cause, actor, amount, version, time, or predecessor document needed by the domain.
- State transitions reject invalid predecessors and protect terminal states.
- Version, sequence, status, reason, caused_by, due_to, and document identity fields are advanced explicitly.
- Amount, quantity, price, fee, or balance invariants are conserved where the domain requires conservation.

If a clean BDD scenario requires reaching through private internals, assembling adapter DTOs, or mocking infrastructure, treat that as design feedback. The behavior method may have the wrong boundary or signature.

## Use Case BDD Workflow

Start from the real use case implementation, not from an imagined API. Identify the command type, authoritative state type, changes type, reply type, and replayable event mapping.

Write scenarios around the business contract:

- `pre_check_command` rejects malformed command facts that do not require state.
- `validate_against_state` rejects commands that conflict with authoritative state.
- `compute_changes` emits the minimal business `Changes` needed to express the successful decision.
- `to_replayable_events` preserves event order, causality, document links, and terminal facts.
- Errors are asserted by business meaning, not by string fragments or technical side effects.

Prefer BDD names that say the business outcome, for example `given_accepted_order_when_trade_settles_then_balances_and_trade_documents_are_linked`. Avoid names that describe framework mechanics.

## Design Feedback Rule

When a BDD test fails, classify the failure before changing code:

- The test expectation is wrong or over-specified.
- The entity behavior method signature is not a good business API.
- The use case boundary is wrong or mixes adjacent business decisions.
- `Changes` lacks a required business fact, document, or terminal state.
- `caused_by`, `due_to`, predecessor, successor, or document-chain links are missing.
- A conservation, status, version, or terminal invariant is absent.

Do not weaken assertions just to make tests pass. A BDD failure is useful when it proves the design cannot express the intended business truth.

## Anti-Patterns

- Do not write primary BDD for adapter, workflow, HTTP, WebSocket, database, UI, logging, or generic end-to-end paths.
- Do not assert only `len()`, `is_ok()`, `is_err()`, or snapshot text.
- Do not treat getters, serializers, DTO conversions, fixture builders, macros, or helper functions as behavior-method BDD targets.
- Do not hide missing business facts behind weak placeholder types, empty branches, broad matches, or lossy test fixtures.
- Do not test implementation order unless the order is a business event order or document-chain requirement.
