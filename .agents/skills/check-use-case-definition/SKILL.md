---
name: check-use-case-definition
description: Check whether RustLOB `StateMachineV2` implementations are defined as business use cases, score files or directories, and explain rubric failures.
---

# Check Use Case Definition

Run the offline checker before giving a freeform verdict:

```bash
python3 .agents/skills/check-use-case-definition/scripts/check_use_case_business_definition.py <path ...>
python3 .agents/skills/check-use-case-definition/scripts/check_use_case_business_definition.py --all
```

Add `--json` for machine-readable output. Exit code `2` means invalid input or no matching implementation; resolve that before judging the design. Exit code `1` means at least one use case has findings.

The checker evaluates:

- business command identity
- separation of `check_command`, `validate_state_given`, and `compute_state_changed_only`
- typed `StateChanged`
- optional `StateMachineV2Diff` and `ReplayableChanges`
- entity/use-case boundaries
- absence of adapter, database, persistence, publish, and transport concerns in core

Use `.agents/skills/shared/use_case_review_scorecard.md` as the rubric. Preserve these response sections because callers depend on them: `Layer Mapping`, `四色建模 Mapping`, `Identity Semantics`, `Score`, `Findings`, and `Minimal Refactor`.
