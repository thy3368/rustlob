---
name: hyperliquid-use-case-define
description: "Use when defining a Hyperliquid use case before implementation. Keywords: define use case, use case design, command, query, port, invariant, GivenState, Events, Error, acceptance, Hyperliquid, trading, 用例定义, 设计用例, 端口, 不变量, 状态"
user-invocable: true
---

# Hyperliquid Use Case Define

> **Purpose**: 为 Hyperliquid 交易域定义 Use Case 的业务边界，不进入生产代码实现细节。

## When to Use

当用户想做这些事时使用：
- 定义一个新 use case
- 梳理 command / query 边界
- 明确 GivenState / Port / Error / Events
- 先做设计，不想陷入代码之海
- 产出 design card 或最小 Rust skeleton

## Deliverables

每次执行必须产出 3 个部分：

1. **Use Case Design Card**
   - 格式：`.md` 文件，顶部必须带 YAML frontmatter
   - 理由：正文用于人类设计评审，frontmatter 用于后续自动化检索与生成
   - 模板：`references/use-case-design-card-template.md`
   - frontmatter 至少包含：
     - `use_case`
     - `kind`
     - `status`
     - `performer`
     - `outputs`
     - `ports`
   - `performer` 表示参与者的业务角色，用于权限控制，并最终进入状态 / 事件 / 审计追溯；禁止使用 engine / handler / gateway / pod / queue 等技术实现名
   - 正文必须包含：
     - Use Case name
     - Performer
     - Command / Query
     - GivenState
     - Entities / Value Objects
     - Invariants
     - Output
     - Port
     - Business Errors
     - Not in Scope
     - Acceptance

2. **Minimal Rust Skeleton**
   - 格式：Rust code block
   - 模板：`references/minimal-rust-use-case-skeleton-template.md`
   - 只允许包含：
     - `UseCase`
     - `Command` or existing command type
     - `GivenState`
     - `Events / Result / View`
     - `Error`
     - `LoadPort`
     - optional `ReplyMapper`
     - optional `DomainEventPipeline`
   - 禁止包含：
     - HTTP
     - JSON
     - reqwest
     - signer
     - gateway implementation
     - database schema

3. **Acceptance Scope**
   - 必须列出 use case 层验收项
   - 每个验收项必须能映射到未来的单元测试

## Design Rules

1. Use Case 只表达业务动作，不表达协议细节
2. Use Case 必须表达可独立触发、授权、审计的业务意图；内部校验步骤不是独立 use case
3. `Check*` / `Validate*` / `Admissibility*` 这类动作默认应先归为 invariant、domain policy 或 `validate_against_state`，除非它本身是业务参与者可独立发起并产生独立业务结果的动作
4. 输入输出必须是业务模型，不是 HTTP / JSON DTO
5. Error 必须是业务可理解错误
6. 命令型 Use Case 明确：
   - `Command`
   - `GivenState`
   - `LoadPort`
   - `Events / Result / Reply`
   - `Error`
7. 设计阶段禁止展开：
   - reqwest
   - serde
   - signer
   - HTTP path
   - gateway impl
   - DB schema

## Process Grouping

当同一材料中出现多个流程域时，先用 XPDL-style process grouping 分组，再定义单个 use case：

- `Package` = bounded context / process collection
- `WorkflowProcess` = 一个流程域，例如 Trading Flow 或 Block Processing
- `Activity` = 一个 use case
- `Performer` = 参与者业务角色，用于权限控制与审计追溯
- `Transition` = use case 之间的顺序、依赖或触发关系
- `ExtendedAttribute` = 跨流程域的产物 / 消费关系，例如 `produces`、`consumes`、`consumed_by`

不要把不同流程域强行合并成一个 use case 列表。XPDL-style grouping 只表达流程组织；单个 use case 的边界仍必须使用 Use Case Design Card 固定。

## Output Contract

回答时固定给出：

1. 这个动作属于哪个 Use Case
2. `Performer`
3. `Command/Query`
4. `GivenState`
5. `Entities / Value Objects`
6. `Invariants`
7. `Port`
8. `Error`
9. `Output`
10. `Acceptance`

如果输入材料包含多个流程域，先给出 XPDL-style grouping：

1. `Package`
2. `WorkflowProcess`
3. `Activity -> Use Case`
4. `Performer`
5. `Transition`
6. `ExtendedAttribute` 跨流程产物 / 消费关系

然后再为每个收敛后的 use case 输出 Design Card。

如果设计收敛，再补一个最小 Rust skeleton，但 skeleton 只验证边界，不是完整实现。

## Done When

一次 define 阶段完成，必须满足：

- [ ] Use Case 名称是业务动作，不是协议动作
- [ ] Use Case 表达独立业务意图，不只是内部校验 / admissibility check / policy
- [ ] Performer 是业务责任主体，不是 HTTP handler / DB / queue / pod
- [ ] Command / Query 输入边界明确
- [ ] GivenState 只包含业务校验所需状态
- [ ] Port 按能力命名，不按 HTTP/API 命名
- [ ] Error 是业务错误，不是 transport error
- [ ] Output 是 Events / Result / View，不是 raw exchange response
- [ ] Acceptance 不测试 HTTP / JSON / signer / reqwest
- [ ] 金融数值使用 `Decimal` 或明确的 Value Object
- [ ] 已列出 Not in Scope

## References

- `references/hyperliquid-use-cases.md`
- `references/use-case-design-card-template.md`
- `references/minimal-rust-use-case-skeleton-template.md`
