# Matrix Completion

The happy-path matrix must be reconstructed from domain semantics, not branch names alone.

## Goal

Build a complete enough happy-path business matrix to answer:
- which supported successful scenarios exist
- which ones are already covered
- which exact supported scenario is missing next

## Required Minimum Dimensions

Every matrix must include these observation dimensions where applicable:
- actor / intent
- liquidity relation or matching condition
- finish state
- event shape
- status transition

These are minimum dimensions, not maximum dimensions. Add use-case-specific axes when needed, such as:
- order side
- execution mode
- time-in-force
- fill pattern
- multi-party interaction shape

## How To Reconstruct The Matrix

1. Read the command and identify who is acting and what business intent is being issued.
2. Read the state and entity methods to identify the supported successful relationships or matching conditions.
3. Read the `Changes` type to identify observable business outcomes.
4. Read `to_replayable_events()` to identify replayable-event contracts that matter.
5. Read existing happy tests to see which cells are already protected.
6. Name the remaining supported happy-path cells in business terms.

## What Not To Do

Do not derive the matrix only from:
- `if` / `match` branch names
- helper-function names
- line-by-line control flow

Those are implementation clues, not the business matrix itself.

## Mapping Rule

Every proposed new test must map to one concrete matrix cell.

State that mapping explicitly:
- selected matrix cell
- why it is currently missing
- which business rule the new test protects

If a proposed test cannot be tied to one concrete cell, the matrix is still too vague.
