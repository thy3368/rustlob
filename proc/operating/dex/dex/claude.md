# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Scope

This CLAUDE.md applies only to the `proc/operating/dex/dex` crate.

The crate is currently a small Rust workspace member focused on modeling trading-command ingestion and batch execution, with Hyperliquid-style exchange flow as research context. It is not yet a full matching engine or complete DEX implementation.

## Common commands

From the workspace root (`/Users/hongyaotang/src/rustlob`):

```bash
cargo check -p dex
cargo test -p dex
cargo test -p dex <test_name>
```

From this crate directory (`proc/operating/dex/dex`):

```bash
cargo check
cargo test
cargo test <test_name>
```

## Architecture overview

### Crate role

`dex` is a thin crate inside the workspace. Right now it mainly defines a command-processing skeleton rather than a full exchange runtime.

`src/lib.rs` exposes two top-level modules:
- `cmd_handler`
- `types`

### Command model

`src/cmd_handler/trading_command.rs` defines the core command vocabulary:
- `OrderSide`
- `PlaceOrderCmd`
- `CancelOrderCmd`
- `AmendOrderCmd`
- `TradingCommand`
- `TradingCommandEnvelope`

The important boundary is `TradingCommandEnvelope`: each command carries `command_id`, `trader_id`, `nonce`, `timestamp_ns`, and the typed command payload. That means the crate is modeling trading operations as explicit, timestamped command objects instead of direct state mutation.

### Submit path

`src/cmd_handler/submit_trading_command_handler.rs` is the ingestion side.

It implements `base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate}` and currently does three things:
- validates and maps a command into a `ChangeSet`
- emits lightweight changelog entries (`SubmitTradingCommandLog`)
- appends accepted commands into an in-memory pending queue (`Mutex<VecDeque<TradingCommandEnvelope>>`)

This file is the main entry point when adding logic around command admission, pre-checks, replay/persistence hooks, or queueing behavior.

### Batch execution path

`src/cmd_handler/execute_trading_batch_handler.rs` is the batch-consumption side.

Today it is intentionally simple: it accepts a batch of `TradingCommandEnvelope` values and returns `BatchExecutionResult`, currently just counting how many place/cancel/amend commands were seen. Conceptually, this is the execution stage that would later evolve into matching, cancellation, amendment, sequencing, or state-application logic.

### Types module

`src/types/mod.rs` is currently effectively empty, which suggests domain state and shared type definitions have not been expanded yet. If the crate grows, this is the likely place for reusable exchange/domain types rather than putting everything into handlers.

## Hyperliquid research context

This crate sits beside research material that helps explain the intended direction:

- `docs/hyperliquid/README.md` describes Hyperliquid at a high level: HyperCore handles fully onchain spot/perp order books, and the system treats order / cancel / trade / liquidation as explicit state transitions.
- `study/hyperliquid_analyzer/README.md` describes a separate tool for fetching and analyzing Hyperliquid block data.

The useful takeaway for this crate is architectural, not dependency-related: the current `TradingCommandEnvelope -> submit handler -> pending queue -> batch executor` flow matches a command-driven exchange pipeline that could later map well to onchain or sequenced exchange execution.

Do not assume Hyperliquid capabilities already exist here. The analyzer and docs are reference material for reverse engineering and design direction, not runtime dependencies of this crate.

## Editing guidance

When extending this crate, preserve the current separation of responsibilities:

- Add or evolve trading operations in `TradingCommand` and the related command structs first.
- Keep command submission concerns in `SubmitTradingCommandHandler`.
- Keep batch processing / execution concerns in `ExecuteTradingBatchHandler`.
- Avoid collapsing ingestion and execution into one handler unless the crate is explicitly being redesigned.
- Model each business use case as a clear command or query.
- Validate use cases primarily by testing the corresponding command or query.

Because the crate is early-stage, prefer documenting only behavior that is visible in code today. Do not describe matching, persistence, consensus, settlement, or Hyperliquid protocol support as implemented unless the code actually exists in this crate.

## 逆向回构参考文档

如果当前任务是在做 Hyperliquid 风格的逆向回构，请优先阅读：

- `docs/hyperliquid_reverse_usecases.md`：按用例域拆分的交易/行情分析
- `docs/hyperliquid_file_design.md`：建议的文件级演进结构

使用方式：
- 想理解“应该支持哪些业务动作”时，先看用例分析文档
- 想决定“新代码应该落在哪些模块”时，先看文件级设计文档

注意：这些文档描述的是当前 crate 的目标演进方向，不代表这些能力已经在代码中实现。
