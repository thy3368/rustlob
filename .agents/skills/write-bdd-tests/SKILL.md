---
name: write-bdd-tests
description: Write RustLOB BDD tests that reverse-validate domain design. Use when Codex should add or review BDD tests for entity behavior methods or command-style use cases, especially to verify entity FSM transitions, use case core business matrices, MI causal chains, document-chain correctness, business invariants, and whether entity/use case design matches the domain. Do not use for adapter, workflow, HTTP, database, UI, or generic end-to-end BDD tests.
---

# Write BDD Tests

## Purpose

Use BDD as a design falsification tool, not as coverage decoration. A good RustLOB BDD test should make a domain design claim executable: given a meaningful business situation, the entity or use case either advances the correct business fact chain or exposes that the current design is wrong.

This skill only targets core business design. Do not use it to demonstrate routing, transport, persistence, macro output, logs, or full-process tutorials.

## Allowed BDD Types

1. Entity behavior method BDD
   - Test public aggregate-root or entity behavior methods.
   - Scope the test target to the public FSM transition `state × event` matrix.
   - Cover valid state-to-state transitions, invalid state/event combinations, and terminal-state rejection of subsequent events.
   - Assert only the starting state, input event, success or failure, and the resulting state on success.
   - Do not expand entity BDD assertions to `Output` business fields, versions or timestamps, amount/quantity invariants, full before/after entity equality on failure, `apply`/`transition` equivalence, constructor field storage, or query/invariant methods.
   - Treat ordinary getters, converters, mappers, and technical helpers as non-BDD targets.

2. Use case BDD
   - Test command-style use cases against the core contract `Command × authoritative StateGiven -> StateChanged`.
   - Treat `compute_state_changed_only` after-truth derivation as the primary business coverage target.
   - Build a core business matrix that covers applicable command intentions, authoritative states, eligibility or preconditions, successful decisions, business rejections, terminal branches, and boundary conditions.
   - Validate each successful `StateChanged`, including its business results, MI causal chain, document chain, conservation rules, changes/events, participating aggregates' after truth, and terminal facts.
   - Exercise the full validation chain through public `compute_state_changed(...)`; do not bypass the public API merely to call `compute_state_changed_only` directly.
   - `StateDiff`, `compute_state_diff`, before/after diff pairs, `ReplayableChanges::to_replayable_events()`, and replay projection are outside this skill's test requirements.

## Read First

Before writing or reviewing tests, read the relevant shared constraints from the repository:

- Entity behavior BDD:
  - `.agents/skills/shared/entity_method_constraints.md`
- Use case BDD:
  - `.agents/skills/shared/use_case_entity_constraints.md`
- MI causal chain or document-chain assertions:
  - `.agents/skills/shared/mi_chain.md`
  - If the task is mainly causal-chain review or scoring, combine with `review-mi-causal-chain`.

Also read the target entity/use case code and nearby tests before editing. Let existing naming, fixtures, builders, and assertion style drive the test shape.

## Entity Behavior BDD Workflow

Start from the public FSM transition and its Rustdoc/business comments. Confirm the event name and parameters are business language, not adapter or persistence language. For this skill, entity BDD means the transition `state × event` matrix; the shared entity architecture policy remains a design reference but does not broaden these test assertions.

Build the matrix and write scenarios that prove:

- Every valid state/event pair reaches the documented next state.
- Invalid state/event pairs fail.
- Terminal states reject every subsequent event that is not documented as valid.

Keep each entity scenario focused on the starting state, the input event, whether the transition succeeds or fails, and the resulting state when it succeeds. Output payloads, versions, timestamps, amounts, quantities, invariants, failure immutability, `apply` behavior, constructor persistence, and query/invariant methods are outside this skill's entity BDD scope unless a separate task explicitly requests them.

If a clean BDD scenario requires reaching through private internals, assembling adapter DTOs, or mocking infrastructure, treat that as design feedback. The behavior method may have the wrong boundary or signature.

## Use Case BDD Workflow

Start from the real use case implementation, its Rustdoc, and domain language, not from an imagined API or the implementation's branch layout. Identify the command intentions/actions, authoritative `StateGiven`, business eligibility and preconditions, `StateChanged` variants, participating aggregates, rejection meanings, and terminal facts.

Before writing scenarios, construct the core business matrix for `Command × authoritative StateGiven -> StateChanged`. Include every applicable dimension and combination:

- Every legal combination and its expected successful business result.
- Every important illegal combination and its specific business rejection.
- State conflicts, repeated operations, terminal states, and meaningful boundary conditions.
- For cross-aggregate collaboration, the after truth of every participating aggregate.
- The required business facts in `StateChanged`, including changes/events, MI causal links, document links, conservation results, and terminal facts.

Each BDD scenario must map to an explicit matrix cell. Judge completeness primarily by business-matrix coverage, not Rust line/branch coverage, function count, fixture count, or a few representative happy paths. If combinations are intentionally equivalent or inapplicable, record the business reason instead of silently omitting them.

Exercise the matrix through public `compute_state_changed(...)`, which runs the complete validation chain. Keep each layer's responsibility clear:

- `check_command` rejects malformed command facts that do not require state.
- `validate_state_given` rejects commands that conflict with authoritative state.
- `compute_state_changed_only` derives the core after truth and is the primary matrix coverage target.
- Successful results contain the precise typed `StateChanged` required to express the decision, including the business result and all required changes/events.
- Errors are asserted by business meaning, not by string fragments or technical side effects.

Do not add `StateDiff`, `compute_state_diff`, before/after diff-pair, replayable-event conversion, or replay-projection tests under this skill. Those are separate concerns and do not count toward the core use case matrix.

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
- Do not claim use case coverage from code branches, function counts, fixture counts, or selected happy paths; show which business-matrix cells the scenarios cover.
- Do not omit important rejection, state-conflict, repeated-operation, terminal-state, or boundary cells from the use case matrix.
- Do not expand use case BDD into `StateDiff`, `compute_state_diff`, before/after diff pairs, replayable-event conversion, or replay projection.
- Do not treat getters, serializers, DTO conversions, fixture builders, macros, or helper functions as behavior-method BDD targets.
- Do not hide missing business facts behind weak placeholder types, empty branches, broad matches, or lossy test fixtures.
- Do not test implementation order unless the order is a business event order or document-chain requirement.
- Do not make entity BDD a catch-all for output fields, version/timestamp checks, amount or quantity invariants, full entity equality on failure, `apply`/`transition` equivalence, constructor field storage, or query/invariant method behavior; those belong to other focused tests or architecture-policy validation.
