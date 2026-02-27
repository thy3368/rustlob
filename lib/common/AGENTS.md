# lib/common/

Common utilities library with 20+ sub-crates following STR Clean Architecture.

##UCTURE

```
lib/common/
├── lob_repo/        # Order matching engine (core)
├── fixed_point/     # Decimal arithmetic
├── decimal/         # Decimal utilities
├── algo/            # Algorithms
├── cqrs/            # Command Query Responsibility Segregation
├── id_generator/    # ID generation
├── db_repo/         # Database repository
└── sbe/             # Simple Binary Encoding
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| LOB (limit order book) | `lib/common/lob_repo/` |
| Decimal math | `lib/common/fixed_point/` |
| Database access | `lib/common/db_repo/` |
| Encoding | `lib/common/sbe/` |

## CONVENTIONS

- Same as root: rustfmt 2024, LTO fat, native CPU
- Each sub-crate has own `Cargo.toml`
- Tests in `tests/` directories

## ANTI-PATTERNS

- Same as root

## NOTES

- 19 lib.rs files (19 crates)
- 11 test directories
