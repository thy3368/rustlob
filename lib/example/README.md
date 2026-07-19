# clean_arch_commandusecase2 示例

这个目录下放了 3 个 crate：

- `core`
- `inbound_adapter`
- `outbound_adapter`

目标是演示你要的两条链路：

- 调用链路：`inbound_adapter -> core -> outbound_adapter`
- 依赖链路：
  - `inbound_adapter -> core`
  - `outbound_adapter -> core`
  - `core` 不依赖任何 adapter

## Core

- `use_case`: `SpotOrderV2UseCaseFamilyV3`
- `entity`: `TradingAccount`, `MarketRules`, `SpotOrderV2GivenStateV3`

`core` 只表达业务动作、业务状态和业务错误，不依赖任何 inbound/outbound adapter。

## Adapter

### inbound

- `http.rs` 把 HTTP 风格请求 DTO 翻译成 `CommandEnvelope<SpotOrderV2CommandV3>`
- `http.rs` 也持有 `POST /orders` 的 route / handler / error mapping
- `cli.rs` 把 CLI 风格命令 DTO 翻译成 `CommandEnvelope<SpotOrderV2CommandV3>`
- `cli.rs` 也持有参数解析和 usage 文案
- 两个入口都调用 `CommandUseCaseExecutor2`
- 两个入口各自用 reply mapper 把领域事件翻译成对外响应

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

最外层机制只存在于 `example_inbound_adapter/examples/*`：

- 组装具体 outbound 实现
- 启动 HTTP runtime 和 worker loop
- 通过 broker message 驱动后两个 use case
- 提供 demo 级别的 `GET /snapshot`

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
  cargo examples / scripts
  in-memory store, std::sync::Mutex
```

### source dependency view

```text
inbound_adapter -> core
outbound_adapter -> core
core -> cmd_handler
```

### call flow view

```text
request
  -> inbound_adapter
  -> core.use_case
  -> outbound_adapter(use-case specific).load_state
  -> outbound_adapter(use-case specific).persist
  -> outbound_adapter(use-case specific).replay
  -> outbound_adapter(use-case specific).publish
  -> inbound_adapter.http_reply_mapper / cli_reply_mapper
  -> response

```

## Supported Inbound Styles

- HTTP: `handle_place_order_http(...)`
- HTTP: `handle_deposit_quote_http(...)`
- CLI: `run_place_order_cli(...)`
- CLI: `run_deposit_quote_cli(...)`

这些入口都只做输入翻译和输出翻译，不直接 new outbound。
HTTP 入口目前同时支持：

- `axum`
- `actix-web`

## Demo Binaries

可以直接运行 demo examples：

```bash
cargo run -p example_inbound_adapter --example http_demo
cargo run -p example_inbound_adapter --example http_actix_demo
cargo run -p example_inbound_adapter --example cli_demo -- trader-1 BTCUSDT 2 100
cargo run -p example_inbound_adapter --example cli_deposit_demo -- trader-1 200
```

### `http_demo`

- 启动真实 HTTP 服务，默认监听 `127.0.0.1:3001`
- 提供四个路由：
  - `POST /orders`
  - `POST /deposits/quote`
  - `POST /withdrawals/quote`
  - `GET /snapshot`
- `POST /orders` 先进入 `inbound_adapter.http`
- 后续撮合和结算通过 worker 消费 broker message，再进入 `inbound_adapter.event`
- 启动地址可用环境变量 `HTTP_DEMO_ADDR` 覆盖

### `http_actix_demo`

- 启动真实 HTTP 服务，默认监听 `127.0.0.1:3002`
- 同样暴露：
  - `POST /orders`
  - `POST /deposits/quote`
  - `POST /withdrawals/quote`
  - `GET /snapshot`
- `POST /orders` 仍然先走 `inbound_adapter.http`
- 启动地址可用环境变量 `HTTP_ACTIX_DEMO_ADDR` 覆盖

示例：

```bash
curl -sS -X POST http://127.0.0.1:3001/orders \
  -H 'content-type: application/json' \
  -d '{
    "trace_id":"trace-http-demo",
    "command_id":"cmd-http-demo",
    "trader_id":"trader-1",
    "symbol":"BTCUSDT",
    "qty":3,
    "price":100
  }'

curl -sS http://127.0.0.1:3001/snapshot
```

### `cli_demo`

- 模拟一个 CLI 命令入口
- 支持位置参数：`<trader_id> <symbol> <qty> <price>`
- 不传参数时使用默认值：`trader-1 BTCUSDT 2 100`
