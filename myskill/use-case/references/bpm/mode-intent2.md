 ## 建模即意图 MPDD 

软件开发里最容易被低估的一件事，是“建模”。

很多开发者把建模理解成画图、写文档、补流程图，甚至把它看成编码之前的形式主义。但真正有效的建模不是装饰，不是把代码换一种格式描述一遍，而是在实现之前回答一个更根本的问题：**这个系统到底承认哪些业务意图？**

所以，建模不是“画出系统长什么样”。

建模是声明：

- 谁可以做什么？
- 为了达成什么业务目标？
- 做之前需要哪些状态？
- 做的时候必须满足哪些不变量？
- 成功后产生什么业务结果？
- 哪些东西明确不属于这个业务边界？

一句话：**建模即意图。**

---

## 1. AI 时代，开发者最稀缺的不是写码速度，而是建模能力

AI 已经越来越擅长生成代码、补样板、接协议、写 DTO、补测试，甚至把一段业务描述快速翻译成一堆能运行的实现。

这带来一个很现实的变化：

> 开发者最值钱的部分，不再是“能不能把代码敲出来”，而是“能不能先把业务意图说清楚”。

因为代码一旦可以被大量生成，真正稀缺的就变成了：

- 系统到底要承认哪些业务动作
- 哪些动作应该成为独立 use case
- 哪些只是内部规则
- 哪些状态必须被追溯
- 哪些边界不能被 HTTP、JSON、queue、DB 这些实现细节污染

如果这些问题没有先被建模固定下来，AI 只会更快地把系统推入另一种混乱：

- 生成大量技术上成立、业务上混乱的代码
- 把 L5 实现细节误当成 L3 业务活动
- 把 handler、gateway、consumer、engine 当成系统的主角
- 把本来应该合并的动作拆碎成一堆“伪 use case”

所以在 AI 时代，真正应该被放大的不是“写码自动化”，而是：

**人把精力放在建模上，用准确建模约束 AI 的实现。**

也就是说：

```text
人负责定义意图
AI 负责展开实现
```

而不是：

```text
AI 先写一堆代码
人再回头猜这些代码想表达什么
```

---

## 2. 为什么开发者需要关心“意图”

代码天然会滑向实现细节。

一旦开始写代码，注意力很容易被这些东西吸走：

- HTTP path 怎么设计
- JSON 字段叫什么
- serde 怎么写
- reqwest 怎么调
- DB schema 怎么建
- signer 怎么签
- gateway 怎么封装

这些都重要，但它们不是业务本身。

如果还没说清楚业务意图就进入实现细节，系统会快速变成“协议驱动”或“框架驱动”：

```text
PostExchangeRequestHandler
SerializeHyperliquidJson
CallExchangeEndpoint
SignPayloadAndSend
```

这些名字能说明系统怎么工作，却不能说明业务为什么存在。

而业务真正关心的是：

```text
PlaceOrdersUseCase
MatchOrdersUseCase
ClearTradesUseCase
BuildCandidateBlockUseCase
CommitCertifiedBlockUseCase
```

这些名字表达的是意图。

这就是建模对开发者的价值：

> 在代码出现之前，先让业务意图变得可见、可讨论、可验证。

---

## 3. 人重点设计和评审 L1、L2、L3；L4、L5 让 AI 干

如果用五级建模来看，AI 时代最实用的分工不是“人写核心代码，AI 写边角料”，而是：

```text
L1 业务价值链 / 业务方向        -> 人重点判断
L2 流程域 / bounded context      -> 人重点划分
L3 业务活动 / Use Case           -> 人重点定义与评审
L4 工作流 / 任务 / 流转关系       -> AI 辅助展开，人复核
L5 操作步骤 / 协议 / 实现细节     -> AI 主力生成，人验收
```

这是因为：

### L1 决定系统“为什么存在”

例如在 DEX 场景里：

- Trading Flow
- Block Execution
- HotStuff Consensus

这些不是代码目录名，而是系统的业务方向与流程域切分依据。

如果 L1/L2 判断错了，后面所有生成出来的代码都会在错误边界上越写越多。

### L2 决定“哪些流程属于同一个域”

例如你现在在 `operating/dex/design/flow` 下做的拆分就很典型：

- `dex_trading_flow.xpdl`
- `dex_block_execution_flow.xpdl`
- `dex_hotstuff_consensus_flow.xpdl`
- `dex_end_to_end_flow_overview.xpdl`

这一步不是“画图”，而是在决定：

- 交易流程域是什么
- 区块执行流程域是什么
- 共识流程域是什么
- 它们之间怎么衔接，但为什么不能混成一个大流程

这种边界判断必须由人主导，因为它本质上是在做业务语义切分。

### L3 决定“哪些动作才配成为 use case”

例如：

```text
PlaceOrdersUseCase            ✅
MatchOrdersUseCase            ✅
ClearTradesUseCase            ✅
BuildCandidateBlockUseCase    ✅
VoteOnProposalUseCase         ✅
CheckOrderAdmissibility       ❌
InsertIntoMempool             ❌
SerializeJson                 ❌
```

这里真正难的不是把名字写出来，而是判断：

- 它是不是独立业务意图
- 它是不是业务角色可以独立触发的动作
- 它是不是需要独立授权与审计
- 它是不是有独立业务结果

这一步如果不由人来评审，AI 很容易把内部规则、协议步骤、技术机制误抬升成 use case。

### L4 / L5 更适合让 AI 展开

一旦 L1/L2/L3 已经被人定义清楚，AI 就非常适合继续展开：

- `Transition` 怎么接
- `Participant / Performer` 怎么补齐
- `GivenState` 怎么列
- `ExtendedAttribute` 怎么组织
- Port / DTO / mapper / adapter 怎么写
- 测试样板怎么铺
- 文档和 skeleton 怎么补

换句话说：

> 人负责决定系统该承认什么；AI 负责把这些决定系统化展开。

---

## 4. Use Case 是意图的最小业务单元

一个好的 use case，不是一个函数，也不是一个 API endpoint。

它应该表达一个可以独立触发、授权、审计的业务意图。

例如在交易系统里：

```text
PlaceOrdersUseCase
```

它的意图不是“调用下单 API”，而是：

> 交易者提交一组订单意图，系统根据账户、市场、权限和风险状态决定是否接受，并产出可审计的业务结果。

这就自然引出 use case 的核心要素：

- `Performer`：谁以什么业务角色执行这个动作
- `Command / Query`：输入边界是什么
- `GivenState`：执行前需要加载哪些状态
- `Entities / Value Objects`：涉及哪些领域对象
- `Invariants`：必须满足哪些业务不变量
- `Port`：依赖哪些外部能力
- `Error`：有哪些业务可理解错误
- `Output`：成功后产出什么事件、结果或视图
- `Acceptance`：如何只验证 use case 本身

这些不是“文档字段”。

它们是在逼问业务意图。

---

## 5. Performer 不是技术组件，而是业务角色

建模时最常见的错误，是把技术实现名当成业务角色。

例如：

```yaml
performer: OrderCheckingEngine
```

这个名字看似合理，但它不具备业务语义。

`OrderCheckingEngine` 是实现组件，不是参与者的业务角色。它不能很好地回答：

- 谁被授权执行这个动作？
- 谁应该进入审计记录？
- 状态追溯时应该追到哪个业务身份？

更好的表达是：

```yaml
performer: Trader
```

或者在撮合、清算、区块执行、共识场景中：

```yaml
performer: Matching
performer: Clearing
performer: BlockProposer
performer: Validator
performer: ConsensusLeader
performer: ValidatorQuorum
```

这里的 `Performer` 表示参与者的业务角色，用于权限控制，并最终落到状态、事件、审计追溯里。

所以要避免：

```text
OrderCheckingEngine
HTTPHandler
Gateway
QueueConsumer
ValidatorPod
DBWorker
```

这些属于实现层，不属于业务意图层。

---

## 6. 不是所有业务规则都应该变成 Use Case

另一个常见错误，是把内部校验步骤误建模成独立 use case。

例如：

```text
CheckOrderAdmissibilityUseCase
```

它听起来像业务，但通常不应该单独成为 use case。

原因是：业务方真正要达成的目标不是“执行一次订单准入校验”，而是“下单”。

订单准入校验应该是 `PlaceOrdersUseCase` 内部的：

- invariant
- domain policy
- `pre_check_command`
- `validate_against_state`

而不是独立 use case。

判断一个动作是否应该成为 use case，可以问六个问题：

1. 它是否有独立业务意图？
2. 它是否能被业务参与者独立触发？
3. 它是否需要独立授权？
4. 它是否需要独立审计？
5. 它是否产生独立业务结果？
6. 它是否不是单纯的内部校验、协议搬运或基础设施机制？

如果答案是否定的，它大概率不该成为 use case。

所以：

```text
PlaceOrdersUseCase        ✅
MatchOrdersUseCase        ✅
ClearTradesUseCase        ✅
BuildCandidateBlockUseCase ✅
VoteOnProposalUseCase     ✅
CheckOrderAdmissibility   ❌ 更适合做 policy / invariant
LoadOraclePrice           ❌ 更适合做 Port 背后的能力
BroadcastRequest          ❌ 网络传播机制
InsertIntoMempool         ❌ 基础设施排队机制
```

---

## 7. XPDL 表达流程组织，Use Case Card 固定意图边界

当业务材料里出现多个流程域时，不应该把所有动作平铺成一个 use case 列表。

更好的做法是先用 XPDL-style grouping 做流程分组。

可以这样理解：

```text
Package = bounded context / process collection
WorkflowProcess = 一个流程域
Activity = 一个 use case
Performer = 参与者业务角色
Transition = use case 之间的顺序或依赖
ExtendedAttribute = 跨流程域的产物 / 消费关系
```

参考 `operating/dex/design/flow` 下面现在的结构：

```text
dex_trading_flow.xpdl
dex_block_execution_flow.xpdl
dex_hotstuff_consensus_flow.xpdl
dex_end_to_end_flow_overview.xpdl
```

它实际上表达了一个很清楚的 AI 时代建模顺序：

### 第一步：先按流程域拆开

```text
Trading Flow Domain
Block Execution Domain
Consensus Domain
```

### 第二步：每个流程域内部再识别 L3 use case

例如 Trading Flow：

```text
PlaceOrdersUseCase
  -> MatchOrdersUseCase
  -> ClearTradesUseCase
```

Block Execution：

```text
BuildCandidateBlockUseCase
  -> ValidateBlockExecutionUseCase
```

HotStuff Consensus：

```text
EnterNewViewUseCase
  -> ProposeConsensusBlockUseCase
  -> VoteOnProposalUseCase
  -> FormQuorumCertificateUseCase
  -> CommitCertifiedBlockUseCase
```

### 第三步：最后再用 package-level overview 连接起来

```text
TradingFlowProcess
  -> BlockExecutionProcess
  -> HotStuffConsensusProcess
```

这说明：

- XPDL 负责“这些意图如何被组织成流程”
- Use Case Design Card 负责“单个意图的边界是什么”
- AI 可以继续根据这些边界补 L4 / L5 细节

---

## 8. AI 最适合干什么：把 L4/L5 展开，而不是替你决定 L1/L2/L3

当流程域和 use case 已经清楚以后，AI 很适合继续做这些事情：

- 根据流程关系生成 XPDL 文件
- 根据 use case card 生成最小 Rust skeleton
- 展开 `GivenState / Events / Error / Port`
- 生成 adapter、mapper、reply、pipeline 样板
- 展开测试用例框架
- 生成文档、对照表、trace matrix
- 把 package-level overview 与子流程文件互相对齐

但 AI 不应该替代人的部分是：

- 业务价值链判断
- bounded context 划分
- use case 粒度判断
- performer 的业务语义定义
- 哪些状态必须被追溯
- 哪些规则是 invariant，哪些只是实现细节

这是一个边界问题，不是一个“谁更聪明”的问题。

AI 擅长展开，
人必须先决定展开什么。

---

## 9. 一个最实用的协作模式：人定模，AI铺码

如果把上面的原则压缩成一条工作法，可以写成：

```text
人定模，AI铺码
```

更细一点是：

```text
人：
- 定 L1 业务方向
- 划 L2 流程域
- 定 L3 Use Case
- 审 Performer / GivenState / Output / Error / Invariant
- 决定哪些不属于该边界

AI：
- 展开 L4 Transition / Participant / Workflow
- 展开 L5 Port / Adapter / DTO / Infra glue
- 生成 skeleton / tests / mappers / reply / pipeline
- 校对跨文件结构一致性
```

这种分工的好处是：

1. **避免陷入码海**
   不会一开始就被 API、JSON、DB、queue 细节拖走。

2. **避免 AI 把错误东西放大**
   不会因为边界没定好，就生成一堆“技术上能跑、业务上失真”的代码。

3. **把人的注意力放在最值钱的地方**
   也就是意图、边界、授权、审计、状态追溯这些真正决定系统形状的地方。

---

## 10. 一个实用的建模检查清单

当你准备把一个动作建模为 use case 时，先检查：

- [ ] 名称是否是业务动作，而不是协议动作？
- [ ] 是否表达可独立触发、授权、审计的业务意图？
- [ ] `Performer` 是否是业务角色，而不是 engine / handler / gateway / pod？
- [ ] `Command / Query` 输入边界是否明确？
- [ ] `GivenState` 是否只包含业务校验所需状态？
- [ ] `Invariants` 是否清楚？
- [ ] `Port` 是否按能力命名，而不是按 HTTP/API 命名？
- [ ] `Error` 是否是业务错误，而不是 transport error？
- [ ] `Output` 是否是 Events / Result / View，而不是 raw response？
- [ ] 内部校验是否没有被误拆成独立 use case？
- [ ] 如果材料包含多个流程域，是否先做了 XPDL-style grouping？
- [ ] 是否已经明确哪些部分由人评审，哪些部分交给 AI 展开？

这份清单的目的不是让设计变复杂，而是防止系统过早滑入实现泥潭。

---

## 11. 结语：模型不是图，模型是给 AI 和代码的意图约束

在没有 AI 的时候，建模已经很重要。

在 AI 时代，建模更重要了。

因为代码生成能力越强，越需要先把“什么才是对的业务意图”说清楚。

所以建模不是编码之前的附属动作，而是整个实现链条的约束起点：

```text
业务价值
  -> 流程域
  -> Use Case
  -> 工作流关系
  -> 协议与实现细节
  -> 代码生成与落地
```

如果前半段不清楚，后半段只会更快地产生混乱。

如果前半段清楚，AI 就会成为很强的放大器。

所以对软件开发者来说，最重要的不是“要不要用 AI 写代码”，而是：

**有没有先把该由人负责的 L1、L2、L3 建模清楚。**

**有没有用清楚的意图去约束 AI 展开 L4、L5。**

这也是这句话在 AI 时代更完整的含义：

**建模即意图。**

**而准确的建模，就是给 AI 和代码下达正确的业务指令。**

这两个流程会衔接，但不是同一个流程域。

可以用 XPDL 表达：

```xml
<WorkflowProcess Id="TradingFlowProcess" Name="Trading Flow Process">
  <Activities>
    <Activity Id="PlaceOrdersUseCase" Name="PlaceOrdersUseCase">
      <Performer>Trader</Performer>
    </Activity>
    <Activity Id="MatchOrdersUseCase" Name="MatchOrdersUseCase">
      <Performer>Matching</Performer>
    </Activity>
    <Activity Id="ClearTradesUseCase" Name="ClearTradesUseCase">
      <Performer>Clearing</Performer>
    </Activity>
  </Activities>
  <Transitions>
    <Transition Id="T1" From="PlaceOrdersUseCase" To="MatchOrdersUseCase"/>
    <Transition Id="T2" From="MatchOrdersUseCase" To="ClearTradesUseCase"/>
  </Transitions>
</WorkflowProcess>
```

但是 XPDL 只解决“流程如何组织”。

每个 Activity 对应的 use case 仍然需要自己的 Design Card 来固定业务边界。

---

## 6. 从五级建模看：不要过早掉进 L5

参考五级建模，业务建模通常可以粗略分成：

```text
L1 业务价值链
L2 流程域 / 运作模式
L3 业务活动
L4 工作流 / 任务
L5 操作步骤 / 系统细节
```

开发者最容易犯的错误，是还没完成 L2/L3，就直接掉到 L5。

例如一开始就讨论：

- API 怎么转发
- JSON 怎么序列化
- 签名怎么做
- 数据库字段怎么拆
- HTTP client 怎么封装

这些都属于偏 L5 的实现细节。

而 use case 设计阶段更应该停在 L3/L4：

- 这个流程域是什么？
- 这个 Activity 是否真的是 use case？
- Performer 是哪个业务角色？
- GivenState 需要哪些实体？
- Invariants 是什么？
- Output 是什么业务结果？

这也是为什么设计阶段适合产出：

- `.md + YAML frontmatter` 的 Use Case Design Card
- 最小 Rust skeleton

而不是直接写完整生产代码。

---

## 7. 四色建模补足业务语义

四色建模可以帮助开发者避免“只有流程，没有领域对象”。

它提醒我们，一个业务动作通常会涉及四类东西：

| 原型 | 含义 | 在 use case 中的体现 |
|---|---|---|
| Moment-Interval | 业务关键时刻或事件 | Events / Result |
| Role | 参与者角色 | Performer |
| Description | 规则、分类、策略 | Invariants / Policy / Metadata |
| Party/Place/Thing | 稳定实体 | Entities / GivenState |

以 `PlaceOrdersUseCase` 为例：

- Role：`Trader`
- Party/Thing：`Account`、`Order`、`Market`、`Position`
- Description：`FeeSchedule`、`ProductMetadata`、`RiskPolicy`
- Moment-Interval：`OrderAcceptedEvent`、`OrderRejectedError`

这样建模会比“一个 handler 接收 JSON 然后发请求”更接近业务世界。

---

## 8. 建模结果应该能进入代码，但不被代码绑架

好的模型不是停留在白板上。

它应该能自然落到代码结构：

```rust
pub trait CommandUseCase {
    type Command;
    type GivenState;
    type Events;
    type Error;
    type LoadPort;

    fn performer(&self) -> &'static str;
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;
    fn load_state(&self, cmd: &Self::Command, port: &Self::LoadPort) -> Result<Self::GivenState, Self::Error>;
    fn validate_against_state(&self, cmd: &Self::Command, state: &Self::GivenState) -> Result<(), Self::Error>;
    fn then(&self, cmd: &Self::Command, state: Self::GivenState) -> Result<Self::Events, Self::Error>;
}
```

这里每个类型都对应设计阶段的问题：

- `Command`：意图输入
- `GivenState`：执行前状态
- `LoadPort`：状态加载能力
- `validate_against_state`：业务不变量
- `Events`：成功后的业务事实
- `Error`：业务拒绝原因
- `performer`：参与者业务角色

这就是“建模即意图”的落地方式：

> 模型先表达意图，代码再承载意图。

而不是反过来，让 HTTP、JSON、DB、queue 来定义业务。

---

## 9. 一个实用的建模检查清单

当你准备把一个动作建模为 use case 时，先检查：

- [ ] 名称是否是业务动作，而不是协议动作？
- [ ] 是否表达可独立触发、授权、审计的业务意图？
- [ ] `Performer` 是否是业务角色，而不是 engine / handler / gateway / pod？
- [ ] `Command / Query` 输入边界是否明确？
- [ ] `GivenState` 是否只包含业务校验所需状态？
- [ ] `Invariants` 是否清楚？
- [ ] `Port` 是否按能力命名，而不是按 HTTP/API 命名？
- [ ] `Error` 是否是业务错误，而不是 transport error？
- [ ] `Output` 是否是 Events / Result / View，而不是 raw response？
- [ ] Acceptance 是否只测试 use case 本身，不测试 HTTP / JSON / signer / reqwest？
- [ ] 内部校验是否没有被误拆成独立 use case？
- [ ] 如果材料包含多个流程域，是否先做了 XPDL-style grouping？

这份清单的目的不是让设计变复杂，而是防止系统过早滑入实现泥潭。

---

## 10. 结语：模型不是图，模型是选择

建模不是为了“看起来专业”。

建模是在做选择：

- 哪些动作是业务意图？
- 哪些只是内部规则？
- 哪些属于流程域？
- 哪些属于技术机制？
- 哪些状态必须被追溯？
- 哪些角色必须被授权？

这些选择会决定后面的代码形状。

如果模型里只有 API、handler、queue、DB，那么代码最终也只会长成一堆技术管道。

如果模型里先有 Performer、Use Case、GivenState、Invariant、Event，那么代码才会围绕业务意图生长。

所以对软件开发者来说，建模不是编码之前的额外工作。

**建模就是把系统的业务意图固定下来。**

**建模即意图。**


==
todo  1 主要解决了sdd 的6个系统性问题

todo 2 


适用范围
体能评估
MPDD 是一项工程投资。下表根据不同场景对其投资回报率进行评级，从强烈推荐（5 星）到不适用（1 星）。

等级	设想	笔记
★★★★★	规模化、标准化的交付	需要长期维护的高重复性业务逻辑（例如，构建许多类似的 API，自动化核心业务工作流程）。
★★★★★	高度合规性和严格的限制	必须遵守法规、安全标准或严格的架构规则的环境（例如，金融核心系统、多渠道/多客户端部署）。
★★★★☆	团队协作和可审计性	多人交付，变更必须完全可追溯和可审查，贯穿整个流程。
★★★★☆	跨领域一致性工作	复杂的重构，其中逻辑必须在多个微服务或不同语言之间保持紧密同步。
★★☆☆☆	消防热修复	“止血”式生产修复方案，旨在解决速度比架构规范更重要的问题。
★★☆☆☆	探索性尖峰	如果目标是快速验证一个想法，而不是交付生产质量的软件，那么 SPDD 的治理开销就得不偿失了。
★★☆☆☆	一次性脚本	一次性数据清理或临时脚本，适用于 SPDD 前期成本相对于其价值过高的情况。
★☆☆☆☆	背景黑洞	当领域定义不明确且业务规则不清楚时，就无法为模型设定有意义的边界。
★☆☆☆☆	纯粹的创意/视觉作品	以品味和审美而非逻辑为驱动的任务（例如，用户界面视觉探索、营销文案）。

投资回报率

益处	影响	速度	你将获得什么
决定论	高的	即时	将逻辑编码到精确的规范中，可以显著减少幻觉和“创造性”解释。
可追溯性	高的	即时	每一个有意义的变化都可以追溯到结构化的提示，从而形成完整的审核闭环。
更快的审核	高的	短期	代码“最终”更接近团队标准，因此代码审查侧重于逻辑和设计，而不是格式和清理。
可解释性	中高	逐渐地	意图和行为在自然语言层面是可见的，降低了理解和维持的认知负荷。
更安全的进化	高的	长期	明确的界限和分步实施使有针对性的改变风险更低，也更容易迭代。



前期投资

区域	障碍	自然	需要什么
思维转变	高的	持续培训	团队必须适应“先设计后编码”的模式。
前期资深专家	中高	按功能	能够将业务规则转化为清晰的抽象概念和设计约束的工程师。
自动化工具	中等的	基础设施搭建	如果没有自动化，SPDD 的吞吐量会达到瓶颈，并且难以保持提示的一致性。openspdd将本文中的工作流程（从分析和结构化的 REASONS 提示到代码和可选的测试支持）作为可重复的 CLI 步骤运行，从而使工件保持版本控制和可审查性，而不是滞留在聊天记录中。规模较大的组织可能仍然会在其之上叠加一个知识平台，以便大规模地管理和重用资产。

