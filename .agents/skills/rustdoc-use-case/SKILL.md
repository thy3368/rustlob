---
name: rustdoc-use-case
description: Write Rustdoc for RustLOB `CommandUseCase3` use cases. Use when Codex needs to document command structs, business error enums, or use case types in files such as `*/use_case/**/*.rs`, add field-level docs, explain `pre_check_command` vs `validate_against_state`, or add minimal doctest examples without turning Rustdoc into the main test harness.
---

# Rustdoc Use Case

## Overview

Use this skill when a RustLOB use case needs caller-facing Rustdoc.

Target the public surface that a reader or adapter author needs to understand:
- the command type
- the business error type
- the use case type
- sometimes key public fields

Keep Rustdoc short, business-oriented, and example-driven.

Start from these files:
- Contract: `lib/common/cmd_handler/src/use_case_def2/mod.rs`
- Example with Rustdoc: `lib/example/core/src/use_case/trading/spot/place_order.rs`

## What To Document

Document these items first:

1. `*Cmd`
- What business action the command represents
- What the important fields mean
- Where validation happens:
  - command-only checks in `pre_check_command()`
  - state-dependent checks in `validate_against_state()`

2. `*Error`
- What kind of business rejection this enum models
- Short per-variant docs for when each error is returned
- If the error is public, ensure `Display` is useful; default to `thiserror::Error`

3. `*UseCase`
- One short paragraph describing what events it derives
- Mention what it does **not** do:
  - no DB access
  - no publish/persist
  - no HTTP reply shaping

Field docs are useful when a field name is too generic or has business semantics that are not obvious from the type alone.

## Rustdoc Pattern

Use this structure by default:

```rust
/// One-sentence business purpose.
///
/// Optional second paragraph for boundary clarification.
///
/// # Examples
///
/// ```
/// // minimal construction or usage example
/// ```
pub struct SomeCmd { ... }
```

For errors:

```rust
/// Business errors that can reject this use case.
///
/// # Examples
///
/// ```
/// assert_eq!(SomeError::Foo.to_string(), "...");
/// ```
#[derive(Debug, thiserror::Error)]
pub enum SomeError {
    /// Returned when ...
    #[error("...")]
    Foo,
}
```

For the use case type:

```rust
/// Use case that validates X and derives replayable Y events.
///
/// It is deterministic for the same command and loaded state.
#[derive(Debug, Clone, Copy, Default)]
pub struct SomeUseCase;
```

## Doctest Rules

Use doctest as a lightweight harness, not the main harness.

Good doctest targets:
- construct a command
- show one expected error string
- show the public API shape

Do not push complex business flow into doctest when normal `#[test]` is clearer.

Keep doctests:
- small
- compileable
- independent from large fixture setup

Prefer `use example_core::TypeName;` style imports when the type is re-exported publicly.

## What To Avoid

Avoid these Rustdoc mistakes:

- Explaining framework trivia instead of business meaning
- Repeating the field name in prose without adding information
- Copying test-sized setup into doctest
- Documenting private implementation detail that may churn often
- Using Rustdoc as a substitute for real unit tests
- Leaving placeholder comments like `todo rust doc`

## Project Guidance

For RustLOB use cases:

- `CommandUseCase3` is the business boundary
- `pre_check_command()` is for cheap command-only validation
- `validate_against_state()` is for loaded-state business checks
- `compute_output_and_events()` is the business derivation core

Reflect that split in the docs when it helps the reader.

If the file already uses `thiserror`, keep error docs aligned with `#[error(...)]` messages.

## Output Checklist

Before finishing:

- `*Cmd` has a top-level Rustdoc block
- important public fields have concise field docs
- `*Error` has top-level docs plus per-variant docs where useful
- `*UseCase` has a short purpose statement
- doctest examples are minimal and likely to stay stable
- unit tests remain the main business harness
