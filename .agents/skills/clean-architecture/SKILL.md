---
name: clean-architecture
author: Tokenaissance (https://tokenaissance.com)
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

## Deliverables

每次执行必须产出这些部分：

1. **Layer Mapping**
   - 先用三层模型表达：`core`、`adapter`、`infra`
   - `core` 必须拆成：`use_case`、`entity`
   - `adapter` 必须拆成：`inbound`、`outbound`

2. **Architecture Views**
   - 明确给出三种视图：
     - role view:
       ```text
       core
         policies, use_case, entity

       adapter
         glue / translation layer between core and infra

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
   - 如有违规，必须指出是 role、dependency、还是 call flow 被混淆了

3. **Violation Analysis**
   - 指出当前命名或结构里哪些是架构层，哪些只是实现模式
   - 点出 inner policy 是否被 outer mechanism 污染

4. **Minimal Restructuring Advice**
   - 只给满足当前目标的最小调整建议
   - 如果用户在追求复杂方案，必须先做必要性质疑

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
   - `adapter` = `Interface Adapters`
   - `infra` = `Frameworks & Drivers` 的工程化表达

3. `adapter` 是 `core` 与 `infra` 的胶水层，负责在外部机制与核心策略之间做翻译与桥接。

4. 必须显式维护依赖规则：
   - role view:
     ```text
     core
       policies, use_case, entity

     adapter
       glue / translation layer between core and infra

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
   - `outbound adapter` 实现 `use_case` 定义的 port

5. 不要把这些实现模式当作主架构层名直接接受：
   - `controller`
   - `service`
   - `repository`
   - `model`
   - `business layer`
   - `data layer`
   - `infrastructure`

6. 如果用户使用这些词，先把它们翻译回真实架构角色，再给建议。

7. 当用户提出微服务、Kafka、Redis、Kubernetes、service mesh 或额外抽象层时，按这个顺序回答：
   - `Question`
   - `Delete`
   - `Simplify`
   - 只有在前面成立后，才讨论 `Accelerate` / `Automate`

## Output Contract

回答时固定给出：

1. `Core`
   - `use_case`
   - `entity`
2. `Adapter`
   - `inbound`
   - `outbound`
3. `Infra`
4. `Architecture views`（依次写 role view、source dependency view、call flow view）
5. `Violations`
6. `Minimal restructuring advice`

如果问题涉及复杂度或技术选型，再追加：

7. `Question`
8. `Delete`
9. `Simplify`
10. optional `Accelerate`
11. optional `Automate`

## Done When

一次回答完成，必须满足：

- [ ] 先给出 `core / adapter / infra` 映射
- [ ] `core` 明确拆成 `use_case` 与 `entity`
- [ ] `adapter` 明确拆成 `inbound` 与 `outbound`
- [ ] 明确写出 role view：`core` / `adapter` / `infra` 的位置与职责，不用箭头冒充依赖方向
- [ ] 明确写出 source dependency view：`inbound -> use_case -> entity` / `outbound -> port <- use_case` / `outbound -> infra`
- [ ] 明确写出 call flow view `inbound -> use_case -> outbound -> infra`
- [ ] 明确区分 role、dependency、call flow 三种概念
- [ ] 明确写出 `use_case -> entity`
- [ ] 明确说明 `entity` 不知道 `command/query`
- [ ] 明确说明 `inbound` 不承载核心业务规则
- [ ] 明确说明 `outbound` 只实现 port，不定义 port
- [ ] 指出当前结构中哪些是架构层，哪些只是实现模式
- [ ] 没有把 DB / SDK / framework 放进 `core`
- [ ] 没有让 `use_case` 直接依赖具体外部工具
- [ ] 如果建议复杂方案，已经先做必要性质疑与删减
- [ ] 建议是最小重组，而不是大规模重写

## References

- `references/clean-architecture.md`
- `references/architecture-output-template.md`
- `references/musk-algorithm.md`
- `references/engineering-philosophy.md`