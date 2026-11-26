# `lib/example` 架构说明

核心链路由以下角色组成：

- `entity` 与 `use_case`（core）
- `veldra_inbound_adapter` 与 `dex_inbound_adapter`（adapter.inbound）
- `outbound_adapter`（adapter.outbound）

目标是演示你要的两条链路：

- 调用链路：`veldra_inbound_adapter / dex_inbound_adapter -> core -> outbound_adapter`
- 依赖链路：
  - `veldra_inbound_adapter / dex_inbound_adapter -> core`
  - `outbound_adapter -> core`
  - `core` 不依赖任何 adapter

## Core

- `use_case`: `SpotOrderV2UseCaseFamilyV3`
- `entity`: `TradingAccount`, `MarketRules`, `SpotOrderV2GivenStateV3`

`core` 只表达业务动作、业务状态和业务错误，不依赖任何 inbound/outbound adapter。

## Adapter

### inbound

- `veldra_inbound_adapter` 接收交易命令、事件和定时任务，并完成入口编排
- `dex_inbound_adapter` 接收 DEX 命令、事件和定时触发
- 入站 adapter 负责把外围输入翻译为 core 可接受的 command/query，不承载核心业务规则

### outbound

- 实现 `CommandUseCaseOutbound`
- 负责 `load_state / persist / replay / publish`
- 支持两种实现：
  - 内存 backend: `InMemoryStore`
  - MySQL backend: `MySqlStore`
- 每个 use case 都有显式 outbound：
  - `InMemoryPlaceOrderOutbound`
  - `InMemoryDepositQuoteOutbound`
  - `MySqlPlaceOrderOutbound`
  - `MySqlDepositQuoteOutbound`
- 选择哪一种实现，不在 `core` 决定，而在最外层 demo/example 入口里装配

## Infra

这里不再保留 `composition_root` crate。

最外层机制由现存 inbound adapter 的启动入口和 bootstrap 模块负责：

- `veldra_inbound_adapter/src/main.rs`
- `veldra_inbound_adapter/src/bin/*`
- `veldra_inbound_adapter/src/bootstrap/*`

本例里的底层技术细节仍然藏在 `outbound_adapter` 内部：

- `Mutex`
- `HashMap`
- 内存事件存储
- `mysql` driver 和表结构

## Architecture Views

### role view

```text
core
  use_case, entity

adapter
  inbound_adapter
  outbound_adapter

infra
  inbound bootstrap / runtime
  in-memory store, std::sync::Mutex
```

### source dependency view

```text
veldra_inbound_adapter -> core
dex_inbound_adapter -> core
outbound_adapter -> core
core -> common_entity
```

### call flow view

```text
request / event / timer
  -> veldra_inbound_adapter / dex_inbound_adapter
  -> core.use_case
  -> outbound_adapter(use-case specific).load_state
  -> outbound_adapter(use-case specific).persist
  -> outbound_adapter(use-case specific).replay
  -> outbound_adapter(use-case specific).publish
  -> response

```

