---
name: check-use-case-definition
description: Check whether RustLOB `CommandUseCase3` use cases are correctly defined as business use cases. Use when Codex should run the offline checker script, score files or directories, or explain why a use case failed the business-definition rubric.
---

# Check Use Case Definition

Use this skill for RustLOB `CommandUseCase3` business-definition checks. Prefer the offline checker script over freeform scoring.

## Workflow

1. Run the checker first.
- Explicit targets: `python3 scripts/check_use_case_business_definition.py <path ...>`
- Whole repo: `python3 scripts/check_use_case_business_definition.py --all`
- Machine-readable output: add `--json`
- The local script is a compatibility wrapper over the workflow/use-case review tool, so keep the CLI focused on review inputs and thresholds.

2. Treat the script as the primary judge.
- The rubric source is `.agents/skills/review-use-case/references/scorecard.md`.
- Do not invent a second scoring rule in the response.

3. When a use case fails:
- Explain the outcome from the script's `Findings`.
- Turn `Minimal Refactor` into the concrete next actions you recommend.
- Keep the explanation aligned with `Layer Mapping`, `四色建模 Mapping`, `Identity Semantics`, and `Score`.

4. When the script exits `2`:
- Fix the path, readability, or parse issue before giving a design verdict.
