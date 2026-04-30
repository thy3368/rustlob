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

## Testing Guidelines

当用户询问测试放在哪里时，使用以下 Clean Architecture 测试策略：

### 测试分层映射

```
┌─────────────────────────────────────────────────────────────┐
│  E2E 测试                                                   │
│  - 完整用户场景，跨所有层                                    │
│  - 位于: tests/e2e/ 或 e2e/                                  │
├─────────────────────────────────────────────────────────────┤
│  集成测试                                                   │
│  - adapter 与 infra 层交互                                   │
│  - 位于: 与被测代码同级 tests/ 目录                          │
├─────────────────────────────────────────────────────────────┤
│  单元测试                                                   │
│  - core 层: use_case + entity                               │
│  - 位于: 与被测代码同级 tests/ 或内联 #[test]               │
└─────────────────────────────────────────────────────────────┘
```

### 各层测试放置规则

**1. Core 层测试 (单元测试)**
- **use_case**: 测试业务逻辑，mock 所有 outbound ports
  - 位置: `core/use_case/tests/` 或 `#[cfg(test)]` 内联
  - 依赖: 只依赖 entity，不依赖具体 infra
  
- **entity**: 测试业务规则、状态转换、不变式
  - 位置: `core/entity/tests/` 或 `#[cfg(test)]` 内联
  - 特点: 纯逻辑，无外部依赖，最容易测试

**2. Adapter 层测试 (集成测试)**
- **inbound**: 测试输入转换（HTTP/CLI → command）
  - 位置: `adapter/inbound/tests/`
  - 验证: 请求解析、参数校验、调用 use_case
  
- **outbound**: 测试 port 实现（repository → DB/API）
  - 位置: `adapter/outbound/tests/`
  - 使用: 内存数据库或 testcontainers 验证真实交互

**3. Infra 层 (外部工具，无独立测试)**
- **Infra 是外部框架/SDK**（Tokio、PostgreSQL、Redis、Kafka 等），**不**作为项目目录存在
- 不编写针对 infra 的独立测试，因为：
  - 外部工具由各自社区测试
  - 通过 `adapter/outbound` 的集成测试间接验证
  - 通过 `tests/e2e/` 的全流程测试验证

### 测试目录结构示例

```
project/
├── core/                               # 业务核心 (无外部依赖)
│   ├── use_case/
│   │   ├── place_order.rs
│   │   └── tests/                      # use_case 单元测试
│   │       ├── place_order_test.rs     # mock outbound ports
│   │       └── mod.rs
│   └── entity/
│       ├── order.rs
│       └── tests/                      # entity 单元测试
│           └── order_test.rs           # 纯业务规则测试
├── adapter/                            # 适配器层 (连接 core 与外部)
│   ├── inbound/
│   │   ├── http/
│   │   │   ├── order_controller.rs
│   │   │   └── tests/                  # inbound 集成测试
│   │   │       └── order_api_test.rs   # 验证 HTTP → command 转换
│   │   └── cli/
│   └── outbound/
│       ├── repository/
│       │   ├── order_repo.rs           # 实现 repository port
│       │   └── tests/                  # outbound 集成测试
│       │       └── order_repo_test.rs  # 测试与真实 DB 交互
│       └── api_client/
│           ├── external_api_client.rs  # 实现 api_client port
│           └── tests/
│               └── api_client_test.rs  # 测试与外部 API 交互
└── tests/                              # E2E 测试 (顶层)
    └── e2e/
        └── order_flow_test.rs          # 完整用户场景
```

**说明：**
- `core`: 纯业务逻辑，无外部依赖，单元测试全覆盖
- `adapter`: 连接核心业务与外部工具，集成测试验证适配器正确性
- `infra`: 外部框架/工具（Tokio、PostgreSQL、Redis 等），**不**在项目中作为目录存在，通过 adapter 的集成测试间接验证

### 测试原则

1. **单元测试**: 覆盖 core 层，无 I/O，快速执行 (< 10ms)
2. **集成测试**: 覆盖 adapter + infra 层，验证真实交互
3. **E2E 测试**: 覆盖完整流程，验证用户价值
4. **测试金字塔**: 单元 (70%) > 集成 (20%) > E2E (10%)

### 依赖规则（测试也遵守）

```text
test_use_case → use_case → entity
test_use_case mocks outbound ports

test_outbound → outbound adapter → real/test infra
test_inbound  → inbound adapter → mock use_case

test_e2e → full stack → real infra
```

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