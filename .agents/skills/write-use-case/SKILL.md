---
name: write-use-case
description: Write RustLOB command-style use cases around `CommandUseCase2`. Use when Codex needs to add or refactor files such as `*/use_case/*.rs`, define command/state/event/reply types, implement `pre_check_command` or `validate_against_state`, wire `UseCaseReplyMapper`, or keep persistence/load/publish logic out of the core use case.
---

# Write Use Case

## Overview

Implement use cases the way this repository expects: business rules live in `CommandUseCase2`, orchestration lives in `CommandUseCaseExecutor2`, and storage, publish, and reply mapping stay outside the core.

Start from these source files:
- Contract: `lib/common/cmd_handler/src/use_case_def2.rs`
- Small example: `lib/common/cmd_handler/src/use_case_impl.rs`
- Shared calibration examples: `lib/common/cmd_handler/src/use_case_examples/`
- Real L1 examples:
  - `lib/core/l1/src/use_case/command_handler/receive_and_admit_transactions.rs`
  - `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs`

Read `lib/common/cmd_handler/src/use_case_examples/good.rs` and `lib/common/cmd_handler/src/use_case_examples/bad.rs` when you need good-vs-bad source examples before writing a new use case.
If the task is to critique or score a use case, use the sibling skill `review-use-case` instead.

## Workflow

1. Define the types in this order:
- `Error`
- `Cmd`
- `StateSnapshot` or `GivenState`
- `Events`
- optional `Reply`
- optional `ReplyMapper`
- `UseCase`

2. Implement `TraceableEventSet` for the event type.
- `event_count()` should report something meaningful for tracing and metrics.
- Do not fake event counts with unrelated numbers.

3. Implement `CommandUseCase2`.
- `role()` returns the business-game role, not a framework or module name.
- `type Command` should implement `IssuedByParty` when a business party issues the command.
- `format_error()` returns short readable strings for tracing.
- `pre_check_command()` only does cheap checks on the command itself.
- `validate_against_state()` checks business invariants that need loaded state.
- `compute_replayable_events()` derives replayable domain events from command and state.

4. Keep these concerns out of the core use case.
- Do not call DB, cache, broker, HTTP, filesystem, or VM registry lookup directly from the use case unless the state was already injected as part of `GivenState`.
- Do not persist, replay, or publish events inside the use case.
- Do not map domain events to HTTP or API replies inside the use case.
- Do not measure latency inside the business logic. The executor does that.

5. Put adapter concerns in the right places.
- `LoadState<Cmd, State, Err>` belongs to the execution side and loads state from ports or adapters.
- `DomainEventPipeline<E, Err>` owns `persist`, `replay`, and `publish`.
- `UseCaseReplyMapper<E>` maps events to external reply DTOs.
- `CommandEnvelope.meta` owns technical metadata such as `trace_id` and `command_id`.
- `command_id` is the stable business command identity for idempotency and deduplication.

## Project Rules

- Prefer names that read as a business sentence:
  - `ReceiveAndAdmitTransactionsCmd`
  - `ReceiveAndAdmitTransactionsStateSnapshot`
  - `ReceiveAndAdmitTransactionsEvents`
  - `ReceiveAndAdmitTransactionsUseCase`
- `GivenState` should be a domain snapshot, not a repository handle.
- `ThenTraceableEvents` should be replayable business outputs, not transport responses.
- Keep `Error` domain-specific. Avoid stringly typed `Result<_, String>` as the main API.
- Keep `compute_replayable_events()` deterministic for the same command and state.
- `party_id` belongs to the business command, not to `CommandMeta`.
- `trace_id` is only for tracing. Do not use it as the idempotency key.

## Testing

For a new use case, add the smallest useful test set inline in the same file unless the crate already has a stronger convention.

Required tests:
- `role()` returns the intended actor.
- `pre_check_command()` rejects malformed command-only input.
- `validate_against_state()` rejects invalid state transitions.
- `compute_replayable_events()` produces the expected events.
- `ReplyMapper` maps domain events correctly if a reply mapper exists.
- `CommandUseCaseExecutor2::execute()` covers one happy path and one rejection path using stub `LoadState` and stub `DomainEventPipeline`.

## Output Checklist

Before finishing, verify:
- The use case type only encodes business logic.
- External state loading happens through `LoadState`.
- Side effects happen through `DomainEventPipeline`.
- Reply shaping happens through `UseCaseReplyMapper`.
- Tests cover both direct method behavior and executor orchestration.
