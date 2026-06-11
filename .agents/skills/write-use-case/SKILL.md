---
name: write-use-case
description: Write RustLOB command-style use cases around `CommandUseCase2`. Use when Codex needs to add or refactor files such as `*/workflow/*.rs`, define command/state/event/reply types, implement `pre_check_command` or `validate_against_state`, wire `UseCaseReplyMapper`, or keep persistence/load/publish logic out of the core use case.
---

# Write Use Case

## Overview

Implement use cases the way this repository expects: business rules live in `CommandUseCase2`, orchestration lives in `CommandUseCaseExecutor2`, and state loading, persistence, replay, publish, and reply mapping stay outside the core.

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
- optional `Reply`
- optional `ReplyMapper`
- `UseCase`

2. Shape the replayable output around the current contract.
- `compute_replayable_events()` returns `Result<Vec<EntityReplayableEvent>, Self::Error>`.
- Do not invent a wrapper event-set trait unless the surrounding module truly needs one.
- Keep emitted events replayable and derived from `cmd + state`, not from a prebuilt transport reply.

3. Implement `CommandUseCase2`.
- `role()` returns the business-game role, not a framework or module name.
- `type Command` should implement `IssuedByParty` when a business party issues the command.
- `pre_check_command()` only does cheap checks on the command itself.
- `validate_against_state()` checks business invariants that need loaded state.
- `compute_replayable_events()` derives replayable domain events from command and state.

4. Keep these concerns out of the core use case.
- Do not call DB, cache, broker, HTTP, filesystem, or VM registry lookup directly from the use case unless the state was already injected as part of `GivenState`.
- Do not persist, replay, or publish events inside the use case.
- Do not map domain events to HTTP or API replies inside the use case.
- Do not measure latency inside the business logic. The executor does that.
- Do not call another `use_case` from inside the current `use_case`. If multiple business actions must cooperate, move that coordination to a higher orchestration layer.

5. Put adapter concerns in the right places.
- `CommandUseCaseOutbound<Command, State>` belongs to the execution side and loads state from ports or adapters.
- The outbound implementation owns `load_state`, `persist`, `replay`, and `publish`.
- `UseCaseReplyMapper` maps replayable events to external reply DTOs.
- `CommandEnvelope.meta` owns technical metadata such as `trace_id` and `command_id`.
- `command_id` is the stable business command identity for idempotency and deduplication.

## Project Rules

- Prefer names that read as a business sentence:
  - `ReceiveAndAdmitTransactionsCmd`
  - `ReceiveAndAdmitTransactionsStateSnapshot`
  - `ReceiveAndAdmitTransactionsReplyMapper`
  - `ReceiveAndAdmitTransactionsUseCase`
- `GivenState` should be a domain snapshot, not a repository handle.
- `Vec<EntityReplayableEvent>` should represent replayable business outputs, not transport responses.
- Keep `Error` domain-specific. Avoid stringly typed `Result<_, String>` as the main API.
- Keep `compute_replayable_events()` deterministic for the same command and state.
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
- `compute_replayable_events()` produces the expected replayable events.
- `ReplyMapper` maps domain events correctly if a reply mapper exists.
- `CommandUseCaseExecutor2::execute()` covers one happy path and one rejection path using stub `CommandUseCaseOutbound`.

Testing split:
- `compute_replayable_events()` happy-path tests must use the sibling skill `write-use-case-happy-path-tests` and stay as business specification tests.
- Keep `pre_check_command()` and `validate_against_state()` tests separate from the happy-path spec file.
- Use `proptest` only to add invariant coverage; do not replace happy-path spec tests with it.

## Output Checklist

Before finishing, verify:
- The use case type only encodes business logic.
- External state loading happens through `CommandUseCaseOutbound::load_state`.
- Side effects happen through outbound `persist` / `replay` / `publish`.
- Reply shaping happens through `UseCaseReplyMapper`.
- The use case does not directly invoke another use case.
- Any reusable business rule that belongs on an entity is not duplicated inline in the use case.
- Tests cover both direct method behavior and executor orchestration.
