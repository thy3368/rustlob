---
name: kani-use-case
description: Write `kani` proofs for RustLOB command-style use cases. Use when Codex needs to add or review `#[kani::proof]` tests for `CommandUseCase2`, choose good symbolic inputs for command or state, prove business invariants without weak tautologies, or distinguish strong and weak `kani` use-case proofs with shared good and bad examples.
---

# Kani Use Case

## Overview

Use `kani` to prove business invariants of RustLOB use cases, not just arithmetic facts. Good proofs constrain symbolic inputs with meaningful assumptions, target `pre_check_command`, `validate_against_state`, or `compute_replayable_events`, and assert business properties that would matter if broken.

Start from these source files:
- Contract: `lib/common/cmd_handler/src/use_case_def2.rs`
- Shared `kani` examples:
  - `lib/core/l1/tests/use_case_kani_examples/good.rs`
  - `lib/core/l1/tests/use_case_kani_examples/bad.rs`

Read the shared good and bad examples before writing a new proof.

## Workflow

1. Choose the business invariant before writing symbolic inputs.
- Prefer invariants such as:
  - malformed commands are rejected by `pre_check_command`
  - invalid state transitions are rejected by `validate_against_state`
  - successful execution emits replayable events consistent with command and state
  - emitted events preserve actor ownership, amount, count, or status invariants
- Avoid proofs that only restate `checked_add`, copied fixture values, or trivial monotonicity.

2. Keep symbolic data small and domain-shaped.
- Prefer small enums, booleans, integers, and fixed-size arrays over unconstrained `String` or `Vec`.
- If the production command is string-heavy, introduce a tiny local proof model instead of feeding `kani` unbounded text.
- Derive `Arbitrary` for compact helper structs used only in the proof.

3. Use `kani::assume` to model the slice of state space you want to prove over.
- Use assumptions to bound irrelevant input space.
- Do not use `if let` or `if ok { assert! }` to silently weaken the proof unless you are explicitly proving a conditional property.
- If the property is conditional, state that condition clearly in the proof name or comments.

4. Prove the right level.
- For command-shape rules, call `pre_check_command` directly.
- For state-dependent rules, call `validate_against_state` directly.
- For output invariants, call `compute_replayable_events` after proving command and state are admissible.
- Prefer direct use-case proofs before trying to symbolically prove full executor orchestration.

5. Make the assertion business-relevant.
- Assert identities, counts, and emitted field values that matter to the use case.
- `trace_id` is observability metadata, not the business identity.
- `party_id` belongs to the business command and is valid proof material.
- `command_id` matters only when the property is about business deduplication or retries.

## Good vs Bad

- Good example characteristics:
  - proof states a business invariant in its name
  - assumptions bound symbolic inputs to the relevant domain slice
  - assertions check command rejection or emitted event consistency
  - proof targets `CommandUseCase2` methods directly
- Bad example characteristics:
  - proof only checks arithmetic or copied state
  - role is technical, not business-facing
  - trace metadata is confused with business identity
  - proof branches away from the hard case and proves very little

## Output Checklist

- The proof name states the business invariant.
- Symbolic inputs are small enough for `kani` to explore.
- Assumptions narrow irrelevant space but do not hide the bug.
- Assertions check business behavior, not tautologies.
- If the proof is only conditional, the condition is explicit in the proof name or comments.
