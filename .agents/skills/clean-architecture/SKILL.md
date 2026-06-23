---
name: clean-architecture
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
- 轻量评审 `use case` 的层次归属、边界纯度、职责拆分
- 判断某个模块应归属哪个层次
- 分析依赖方向是否错误
- 识别 `service` / `repository` / `controller` / `infrastructure` 这类命名背后的真实架构角色
- 给出现有代码的最小重组建议
- 讨论微服务、Kafka、Redis、Kubernetes、SDK 抽象等复杂度是否必要

开始回答前，先读取共享约束文件：

- `../shared/use_case_entity_constraints.md`
- `../shared/entity_four_color_classification.md` when the task involves classifying a domain object/entity, deciding whether something should be an `entity`, `aggregate root`, `value object`, `role object`, or `policy/description`
- `../shared/use_case_entity_aggregate_boundary.md` when the question involves `use case` vs `entity`, `behavior method`, `helper/query method`, `aggregate root`, `state machine`, or whether an action deserves an independent `use case`
- `../shared/use_case_review_scorecard.md` when the task is to score or lightly review a `use case`

## Design Rules

1. 回答时优先使用三层工程表达：
    - `core`
        - `use_case` — business action, input as `command/query`
        - `entity` — core business rules, invariants, and domain-semantic methods reusable by multiple `use_case`
          - classify the object first with its `four_color_archetype`
          - `behavior method` — 单实体状态迁移或同一聚合内部协调
          - `helper/query method` — 业务判断、业务查询、派生计算
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

4. `use case 组` 的定义：
    - `use case 组` 是 `core/use_case` 下的业务分组边界，用于组织同一业务线中的多个相关 `use_case`
    - `use case 组` 不是单个 `use_case`
    - `use case 组` 也不是跨用例编排代码；跨用例协作属于更上层的 `process` / 应用组装层
    - 一个 `use case 组` 下的多个 `use_case` 通常共享相近的领域语义、实体集合、状态装载方式，目录上体现为 `core/src/use_case/<use_case_group>/...`
    - 例如本仓库中的 `funding`、`trading`，以及更细一级的 `trading/spot`

5. 必须显式维护依赖规则：
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
    - `use_case` 之间不互相调用；一个业务动作必须收敛在单一 `use_case` 中，跨用例协作只能通过上层编排（如 composition root / process）完成，不能在一个 `use_case` 内直接调用另一个 `use_case`
    - `use_case` 与 `entity` 是多对一关系：多个 `use_case` 可以复用同一个 `entity`，但 `entity` 不反向绑定某个特定 `use_case`
    - 单实体状态迁移属于 `core.entity`
    - 同一聚合内部协调仍属于 `core.entity`
    - 跨聚合业务动作协调属于 `core.use_case`
    - `entity` 必须包含有领域语义的方法，而不是只有字段和 getter/setter；这些方法承载可复用的业务规则，供多个 `use_case` 复用
    - `entity` 不依赖 `use_case`
    - `entity` 不知道 `command/query`
    - `use_case` 不依赖具体 DB/HTTP/SDK/ORM/framework
    - `inbound` 只负责把外部输入翻译到 `use_case`，不承载核心业务规则
    - `outbound` 只实现 `use_case` 定义的 port，不定义 port

6. 如果用户使用以下实现模式或传统分层名，不要直接当作主架构层接受；先翻译回真实架构角色，再给建议：
    - `controller`
    - `service`
    - `repository`
    - `model`
    - `business layer`
    - `data layer`
    - `infrastructure`

7. 当用户提出微服务、Kafka、Redis、Kubernetes、service mesh 或额外抽象层时，先判断是否真的需要：
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
- `lib/example/core/src/use_case/<use_case_group>/<use_case>.rs` 对应具体业务用例，例如 `trading/spot/place_order.rs`
- `lib/example/inbound_adapter` 对应 `adapter.inbound`
- `lib/example/outbound_adapter` 对应 `adapter.outbound`
## Output Contract


每次执行必须按以下顺序组织回答：

1. `Layer Mapping`: `Core` / `Adapter` / `Infra`
    - `Core` 必须拆成 `use_case` 与 `entity`
    - `Adapter` 必须拆成 `inbound` 与 `outbound`
2. `Architecture Views`: responsibility view / source dependency view / call flow view
    - 如有违规，指出是 responsibility、dependency、还是 call flow 被混淆
3. `Violations`: 指出是否把实现模式误当成架构层；检查 core 是否直接依赖 DB / HTTP / SDK / ORM / framework 等外部技术
    - 必查：是否存在 `use_case` 互调
    - 必查：是否把跨聚合编排误塞进 `entity` / `aggregate`
    - 必查：是否把 `entity` 设计成只服务单一 `use_case` 的“私有流程对象”，从而破坏 `use_case` 与 `entity` 的多对一关系
    - 必查：`entity` 是否退化为贫血数据结构，只剩字段搬运，没有可被 `use_case` 复用的领域语义方法
    - 必查：是否把 `entity method` 全做成贫血 `helper`，缺少真正业务演化方法
    - 必查：是否把 `query/helper` 假装成 `use case` 或 `service`
    - 必查：是否把 `Description` 错放进 `entity`
    - 必查：是否把 `Role` 过度实体化
    - 必查：是否把真正的 `Moment-Interval` 压成 DTO 或 `description`
4. `Minimal restructuring advice`: 只给满足当前目标的最小调整建议
    - 如果问题涉及复杂度或技术选型，按 `Design Rules` 第 6 条先判断是否真的需要
    - 如果问题涉及 `entity` 建模边界，先用 `four_color_archetype` 说明对象归类，再给最小调整建议

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
- `../shared/use_case_entity_constraints.md`
