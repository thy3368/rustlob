---
name: workflow-use-case-modeling
description: Generate RustLOB workflow modules and `CommandUseCase3` use case skeletons from business text by following `lib/example` exactly. Use when Codex should map business text to `example_core` / `example_inbound_adapter` / `example_outbound_adapter` / `example_composition_root`, generate code-first skeletons, or review whether a design matches the example architecture.
---

# Workflow Use Case Modeling

按 `lib/example` 写。不要发明新分层。

## Canonical Shape

只对齐这 4 个 crate：

- `lib/example/core` -> `example_core`
- `lib/example/inbound_adapter` -> `example_inbound_adapter`
- `lib/example/outbound_adapter` -> `example_outbound_adapter`
- `lib/example/app/composition_root` -> `example_composition_root`

当前 `lib/example` 的业务面：

- trading: `PlaceOrder`
- funding: `DepositQuote`
- funding: `WithdrawQuote`

当前 `lib/example` 的入口面：

- HTTP
- CLI

当前 `lib/example` 的出站面：

- in-memory
- MySQL

## Hard Rules

### `example_core`

只放：

- entity
- `CommandUseCase3`
- command
- `GivenState`
- `Output`
- domain error
- business output 与 replayable event 计算

不要放：

- HTTP
- CLI
- DB
- MySQL
- runtime
- wiring

### `example_inbound_adapter`

只放：

- HTTP/CLI request dto
- CLI args parse
- dto -> `CommandEnvelope<Cmd>`
- executor 调用
- reply / error mapping
- route / handler

不要放：

- 业务规则
- `load_state`
- DB / store 访问
- event persist / replay / publish

### `example_outbound_adapter`

只放：

- `CommandUseCaseOutbound`
- `load_state`
- `persist`
- `replay`
- `publish`
- in-memory impl
- MySQL impl

不要放：

- HTTP / CLI
- route / handler
- 核心业务判断

### `example_composition_root`

只放：

- outbound 选型
- inbound/outbound 注入
- app wiring
- demo bin

不要放：

- 业务规则
- 输入协议细节
- 事件推导

## Dependency Rules

允许：

- `example_inbound_adapter -> example_core`
- `example_outbound_adapter -> example_core`
- `example_composition_root -> example_core`
- `example_composition_root -> example_inbound_adapter`
- `example_composition_root -> example_outbound_adapter`
- `core/adapter -> cmd_handler`

禁止：

- `example_core -> adapter/composition_root`
- `example_inbound_adapter -> example_outbound_adapter`
- `example_outbound_adapter -> example_inbound_adapter`

## Call Flow

只允许：

`external -> inbound_adapter -> core.use_case -> outbound_adapter -> infra`

组合根只接线，不判业务。

## How To Model

先做最小建模：

- `Role` -> `role()`
- `Moment-Interval` -> 1 个 `CommandUseCase3`
- `Party/Place/Thing` -> `GivenState` / entity / `UseCaseOutput.events`
- `Description` -> command / error / output / event 命名

然后强制分配到 4 个 crate。

如果说不清每段代码属于哪个 crate，就不要写代码。

## What To Generate

默认输出顺序：

1. 四色映射
2. crate 分配
3. 目录树
4. core skeleton
5. inbound skeleton
6. outbound skeleton
7. composition root wiring
8. tests

## Core File Shape

`example_core` 里的 use case 文件按这个顺序：

1. `Error`
2. `Cmd`
3. `IssuedByParty impl`
4. `GivenState`
5. `Output`
6. optional `Reply`
7. optional `ReplyMapper3`
8. `UseCase`
9. `impl CommandUseCase3`
10. tests

## Minimum Tests

至少生成：

- `role()`
- `pre_check_command()`
- `validate_against_state()`
- `compute_output_and_events()`
- `ReplyMapper3`
- `CommandUseCaseExecutor3::execute()`
- `CommandUseCaseExecutor3::execute_and_map_reply()`

## Exact Repo Patterns

直接参考这些真实文件：

- core
  - `lib/example/core/src/use_case/trading/spot/place_order/immediate_order.rs`
  - `lib/example/core/src/use_case/trading/spot/match_order.rs`
  - `lib/example/core/src/use_case/trading/spot/settle_trade.rs`
  - `lib/example/core/src/use_case/trading/spot/execute_immediate_order_pipeline.rs`
- inbound
  - `lib/example/inbound_adapter/src/common.rs`
- outbound
  - `lib/example/outbound_adapter/src/trading/place_order_in_memory.rs`
  - `lib/example/outbound_adapter/src/trading/place_order_mysql.rs`
  - `lib/example/outbound_adapter/src/funding/deposit_quote_in_memory.rs`
  - `lib/example/outbound_adapter/src/funding/deposit_quote_mysql.rs`
  - `lib/example/outbound_adapter/src/funding/withdraw_quote_in_memory.rs`
  - `lib/example/outbound_adapter/src/funding/withdraw_quote_mysql.rs`
- composition root
  - `lib/example/app/composition_root/src/lib.rs`
  - `lib/example/app/composition_root/src/bin/http_demo.rs`
  - `lib/example/app/composition_root/src/bin/http_actix_demo.rs`
  - `lib/example/app/composition_root/src/bin/cli_demo.rs`
  - `lib/example/app/composition_root/src/bin/cli_deposit_demo.rs`
  - `lib/example/app/composition_root/src/bin/cli_withdraw_demo.rs`

## Concrete Mapping

按 `PlaceImmediateOrder` 理解：

- `example_core`
  - `PlaceImmediateOrderCmd`
  - `PlaceImmediateOrderState`
  - `PlaceImmediateOrderOutput`
  - `PlaceImmediateOrderUseCase`
- `example_inbound_adapter`
  - HTTP payload / CLI args
  - `CommandEnvelope<PlaceImmediateOrderCmd>`
  - HTTP/CLI reply mapping
- `example_outbound_adapter`
  - `InMemoryPlaceOrderOutbound`
  - `MySqlPlaceOrderOutbound`
  - `load_state / persist / replay / publish`
- `example_composition_root`
  - `ExampleApplication`
  - outbound 注入
  - HTTP / CLI demo

`DepositQuote` 和 `WithdrawQuote` 同理，不要新造结构。

## Fail Fast

以下直接判失败：

- `core` 出现 HTTP / CLI / MySQL / runtime
- `inbound_adapter` 直接访问 DB / store
- `outbound_adapter` 做业务分支
- `composition_root` 做业务判断

## Ask Only If Needed

只在这些问题不清楚时提问：

- 这是 1 个还是多个 `Moment-Interval`
- 它属于 trading 还是 funding，或需要新 workflow
- 是否需要 HTTP、CLI，还是两者都要
- 是否需要 in-memory、MySQL，还是两者都要

其余默认按 `lib/example` 现状生成。

## References

- `../../../lib/example/README.md`
- `../../../lib/common/cmd_handler/src/command_use_case_def2/use_case.rs`
