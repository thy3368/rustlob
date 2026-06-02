---
name: clean-architecture
author: Tang Hong Yao
description: >
  整洁架构实战指南。当用户询问架构设计、代码分层、重构、依赖管理、技术选型、过度设计时使用。
  回答优先使用 core / adapter / infra 三层表达，并保持与 Clean Architecture 的标准语义对齐。
  务必在以下场景使用：分层架构、依赖反转、接口设计、组件划分、重构建议、技术选型评估、
  识别过度设计、代码解耦、边界划分、模块组织。
---

# Clean Architecture

> **Purpose**: 用 `core / adapter / infra` 的工程表达回答架构问题，同时保持 Clean Architecture 的依赖方向与边界规则正确。

## When to Use

当用户想做这些事时使用：

- 设计或评审代码分层
- 判断某个模块应归属哪个层次
- 分析依赖方向是否错误
- 识别 `service` / `repository` / `controller` / `infrastructure` 这类命名背后的真实架构角色
- 给出现有代码的最小重组建议
- 讨论微服务、Kafka、Redis、Kubernetes、SDK 抽象等复杂度是否必要

## Design Rules

1. 回答时优先使用三层工程表达：
    - `core`
        - `use_case` — business action, input as `command/query`
        - `entity` — core business rules and invariants
    - `adapter`
        - `inbound` — HTTP/CLI/Event/GUI input to use case
        - `outbound` — use case port to DB/API/SDK/presenter
    - `infra` — external frameworks, SDKs, DB drivers, runtimes, third-party tools

2. 同时保持与标准 Clean Architecture 的映射：
    - `core.entity` = `Entities`
    - `core.use_case` = `Use Cases`
    - `adapter` ≈ `Interface Adapters`
        - `adapter.inbound` ≈ Controllers
        - `adapter.outbound` ≈ Presenters / Gateways / Repository implementations
    - `infra` ≈ `Frameworks & Drivers`，包括 frameworks、SDKs、DB drivers、runtimes、third-party tools

3. `adapter` 负责在外部机制与 `core` 边界之间做适配：
    - `inbound` 把 HTTP / CLI / MQ / GUI 等输入转换为 `use_case` 的 command/query
    - `outbound` 实现 `use_case` 定义的 port，并把 core 输出转换给 DB / SDK / API / message broker 等外部系统

4. 必须显式维护依赖规则：
    - responsibility view / 职责视图:
      ```text
      core
        use_case, entity
 
      adapter
        inbound: translate external input into use_case command/query
        outbound: implement use_case ports and translate core output to infra/external systems
 
      infra
        frameworks, SDKs, drivers, runtimes, third-party tools
      ```
    - source dependency view:
      ```text
      inbound -> use_case -> entity
      outbound -> port <- use_case
      outbound -> infra
      ```
    - call flow view: `inbound -> use_case -> outbound -> infra`
    - `use_case -> entity`
    - `entity` 不依赖 `use_case`
    - `entity` 不知道 `command/query`
    - `use_case` 不依赖具体 DB/HTTP/SDK/ORM/framework
    - `inbound` 只负责把外部输入翻译到 `use_case`，不承载核心业务规则
    - `outbound` 只实现 `use_case` 定义的 port，不定义 port

5. 如果用户使用以下实现模式或传统分层名，不要直接当作主架构层接受；先翻译回真实架构角色，再给建议：
    - `controller`
    - `service`
    - `repository`
    - `model`
    - `business layer`
    - `data layer`
    - `infrastructure`

6. 当用户提出微服务、Kafka、Redis、Kubernetes、service mesh 或额外抽象层时，先判断是否真的需要：
    - `Question`: 问清楚要解决的真实问题
    - `Delete`: 能不用就不用
    - `Simplify`: 必须用时，先选最简单方案
    - `Accelerate`: 方案成立后再优化性能
    - `Automate`: 最后再自动化部署、运维和扩缩容


## 工程目录结构参考

本仓库的标准参考实现位于 `/Users/hongyaotang/src/rustlob/lib/example`。

使用本 skill 给目录建议前，必须先查看当前目录结构：

```bash
find lib/example -maxdepth 4 -type d | sort
find lib/example -maxdepth 4 -type f | sort | sed -n '1,160p'
```

以实际文件为准，不要依赖本文档中的过期快照。

使用该结构作为目录建议时：

- `lib/example/core` 对应 `core`
- `lib/example/core/src/use_case/<workflow>/<use_case>.rs` 对应具体业务用例，例如 `trading/spot/place_order.rs`
- `lib/example/inbound_adapter` 对应 `adapter.inbound`
- `lib/example/outbound_adapter` 对应 `adapter.outbound`
- `lib/example/app/composition_root` 负责组装 use case、adapter 和 infra

## Output Contract


每次执行必须按以下顺序组织回答：

1. `Layer Mapping`: `Core` / `Adapter` / `Infra`
    - `Core` 必须拆成 `use_case` 与 `entity`
    - `Adapter` 必须拆成 `inbound` 与 `outbound`
2. `Architecture Views`: responsibility view / source dependency view / call flow view
    - 如有违规，指出是 responsibility、dependency、还是 call flow 被混淆
3. `Violations`: 指出是否把实现模式误当成架构层；检查 core 是否直接依赖 DB / HTTP / SDK / ORM / framework 等外部技术
4. `Minimal restructuring advice`: 只给满足当前目标的最小调整建议
    - 如果问题涉及复杂度或技术选型，按 `Design Rules` 第 6 条先判断是否真的需要

## Testing Guidelines

当用户询问测试放在哪里时，使用以下 Clean Architecture 测试策略：

### 测试放置规则

- **Core / use_case**: 单元测试业务逻辑，mock 所有 outbound ports
    - 位置: `core/use_case/tests/` 或 `#[cfg(test)]` 内联
    - 依赖: 只依赖 entity，不依赖具体 infra

- **Core / entity**: 单元测试业务规则、状态转换、不变式
    - 位置: `core/entity/tests/` 或 `#[cfg(test)]` 内联
    - 特点: 纯逻辑，无外部依赖，最容易测试

- **Adapter / inbound**: 集成测试输入转换（HTTP/CLI → command）
    - 位置: `adapter/inbound/tests/`
    - 验证: 请求解析、参数校验、调用 use_case

- **Adapter / outbound**: 集成测试 port 实现（repository → DB/API）
    - 位置: `adapter/outbound/tests/`
    - 使用: 内存数据库或 testcontainers 验证真实交互

- **Infra**: 外部框架/SDK（Tokio、PostgreSQL、Redis、Kafka 等），不作为项目目录存在，也不写独立 infra 测试
    - 外部工具由各自社区测试
    - 通过 `adapter/outbound` 的集成测试间接验证
    - 通过 `tests/e2e/` 的全流程测试验证


### 测试目录结构示例

以 `lib/example` 当前结构为准，按以下位置放测试：

- `lib/example/core/tests/<workflow>_<use_case>_test.rs`: use_case 单元测试，mock outbound ports
- `lib/example/core/tests/entity_<entity>_test.rs`: entity 纯业务规则测试
- `lib/example/inbound_adapter/tests/<workflow>_<entrypoint>_to_command_test.rs`: inbound 输入转换测试
- `lib/example/outbound_adapter/tests/<workflow>_<port_impl>_test.rs`: outbound port 实现测试
- `lib/example/app/composition_root/tests/<workflow>_flow_test.rs`: E2E / composition test

### 测试原则

1. **单元测试**: 覆盖 core 层，无 I/O，快速执行 (< 10ms)
2. **集成测试**: 覆盖 adapter + infra 层，验证真实交互
3. **E2E 测试**: 覆盖完整流程，验证用户价值
4. **测试金字塔**: 单元 (70%) > 集成 (20%) > E2E (10%)

## Done When

一次回答完成，必须满足：

- [ ] 符合 `Output Contract`
- [ ] 没有违反 `Design Rules`
- [ ] 建议是最小重组，而不是大规模重写

## References

- `references/clean-architecture.md`
- `references/architecture-output-template.md`
- `references/musk-algorithm.md`
- `references/engineering-philosophy.md`
