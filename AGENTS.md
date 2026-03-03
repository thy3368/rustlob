# PROJECT KNOWLEDGE BASE

**Generated:** 2026-02-27
**Commit:** 4cfca3d
**Branch:** main

## OVERVIEW

RustLOB — Ultra-low latency cryptocurrency exchange written in Rust. Monorepo workspace with 31 crates following Clean Architecture + DDD principles.

## STRUCTURE

```
rustlob/
├── app/           # Binary services (HTTP, WebSocket, XDP, Gateway)
├── lib/            # Domain libraries + utilities (24 crates)
├── proc/           # Business process orchestration (5 crates)
├── study/          # Research experiments (HotStuff, ZK-SNARKs, Web3)
├── design/         # Architecture docs + API references
└── Cargo.toml      # Workspace root (31 members)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Order matching engine | `lib/common/lob_repo/` | Core LOB implementation |
| HTTP API | `app/axum_server/` | REST API server |
| WebSocket | `app/websocket_sockudo/` | Real-time trading |
| XDP acceleration | `app/xdp_libbpf/` | Network performance |
| Exchange logic | `proc/operating/exchange/` | Spot, derivatives, wallet |
| Decimal math | `lib/common/fixed_point/` | Precision arithmetic |

## CODE MAP

| Symbol | Type | Location | Refs |
|--------|------|----------|------|
| lob_repo | lib | lib/common/lob_repo | Core |
| exchange | lib | proc/operating/exchange | Business |
| axum_server | bin | app/axum_server | Entry |
| websocket_sockudo | bin | app/websocket_sockudo | Entry |

## CONVENTIONS (THIS PROJECT)

- **Layout**: No `src/` root — uses `app/` + `lib/` at root (Cargo workspace)
- **Formatting**: `rustfmt.toml` — style edition 2024, imports granularity=module
- **Build**: `lto = "fat"`, `codegen-units = 1`, `target-cpu = "native"`
- **Naming**: snake_case for modules, PascalCase for types
- **Error handling**: `Result<T, E>` with `?` operator, no `unwrap()` in prod
- **Testing**: Inline `#[test]` + `tests/` directories per crate

## ANTI-PATTERNS (THIS PROJECT)

- ❌ `unwrap()` / `expect()` in production code
- ❌ `unsafe` blocks without safety comments
- ❌ `as any` type casts
- ❌ Empty error handling (`catch(e) {}`)
- ❌ Global mutable state
- ❌ TODO comments left in production (74 TODOs exist, track in issue)

## UNIQUE STYLES

- **Low-latency focus**: Custom allocators, SIMD hints, lock-free structures
- **Embedded XDP**: eBPF programs for network acceleration
- **DDD structure**: Domain/Application/Infrastructure layers via crate boundaries
- **External API docs**: Binance/OKX integration specs in `design/`

## COMMANDS

```bash
# Build
cargo build --release

# Test (48 tests across workspace)
cargo test --workspace

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt --all
```

## NOTES

- **No CI/CD**: Root `.github/workflows/` missing — manual builds required
- **TODOs**: 74 incomplete implementations tracked in source
- **Depth**: 8-level directory nesting max (design/docs are external API refs)
- **Workspace resolver**: v2 (Rust 2021 edition)
