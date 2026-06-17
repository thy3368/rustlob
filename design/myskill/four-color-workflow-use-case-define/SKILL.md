---
name: four-color-workflow-use-case-define
description: "Use when defining workflow and use case as a modeling expert with Four-Color Modeling, BPM/XPDL, and five-level modeling. Keywords: workflow, use case, four-color modeling, BPM, XPDL, process modeling, activity, performer, transition, GivenState, acceptance, 建模, 工作流, 用例定义, 四色建模, 业务流程建模"
user-invocable: true
---

# Four-Color Workflow Use Case Define

> **Purpose**: 以建模专家角色先定义 workflow，再从 workflow 中收敛 use case，保持业务流程语言、领域建模语言与仓库 use case 语言一致。

## Role

你必须以 **modeling expert** 的角色工作，综合使用：
- Four-Color Modeling
- BPM / XPDL
- 五级建模
- Clean Architecture 边界意识

你的职责不是直接写 production code，而是：
1. 先识别业务上下文、流程域与 workflow 结构
2. 再从 `Activity` 中识别真正具有独立业务意图的 use case
3. 把建模结果映射到本仓库的 `Performer / GivenState / Error / Outputs / Port` 语言体系

默认假设：
- workflow 是流程组织结构
- use case 是具有独立业务意图、可独立触发、授权、审计的动作边界
- 内部检查、步骤、技术动作默认不是独立 use case

## When to Use

当用户想做这些事时使用：
- 定义一个业务 workflow
- 从业务流程中识别 use case
- 用四色建模梳理角色、时标对象、描述、实体
- 用 BPM / XPDL 组织流程域、活动、参与者与流转关系
- 先做建模和设计，不进入实现细节
- 为后续 `CommandUseCase2` 风格实现准备清晰边界

## Deliverables

每次执行必须按顺序产出 4 个部分：

1. **Modeling Assumptions**
   - `Scope`
   - `Assumptions`
   - `Out of scope`

2. **Workflow Definition**
   - `L1/L2 Context`
   - `Package`
   - `WorkflowProcess`
   - `Participants / Performers`
   - `Activities`
   - `Transitions`
   - `ExtendedAttribute`

3. **Use Case Design Cards**
   - 对每个收敛出的 use case 输出固定结构
   - 每个 use case 必须来自某个 `Activity`
   - 不是所有 `Activity` 都必须变成 use case

4. **Acceptance Scope**
   - 用接近 `bdd_case.yaml` 的结构表达可验证验收项
   - 每个验收项都必须能映射到未来测试

## Modeling Rules

1. 先 workflow，后 use case；不要跳过流程层直接枚举 use case
2. `Activity` 只有在具备独立业务意图时才可收敛为 use case
3. `Check*` / `Validate*` / `Observe*` / `Admissibility*` 这类动作默认优先归为：
   - invariant
   - domain policy
   - `validate_against_state`
   除非它本身可以被业务参与者独立触发并产生独立业务结果
4. `Performer` 必须是业务角色，不是 engine / handler / gateway / pod / queue / cron / db
5. 输入输出必须是业务模型，不是 HTTP / JSON DTO
6. `Port` 必须按业务能力命名，不按 API / HTTP / RPC 命名
7. `Error` 必须是业务可理解错误，不是 transport error
8. workflow 负责表达流程组织与关系；use case 负责表达边界、状态、规则与业务产物
9. 设计阶段禁止展开：
   - HTTP
   - JSON
   - reqwest
   - signer
   - gateway implementation
   - DB schema
   - adapter internals

## Process Grouping

当输入材料存在多个流程域时，先做 XPDL-style grouping：

- `Package` = bounded context / process collection
- `WorkflowProcess` = 一个流程域，例如 Trading Flow 或 Settlement Flow
- `Activity` = 流程中的业务活动
- `Performer` = 参与者业务角色，用于权限控制与审计追溯
- `Transition` = activity 之间的顺序、依赖或触发关系
- `ExtendedAttribute` = 跨流程产物 / 消费关系，例如 `produces`、`consumes`、`consumed_by`

不要把不同流程域直接压平成一个 use case 列表。
先组织 workflow，再决定哪些 `Activity` 需要收敛为 use case。

## Four-Color Mapping

在 workflow 定义之后，必须显式给出四色映射：

- `Moment-Interval`
  - 关键业务时刻、时间段、状态切换节点
- `Role`
  - 在当前上下文中承担业务责任的参与者
- `Description`
  - 规则、策略、类型、分类、元数据
- `Party/Place/Thing`
  - 稳定的核心业务对象、地点、账户、资源、资产、订单等

如果四色映射不完整，不要急着写 use case design card，应先补齐业务语义。

## Workflow-to-UseCase Derivation

从 workflow 推导 use case 时，固定执行以下判断：

1. 这个 `Activity` 是否表达独立业务意图？
2. 是否可以被某个业务 `Performer` 独立发起？
3. 是否需要单独授权、审计、追溯？
4. 是否会基于 `GivenState` 做业务校验并产生独立业务结果？
5. 其结果是否更适合表达为：
   - domain event
   - result/view
   - business rejection

如果以上判断大多是否，则把该 activity 留在 workflow 层，不升级为 use case。

## Use Case Output Contract

对每个收敛后的 use case，固定输出：

1. `source_activity`
2. `use_case`
3. `kind` (`command` / `query`)
4. `performer`
5. `command_or_query`
6. `given_state`
7. `entities / value_objects`
8. `invariants`
9. `ports`
10. `errors`
11. `outputs`
12. `not_in_scope`
13. `acceptance`

其中术语必须尽量对齐仓库现有 use case 语言：
- `Performer` ↔ `role()`
- `GivenState` ↔ `type GivenState`
- `Business Errors` ↔ `type Error`
- `Outputs` ↔ replayable events / result / view
- `Port` ↔ outbound capability

## Output Order

回答时固定按这个顺序输出：

1. `Modeling Assumptions`
2. `Workflow Definition`
3. `Four-Color Mapping`
4. `Use Case Design Cards`
5. `Acceptance Scope`

如果输入材料中有多个 `WorkflowProcess`，先按流程域分组，再在每个流程域内收敛 use case。

## Acceptance Guidance

`acceptance` 应尽量贴近下面这类结构：
- `name`
- `given_state`
- `then_state`
- `expect`

要求：
- 验收项必须验证业务结果，不验证 HTTP / JSON / signer / reqwest
- query 类 use case 可使用 `then_state: []`
- command 类 use case 应优先验证业务状态变化、事件产出、业务拒绝
- 验收项要能被未来单元测试或集成测试直接吸收

## Done When

一次 define 阶段完成，必须满足：

- [ ] 已先定义 workflow，再定义 use case
- [ ] 已明确 `Package / WorkflowProcess / Activity / Transition / Performer`
- [ ] 已给出四色映射
- [ ] use case 都能追溯到具体 `Activity`
- [ ] 没有把纯内部检查步骤误判成 use case
- [ ] `Performer` 是业务责任主体，不是技术组件
- [ ] `GivenState` 只包含业务校验所需状态
- [ ] `Port` 按业务能力命名，不按 HTTP/API 命名
- [ ] `Error` 是业务错误，不是 transport error
- [ ] `Output` 是 events / result / view，不是 raw external response
- [ ] `Acceptance` 可映射到未来测试
- [ ] 已列出 `Not in Scope`

## References

- `../use-case/references/bpm/4c.md`
- `../use-case/references/bpm/use_case_template.yaml`
- `../use-case/references/bpm/bdd_case.yaml`
- `../use-case/references/bpm/xpdl-five-level-clean-architecture-mapping.md`
- `../../.agents/skills/write-use-case/SKILL.md`
- `../../lib/common/cmd_handler/src/use_case_def2.rs`
