# Code Output Contract

Treat generated Rust code as the design artifact.

## Module Mapping

- `workflow -> module`
- `use case -> file`
- prefer business names over technical names
- avoid `service`, `manager`, `engine`, `handler` as the main business module name

## Use Case Skeleton

Each generated `CommandUseCase2` file should contain:
- domain error
- command
- `IssuedByParty`
- given state snapshot
- optional reply and mapper
- use case type
- `impl CommandUseCase2`
- tests

## Test Lock

Tests should express the design before full implementation:
- actor role
- invalid command-only input
- invalid state transition
- emitted business events
- orchestration happy path
- orchestration rejection path
