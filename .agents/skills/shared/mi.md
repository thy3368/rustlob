# MI 扩展判定与校准

本文件是 [`moment_interval_definition.md`](./moment_interval_definition.md) 的扩展参考，专门处理 `MI` 识别深化、主次 `MI`、资金侧 `MI`、审计链、`settled fact`、`append-only` 留痕，以及 `MI -> entity` 的命名校准。

基础定义与最小判定标准以 [`moment_interval_definition.md`](./moment_interval_definition.md) 为准；本文件不重复改写基础定义，也不单独放宽或改写其门槛。

当任务涉及以下主题时，应在读完基础定义后继续读本文件：

- `MI 识别` 的深化判断
- `主 MI / 次级 MI`
- `资金侧 MI`
- `审计凭证型 MI`
- `端到端 MI 链`
- `settled fact`
- `业务事实留痕`
- 哪些事实必须 append-only
- `MI -> entity` 后推荐业务名该怎么收敛

## 适用范围与非目标

本文件用于 `design-use-case-group` 及相关 skill 的概念校准，帮助判断：

- 某个对象是否应作为独立 `MI`
- 某条链上哪些事实必须单独留痕
- 某个 `MI` 落成 `entity` 后该如何命名
- 某个 group 是否真正走到了边界内的最终落定事实

本文件不是：

- 数据库建模规范
- 事件命名规范总册
- 事件溯源框架说明
- 所有领域对象的通用分类手册

不要把 `MI` 泛化成“一切需要留痕的对象”。

## 扩展判定视角

基础定义已经说明：时标对象是“业务上值得被单独记住、可被追溯、并沿时间发生合法演化的事实性对象”。

本文件只补充几个容易混淆的扩展判断视角。

### 独立身份不是否定派生关系

很多次级 `MI` 都是由主 `MI` 推进后派生出来的。

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
3. 一个 `command` 推进主 `MI`，同时派生一个次级 `MI`

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

## `MI` 落成 `entity` 后的命名规则

`MI` 首先是业务事实判断，其次才常常落成 `entity`。

一旦某个 `MI` 被确认要落成 `entity`、聚合内核心对象，或独立审计事实对象，命名必须优先表达“业务上到底是什么对象”，而不是“系统对它做了什么动作”。

命名顺序必须是：

1. 先判定它是否为 `MI`
2. 再判定它是否落成 `entity` / aggregate root / 聚合内核心对象
3. 最后再命名

不要倒过来因为一个名字“看起来像 entity”就把它误判成 `MI`。

### 命名原则

1. 把 `MI` 当成“值得独立命名的业务事实对象候选”，不是 command、step、技术动作或状态字段的别名。
2. 命名优先使用业务主语、事实对象名、稳定名词。
3. 不要用命令名、实现动作名、流程步骤名、技术事件名给 `MI` 命名。
4. 如果对象表达“持续生命周期中的事实对象”，优先使用裸业务名或稳定名词。
5. 如果对象表达“必须独立留痕的凭证/事件事实”，允许使用 `Event` / `Record` / `Execution` / `Transfer` / `Adjustment` 等业务后缀。
6. 这些后缀必须表达事实形态，不得只是技术事件包装。

### 正反例

- `FundHold` 优于 `Freeze`
- `Order` 优于 `PersistOrder`
- `BankruptcyShortfallEvent` 优于 `CalculateShortfall`
- `ADLDeleveragingRecord` 优于 `AdlStepItem`
- `SettlementTransfer` 优于 `SettleBalance`
- `InsuranceFundTransfer` 或 `InsuranceFundAdjustment` 优于 `InsuranceFundTx(SURPLUS | REPLENISH)`
- `LiquidationExecution` 可以成立；`RunLiquidationStep2` 不行
- `AvailableBalanceChanged` 这类状态字段式名字通常不应作为推荐业务名

如果需要表达资金向保险基金回充或从保险基金拨付，优先保留业务对象名，再把差异收在文内字段说明里，例如：

- `InsuranceFundTransfer`
- `reason = Surplus | Replenish`

### 后缀使用约束

以下后缀只有在确实表达业务事实形态时才成立：

- `Event`: 某个必须留痕、可引用、可追责的已发生事实
- `Record`: 某个必须保留的审计凭证或结果留痕
- `Execution`: 某个已执行并可独立核对的业务执行事实
- `Transfer`: 某个资金或资产划拨事实
- `Adjustment`: 某个需要独立记账和归因的余额或基金调整事实

如果只是为了让名字“像个 entity”，不要机械追加这些后缀。

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

## 主 `MI`

`主 MI` 是某个 `use case group` 的 `business_truth_center`。

它负责回答：

- 这一组业务动作究竟围绕谁的合法演化展开
- 谁是这条业务真相链的起点锚
- 谁定义了这一组 use case 的主要生命周期

例子：

- 交易下单链中，`Order` 常是主 `MI`
- 清算链中，`LiquidationEvent` 常是主 `MI`
- 资金保留链中，`FundHold` 常是主 `MI`

## 次级 `MI`

`次级 MI` 是由主 `MI` 推进过程中派生出来、但自身也具备独立业务事实意义的对象。

次级 `MI` 可以由主 `MI` 推进派生，但派生后必须具备可单独引用、单独留痕、单独复原的业务身份。

它通常：

- 被主 `MI` 引发
- 有独立身份
- 有独立审计价值
- 不一定与主 `MI` 属于同一个 group

例子：

- `Order` 推进派生 `Trade`
- `LiquidationEvent` 推进派生 `BankruptcyShortfallEvent`
- `ADLRound` 推进派生多个 `ADLDeleveragingRecord`

## 资金侧 `MI`

`资金侧 MI` 是专门承载资金占用、释放、划拨、结算、扣减、回充等事实的 `MI`。

判断重点不是“它是不是动了余额”，而是：

- 是否是一笔独立可追溯的资金事实
- 是否需要回答“为何产生、何时成立、因谁而起、何时关闭”
- 是否需要 append-only 历史，而不是只看余额快照

典型例子：

- `FundHold`
- `InsuranceFundDrawdownEvent`
- `InsuranceFundTransfer`
- `SettlementTransfer`

其中 `InsuranceFundTransfer` 或 `InsuranceFundAdjustment` 更适合作为业务对象名；若需表达差异，应用 `reason`、`variant` 等字段说明 `Surplus` 或 `Replenish`。

`available_balance`、`frozen_amount` 这类数值本身不是资金侧 `MI`。

## 审计凭证型 `MI`

`审计凭证型 MI` 指那些即使生命周期不长，也必须以“独立凭证”形式存在的事实对象。

它们往往用于：

- 争议处理
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

## 端到端 `MI` 链设计法

设计一条业务链时，不要只看 API 顺序，也不要只看 handler 调用图。

应该按以下顺序建模：

1. 先声明 `group_boundary`
2. 找到该边界内的 `main_mi`
3. 写出它从成立到闭环的合法推进路径
4. 标出推进过程中派生出的 `secondary_mis`
5. 标出哪些事实必须 append-only 留痕
6. 标出这条边界内的 `final_settled_fact`
7. 反查哪些对象只是步骤、字段、检查、状态值，不应冒充 `MI`

### 具体问题模板

对任意候选对象，都用下面的问题逼近：

1. 它是不是这一组业务动作真正围绕的主语？
2. 如果不是主语，它是否仍然构成必须独立留痕的次级事实？
3. 如果没有它，争议时是否无法复原完整因果链？
4. 它是一次独立凭证，还是主 `MI` 内部的一步状态推进？
5. 它是否触发了新的授权、失败语义、审计语义、资金语义？
6. 它是否应该拆出独立 `use case group`，还是仍留在主链内部？

### 何时拆成独立 `use case group`

当一个次级对象同时满足以下大部分条件时，通常应考虑拆组：

1. 它有独立身份
2. 它有独立生命周期
3. 它有独立授权或失败语义
4. 它有独立审计意义
5. 业务会围绕它定义多种合法动作

如果它只是主 `MI` 内部推进的一步，或只是为了完成主链闭环而产生的一次性子事实，则通常不必单独升格为 group center。

## 最终 `settled fact` 的判定法

`final_settled_fact` 不是固定名词，也不等于“最后一次数据库更新”。

它是：

- 在声明的 `group_boundary` 内
- 这条业务真相链真正完成闭环后
- 对业务来说可被认定为“已经落定”的最终事实

判断方法：

1. 问“如果链条停在这里，业务会不会认为这件事已经真正办完？”
2. 问“此后还有没有必须继续推进才能成立的核心事实？”
3. 问“外部争议/对账时，是否可以把这里当作该边界内的最终结论？”

例子：

- 仅下单边界里，`Order accepted/rejected/cancelled/filled` 可能已是 settled fact
- `Order -> Trade -> Settlement` 的完整交易履约边界里，`Trade` 不是最终 settled fact，`Settlement` 才更接近
- 清算边界里，如果声明覆盖到“缺口被 IF 或 ADL 处理完成”，那么 `LiquidationExecution` 不是最终 settled fact，`shortfall closed / adl resolved` 才是

不要把“技术流程暂时结束”误判成 settled fact。

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

## 交易例子：`Order / Trade / Settlement`

- `Order` 常是主 `MI`
- `Trade` 常是次级 `MI`
- 如果边界只到撮合完成，`Trade` 可能已是该边界内 settled fact
- 如果边界声明覆盖完整履约，`Settlement` 才更接近最终 settled fact

不要因为 `Trade` 重要，就自动让它与 `Order` 共用同一个 group center。

## 清算例子：`Liquidation / Shortfall / IF / ADL`

这是一条典型的端到端 `MI` 链：

1. `LiquidationEvent` 成立
2. 派生 `LiquidationExecution / Fill`
3. 若执行价劣于破产价，派生 `BankruptcyShortfallEvent`
4. 若需基金兜底，产生 `InsuranceFundDrawdownEvent`
5. 若基金不足，产生 `ADLRound`
6. 批次内产生多个 `ADLDeleveragingRecord`
7. 必要时以 `InsuranceFundTransfer` 或 `InsuranceFundAdjustment` 收尾，`reason` 可为 `Replenish`

这里：

- `LiquidationEvent` 常是主 `MI`
- `BankruptcyShortfallEvent`、`ADLRound`、`ADLDeleveragingRecord` 常是必须独立留痕的次级或凭证型 `MI`
- `available_balance` 不是 `MI`
- `CheckRisk` 不是 `MI`
- `run_adl_step_3` 不是 `MI`

如果声明的 group boundary 覆盖“缺口被彻底处理完成”，则最终 settled fact 更接近：

- `shortfall fully covered`
- 或 `adl resolved / exhausted with explicit result`

而不是“刚触发强平”。

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

## 审计留痕校验问句

做完建模后，用这组问句自检：

1. 任意争议发生时，能否从主 `MI` 一路追到最终 settled fact？
2. 链上的关键因果事实，是否都以 append-only 凭证存在？
3. 是否有哪个对象只是状态字段，却被错误包装成 `MI`？
4. 是否有哪个必须独立留痕的资金事实，被偷懒折叠进余额快照？
5. 是否有哪个次级事实其实已具备独立身份与审计意义，却还被埋在主对象内部？

如果这些问题答不稳，说明 `MI` 边界大概率还没建对。

## 输出约束

当回答 `MI` 建模问题，且建议某个对象应升格为独立 `MI`、`entity` 或审计事实对象时，结果里应显式给出：

- `推荐业务名`
- 它为什么是 `MI`
- 它为什么不是 `command / step / 状态字段 / 技术节点`
- 它在链上的角色：`main_mi`、`secondary_mi`、资金侧 `MI` 或审计凭证型 `MI`
- 若相关，再补充它与 `final_settled_fact` 的关系
