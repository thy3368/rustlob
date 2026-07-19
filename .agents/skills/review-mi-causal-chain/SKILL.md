---
name: review-mi-causal-chain
description: Review business MI causal chains and score them out of 100. Use when Codex should audit Moment-Interval evidence chains from design text or code, `caused_by` / `due_to` links, predecessor-to-successor predicates, conservation invariants, terminal settled facts, compensation/red-chong behavior, business traceability, or RustLOB use case/entity MI evidence for trading, clearing, settlement, wallet, liquidation, ADL, insurance fund, or similar domains.
---

# Review MI Causal Chain

## Purpose

Review whether a proposed business `MI` causal chain can serve as an auditable evidence chain from original business facts to terminal settled facts.

This is a review skill, not a code-writing skill and not a replacement for the shared MI references.

## Capability Boundaries

This skill is a business evidence-chain auditor. It can answer whether a proposed MI chain is complete, traceable, and auditable as business history.

It can answer:

- Whether candidate facts qualify as MIs under the shared references.
- Whether the causal chain from source fact to terminal settled fact is complete.
- Whether `caused_by` / `due_to` pointers make each successor fact accountable to its predecessor evidence.
- Whether predecessor-to-successor predicates are clear enough to explain why the next business fact is produced.
- Whether asset, quantity, fee, hold/release, transfer, and counterparty conservation can be audited.
- Whether the terminal business fact closes the chain without overwriting historical truth.

It cannot answer:

- Whether the business strategy or product policy is desirable.
- Whether the product should be designed this way.
- Whether arbitrary code is correct outside the MI evidence-chain surface.
- Whether the database schema, event store, or API shape is optimal.
- Whether financial statements are compliant or complete.
- How legal responsibility should be assigned.

When adjacent questions appear, keep the review scoped to the MI evidence chain. In code review mode, inspect code only as evidence for MI identity, causal traceability, conservation, and terminal closure. It is acceptable to say that a finding creates product, engineering, finance, risk, or legal follow-up work, but do not make the final decision for those roles.

## Role-Based Audit Lens

Use role lenses to expose different audit concerns without leaving the MI-chain scope.

- Product / business: Check whether the chain expresses the real business promise, user-visible commitment, lifecycle boundary, and terminal settled fact.
- Risk control: Check whether preconditions, failure branches, forced progression conditions, margin/limit predicates, and unsafe continuation cases are explicit.
- Finance / clearing and settlement: Check whether assets, fees, holds, releases, transfers, liabilities, and counterparty legs are conserved and reconcilable.
- Compliance / audit: Check whether facts are append-only, traceable, replayable, recoverable, and attributable to original evidence without mutation or deletion.
- Engineering: Separate business facts from adapter mappings, persistence steps, publication steps, executor calls, validations, field values, and other technical artifacts. When reviewing code, use file/module membership and entity/change fields as evidence, not as automatic proof of a business MI.

Do not let role lenses turn the review into product strategy, risk policy approval, financial reporting, legal advice, or general code review. The output should identify MI evidence-chain gaps those roles must decide or implement.

## Misuse Warnings

- Do not treat a process flowchart, handler sequence, table-write sequence, message bus path, or API workflow as an MI causal chain unless the items are real business facts.
- Do not classify state fields, balance fields, commands, validation checks, requests, responses, adapters, persistence records, or publish steps as MIs merely because they appear in the flow.
- Do not use a high score to hide a fatal missing terminal fact, missing causal pointer, broken conservation invariant, or non-auditable mutation.
- Do not replace missing business predicates with technical statements such as "handler succeeded", "database updated", or "event published".
- Do not infer settlement closure from intermediate facts such as match, risk check, reservation, or accepted command unless the terminal settled fact is explicitly modeled.

## Required References

Read these references in order before scoring.

1. [`../shared/mi_chain.md`](../shared/mi_chain.md)
   Primary source for MI causal chains, `caused_by`, audit traceability, and terminal closure.
2. [`../shared/moment_interval_definition.md`](../shared/moment_interval_definition.md)
   Base definition for `Moment-Interval` and minimum judgment threshold.
3. [`../shared/mi.md`](../shared/mi.md)
   Extended MI judgment, main/secondary MI calibration, audit facts, append-only facts, naming, and settled facts.

If this skill and a shared reference disagree, follow the shared reference.

## Review Workflow

Use this fixed sequence.

1. Identify candidate `MI` values and non-`MI` items.
   Reject commands, validation steps, field values, persistence steps, publish steps, executor steps, adapter mappings, and pure technical artifacts unless the shared references justify them as business facts.
2. Reconstruct the claimed causal chain.
   Write the chain as ordered business facts, not as handler calls, table writes, or message flow.
3. Check each causal link as:
   `predecessor MI -> predicate / business rule -> successor MI`
   A link is weak when the predicate is implicit, purely technical, or unable to explain why the successor fact is legally/business-wise produced by the predecessor.
4. Check `caused_by` / `due_to` traceability.
   Every successor fact that claims causality should point to the predecessor MI or evidence fact that produced it. Missing pointers reduce auditability even when the business story sounds plausible.
5. Check conservation invariants.
   For trading, clearing, settlement, and wallet flows, look for balanced asset movement, fee accounting, hold/release behavior, transfer legs, counterparty symmetry, and no unexplained creation or disappearance of quantity/value.
6. Check terminal settled facts and compensation/red-chong behavior.
   The chain should state where the business truth closes. If reversal, compensation, red-chong, cancellation, or correction can occur, the chain should model how later facts preserve audit truth rather than overwriting history.
7. Score and recommend improvements.
   Deduct points for missing evidence, vague predicates, broken traceability, weak invariants, premature closure, non-MI misclassification, and non-actionable advice.

## Code Review Mode

Use this mode when the user asks to review MI chains from Rust/RustLOB code, use case files, entity files, `risk/` modules, or implementation artifacts.

Read code in this order:

1. Read the use case group `mod.rs`.
   Identify declared group boundary, main MI, secondary MIs, module exports, and files that exist but are not wired into the group.
2. Read each relevant use case file.
   Inspect `Command`, `GivenState`, `Changes`, `validate_against_state`, `compute_changes`, and `ReplayableChanges` / replayable event conversion. Treat comments and type names as claims, then verify whether fields and change records provide evidence.
3. Read the referenced entity files.
   Classify fields as explicit causal pointers (`caused_by`, `due_to`, `*_caused_by_*`, `source_*_id` with documented causal semantics), ordinary ID traces (`*_id`, `order_id`, `position_id`, `user_id`), status/lifecycle fields, quantities/balances, and append-only business facts.
4. Reconstruct the code-evidenced chain.
   Label each MI as coming from an entity, a changes type, a replayable event, or a comment/type declaration. Label each edge as `explicit caused_by`, `ID trace only`, or `inferred`.

Code evidence rules:

- A field named only `*_id` proves correlation or traceability, not causality. Do not upgrade it to `caused_by` unless the code or docs state predecessor evidence semantics.
- A status transition proves lifecycle movement, not a settled fact. Terminal closure needs an independent settled fact, close reason, exhaustion fact, or append-only record that states why the chain is closed.
- A balance, margin, position, or aggregate-state update proves a derived state changed, not that funds or positions were conserved. Conservation needs transfer legs, allocation records, fill records, shortfall records, or equivalent append-only vouchers.
- A `Changes` struct can be MI evidence only when it records a business fact or produces replayable business events. Pure state mutation diffs, cache updates, and technical publications are not MIs by themselves.
- A file that exists but is not exported or referenced from the group `mod.rs` is a chain-completeness doubt. Report it separately from business causality gaps.
- Comments and Rustdoc can clarify intended MIs and predicates, but they cannot replace missing fields or replayable facts.

## Scoring Rubric

Score out of 100.

- `MI识别准确性` 15
  Correctly separates real MIs from commands, fields, checks, adapter steps, and other non-MIs.
- `因果指针完整性` 20
  Each successor MI has explicit `caused_by` / `due_to` traceability to predecessor business facts.
- `判定规则清晰度` 20
  Each causal edge has a clear predicate, condition, or business rule that explains the transition.
- `守恒不变量` 15
  Asset, quantity, fee, hold/release, and counterparty invariants are explicit and balanced.
- `终局事实闭合` 15
  The chain reaches a real terminal settled fact and handles compensation/red-chong without rewriting history.
- `审计可追责性` 10
  The chain preserves who/what/when/why evidence and supports replay or audit reconstruction.
- `改进建议可执行性` 5
  Suggestions are concrete rewrites, missing facts, predicates, or invariant additions.

Use these verdict thresholds by default:
- `通过`: 85-100, no severe broken causal link or missing terminal closure.
- `需要修改`: 60-84, mostly valid structure with fixable omissions.
- `需要重建`: 0-59, wrong MI center, missing causal spine, missing settled fact, or non-auditable chain.

Do not let a high total score hide a fatal issue. If the chain lacks a terminal settled fact or has no auditable causal pointers, cap the verdict at `需要修改`; if the claimed center is not an MI and the chain cannot be reconstructed, cap the verdict at `需要重建`.

## Code-Based Deductions

Apply these caps and deductions when reviewing code evidence:

- If successor facts only contain ordinary `*_id` references and no explicit causal pointer or documented evidence field, `因果指针完整性` cannot receive full marks.
- If the chain closes by status update alone, with no settled fact, close reason, exhaustion fact, or equivalent append-only terminal record, deduct from `终局事实闭合`.
- If the implementation only updates aggregate state, position fields, margin fields, balances, or derived totals without transfer/allocation/fill/shortfall vouchers, deduct from `守恒不变量`.
- If a use case file exists but is not imported/exported by the group `mod.rs`, list it under broken or missing links as a chain-wiring doubt.
- If `ReplayableChanges` emits technical events but not business facts, deduct from `审计可追责性`.
- If code uses naming that implies an MI but the fields only describe commands, validations, or state snapshots, deduct from `MI识别准确性`.

## Calibration Checks

Use these checks for spot-trading-like chains:

- Missing `TradeExecuted` between order match and settlement should lose causal-chain and audit points.
- Missing four transfer legs or equivalent balanced settlement representation should lose invariant points.
- Missing `caused_by` / `due_to` pointers should lose traceability points even if facts are listed in the right order.
- A chain that stops at `OrderMatched` while claiming settlement closure is not closed.
- A correction that mutates or deletes the original fact instead of appending compensation/red-chong facts is not auditable.

Use these checks for derivatives liquidation / risk chains:

- Expected causal spine:
  `LiquidationEvent -> LiquidationExecution/Fill -> BankruptcyShortfall -> InsuranceFundAllocation/Transfer -> ADLBatch -> ADLExecution -> ADLDeleveragingRecord -> Closed/Exhausted`.
- `HyperliquidPerpLiquidation` or equivalent forced-liquidation fact is a valid main MI when it captures the user, position, trigger condition, required recovery, and liquidation lifecycle as business evidence.
- Require `required_quote = recovered_quote + shortfall_quote` or an equivalent named invariant for liquidation execution and shortfall creation.
- Require `shortfall_quote = covered_by_insurance_quote + covered_by_adl_quote + remaining_quote` or an equivalent named invariant for IF/ADL coverage.
- Insurance fund coverage should have an allocation or transfer voucher, not only a reduced shortfall field.
- ADL coverage should have execution and deleveraging records that identify affected counterparties and position/quote movement, not only an ADL status flag.
- `Closed` must correspond to complete gap coverage or an equivalent settled fact. `Exhausted` must correspond to explicit remaining-gap handoff or a modeled path-exhaustion fact.
- Do not treat liquidation order placement as equivalent to liquidation execution unless fill/execution facts and recovery amounts are modeled.

## Output Format

Return the review in this structure.

### Overall Verdict
- `verdict`: `通过` / `需要修改` / `需要重建`
- `score`: `<total>/100`
- `score_breakdown`:
  - `MI识别准确性`: `<n>/15`
  - `因果指针完整性`: `<n>/20`
  - `判定规则清晰度`: `<n>/20`
  - `守恒不变量`: `<n>/15`
  - `终局事实闭合`: `<n>/15`
  - `审计可追责性`: `<n>/10`
  - `改进建议可执行性`: `<n>/5`

### Code Evidence
Include this section when reviewing code.

- `files_read`: list key files with line numbers.
- `mi_sources`: for each MI, state whether it comes from an entity, changes type, replayable event, or comment/type declaration.
- `edge_evidence`: for each causal edge, label it as `explicit caused_by`, `ID trace only`, or `inferred`.
- `module_wiring`: note files that exist but are not exported or referenced by the group `mod.rs`.

### Strongest Causal Links
List links that already satisfy:
`predecessor MI -> predicate -> successor MI`

### Broken Or Missing Links
List missing, vague, reversed, or technically framed links. Name the missing predecessor, predicate, successor, or causal pointer.

### Non-MI Misclassifications
List items that should not be treated as MIs and explain why using the shared references.

### Missing Predicates Or Invariants
List missing business rules, guard conditions, balance checks, transfer symmetry, fee equations, hold/release invariants, or audit constraints.

### Concrete Rewrite Suggestions
Give direct edits: add facts, rename MIs, add `caused_by` fields, write predicates, add invariants, or introduce compensation facts.

### Improved MI Causal Chain Sketch
Provide a compact corrected chain in the form:

```text
MI_A --[predicate, caused_by: ...]--> MI_B --[predicate, caused_by: ...]--> MI_C
terminal_settled_fact: ...
invariants: ...
compensation/red_chong: ...
```

Keep the sketch business-level. Do not convert it into code, database schema, or adapter workflow unless the user explicitly asks.
