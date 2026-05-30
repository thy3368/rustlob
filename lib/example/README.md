# clean_arch_commandusecase2 示例

这个目录下放了 4 个 crate：

- `core`
- `inbound_adapter`
- `outbound_adapter`
- `app/composition_root`

目标是演示你要的两条链路：

- 调用链路：`inbound_adapter -> core -> outbound_adapter`
- 依赖链路：
  - `inbound_adapter -> core`
  - `outbound_adapter -> core`
  - `composition_root -> inbound_adapter`
  - `composition_root -> outbound_adapter`
  - `core` 不依赖任何 adapter

## Core

- `use_case`: `PlaceOrderUseCase`
- `entity`: `TradingAccount`, `MarketRules`, `PlaceOrderState`

`core` 只表达业务动作、业务状态和业务错误，不依赖任何 inbound/outbound adapter。

## Adapter

### inbound

- `http.rs` 把 HTTP 风格请求 DTO 翻译成 `CommandEnvelope<PlaceOrderCmd>`
- `http.rs` 也持有 `POST /orders` 的 route / handler / error mapping
- `cli.rs` 把 CLI 风格命令 DTO 翻译成 `CommandEnvelope<PlaceOrderCmd>`
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
- 选择哪一种实现，不在 `core` 决定，而在 `composition_root` 注入

## Infra

`app/composition_root` 负责：

- 组装具体 outbound 实现
- 把 outbound 注入 inbound adapter
- 启动 runtime 和 listener
- 对外暴露 HTTP 和 CLI 两种运行入口

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
  composition_root
  in-memory store, std::sync::Mutex
```

### source dependency view

```text
inbound_adapter -> core
outbound_adapter -> core
composition_root -> inbound_adapter
composition_root -> outbound_adapter
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

这两个入口都只做输入翻译和输出翻译，不直接 new outbound。
HTTP 入口目前同时支持：

- `axum`
- `actix-web`

## Demo Binaries

可以直接运行两个 demo：

```bash
cargo run -p example_composition_root --bin http_demo
cargo run -p example_composition_root --bin http_actix_demo
cargo run -p example_composition_root --bin cli_demo -- trader-1 BTCUSDT 2 100
cargo run -p example_composition_root --bin cli_deposit_demo -- trader-1 200
```

如果要从组合根切到 MySQL outbound，可以直接在代码里改成：

```rust
let app =
    ExampleApplication::new_mysql_seeded("mysql://root:password@127.0.0.1:3306/example_clean_arch")?;
```

### `http_demo`

- 启动真实 HTTP 服务，默认监听 `127.0.0.1:3001`
- 提供两个路由：
  - `POST /orders`
  - `POST /deposits/quote`
  - `GET /snapshot`
- 其中 `POST /orders` 在 `inbound_adapter`
- `http_demo` 自己只保留启动和 demo 专用 `GET /snapshot`
- 启动地址可用环境变量 `HTTP_DEMO_ADDR` 覆盖

### `http_actix_demo`

- 启动真实 HTTP 服务，默认监听 `127.0.0.1:3002`
- 同样暴露：
  - `POST /orders`
  - `POST /deposits/quote`
  - `GET /snapshot`
- `POST /orders` 仍然走 `inbound_adapter`
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
