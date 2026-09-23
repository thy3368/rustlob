# RustLOB

RustLOB 是一个用 Rust 编写的交易系统与 Clean Architecture 实践仓库。仓库同时包含领域实体、业务用例、执行器、入站与出站适配器，以及若干独立的基础设施和研究项目。

当前 workspace 以源码和 Cargo metadata 为准：

- 37 个 workspace member
- 29 个 default member
- 当前最完整、最主要的交易业务实现位于 [`lib/example/`](lib/example/)

`example` 是历史目录名称。它现在承载了仓库的主要核心业务链路，不应理解为只有玩具代码的临时目录。

## 项目定位

RustLOB 当前优先用于：

- 用实体表达订单、账户、余额、冻结、成交和资金变更等领域规则；
- 用 CommandUseCase4 组织现货、资金、衍生品和风险相关业务动作；
- 用执行器和 outbound port 连接状态加载、事件持久化、回放、发布和 broker；
- 用 HTTP、CLI、事件、定时任务和 DEX/交易入口验证不同接入方式；
- 通过测试、BDD 场景和 [`lib/example/gate.md`](lib/example/gate.md) 持续校验核心行为。

这是一个持续演进中的工程仓库。README 描述当前实现和可运行入口，不把尚未实现或尚未验证的生产能力当作系统承诺。

## 当前核心实现

`lib/example/` 采用 `core / adapter / infra` 的 Clean Architecture 视角组织代码：

### Core

- [`lib/example/entity/`](lib/example/entity/)：订单、账户、余额、冻结/预约、成交、市场规则，以及实体自身的业务行为和不变量。
- [`lib/example/use_case/`](lib/example/use_case/)：现货下单、撮合、撤单、改单、资金充提报价，以及永续合约资金、风险和清算相关 CommandUseCase4 用例。

用例的基本形状是：

```text
Command + GivenState -> Changes -> ReplayableEvents
```

`entity` 与 `use_case` 不依赖 HTTP、WebSocket、数据库或具体 SDK。用例负责基于已加载状态推导业务变化，事件的持久化、回放和发布由外围适配器负责。

### Adapter

- [`lib/example/use_case_executor/`](lib/example/use_case_executor/)：将用例、状态访问和 outbound 调用组合成可复用的执行流程，当前重点覆盖现货订单及区块执行。
- [`lib/example/inbound_adapter/`](lib/example/inbound_adapter/)：HTTP、CLI 入站适配，以及 API 描述生成。当前提供 Axum 和 Actix Web demo。
- [`lib/example/outbound_adapter/`](lib/example/outbound_adapter/)：内存和 MySQL 状态访问、事件持久化与回放、事件发布、broker，以及现货交易 outbound。
- [`lib/example/veldra_inbound_adapter/`](lib/example/veldra_inbound_adapter/)：面向交易命令、事件和定时任务的入口编排，包含交易和信息 API 的接入路径。
- [`lib/example/dex_inbound_adapter/`](lib/example/dex_inbound_adapter/)：DEX 请求的入站示例。

### Infra 与演示

- [`lib/example/hotstuff_use_case_demo/`](lib/example/hotstuff_use_case_demo/)：HotStuff 网络/节点演示与交易用例结合的实验入口。
- `inbound_adapter` 和 `outbound_adapter` 内部使用的 Axum、Actix Web、Tokio、MySQL、broker 等具体技术属于外围机制，由最外层入口装配。

## 依赖方向

以使用执行器的交易入口为代表，当前架构可以概括为：

```text
inbound_adapter ─────────┐
veldra_inbound_adapter ──┼──> use_case_executor ───> outbound_adapter
dex_inbound_adapter ─────┘              │
                                       v
                              entity + use_case
```

需要注意：

- `entity` 与 `use_case` 不依赖 inbound adapter；
- `outbound_adapter` 实现 core 所需的状态、事件和外部调用端口；
- 具体 demo 入口负责装配实现；
- HTTP/CLI 示例为了保持简单，部分路径会直接绑定特定的 use case 和 outbound；这不改变 core 与 adapter 的边界；
- outbound 不把数据库或 broker 规则反向塞入实体和用例。

完整的目录级说明见 [`lib/example/README.md`](lib/example/README.md)。

## 可运行示例

以下命令均从仓库根目录执行。

### HTTP

```bash
cargo run -p example_inbound_adapter --example http_demo
```

默认监听 `127.0.0.1:3001`，可通过 `HTTP_DEMO_ADDR` 覆盖。提供：

- `POST /orders`
- `POST /deposits/quote`
- `POST /withdrawals/quote`
- `GET /snapshot`

Actix Web 版本：

```bash
cargo run -p example_inbound_adapter --example http_actix_demo
```

默认监听 `127.0.0.1:3002`，可通过 `HTTP_ACTIX_DEMO_ADDR` 覆盖。

### CLI

现货下单：

```bash
cargo run -p example_inbound_adapter --example cli_demo -- trader-1 BTCUSDT 2 100
```

入金报价：

```bash
cargo run -p example_inbound_adapter --example cli_deposit_demo -- trader-1 200
```

出金报价：

```bash
cargo run -p example_inbound_adapter --example cli_withdraw_demo -- trader-1 200
```

### API 描述

`example_inbound_adapter` 还提供 API 描述生成示例：

```bash
cargo run -p example_inbound_adapter --example generate_api_docs
```

生成或维护的描述文件位于：

- [`lib/example/api-manifest.json`](lib/example/api-manifest.json)
- [`lib/example/cli-schema.json`](lib/example/cli-schema.json)
- [`lib/example/openapi.json`](lib/example/openapi.json)

## 测试与质量门禁

默认 workspace 测试：

```bash
cargo test
```

核心实体与用例测试：

```bash
cargo test -p example_core_entity
cargo test -p example_core_use_case
```

入站和事件入口测试：

```bash
cargo test -p example_inbound_adapter
cargo test -p example_veldra_inbound_adapter
```

核心架构 lint：

```bash
cargo clippy -p example_core_entity -p example_core_use_case -- -D warnings
```

核心质量门禁和覆盖率命令见 [`lib/example/gate.md`](lib/example/gate.md)。`cargo test --workspace` 会包含更多平台相关或实验性 crate；例如 `app/xdp_libbpf` 依赖 Linux 内核头和 `libelf`，在 macOS 上不应强行构建。

## 目录导航

```text
rustlob/
├── lib/
│   ├── example/                 # 当前主要核心业务实现
│   │   ├── entity/              # 领域实体与规则
│   │   ├── use_case/            # CommandUseCase4 业务用例
│   │   ├── use_case_executor/   # 用例执行编排
│   │   ├── inbound_adapter/     # HTTP、CLI 和 API 描述入口
│   │   ├── outbound_adapter/    # 状态、事件、broker、MySQL 等 outbound
│   │   ├── veldra_inbound_adapter/
│   │   ├── dex_inbound_adapter/
│   │   └── hotstuff_use_case_demo/
│   ├── common/                  # 基础类型、命令处理、LOB、仓储和 HotStuff 通用能力
│   ├── core/                    # L1 与底层核心能力，当前作为基础/实验模块
│   └── veldra/                  # 独立 bounded context
├── operating/
│   └── cex/                     # 并行或历史 CEX 业务实现与推送模块
├── inbound_adapter/             # 独立 HTTP / WebSocket 服务入口
├── app/                         # client、网关及网络相关应用
├── study/                       # HotStuff、Web3、Hyperliquid 等研究项目
└── design/                      # 架构、业务流程和外部协议资料
```

### 其他模块的定位

- [`lib/common/`](lib/common/)：基础类型、命令处理、实体通用能力、LOB、数据库仓储、ID 生成和 HotStuff 通用组件。
- [`lib/core/`](lib/core/)：L1 和更底层的核心能力，当前不作为主要交易业务入口。
- [`operating/cex/`](operating/cex/)：现有 CEX 交易、衍生品、钱包、行情和推送实现，与 `lib/example` 并行演进或保留历史业务路径。
- [`inbound_adapter/`](inbound_adapter/)：独立 HTTP/WebSocket 接入服务。
- [`app/`](app/)：客户端、Pingora 网关和其他运行时/网络应用；[`app/xdp_libbpf/`](app/xdp_libbpf/) 需要 Linux 环境。
- [`study/`](study/)：HotStuff、Web3、Hyperliquid 分析等研究和实验项目。
- [`design/`](design/)：架构文档、业务流程、协议资料和部署研究。

## 路线图

路线图表示演进方向，不表示对应能力已经完成：

1. 完善 `lib/example` 中现货订单从下单、冻结、撮合、成交到撤单/改单的生命周期。
2. 完善资金充提、事件链、replay、snapshot、broker 和持久化边界。
3. 继续扩展衍生品、账户、清算、风险和市场数据相关用例。
4. 为稳定的核心能力补齐更明确的 production/core crate 命名和装配边界。
5. 在业务行为和边界稳定后，再评估更复杂的共识、网络和部署方案。

当前 README 仅记录可从源码和测试验证的能力，不对分布式共识、容灾指标、极低时延 SLA、机构协议、内核旁路或多活部署作已完成承诺。

## 开发约定

- 使用 Rust 2021 edition 和 Cargo workspace。
- 核心业务优先遵守 `core / adapter / infra` 边界。
- 新业务动作优先参考 `lib/example/entity`、`lib/example/use_case` 和现有 executor/outbound 模式。
- 不在生产代码新增无必要的 `unwrap()`、`expect()` 或没有安全说明的 `unsafe`。
- 修改核心行为时同步补充实体、用例或 BDD 测试。
- 运行格式化和 lint：

```bash
cargo fmt --all
cargo clippy -- -D warnings
```

更具体的本地开发约束见 [`CLAUDE.md`](CLAUDE.md) 和 [`AGENTS.md`](AGENTS.md)。
