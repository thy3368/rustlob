# MI 扩展判定与校准

本文件是 [`moment_interval_definition.md`](./moment_interval_definition.md) 的扩展参考，只回答三类问题：

- 一个候选对象是不是 `MI`
- 它在业务事实层更接近哪类 `MI`
- 哪些事实必须 append-only 留痕

基础定义与最小判定标准以 [`moment_interval_definition.md`](./moment_interval_definition.md) 为准；本文件不重复改写基础定义，也不单独放宽或改写其门槛。

当任务涉及以下主题时，应在读完基础定义后继续读本文件：

- `MI 识别` 的深化判断
- `main_mi` / `secondary_mis`
- 资金侧 `MI`
- 审计凭证型 `MI`
- 哪些事实必须 append-only
- `MI` 与 `command` / `state` / `entity` / `aggregate` / `description` 的边界

当任务涉及 `group_boundary`、`business_truth_center`、`final_settled_fact`、端到端链路闭合或何时拆成独立 `use case group` 时，继续阅读 [`use_case_group_boundary.md`](./use_case_group_boundary.md)。这些问题不由本文件承载。

## 适用范围与非目标

本文件用于 `design-use-case-group`、`review-mi-causal-chain` 及相关 skill 的概念校准，帮助判断：

- 某个对象是否应作为独立 `MI`
- 某个 `MI` 是主事实、派生事实、资金事实还是审计凭证事实
- 某条业务事实是否必须 append-only 留痕
- 某个候选对象为什么不是 `MI`

本文件不是：

- 数据库建模规范
- 事件命名规范总册
- 事件溯源框架说明
- 所有领域对象的通用分类手册
- `use case group` 边界判断手册
- `MI -> entity` 命名校准手册

不要把 `MI` 泛化成“一切需要留痕的对象”。

## 扩展判定视角

基础定义已经说明：时标对象是“业务上值得被单独记住、可被追溯、并沿时间发生合法演化的事实性对象”。

本文件只补充几个容易混淆的扩展判断视角。

### 独立身份不是否定派生关系

很多 `secondary_mis` 都是由 `main_mi` 推进后派生出来的。

这里说“有自己的业务身份”，不是否定派生关系，而是强调：

- 它不能只作为别的对象附属字段存在
- 它必须能被单独引用
- 它必须能被单独留痕
- 它必须能在争议或回放时被单独复原

如果某个东西只是“主对象里顺手多记的一个字段变化”，而不能独立复原，它通常还不够格成为独立 `MI`。

### 审计要求是强化判断，不是基础门槛替代物

基础定义已经足够判断大多数 `MI`。

当某个对象承担独立争议处理、仲裁、财务核对、责任归因或合规留痕责任时，应额外问：

1. 它是否必须以独立凭证存在？
2. 它是否必须 append-only 保存？
3. 争议时是否必须把它单独拿出来复原真相？

如果这些答案大多是“会”，它更接近本文件所说的“审计凭证型 `MI`”。

## 与 `command` 的边界

`command` 是“有人请求系统做什么”。

`MI` 是“业务认可为已经成立、值得被记住，并会继续演化或被持续引用的事实对象”。

不要把“请求”误当成“事实”。

典型关系只有三种：

1. 一个 `command` 创建一个新的 `MI`
2. 一个 `command` 推进一个已有 `MI`
3. 一个 `command` 推进 `main_mi`，同时派生一个 `secondary_mi`

如果某个东西只在命令入口有意义，进入业务事实层后并不作为独立真相存在，它就不是 `MI`。

## 与 `state` 的边界

`state` 是某个业务对象在当前时刻的状态表达。

`MI` 是承载业务事实及其合法演化的对象本身。

不要把“当前状态值”当成 `MI`。

例如：

- `Order` 可能是 `MI`
- `OrderStatus = Working` 不是 `MI`
- `available_balance = 100` 不是 `MI`
- `FundHold` 可能是 `MI`
- `frozen_amount = 100` 只是某个状态字段

如果一个对象只能表达“现在是多少”，而不能表达“什么事实成立过、如何推进到这里”，它通常不是 `MI`。

## 与 `entity` 的边界

`entity` 是代码/领域建模里的对象形态。

`MI` 是业务真相层面的对象类型判断。

二者经常重叠，但不等价：

- 一个 `MI` 往往会落成 `entity` 或聚合内核心对象
- 不是所有 `entity` 都是 `MI`
- 一些 `Description`、policy carrier、snapshot object 也可能是 `entity`，但不是 `MI`

先判断“它是不是值得独立记住的业务事实”，再决定它在代码里怎么落。

本文件不提供 `MI -> entity` 命名校准规则。

## 与 `aggregate` 的边界

`aggregate` 解决的是一致性边界。

`MI` 解决的是业务事实边界与审计边界。

不要因为某个对象需要事务一致性，就自动把它当成 `MI`；
也不要因为某个对象是 `MI`，就默认它必须单独成为一个聚合。

判断顺序应是：

1. 先识别 `MI`
2. 再判断哪些 `MI` 或状态对象应落在同一聚合内保持一致
3. 最后再判断哪些跨聚合协调必须留在 `use case`

## 与 `description` 的边界

`description` 是规则、分类、能力、约束、阈值、模板、市场配置等描述性对象。

它解释业务如何运行，但它本身通常不是“发生过的事实”。

例如：

- 风控参数表
- 杠杆档位配置
- 费用规则
- liquidation policy
- ADL 排序规则

这些都可能非常重要，但通常是 `Description`，不是 `MI`。

## `main_mi`

`main_mi` 是一组业务事实判断中最主要、最能承载合法演化空间的 `MI`。

它通常负责回答：

- 这一组业务动作究竟围绕谁的合法演化展开
- 谁是业务事实链的起点锚
- 谁定义了主要生命周期

例子：

- 交易下单链中，`Order` 常是 `main_mi`
- 清算链中，`LiquidationEvent` 常是 `main_mi`
- 资金保留链中，`FundHold` 常是 `main_mi`

`main_mi` 是否成为某个 `use case group` 的 `business_truth_center`，由 [`use_case_group_boundary.md`](./use_case_group_boundary.md) 判断。

## `secondary_mis`

`secondary_mis` 是由 `main_mi` 推进过程中派生出来、但自身也具备独立业务事实意义的对象。

`secondary_mis` 可以由 `main_mi` 推进派生，但派生后必须具备可单独引用、单独留痕、单独复原的业务身份。

它通常：

- 被 `main_mi` 引发
- 有独立身份
- 有独立审计价值
- 不一定与 `main_mi` 属于同一个 `use case group`

例子：

- `Order` 推进派生 `Trade`
- `LiquidationEvent` 推进派生 `BankruptcyShortfallEvent`
- `ADLRound` 推进派生多个 `ADLDeleveragingRecord`

## 资金侧 `MI`

资金侧 `MI` 是专门承载资金占用、释放、划拨、结算、扣减、回充等事实的 `MI`。

判断重点不是“它是不是动了余额”，而是：

- 是否是一笔独立可追溯的资金事实
- 是否需要回答“为何产生、何时成立、因谁而起、何时关闭”
- 是否需要 append-only 历史，而不是只看余额快照

典型例子：

- `FundHold`
- `InsuranceFundDrawdownEvent`
- `InsuranceFundTransfer`
- `SettlementTransfer`

`available_balance`、`frozen_amount` 这类数值本身不是资金侧 `MI`。

## 审计凭证型 `MI`

审计凭证型 `MI` 指那些即使生命周期不长，也必须以“独立凭证”形式存在的事实对象。

它们往往用于：

- 外部仲裁
- 财务核对
- 合规留痕
- 回放与责任归因

它们的核心价值不在“活得久”，而在“不能丢、不能改、必须能被引用”。

典型例子：

- `LiquidationExecution`
- `BankruptcyShortfallEvent`
- `InsuranceFundDrawdownEvent`
- `ADLDeleveragingRecord`

## 哪些事实必须 append-only 留痕

优先把以下对象看作 append-only 候选：

- 会成为后续因果链引用锚点的事实
- 外部争议时必须单独举证的事实
- 资金出入账、占用、释放、拨付、回充等事实
- 批次执行、清算执行、减仓明细这类不可重写凭证

反过来，以下通常不该被误包装成 append-only `MI`：

- `CheckRisk`
- `ValidateInput`
- `PersistOrder`
- `LoadState`
- `PublishEvent`
- `MapReply`
- `matching_step`
- `db_row`
- `available_balance`
- `frozen_amount`（仅为数值字段时）
- `price`
- `quantity`
- `status enum value`

## 交易例子：`Order / Trade / FundHold`

- `Order` 常是 `main_mi`
- `Trade` 常是 `secondary_mi`
- `FundHold` 常是资金侧 `MI`
- `Order.status`、`Order.remaining_qty`、`Account.available_balance` 通常只是状态表达或派生视图

不要因为 `Trade` 重要，就自动让它与 `Order` 共用同一个 group center；group 边界另见 [`use_case_group_boundary.md`](./use_case_group_boundary.md)。

## 清算例子：`Liquidation / Shortfall / IF / ADL`

在清算链路中：

- `LiquidationEvent` 常是 `main_mi`
- `LiquidationExecution / Fill` 常是审计凭证型 `MI`
- `BankruptcyShortfallEvent` 常是必须独立留痕的 `secondary_mi`
- `InsuranceFundDrawdownEvent` 常是资金侧或审计凭证型 `MI`
- `ADLRound`、`ADLDeleveragingRecord` 常是必须独立留痕的 `secondary_mi`

以下通常不是 `MI`：

- `available_balance`
- `CheckRisk`
- `run_adl_step_3`

## 资金冻结例子：`Freeze / FundHold / Balance`

`Freeze` 这个词不能直接判定。

如果它只是订单提交流程里的一个内部动作，例如“下单时先冻结资产”，它常只是主链内部推进。

但若业务明确存在：

- 冻结编号
- 独立释放/消耗/过期
- 可争议、可审计、可查询
- 可被多个业务动作引用

那么更合理的建模常是把它提升为 `FundHold` 之类的资金侧 `MI`，而不是停留在“freeze 这个动作”。

`Balance` 或 `available_balance` 通常只是状态快照或派生值，不是 `MI`。

## 输出约束

当回答 `MI` 建模问题，且建议某个对象应升格为独立 `MI` 或审计事实对象时，结果里应显式给出：

- 它为什么是 `MI`
- 它为什么不是 `command / step / 状态字段 / 技术节点`
- 它在事实层的角色：`main_mi`、`secondary_mi`、资金侧 `MI` 或审计凭证型 `MI`
- 若相关，哪些事实必须 append-only 留痕

不要在本文件的职责内输出 `group_boundary`、`business_truth_center` 或 `final_settled_fact` 的完整判断；这些内容应转到 [`use_case_group_boundary.md`](./use_case_group_boundary.md)。
