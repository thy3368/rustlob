---
name: review-use-case
description: Review and score RustLOB command-style use cases. Use when Codex needs to inspect `*/workflow/*.rs`, map a use case to `core / adapter / infra`, apply four-color modeling, identify boundary or naming problems, or score the design with a Clean Architecture and four-color rubric.
---

# Review Use Case

## Overview

Review a RustLOB use case as design, not just syntax. The job is to determine whether the use case keeps business logic in `CommandUseCase4`, keeps orchestration outside, and models the business action cleanly under Clean Architecture and four-color modeling.

Start from these source files:
- Contract: `lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`
- Shared calibration examples: `lib/common/cmd_handler/src/use_case_examples/`
- Shared constraints: `.agents/skills/shared/use_case_entity_constraints.md`
- Shared `Changes` rule: `.agents/skills/shared/changes_pair_first_rule.md`
- Real V4 examples:
  - `lib/example/core/src/use_case/trading/derivatives/hyperliquid_perp/execution/match_perp_order.rs`
  - `lib/example/core/src/use_case/trading/spot/cancel_order.rs`

Load [references/scorecard.md](references/scorecard.md) before scoring.
Read `.agents/skills/shared/use_case_entity_constraints.md` before reviewing.
Load `.agents/skills/shared/changes_pair_first_rule.md` before scoring `Changes`.
Read `lib/common/cmd_handler/src/use_case_examples/good.rs` and `lib/common/cmd_handler/src/use_case_examples/bad.rs` when you need a fast good-vs-bad calibration before reviewing a real use case.

## Workflow

1. Identify the use case surface.
- Find the `CommandUseCase4` implementation.
- Identify the command, `GivenState`, `Changes`, replayable-event projection, error, reply mapper, load port, event pipeline, and any `CommandEnvelope` metadata around it.

2. Produce layer mapping with Clean Architecture terms.
- `core.use_case`: the `CommandUseCase4` implementation
- `core.entity`: the domain state and invariants it depends on
- `adapter.outbound`: load, persist, replay, publish, and reply mapping collaborators
- `infra`: DB, broker, HTTP, runtime, SDK, filesystem, VM engine, tracing backend

3. Produce four-color mapping.
- `Role`: the business-game role the party is playing in this use case
- `Moment-Interval`: the business action, transaction, or lifecycle step the use case represents
- `Party/Place/Thing`: the aggregate, account, order, block, request, wallet, market, or other business object
- `Description`: policies, classifications, capabilities, commands, error categories, and event descriptions

4. Check actor semantics before scoring.
- `party_id` belongs to the business command, not to technical metadata.
- `role()` should describe the business-game role, not a technical component or module.
- `Changes` should stay a pure business result, not a transport DTO, adapter type, or raw event bag.
- update 场景默认应该是 pair-first；如果同时维护 `UpdatedEntityPair<T>` 和重复 `*_after`，应视为 violation。
- Read the relationship as:
  - `party_id` = which business party
  - `role()` = which business role that party is playing
  - `command` = which business action that party is issuing
- If a role name sounds like a module, gateway, engine, executor, service, or adapter, score it down unless it is also a real business role in the domain.
- If `trace_id` is used as business identity or idempotency identity, score it down.
- `command_id` should be interpreted as the stable business command identity for idempotency and deduplication.

5. Score the use case with the rubric.
- Use the category weights in `references/scorecard.md`.
- Report both category scores and total score out of 100.

6. Give findings and minimal refactor advice.
- Findings must lead with the biggest design violations.
- Refactor advice must be the smallest set of changes that materially raises the score.
- Explicitly check whether the use case is duplicating reusable business rules that should live on an entity.

## Scoring Heuristics

- Score down when the use case directly calls repositories, clients, HTTP, DB, SDK, filesystem, or runtime machinery.
- Score down when one use case directly calls another use case.
- Score down when state snapshots already contain precomputed answers, so the use case is only copying results out.
- Score down when the use case directly hand-builds `EntityReplayableEvent` instead of deriving strong typed `Changes` first.
- Score down when `Changes` is only an event container and does not encode real business meaning.
- Score down when business derivation and replayable-event projection are mixed into one opaque step.
- Score down when update `Changes` violates the shared pair-first rule by keeping both `UpdatedEntityPair<T>` and a duplicate `*_after` snapshot.
- Score down when entity logic is missing and reusable business rules are duplicated inline in the use case.
- Score down when an entity appears to exist only for one use case instead of serving as a reusable business object.
- Score down when `role()` names a technical component instead of a business-game role.
- Score down when one use case bundles multiple unrelated business moments.
- Score down when names hide business meaning behind generic technical words.
- Score down when `party_id` is missing even though the command is clearly issued by a business party.
- Score down when `trace_id` and `command_id` are semantically confused.

## Output Format

Always return these sections:
- `Layer Mapping`
- `四色建模 Mapping`
- `Identity Semantics`
- `Score`
- `Findings`
- `Minimal Refactor`

Keep the summary brief. The main value is the score rationale and the findings.

When pair-first violations exist:
- call them out explicitly in `Findings`
- explain why the duplicate `after` field creates double-truth ambiguity
- include removing the duplicate `after` snapshot in `Minimal Refactor`
