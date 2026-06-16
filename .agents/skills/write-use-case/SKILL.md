---
name: write-use-case
description: Write RustLOB command-style use cases around `CommandUseCase4`. Use when Codex needs to add or refactor files such as `*/workflow/*.rs`, define command/state/changes/reply types, implement `pre_check_command` or `validate_against_state`, wire `UseCaseReplyMapper`, or keep persistence/load/publish logic out of the core use case.
---

# Write Use Case

## Overview

Implement use cases the way this repository now expects: business rules live in `CommandUseCase4`, orchestration lives in `CommandUseCaseExecutor4`, and state loading, persistence, replay, publish, and reply mapping stay outside the core.

Treat the source code as the contract of truth. This skill summarizes the expected shape, but if the skill and the trait differ, follow the current code under `lib/common/cmd_handler/src/command_use_case_def2/`.

Start from these source files:
- Contract: `lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`
- Executor: `lib/common/cmd_handler/src/command_use_case_def2/executor.rs`
- Shared calibration examples: `lib/common/cmd_handler/src/use_case_examples/`
- Shared constraints: `.agents/skills/shared/use_case_entity_constraints.md`
- Shared `Changes` rule: `.agents/skills/shared/changes_pair_first_rule.md`

Read `lib/common/cmd_handler/src/use_case_examples/good.rs` and `lib/common/cmd_handler/src/use_case_examples/bad.rs` when you need good-vs-bad source examples before writing a new use case.
Read `.agents/skills/shared/use_case_entity_constraints.md` before writing or refactoring a use case.
Read `.agents/skills/shared/changes_pair_first_rule.md` before shaping `Changes`.
If the task is to critique or score a use case, use the sibling skill `review-use-case` instead.
优先参考现有 V4 真实现例：
- `lib/example/core/src/use_case/trading/derivatives/hyperliquid_perp/execution/match_perp_order.rs`
- `lib/example/core/src/use_case/trading/spot/cancel_order.rs`

## Workflow

1. Define the types in this order:
- `Error`
- `Cmd`
- `GivenState`
- `Changes`
- optional `Reply`
- optional `ReplyMapper`
- `UseCase`

2. Shape business output around the current contract.
- `type Changes: ReplayableChanges` is the use case's only business truth.
- `compute_changes()` returns `Result<Self::Changes, Self::Error>`.
- replayable events are a projection of `changes`, not a sibling derivation path.
- `Changes` must carry business semantics, not just wrap `Vec<EntityReplayableEvent>`.
- update 场景按 shared 规范默认 pair-first，优先使用 `UpdatedEntityPair<T>` 表达业务变化。
- 避免并列维护可由 pair `after` 直接投影出的重复 `*_after` 快照。

3. Implement `CommandUseCase4`.
- `role()` returns the business-game role, not a framework or module name.
- `type Command` should implement `IssuedByParty` when a business party issues the command.
- `pre_check_command()` only does cheap checks on the command itself.
- `validate_against_state()` checks business invariants that need loaded state.
- `compute_changes()` derives strong typed domain changes first.
- `ReplayableChanges::to_replayable_events()` projects those changes into persist / replay / publish facts.

4. Keep these concerns out of the core use case.
- Do not call DB, cache, broker, HTTP, filesystem, or VM registry lookup directly from the use case unless the state was already injected as part of `GivenState`.
- Do not persist, replay, or publish events inside the use case.
- Do not map domain changes or events to HTTP or API replies inside the use case.
- Do not measure latency inside the business logic. The executor does that.
- Do not call another `use_case` from inside the current `use_case` unless the current use case is intentionally modeling a higher-level business pipeline in core and still returns one coherent `Changes`.

5. Put adapter concerns in the right places.
- `CommandUseCaseOutbound<Command, State>` belongs to the execution side and loads state from ports or adapters.
- The outbound implementation owns `load_state`, `persist`, `replay`, and `publish`.
- `CommandUseCaseExecutor4::execute()` returns `UseCaseChanges<Self::Changes>`.
- `UseCaseReplyMapper` maps replayable events to external reply DTOs.
- `CommandEnvelope.meta` owns technical metadata such as `trace_id` and `command_id`.
- `command_id` is the stable business command identity for idempotency and deduplication.

## Project Rules

- Prefer names that read as a business sentence:
  - `ReceiveAndAdmitTransactionsCmd`
  - `ReceiveAndAdmitTransactionsGivenState`
  - `ReceiveAndAdmitTransactionsChanges`
  - `ReceiveAndAdmitTransactionsReplyMapper`
  - `ReceiveAndAdmitTransactionsUseCase`
- `GivenState` should be a domain snapshot, not a repository handle.
- `Changes` should represent pure business facts before projection, not transport responses and not a raw event bag.
- update 型 `Changes` 默认先建模 authoritative pair，再考虑是否真的需要额外业务结果字段。
- Keep `Error` domain-specific. Avoid stringly typed `Result<_, String>` as the main API.
- Keep `compute_changes()` deterministic for the same command and state.
- Keep `to_replayable_events()` deterministic for the same changes.
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
- `compute_changes()` produces the expected strong typed changes.
- `to_replayable_events()` projects the expected replayable events from those changes.
- `UseCaseReplyMapper` maps replayable events to external reply correctly if a reply mapper exists.
- `CommandUseCaseExecutor4::execute()` covers one happy path and one rejection path using stub `CommandUseCaseOutbound`.

Testing split:
- `compute_changes()` happy-path tests must use the sibling skill `write-use-case-happy-path-tests` and stay as business specification tests.
- Assert `changes` first, then assert `changes.to_replayable_events()` when the replayable-event contract matters.
- Keep `pre_check_command()` and `validate_against_state()` tests separate from the happy-path spec file.
- Use `proptest` only to add invariant coverage; do not replace happy-path spec tests with it.

## Output Checklist

Before finishing, verify:
- The use case type only encodes business logic.
- `Changes` is the pure business result, not an adapter or transport DTO.
- replayable events are projected from the same `Changes`, not derived by a separate business path.
- External state loading happens through `CommandUseCaseOutbound::load_state`.
- Side effects happen through outbound `persist` / `replay` / `publish`.
- Reply shaping happens through `UseCaseReplyMapper`.
- The use case does not directly invoke outer technical workflow machinery.
- Any reusable business rule that belongs on an entity is not duplicated inline in the use case.
- Tests cover both direct method behavior and executor orchestration.
