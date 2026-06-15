---
name: write-use-case
description: Write RustLOB command-style use cases around `CommandUseCase3`. Use when Codex needs to add or refactor files such as `*/workflow/*.rs`, define command/state/output/reply types, implement `pre_check_command` or `validate_against_state`, wire `UseCaseReplyMapper3`, or keep persistence/load/publish logic out of the core use case.
---

# Write Use Case

## Overview

Implement use cases the way this repository now expects: business rules live in `CommandUseCase3`, orchestration lives in `CommandUseCaseExecutor3`, and state loading, persistence, replay, publish, and reply mapping stay outside the core.

Treat the source code as the contract of truth. This skill summarizes the expected shape, but if the skill and the trait differ, follow the current code under `lib/common/cmd_handler/src/command_use_case_def2/`.

Start from these source files:
- Contract: `lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`
- Executor: `lib/common/cmd_handler/src/command_use_case_def2/executor.rs`
- Shared calibration examples: `lib/common/cmd_handler/src/use_case_examples/`
- Shared constraints: `.agents/skills/shared/use_case_entity_constraints.md`

Read `lib/common/cmd_handler/src/use_case_examples/good.rs` and `lib/common/cmd_handler/src/use_case_examples/bad.rs` when you need good-vs-bad source examples before writing a new use case.
Read `.agents/skills/shared/use_case_entity_constraints.md` before writing or refactoring a use case.
If the task is to critique or score a use case, use the sibling skill `review-use-case` instead.

## Workflow

1. Define the types in this order:
- `Error`
- `Cmd`
- `StateSnapshot` or `GivenState`
- `Output`
- optional `Reply`
- optional `ReplyMapper3`
- `UseCase`

2. Shape business output around the current contract.
- `compute_output_and_events()` returns `Result<UseCaseOutput<Self::Output>, Self::Error>`.
- `output` is the current use case's strong typed business result for internal reuse.
- `events` are the only facts meant for persist / replay / publish.
- Do not maintain two unrelated derivation paths for `output` and `events`.

3. Implement `CommandUseCase3`.
- `role()` returns the business-game role, not a framework or module name.
- `type Command` should implement `IssuedByParty` when a business party issues the command.
- `pre_check_command()` only does cheap checks on the command itself.
- `validate_against_state()` checks business invariants that need loaded state.
- `compute_output_and_events()` derives typed business output first, then the replayable domain events implied by that output.

4. Keep these concerns out of the core use case.
- Do not call DB, cache, broker, HTTP, filesystem, or VM registry lookup directly from the use case unless the state was already injected as part of `GivenState`.
- Do not persist, replay, or publish events inside the use case.
- Do not map domain output or events to HTTP or API replies inside the use case.
- Do not measure latency inside the business logic. The executor does that.
- Do not call another `use_case` from inside the current `use_case` unless the current use case is intentionally modeling a higher-level business pipeline in core and still returns one `UseCaseOutput`.

5. Put adapter concerns in the right places.
- `CommandUseCaseOutbound<Command, State>` belongs to the execution side and loads state from ports or adapters.
- The outbound implementation owns `load_state`, `persist`, `replay`, and `publish`.
- `UseCaseReplyMapper3` maps `UseCaseOutput<Self::Output>` to external reply DTOs.
- `CommandEnvelope.meta` owns technical metadata such as `trace_id` and `command_id`.
- `command_id` is the stable business command identity for idempotency and deduplication.

## Project Rules

- Prefer names that read as a business sentence:
  - `ReceiveAndAdmitTransactionsCmd`
  - `ReceiveAndAdmitTransactionsStateSnapshot`
  - `ReceiveAndAdmitTransactionsOutput`
  - `ReceiveAndAdmitTransactionsReplyMapper`
  - `ReceiveAndAdmitTransactionsUseCase`
- `GivenState` should be a domain snapshot, not a repository handle.
- `UseCaseOutput<Self::Output>` should represent a pure business result plus replayable business facts, not transport responses.
- Keep `Error` domain-specific. Avoid stringly typed `Result<_, String>` as the main API.
- Keep `compute_output_and_events()` deterministic for the same command and state.
- `party_id` belongs to the business command, not to `CommandMeta`.
- `trace_id` is only for tracing. Do not use it as the idempotency key.
- Treat `entity` as a reusable core collaborator, not a private struct owned by one use case.
- If business logic is likely reusable across use cases, prefer a domain-semantic entity method over duplicating the rule in the use case.

## Testing

For a new use case, add the smallest useful test set inline in the same file unless the crate already has a stronger convention.

Required tests:
- `role()` returns the intended actor.
- `pre_check_command()` rejects malformed command-only input.
- `validate_against_state()` rejects invalid state transitions.
- `compute_output_and_events()` produces the expected typed output and replayable events.
- `ReplyMapper3` maps business output to external reply correctly if a reply mapper exists.
- `CommandUseCaseExecutor3::execute()` covers one happy path and one rejection path using stub `CommandUseCaseOutbound`.
- `CommandUseCaseExecutor3::execute_and_map_reply()` covers typed output to reply mapping once the reply surface exists.

Testing split:
- `compute_output_and_events().events` happy-path tests must use the sibling skill `write-use-case-happy-path-tests` and stay as business specification tests.
- Keep `pre_check_command()` and `validate_against_state()` tests separate from the happy-path spec file.
- Use `proptest` only to add invariant coverage; do not replace happy-path spec tests with it.

## Output Checklist

Before finishing, verify:
- The use case type only encodes business logic.
- `Output` is a pure business result, not an adapter or transport DTO.
- `events` are derived from the same business decision as `output`.
- External state loading happens through `CommandUseCaseOutbound::load_state`.
- Side effects happen through outbound `persist` / `replay` / `publish`.
- Reply shaping happens through `UseCaseReplyMapper3`.
- The use case does not directly invoke outer technical workflow machinery.
- Any reusable business rule that belongs on an entity is not duplicated inline in the use case.
- Tests cover both direct method behavior and executor orchestration.
