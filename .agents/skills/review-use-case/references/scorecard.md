# Use Case Scorecard

Use this rubric to score a RustLOB use case.

## 1. Clean Architecture: 60

### 1.1 Boundary purity: 15
- `15`: only business rules; no DB, API, broker, runtime, file, or metrics logic
- `8`: minor leakage such as tracing, formatting, or adapter-shaped DTO coupling
- `0`: clear infrastructure or orchestration logic in the use case

### 1.2 Dependency direction: 15
- `15`: depends only on plain domain state, ports, or generic contracts
- `8`: depends on semi-adapter concepts or transport-shaped state
- `0`: depends directly on repositories, clients, SDK types, or framework details

### 1.3 Responsibility focus: 15
- `15`: `pre_check_command`, `validate_against_state`, and `compute_replayable_events` are sharply separated
- `8`: some duplication or mixed concerns, but still understandable
- `0`: validation, state loading, mutation, and side effects are tangled

### 1.4 Orchestration split: 15
- `15`: loading, persistence, replay, publish, reply mapping, and metrics all stay outside
- `8`: one of those concerns leaks inward
- `0`: use case coordinates outer workflow directly

## 2. 四色建模: 40

四色建模在这里按四类业务概念检查：
- `Role`
- `Moment-Interval`
- `Party/Place/Thing`
- `Description`

### 2.1 Role clarity: 10
- `10`: `role()` clearly names the business-game role the party is playing
- `5`: role exists but is vague or half business, half implementation
- `0`: role is missing, or is purely technical such as gateway, engine, executor, service, or adapter

### 2.2 Moment-Interval clarity: 10
- `10`: one use case represents one coherent business action or lifecycle step
- `5`: action is understandable but too broad or split oddly
- `0`: use case bundles multiple unrelated business moments

### 2.3 Party/Place/Thing modeling: 10
- `10`: `GivenState` and events center on real domain objects and state
- `5`: partial domain modeling mixed with transport or storage shapes
- `0`: state is mostly repositories, raw payloads, or infrastructure containers

### 2.4 Description quality: 10
- `10`: command, error, event, and policy names use clear domain language
- `5`: naming is mixed between business and implementation terms
- `0`: names are generic, technical, or hide business meaning

## 2.5 Identity semantics check

This check influences both `Role clarity` and `Description quality`:
- `party_id`: business party instance, carried by the command
- `role()`: business-game role played by that party in this use case
- `command_id`: stable business command identity, used for idempotency and deduplication
- `trace_id`: tracing correlation id only, not the business identity of the command

Downgrade when:
- `party_id` is treated as transport-only metadata
- `role()` names a machine component instead of a business role
- `trace_id` is used as the deduplication or idempotency key
- `command_id` is absent even though retries or duplicate delivery matter

## 3. Review Template

Use this output shape:

```text
Layer Mapping
- core.use_case: ...
- core.entity: ...
- adapter.outbound: ...
- infra: ...

四色建模 Mapping
- Role: ...
- Moment-Interval: ...
- Party/Place/Thing: ...
- Description: ...

Identity Semantics
- party_id: ...
- role(): ...
- command_id: ...
- trace_id: ...

Score
- Clean Architecture: 52/60
- 四色建模: 31/40
- Total: 83/100

Findings
- ...
- ...

Minimal Refactor
- ...
- ...
```

## 4. Downgrade Triggers

Subtract aggressively when you see these:
- use case calls repository, client, HTTP, or DB directly
- use case returns transport replies instead of domain events
- state type is just a bag of adapters
- one method both validates and persists
- business actor is unnamed
- `role()` is actually a technical component name
- `party_id` is missing from a command that is clearly issued by a business party
- `trace_id` is being used as the idempotency key
- one use case handles multiple unrelated workflows
- command, error, or event names hide domain meaning behind generic words like `process`, `handle`, `data`, or `result`
